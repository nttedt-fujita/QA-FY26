//! SQLiteリポジトリ実装
//!
//! 検査結果をSQLiteに保存・取得

use rusqlite::{Connection, params};
use super::{InspectionRecord, RepositoryError};

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

    /// テーブル初期化
    fn init_tables(&self) -> Result<(), RepositoryError> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS inspection_records (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                device_serial TEXT NOT NULL,
                inspected_at TEXT NOT NULL,
                overall_verdict TEXT NOT NULL,
                fw_version TEXT,
                connectivity_verdict TEXT,
                fw_version_verdict TEXT,
                serial_number_verdict TEXT,
                output_rate_verdict TEXT,
                port_config_verdict TEXT,
                notes TEXT
            )",
            [],
        ).map_err(|e| RepositoryError::Sql(e.to_string()))?;

        Ok(())
    }

    /// 検査結果を保存
    pub fn save(&self, record: &InspectionRecord) -> Result<i64, RepositoryError> {
        self.conn.execute(
            "INSERT INTO inspection_records (
                device_serial, inspected_at, overall_verdict, fw_version,
                connectivity_verdict, fw_version_verdict, serial_number_verdict,
                output_rate_verdict, port_config_verdict, notes
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                record.device_serial,
                record.inspected_at,
                record.overall_verdict,
                record.fw_version,
                record.item_verdicts.get("connectivity"),
                record.item_verdicts.get("fw_version"),
                record.item_verdicts.get("serial_number"),
                record.item_verdicts.get("output_rate"),
                record.item_verdicts.get("port_config"),
                record.notes,
            ],
        ).map_err(|e| RepositoryError::Sql(e.to_string()))?;

        Ok(self.conn.last_insert_rowid())
    }

    /// 全件取得
    pub fn get_all(&self) -> Result<Vec<InspectionRecord>, RepositoryError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, device_serial, inspected_at, overall_verdict, fw_version,
                    connectivity_verdict, fw_version_verdict, serial_number_verdict,
                    output_rate_verdict, port_config_verdict, notes
             FROM inspection_records
             ORDER BY inspected_at DESC"
        ).map_err(|e| RepositoryError::Sql(e.to_string()))?;

        let records = stmt.query_map([], |row| {
            Ok(self.row_to_record(row))
        }).map_err(|e| RepositoryError::Sql(e.to_string()))?;

        let mut result = Vec::new();
        for record in records {
            result.push(record.map_err(|e| RepositoryError::Sql(e.to_string()))?);
        }
        Ok(result)
    }

    /// シリアル番号で検索
    pub fn find_by_serial(&self, serial: &str) -> Result<Vec<InspectionRecord>, RepositoryError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, device_serial, inspected_at, overall_verdict, fw_version,
                    connectivity_verdict, fw_version_verdict, serial_number_verdict,
                    output_rate_verdict, port_config_verdict, notes
             FROM inspection_records
             WHERE device_serial = ?1
             ORDER BY inspected_at DESC"
        ).map_err(|e| RepositoryError::Sql(e.to_string()))?;

        let records = stmt.query_map([serial], |row| {
            Ok(self.row_to_record(row))
        }).map_err(|e| RepositoryError::Sql(e.to_string()))?;

        let mut result = Vec::new();
        for record in records {
            result.push(record.map_err(|e| RepositoryError::Sql(e.to_string()))?);
        }
        Ok(result)
    }

    /// 行をInspectionRecordに変換
    fn row_to_record(&self, row: &rusqlite::Row) -> InspectionRecord {
        let mut item_verdicts = std::collections::HashMap::new();

        if let Ok(Some(v)) = row.get::<_, Option<String>>(5) {
            item_verdicts.insert("connectivity".to_string(), v);
        }
        if let Ok(Some(v)) = row.get::<_, Option<String>>(6) {
            item_verdicts.insert("fw_version".to_string(), v);
        }
        if let Ok(Some(v)) = row.get::<_, Option<String>>(7) {
            item_verdicts.insert("serial_number".to_string(), v);
        }
        if let Ok(Some(v)) = row.get::<_, Option<String>>(8) {
            item_verdicts.insert("output_rate".to_string(), v);
        }
        if let Ok(Some(v)) = row.get::<_, Option<String>>(9) {
            item_verdicts.insert("port_config".to_string(), v);
        }

        InspectionRecord {
            id: row.get(0).ok(),
            device_serial: row.get(1).unwrap_or_default(),
            inspected_at: row.get(2).unwrap_or_default(),
            overall_verdict: row.get(3).unwrap_or_default(),
            fw_version: row.get(4).ok().flatten(),
            item_verdicts,
            notes: row.get(10).ok().flatten(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ===========================================
    // D1-D2: DB/テーブル作成テスト
    // ===========================================

    /// D1: インメモリDBを作成できる
    #[test]
    fn test_d1_create_in_memory_db() {
        let repo = SqliteRepository::in_memory();
        assert!(repo.is_ok());
    }

    /// D2: テーブルが自動作成される
    #[test]
    fn test_d2_table_created_automatically() {
        let repo = SqliteRepository::in_memory().unwrap();

        // テーブルが存在するか確認（get_allが成功すればOK）
        let result = repo.get_all();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    // ===========================================
    // R1-R2: 保存テスト
    // ===========================================

    /// R1: InspectionRecordを保存できる
    #[test]
    fn test_r1_save_inspection_record() {
        let repo = SqliteRepository::in_memory().unwrap();

        let record = InspectionRecord::new(
            "0102030405060708".to_string(),
            "2026-03-11T09:00:00+09:00".to_string(),
            "Pass".to_string(),
        )
        .with_fw_version("HPG 1.32".to_string())
        .with_item_verdict("connectivity", "Pass")
        .with_item_verdict("fw_version", "Pass");

        let result = repo.save(&record);
        assert!(result.is_ok());
    }

    /// R2: 保存後にIDが返る
    #[test]
    fn test_r2_save_returns_id() {
        let repo = SqliteRepository::in_memory().unwrap();

        let record = InspectionRecord::new(
            "0102030405060708".to_string(),
            "2026-03-11T09:00:00+09:00".to_string(),
            "Pass".to_string(),
        );

        let id = repo.save(&record).unwrap();
        assert!(id > 0);

        // 2件目は違うIDになる
        let id2 = repo.save(&record).unwrap();
        assert!(id2 > id);
    }

    // ===========================================
    // R3-R4: 検索テスト
    // ===========================================

    /// R3: シリアル番号で検索できる
    #[test]
    fn test_r3_find_by_serial() {
        let repo = SqliteRepository::in_memory().unwrap();

        // 2台分のレコードを保存
        let record1 = InspectionRecord::new(
            "AAAA".to_string(),
            "2026-03-11T09:00:00+09:00".to_string(),
            "Pass".to_string(),
        );
        let record2 = InspectionRecord::new(
            "BBBB".to_string(),
            "2026-03-11T10:00:00+09:00".to_string(),
            "Fail".to_string(),
        );
        let record3 = InspectionRecord::new(
            "AAAA".to_string(),
            "2026-03-11T11:00:00+09:00".to_string(),
            "Pass".to_string(),
        );

        repo.save(&record1).unwrap();
        repo.save(&record2).unwrap();
        repo.save(&record3).unwrap();

        // AAAAで検索すると2件
        let results = repo.find_by_serial("AAAA").unwrap();
        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|r| r.device_serial == "AAAA"));
    }

    /// R4: 全件取得できる
    #[test]
    fn test_r4_get_all() {
        let repo = SqliteRepository::in_memory().unwrap();

        let record1 = InspectionRecord::new(
            "AAAA".to_string(),
            "2026-03-11T09:00:00+09:00".to_string(),
            "Pass".to_string(),
        );
        let record2 = InspectionRecord::new(
            "BBBB".to_string(),
            "2026-03-11T10:00:00+09:00".to_string(),
            "Fail".to_string(),
        );

        repo.save(&record1).unwrap();
        repo.save(&record2).unwrap();

        let results = repo.get_all().unwrap();
        assert_eq!(results.len(), 2);
    }

    // ===========================================
    // E2: 空の結果テスト
    // ===========================================

    /// E2: 検索結果が空の場合、空Vecが返る
    #[test]
    fn test_e2_empty_result_returns_empty_vec() {
        let repo = SqliteRepository::in_memory().unwrap();

        // 何も保存していない状態で検索
        let results = repo.find_by_serial("NONEXISTENT").unwrap();
        assert_eq!(results.len(), 0);
    }

    // ===========================================
    // 追加: データ整合性テスト
    // ===========================================

    /// 保存したデータを正しく取得できる
    #[test]
    fn test_data_integrity() {
        let repo = SqliteRepository::in_memory().unwrap();

        let record = InspectionRecord::new(
            "0102030405060708".to_string(),
            "2026-03-11T09:00:00+09:00".to_string(),
            "Pass".to_string(),
        )
        .with_fw_version("HPG 1.32".to_string())
        .with_item_verdict("connectivity", "Pass")
        .with_item_verdict("fw_version", "Fail")
        .with_notes("テスト備考".to_string());

        let id = repo.save(&record).unwrap();

        let results = repo.get_all().unwrap();
        assert_eq!(results.len(), 1);

        let loaded = &results[0];
        assert_eq!(loaded.id, Some(id));
        assert_eq!(loaded.device_serial, "0102030405060708");
        assert_eq!(loaded.inspected_at, "2026-03-11T09:00:00+09:00");
        assert_eq!(loaded.overall_verdict, "Pass");
        assert_eq!(loaded.fw_version, Some("HPG 1.32".to_string()));
        assert_eq!(loaded.item_verdicts.get("connectivity"), Some(&"Pass".to_string()));
        assert_eq!(loaded.item_verdicts.get("fw_version"), Some(&"Fail".to_string()));
        assert_eq!(loaded.notes, Some("テスト備考".to_string()));
    }
}
