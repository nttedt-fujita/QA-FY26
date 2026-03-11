//! SQLiteリポジトリ実装
//!
//! 統合DB設計に基づく検査結果の保存・取得
//! 参照: db/schema.sql

use rusqlite::{Connection, params};
use super::{
    Device, IndoorInspection, InspectionItemName, InspectionItemResult,
    Lot, RepositoryError, Verdict,
};

/// SQLiteリポジトリ
pub struct SqliteRepository {
    conn: Connection,
}

impl SqliteRepository {
    /// 新しいリポジトリを作成（ファイルパス指定）
    pub fn new(path: &str) -> Result<Self, RepositoryError> {
        let conn = Connection::open(path)
            .map_err(|e| RepositoryError::Connection(e.to_string()))?;

        let repo = Self { conn };
        repo.init_tables()?;
        Ok(repo)
    }

    /// インメモリDBで作成（テスト用）
    pub fn in_memory() -> Result<Self, RepositoryError> {
        let conn = Connection::open_in_memory()
            .map_err(|e| RepositoryError::Connection(e.to_string()))?;

        let repo = Self { conn };
        repo.init_tables()?;
        Ok(repo)
    }

    /// テーブル初期化（屋内検査関連のみ）
    fn init_tables(&self) -> Result<(), RepositoryError> {
        // 外部キー制約を有効化
        self.conn.execute("PRAGMA foreign_keys = ON", [])
            .map_err(|e| RepositoryError::Sql(e.to_string()))?;

        // lots
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS lots (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                lot_number TEXT NOT NULL UNIQUE,
                expected_rate_ms INTEGER,
                expected_port_in_proto TEXT,
                expected_port_out_proto TEXT,
                memo TEXT,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                updated_at TEXT NOT NULL DEFAULT (datetime('now'))
            )",
            [],
        ).map_err(|e| RepositoryError::Sql(e.to_string()))?;

        // devices
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS devices (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                lot_id INTEGER,
                serial_number TEXT NOT NULL UNIQUE,
                model_number TEXT,
                fw_version TEXT,
                memo TEXT,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                updated_at TEXT NOT NULL DEFAULT (datetime('now')),
                FOREIGN KEY (lot_id) REFERENCES lots(id)
            )",
            [],
        ).map_err(|e| RepositoryError::Sql(e.to_string()))?;

        // indoor_inspections
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS indoor_inspections (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                device_id INTEGER NOT NULL,
                inspected_at TEXT NOT NULL,
                overall_result TEXT,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                updated_at TEXT NOT NULL DEFAULT (datetime('now')),
                FOREIGN KEY (device_id) REFERENCES devices(id)
            )",
            [],
        ).map_err(|e| RepositoryError::Sql(e.to_string()))?;

        // inspection_item_results
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS inspection_item_results (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                inspection_id INTEGER NOT NULL,
                item_name TEXT NOT NULL,
                verdict TEXT NOT NULL,
                actual_value TEXT,
                expected_value TEXT,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                FOREIGN KEY (inspection_id) REFERENCES indoor_inspections(id)
            )",
            [],
        ).map_err(|e| RepositoryError::Sql(e.to_string()))?;

        Ok(())
    }

    // ===========================================
    // Lot CRUD
    // ===========================================

    /// ロットを保存
    pub fn insert_lot(&self, lot: &Lot) -> Result<i64, RepositoryError> {
        self.conn.execute(
            "INSERT INTO lots (lot_number, expected_rate_ms, expected_port_in_proto, expected_port_out_proto, memo)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                lot.lot_number,
                lot.expected_rate_ms,
                lot.expected_port_in_proto,
                lot.expected_port_out_proto,
                lot.memo,
            ],
        ).map_err(|e| RepositoryError::Sql(e.to_string()))?;

        Ok(self.conn.last_insert_rowid())
    }

    /// ロットをIDで取得
    pub fn get_lot(&self, id: i64) -> Result<Lot, RepositoryError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, lot_number, expected_rate_ms, expected_port_in_proto, expected_port_out_proto, memo
             FROM lots WHERE id = ?1"
        ).map_err(|e| RepositoryError::Sql(e.to_string()))?;

        let lot = stmt.query_row([id], |row| {
            Ok(Lot {
                id: Some(row.get(0)?),
                lot_number: row.get(1)?,
                expected_rate_ms: row.get(2)?,
                expected_port_in_proto: row.get(3)?,
                expected_port_out_proto: row.get(4)?,
                memo: row.get(5)?,
            })
        }).map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => RepositoryError::NotFound(format!("Lot id={}", id)),
            _ => RepositoryError::Sql(e.to_string()),
        })?;

        Ok(lot)
    }

    /// 全ロットを取得
    pub fn get_all_lots(&self) -> Result<Vec<Lot>, RepositoryError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, lot_number, expected_rate_ms, expected_port_in_proto, expected_port_out_proto, memo
             FROM lots ORDER BY id DESC"
        ).map_err(|e| RepositoryError::Sql(e.to_string()))?;

        let rows = stmt.query_map([], |row| {
            Ok(Lot {
                id: Some(row.get(0)?),
                lot_number: row.get(1)?,
                expected_rate_ms: row.get(2)?,
                expected_port_in_proto: row.get(3)?,
                expected_port_out_proto: row.get(4)?,
                memo: row.get(5)?,
            })
        }).map_err(|e| RepositoryError::Sql(e.to_string()))?;

        let mut result = Vec::new();
        for row in rows {
            result.push(row.map_err(|e| RepositoryError::Sql(e.to_string()))?);
        }
        Ok(result)
    }

    /// ロットを更新
    pub fn update_lot(
        &self,
        id: i64,
        expected_rate_ms: Option<i32>,
        expected_port_in_proto: Option<&str>,
        expected_port_out_proto: Option<&str>,
        memo: Option<&str>,
    ) -> Result<(), RepositoryError> {
        let affected = self.conn.execute(
            "UPDATE lots SET
                expected_rate_ms = ?1,
                expected_port_in_proto = ?2,
                expected_port_out_proto = ?3,
                memo = ?4,
                updated_at = datetime('now')
             WHERE id = ?5",
            params![
                expected_rate_ms,
                expected_port_in_proto,
                expected_port_out_proto,
                memo,
                id,
            ],
        ).map_err(|e| RepositoryError::Sql(e.to_string()))?;

        if affected == 0 {
            return Err(RepositoryError::NotFound(format!("Lot id={}", id)));
        }
        Ok(())
    }

    // ===========================================
    // Device CRUD
    // ===========================================

    /// 装置を保存
    pub fn insert_device(&self, device: &Device) -> Result<i64, RepositoryError> {
        self.conn.execute(
            "INSERT INTO devices (lot_id, serial_number, model_number, fw_version, memo)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                device.lot_id,
                device.serial_number,
                device.model_number,
                device.fw_version,
                device.memo,
            ],
        ).map_err(|e| RepositoryError::Sql(e.to_string()))?;

        Ok(self.conn.last_insert_rowid())
    }

    /// 装置をIDで取得
    pub fn get_device(&self, id: i64) -> Result<Device, RepositoryError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, lot_id, serial_number, model_number, fw_version, memo
             FROM devices WHERE id = ?1"
        ).map_err(|e| RepositoryError::Sql(e.to_string()))?;

        let device = stmt.query_row([id], |row| {
            Ok(Device {
                id: Some(row.get(0)?),
                lot_id: row.get(1)?,
                serial_number: row.get(2)?,
                model_number: row.get(3)?,
                fw_version: row.get(4)?,
                memo: row.get(5)?,
            })
        }).map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => RepositoryError::NotFound(format!("Device id={}", id)),
            _ => RepositoryError::Sql(e.to_string()),
        })?;

        Ok(device)
    }

    /// シリアル番号で装置を取得
    pub fn get_device_by_serial(&self, serial: &str) -> Result<Device, RepositoryError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, lot_id, serial_number, model_number, fw_version, memo
             FROM devices WHERE serial_number = ?1"
        ).map_err(|e| RepositoryError::Sql(e.to_string()))?;

        let device = stmt.query_row([serial], |row| {
            Ok(Device {
                id: Some(row.get(0)?),
                lot_id: row.get(1)?,
                serial_number: row.get(2)?,
                model_number: row.get(3)?,
                fw_version: row.get(4)?,
                memo: row.get(5)?,
            })
        }).map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => RepositoryError::NotFound(format!("Device serial={}", serial)),
            _ => RepositoryError::Sql(e.to_string()),
        })?;

        Ok(device)
    }

    /// ロットIDで装置一覧を取得
    pub fn get_devices_by_lot(&self, lot_id: i64) -> Result<Vec<Device>, RepositoryError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, lot_id, serial_number, model_number, fw_version, memo
             FROM devices WHERE lot_id = ?1 ORDER BY id"
        ).map_err(|e| RepositoryError::Sql(e.to_string()))?;

        let rows = stmt.query_map([lot_id], |row| {
            Ok(Device {
                id: Some(row.get(0)?),
                lot_id: row.get(1)?,
                serial_number: row.get(2)?,
                model_number: row.get(3)?,
                fw_version: row.get(4)?,
                memo: row.get(5)?,
            })
        }).map_err(|e| RepositoryError::Sql(e.to_string()))?;

        let mut result = Vec::new();
        for row in rows {
            result.push(row.map_err(|e| RepositoryError::Sql(e.to_string()))?);
        }
        Ok(result)
    }

    /// 装置のFWバージョンを更新
    pub fn update_device_fw_version(&self, device_id: i64, fw_version: &str) -> Result<(), RepositoryError> {
        let affected = self.conn.execute(
            "UPDATE devices SET fw_version = ?1, updated_at = datetime('now') WHERE id = ?2",
            params![fw_version, device_id],
        ).map_err(|e| RepositoryError::Sql(e.to_string()))?;

        if affected == 0 {
            return Err(RepositoryError::NotFound(format!("Device id={}", device_id)));
        }
        Ok(())
    }

    // ===========================================
    // IndoorInspection CRUD
    // ===========================================

    /// 屋内検査を保存
    pub fn insert_inspection(&self, inspection: &IndoorInspection) -> Result<i64, RepositoryError> {
        self.conn.execute(
            "INSERT INTO indoor_inspections (device_id, inspected_at, overall_result)
             VALUES (?1, ?2, ?3)",
            params![
                inspection.device_id,
                inspection.inspected_at,
                inspection.overall_result,
            ],
        ).map_err(|e| RepositoryError::Sql(e.to_string()))?;

        Ok(self.conn.last_insert_rowid())
    }

    /// 屋内検査をIDで取得
    pub fn get_inspection(&self, id: i64) -> Result<IndoorInspection, RepositoryError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, device_id, inspected_at, overall_result
             FROM indoor_inspections WHERE id = ?1"
        ).map_err(|e| RepositoryError::Sql(e.to_string()))?;

        let inspection = stmt.query_row([id], |row| {
            Ok(IndoorInspection {
                id: Some(row.get(0)?),
                device_id: row.get(1)?,
                inspected_at: row.get(2)?,
                overall_result: row.get(3)?,
            })
        }).map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => RepositoryError::NotFound(format!("Inspection id={}", id)),
            _ => RepositoryError::Sql(e.to_string()),
        })?;

        Ok(inspection)
    }

    /// 装置IDで屋内検査一覧を取得
    pub fn get_inspections_by_device(&self, device_id: i64) -> Result<Vec<IndoorInspection>, RepositoryError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, device_id, inspected_at, overall_result
             FROM indoor_inspections WHERE device_id = ?1 ORDER BY inspected_at DESC"
        ).map_err(|e| RepositoryError::Sql(e.to_string()))?;

        let rows = stmt.query_map([device_id], |row| {
            Ok(IndoorInspection {
                id: Some(row.get(0)?),
                device_id: row.get(1)?,
                inspected_at: row.get(2)?,
                overall_result: row.get(3)?,
            })
        }).map_err(|e| RepositoryError::Sql(e.to_string()))?;

        let mut result = Vec::new();
        for row in rows {
            result.push(row.map_err(|e| RepositoryError::Sql(e.to_string()))?);
        }
        Ok(result)
    }

    /// 屋内検査の総合判定を更新
    pub fn update_inspection_result(&self, inspection_id: i64, result: &str) -> Result<(), RepositoryError> {
        let affected = self.conn.execute(
            "UPDATE indoor_inspections SET overall_result = ?1, updated_at = datetime('now') WHERE id = ?2",
            params![result, inspection_id],
        ).map_err(|e| RepositoryError::Sql(e.to_string()))?;

        if affected == 0 {
            return Err(RepositoryError::NotFound(format!("Inspection id={}", inspection_id)));
        }
        Ok(())
    }

    // ===========================================
    // InspectionItemResult CRUD
    // ===========================================

    /// 検査項目結果を保存
    pub fn insert_item_result(&self, result: &InspectionItemResult) -> Result<i64, RepositoryError> {
        self.conn.execute(
            "INSERT INTO inspection_item_results (inspection_id, item_name, verdict, actual_value, expected_value)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                result.inspection_id,
                result.item_name.as_str(),
                result.verdict.as_str(),
                result.actual_value,
                result.expected_value,
            ],
        ).map_err(|e| RepositoryError::Sql(e.to_string()))?;

        Ok(self.conn.last_insert_rowid())
    }

    /// 検査IDで項目結果一覧を取得
    pub fn get_item_results_by_inspection(&self, inspection_id: i64) -> Result<Vec<InspectionItemResult>, RepositoryError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, inspection_id, item_name, verdict, actual_value, expected_value
             FROM inspection_item_results WHERE inspection_id = ?1"
        ).map_err(|e| RepositoryError::Sql(e.to_string()))?;

        let rows = stmt.query_map([inspection_id], |row| {
            let item_name_str: String = row.get(2)?;
            let verdict_str: String = row.get(3)?;

            Ok(InspectionItemResult {
                id: Some(row.get(0)?),
                inspection_id: row.get(1)?,
                item_name: InspectionItemName::from_str(&item_name_str).unwrap_or(InspectionItemName::Communication),
                verdict: Verdict::from_str(&verdict_str).unwrap_or(Verdict::Error),
                actual_value: row.get(4)?,
                expected_value: row.get(5)?,
            })
        }).map_err(|e| RepositoryError::Sql(e.to_string()))?;

        let mut result = Vec::new();
        for row in rows {
            result.push(row.map_err(|e| RepositoryError::Sql(e.to_string()))?);
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ===========================================
    // DB初期化テスト
    // ===========================================

    #[test]
    fn test_create_in_memory_db() {
        let repo = SqliteRepository::in_memory();
        assert!(repo.is_ok());
    }

    // ===========================================
    // Lot CRUDテスト
    // ===========================================

    #[test]
    fn test_lot_insert_and_get() {
        let repo = SqliteRepository::in_memory().unwrap();

        let lot = Lot::new("LOT-2026-001".to_string())
            .with_expected_rate(100)
            .with_expected_port_proto("UBX+NMEA", "UBX+NMEA");

        let id = repo.insert_lot(&lot).unwrap();
        assert!(id > 0);

        let loaded = repo.get_lot(id).unwrap();
        assert_eq!(loaded.lot_number, "LOT-2026-001");
        assert_eq!(loaded.expected_rate_ms, Some(100));
        assert_eq!(loaded.expected_port_in_proto, Some("UBX+NMEA".to_string()));
    }

    #[test]
    fn test_lot_get_all() {
        let repo = SqliteRepository::in_memory().unwrap();

        repo.insert_lot(&Lot::new("LOT-001".to_string())).unwrap();
        repo.insert_lot(&Lot::new("LOT-002".to_string())).unwrap();

        let lots = repo.get_all_lots().unwrap();
        assert_eq!(lots.len(), 2);
    }

    #[test]
    fn test_lot_not_found() {
        let repo = SqliteRepository::in_memory().unwrap();

        let result = repo.get_lot(999);
        assert!(matches!(result, Err(RepositoryError::NotFound(_))));
    }

    // ===========================================
    // Device CRUDテスト
    // ===========================================

    #[test]
    fn test_device_insert_and_get() {
        let repo = SqliteRepository::in_memory().unwrap();

        // ロット作成
        let lot_id = repo.insert_lot(&Lot::new("LOT-001".to_string())).unwrap();

        // 装置作成
        let device = Device::new("0102030405060708".to_string())
            .with_lot(lot_id)
            .with_model("ZED-F9P")
            .with_fw_version("HPG 1.32");

        let device_id = repo.insert_device(&device).unwrap();
        assert!(device_id > 0);

        let loaded = repo.get_device(device_id).unwrap();
        assert_eq!(loaded.serial_number, "0102030405060708");
        assert_eq!(loaded.lot_id, Some(lot_id));
        assert_eq!(loaded.fw_version, Some("HPG 1.32".to_string()));
    }

    #[test]
    fn test_device_get_by_serial() {
        let repo = SqliteRepository::in_memory().unwrap();

        let device = Device::new("ABCD1234".to_string());
        repo.insert_device(&device).unwrap();

        let loaded = repo.get_device_by_serial("ABCD1234").unwrap();
        assert_eq!(loaded.serial_number, "ABCD1234");
    }

    #[test]
    fn test_device_get_by_lot() {
        let repo = SqliteRepository::in_memory().unwrap();

        let lot_id = repo.insert_lot(&Lot::new("LOT-001".to_string())).unwrap();

        repo.insert_device(&Device::new("DEV-001".to_string()).with_lot(lot_id)).unwrap();
        repo.insert_device(&Device::new("DEV-002".to_string()).with_lot(lot_id)).unwrap();
        repo.insert_device(&Device::new("DEV-003".to_string())).unwrap(); // 別ロット

        let devices = repo.get_devices_by_lot(lot_id).unwrap();
        assert_eq!(devices.len(), 2);
    }

    #[test]
    fn test_device_update_fw_version() {
        let repo = SqliteRepository::in_memory().unwrap();

        let device_id = repo.insert_device(&Device::new("DEV-001".to_string())).unwrap();

        repo.update_device_fw_version(device_id, "HPG 1.33").unwrap();

        let loaded = repo.get_device(device_id).unwrap();
        assert_eq!(loaded.fw_version, Some("HPG 1.33".to_string()));
    }

    // ===========================================
    // IndoorInspection CRUDテスト
    // ===========================================

    #[test]
    fn test_inspection_insert_and_get() {
        let repo = SqliteRepository::in_memory().unwrap();

        let device_id = repo.insert_device(&Device::new("DEV-001".to_string())).unwrap();

        let inspection = IndoorInspection::new(device_id, "2026-03-11T09:00:00+09:00".to_string())
            .with_result("Pass");

        let inspection_id = repo.insert_inspection(&inspection).unwrap();
        assert!(inspection_id > 0);

        let loaded = repo.get_inspection(inspection_id).unwrap();
        assert_eq!(loaded.device_id, device_id);
        assert_eq!(loaded.overall_result, Some("Pass".to_string()));
    }

    #[test]
    fn test_inspection_get_by_device() {
        let repo = SqliteRepository::in_memory().unwrap();

        let device_id = repo.insert_device(&Device::new("DEV-001".to_string())).unwrap();

        repo.insert_inspection(&IndoorInspection::new(device_id, "2026-03-11T09:00:00+09:00".to_string())).unwrap();
        repo.insert_inspection(&IndoorInspection::new(device_id, "2026-03-11T10:00:00+09:00".to_string())).unwrap();

        let inspections = repo.get_inspections_by_device(device_id).unwrap();
        assert_eq!(inspections.len(), 2);
    }

    #[test]
    fn test_inspection_update_result() {
        let repo = SqliteRepository::in_memory().unwrap();

        let device_id = repo.insert_device(&Device::new("DEV-001".to_string())).unwrap();
        let inspection_id = repo.insert_inspection(
            &IndoorInspection::new(device_id, "2026-03-11T09:00:00+09:00".to_string())
        ).unwrap();

        repo.update_inspection_result(inspection_id, "Fail").unwrap();

        let loaded = repo.get_inspection(inspection_id).unwrap();
        assert_eq!(loaded.overall_result, Some("Fail".to_string()));
    }

    // ===========================================
    // InspectionItemResult CRUDテスト
    // ===========================================

    #[test]
    fn test_item_result_insert_and_get() {
        let repo = SqliteRepository::in_memory().unwrap();

        let device_id = repo.insert_device(&Device::new("DEV-001".to_string())).unwrap();
        let inspection_id = repo.insert_inspection(
            &IndoorInspection::new(device_id, "2026-03-11T09:00:00+09:00".to_string())
        ).unwrap();

        let result = InspectionItemResult::new(inspection_id, InspectionItemName::Communication, Verdict::Pass);
        let result_id = repo.insert_item_result(&result).unwrap();
        assert!(result_id > 0);

        let results = repo.get_item_results_by_inspection(inspection_id).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].item_name, InspectionItemName::Communication);
        assert_eq!(results[0].verdict, Verdict::Pass);
    }

    #[test]
    fn test_item_result_with_values() {
        let repo = SqliteRepository::in_memory().unwrap();

        let device_id = repo.insert_device(&Device::new("DEV-001".to_string())).unwrap();
        let inspection_id = repo.insert_inspection(
            &IndoorInspection::new(device_id, "2026-03-11T09:00:00+09:00".to_string())
        ).unwrap();

        // FWバージョンを記録（Recorded判定）
        let fw_result = InspectionItemResult::new(inspection_id, InspectionItemName::Fw, Verdict::Recorded)
            .with_actual("HPG 1.32");
        repo.insert_item_result(&fw_result).unwrap();

        // 出力レートを検査（Pass判定）
        let rate_result = InspectionItemResult::new(inspection_id, InspectionItemName::Rate, Verdict::Pass)
            .with_actual("100")
            .with_expected("100");
        repo.insert_item_result(&rate_result).unwrap();

        let results = repo.get_item_results_by_inspection(inspection_id).unwrap();
        assert_eq!(results.len(), 2);

        // FWの確認
        let fw = results.iter().find(|r| r.item_name == InspectionItemName::Fw).unwrap();
        assert_eq!(fw.verdict, Verdict::Recorded);
        assert_eq!(fw.actual_value, Some("HPG 1.32".to_string()));

        // レートの確認
        let rate = results.iter().find(|r| r.item_name == InspectionItemName::Rate).unwrap();
        assert_eq!(rate.verdict, Verdict::Pass);
        assert_eq!(rate.actual_value, Some("100".to_string()));
        assert_eq!(rate.expected_value, Some("100".to_string()));
    }

    // ===========================================
    // 統合テスト: 検査フロー全体
    // ===========================================

    #[test]
    fn test_full_inspection_flow() {
        let repo = SqliteRepository::in_memory().unwrap();

        // 1. ロット作成
        let lot_id = repo.insert_lot(
            &Lot::new("LOT-2026-001".to_string())
                .with_expected_rate(100)
                .with_expected_port_proto("UBX+NMEA", "UBX+NMEA")
        ).unwrap();

        // 2. 装置作成（ロットに紐づけ）
        let device_id = repo.insert_device(
            &Device::new("0102030405060708".to_string())
                .with_lot(lot_id)
                .with_model("ZED-F9P")
        ).unwrap();

        // 3. 屋内検査作成
        let inspection_id = repo.insert_inspection(
            &IndoorInspection::new(device_id, "2026-03-11T09:00:00+09:00".to_string())
        ).unwrap();

        // 4. 検査項目結果を記録
        repo.insert_item_result(&InspectionItemResult::new(
            inspection_id, InspectionItemName::Communication, Verdict::Pass
        )).unwrap();

        repo.insert_item_result(&InspectionItemResult::new(
            inspection_id, InspectionItemName::Serial, Verdict::Pass
        ).with_actual("0102030405060708")).unwrap();

        repo.insert_item_result(&InspectionItemResult::new(
            inspection_id, InspectionItemName::Fw, Verdict::Recorded
        ).with_actual("HPG 1.32")).unwrap();

        repo.insert_item_result(&InspectionItemResult::new(
            inspection_id, InspectionItemName::Rate, Verdict::Pass
        ).with_actual("100").with_expected("100")).unwrap();

        repo.insert_item_result(&InspectionItemResult::new(
            inspection_id, InspectionItemName::Port, Verdict::Pass
        ).with_actual("UBX+NMEA/UBX+NMEA").with_expected("UBX+NMEA/UBX+NMEA")).unwrap();

        // 5. 装置のFWバージョンを更新
        repo.update_device_fw_version(device_id, "HPG 1.32").unwrap();

        // 6. 総合判定を更新
        repo.update_inspection_result(inspection_id, "Pass").unwrap();

        // 検証
        let device = repo.get_device(device_id).unwrap();
        assert_eq!(device.fw_version, Some("HPG 1.32".to_string()));

        let inspection = repo.get_inspection(inspection_id).unwrap();
        assert_eq!(inspection.overall_result, Some("Pass".to_string()));

        let results = repo.get_item_results_by_inspection(inspection_id).unwrap();
        assert_eq!(results.len(), 5);
    }
}
