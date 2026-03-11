//! 検査サービス
//!
//! InspectionEngineとRepositoryを統合した検査フロー

use chrono::Local;

use crate::device::manager::{DeviceManager, SerialPortProvider};
use crate::inspection::engine::{InspectionEngine, InspectionError};
use crate::inspection::types::InspectionResult as EngineResult;
use crate::repository::{Device, IndoorInspection, RepositoryError, SqliteRepository};

use super::converter::{calculate_overall_result, engine_result_to_repo};

/// 検査サービスのエラー
#[derive(Debug)]
pub enum ServiceError {
    /// 検査エラー
    Inspection(InspectionError),
    /// リポジトリエラー
    Repository(RepositoryError),
    /// デバイス未接続
    DeviceNotConnected,
    /// シリアル番号取得失敗
    SerialNumberNotFound,
}

impl std::fmt::Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceError::Inspection(e) => write!(f, "Inspection error: {}", e),
            ServiceError::Repository(e) => write!(f, "Repository error: {}", e),
            ServiceError::DeviceNotConnected => write!(f, "Device not connected"),
            ServiceError::SerialNumberNotFound => write!(f, "Serial number not found in results"),
        }
    }
}

impl std::error::Error for ServiceError {}

impl From<InspectionError> for ServiceError {
    fn from(e: InspectionError) -> Self {
        ServiceError::Inspection(e)
    }
}

impl From<RepositoryError> for ServiceError {
    fn from(e: RepositoryError) -> Self {
        ServiceError::Repository(e)
    }
}

/// 検査実行結果
#[derive(Debug)]
pub struct InspectionReport {
    /// 検査ID（DB上のID）
    pub inspection_id: i64,
    /// 装置ID（DB上のID）
    pub device_id: i64,
    /// 総合判定
    pub overall_result: String,
    /// 各項目の結果
    pub item_results: Vec<EngineResult>,
}

/// 検査サービス
///
/// 検査の実行からDB保存までを一括で行う
pub struct InspectionService<'a> {
    repo: &'a SqliteRepository,
    engine: InspectionEngine,
}

impl<'a> InspectionService<'a> {
    /// 新しいサービスを作成
    pub fn new(repo: &'a SqliteRepository) -> Self {
        Self {
            repo,
            engine: InspectionEngine::new(),
        }
    }

    /// カスタムエンジンでサービスを作成（テスト用）
    pub fn with_engine(repo: &'a SqliteRepository, engine: InspectionEngine) -> Self {
        Self { repo, engine }
    }

    /// 検査を実行してDBに保存
    ///
    /// 1. DeviceManagerから検査を実行
    /// 2. シリアル番号を取得（検査結果から）
    /// 3. 装置をDBに登録（または取得）
    /// 4. 検査結果をDBに保存
    /// 5. 総合判定を計算して保存
    pub fn run_and_save<P: SerialPortProvider>(
        &self,
        manager: &mut DeviceManager<P>,
        lot_id: Option<i64>,
    ) -> Result<InspectionReport, ServiceError> {
        // 1. 検査実行
        let results = self.engine.run(manager)?;

        // 2. シリアル番号を取得
        let serial_number = self.extract_serial_number(&results)?;

        // 3. 装置をDBに登録（または取得）
        let device_id = self.get_or_create_device(&serial_number, lot_id)?;

        // 4. FWバージョンを取得して装置を更新
        if let Some(fw_version) = self.extract_fw_version(&results) {
            // エラーは無視（更新できなくても検査は続行）
            let _ = self.repo.update_device_fw_version(device_id, &fw_version);
        }

        // 5. 検査レコードを作成
        let now = Local::now().format("%Y-%m-%dT%H:%M:%S%:z").to_string();
        let inspection = IndoorInspection::new(device_id, now);
        let inspection_id = self.repo.insert_inspection(&inspection)?;

        // 6. 各項目結果を保存
        let mut repo_verdicts = Vec::new();
        for result in &results {
            let repo_result = engine_result_to_repo(result, inspection_id);
            repo_verdicts.push(repo_result.verdict);
            self.repo.insert_item_result(&repo_result)?;
        }

        // 7. 総合判定を計算して保存
        let overall_result = calculate_overall_result(&repo_verdicts).to_string();
        self.repo
            .update_inspection_result(inspection_id, &overall_result)?;

        Ok(InspectionReport {
            inspection_id,
            device_id,
            overall_result,
            item_results: results,
        })
    }

    /// 検査結果からシリアル番号を取得
    fn extract_serial_number(&self, results: &[EngineResult]) -> Result<String, ServiceError> {
        use crate::inspection::types::ItemType;

        for result in results {
            if matches!(result.item_type, ItemType::SerialNumber) {
                if let Some(serial) = &result.actual_value {
                    return Ok(serial.clone());
                }
            }
        }

        Err(ServiceError::SerialNumberNotFound)
    }

    /// 検査結果からFWバージョンを取得
    fn extract_fw_version(&self, results: &[EngineResult]) -> Option<String> {
        use crate::inspection::types::ItemType;

        for result in results {
            if matches!(result.item_type, ItemType::FwVersion) {
                return result.actual_value.clone();
            }
        }

        None
    }

    /// 装置をDBから取得または新規作成
    fn get_or_create_device(
        &self,
        serial_number: &str,
        lot_id: Option<i64>,
    ) -> Result<i64, ServiceError> {
        // 既存装置を検索
        match self.repo.get_device_by_serial(serial_number) {
            Ok(device) => Ok(device.id.unwrap()),
            Err(RepositoryError::NotFound(_)) => {
                // 新規作成
                let mut device = Device::new(serial_number.to_string());
                if let Some(lot_id) = lot_id {
                    device = device.with_lot(lot_id);
                }
                let id = self.repo.insert_device(&device)?;
                Ok(id)
            }
            Err(e) => Err(ServiceError::Repository(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::filter::PortInfo;
    use crate::device::manager::{DeviceManagerError, SerialPort};
    use crate::repository::Lot;
    use std::collections::VecDeque;
    use std::io;
    use std::sync::{Arc, Mutex};
    use std::time::Duration;

    // ===========================================
    // モック実装（InspectionEngineテストと同じ、Send対応）
    // ===========================================

    struct MockSerialPort {
        write_data: Arc<Mutex<Vec<u8>>>,
        read_queue: Arc<Mutex<VecDeque<Vec<u8>>>>,
        should_timeout: bool,
    }

    impl SerialPort for MockSerialPort {
        fn write(&mut self, data: &[u8]) -> Result<usize, io::Error> {
            self.write_data.lock().unwrap().extend_from_slice(data);
            Ok(data.len())
        }

        fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
            if self.should_timeout {
                return Err(io::Error::new(io::ErrorKind::TimedOut, "timeout"));
            }

            let mut queue = self.read_queue.lock().unwrap();
            if let Some(data) = queue.pop_front() {
                let len = buf.len().min(data.len());
                buf[..len].copy_from_slice(&data[..len]);
                Ok(len)
            } else {
                Err(io::Error::new(io::ErrorKind::TimedOut, "no data"))
            }
        }

        fn set_timeout(&mut self, _timeout: Duration) -> Result<(), io::Error> {
            Ok(())
        }
    }

    struct MockProvider {
        ports: Vec<PortInfo>,
        write_data: Arc<Mutex<Vec<u8>>>,
        read_queue: Arc<Mutex<VecDeque<Vec<u8>>>>,
        should_timeout: bool,
    }

    impl MockProvider {
        fn new() -> Self {
            Self {
                ports: vec![],
                write_data: Arc::new(Mutex::new(Vec::new())),
                read_queue: Arc::new(Mutex::new(VecDeque::new())),
                should_timeout: false,
            }
        }

        fn with_ports(mut self, ports: Vec<PortInfo>) -> Self {
            self.ports = ports;
            self
        }

        fn with_responses(mut self, responses: Vec<Vec<u8>>) -> Self {
            self.read_queue = Arc::new(Mutex::new(responses.into()));
            self
        }
    }

    impl SerialPortProvider for MockProvider {
        fn available_ports(&self) -> Result<Vec<PortInfo>, DeviceManagerError> {
            Ok(self.ports.clone())
        }

        fn open(
            &self,
            path: &str,
            _baud_rate: u32,
        ) -> Result<Box<dyn SerialPort>, DeviceManagerError> {
            if !self.ports.iter().any(|p| p.path == path) {
                return Err(DeviceManagerError::PortNotFound(path.to_string()));
            }
            Ok(Box::new(MockSerialPort {
                write_data: self.write_data.clone(),
                read_queue: self.read_queue.clone(),
                should_timeout: self.should_timeout,
            }))
        }
    }

    // ===========================================
    // ヘルパー関数
    // ===========================================

    const F9P_VID: u16 = 0x1546;
    const F9P_PID: u16 = 0x01A9;

    fn f9p_port(path: &str) -> PortInfo {
        PortInfo {
            path: path.to_string(),
            vid: Some(F9P_VID),
            pid: Some(F9P_PID),
            serial_number: Some("TEST123".to_string()),
        }
    }

    fn build_ubx_frame(class: u8, id: u8, payload: &[u8]) -> Vec<u8> {
        let len = payload.len() as u16;
        let mut frame = vec![0xB5, 0x62, class, id, (len & 0xFF) as u8, (len >> 8) as u8];
        frame.extend_from_slice(payload);

        let mut ck_a: u8 = 0;
        let mut ck_b: u8 = 0;
        for &byte in &frame[2..] {
            ck_a = ck_a.wrapping_add(byte);
            ck_b = ck_b.wrapping_add(ck_a);
        }
        frame.push(ck_a);
        frame.push(ck_b);
        frame
    }

    fn nav_status_response() -> Vec<u8> {
        build_ubx_frame(0x01, 0x03, &[0u8; 16])
    }

    fn mon_ver_response(sw_version: &str) -> Vec<u8> {
        let mut payload = vec![0u8; 40];
        let bytes = sw_version.as_bytes();
        let len = bytes.len().min(30);
        payload[..len].copy_from_slice(&bytes[..len]);
        build_ubx_frame(0x0A, 0x04, &payload)
    }

    fn sec_uniqid_response(unique_id: &[u8; 5]) -> Vec<u8> {
        let mut payload = vec![0u8; 9];
        payload[4..9].copy_from_slice(unique_id);
        build_ubx_frame(0x27, 0x03, &payload)
    }

    fn cfg_rate_response(meas_rate_ms: u16) -> Vec<u8> {
        let mut payload = vec![0u8; 6];
        payload[0..2].copy_from_slice(&meas_rate_ms.to_le_bytes());
        build_ubx_frame(0x06, 0x08, &payload)
    }

    fn cfg_prt_response(baud_rate: u32) -> Vec<u8> {
        let mut payload = vec![0u8; 20];
        payload[8..12].copy_from_slice(&baud_rate.to_le_bytes());
        build_ubx_frame(0x06, 0x00, &payload)
    }

    fn all_pass_responses() -> Vec<Vec<u8>> {
        vec![
            nav_status_response(),
            mon_ver_response("HPG 1.32"),
            sec_uniqid_response(&[0xAB, 0xCD, 0xEF, 0x12, 0x34]),
            cfg_rate_response(100),
            cfg_prt_response(115200),
        ]
    }

    // ===========================================
    // S1: 検査実行とDB保存の統合テスト
    // ===========================================

    #[test]
    fn test_s1_run_and_save_creates_records() {
        // セットアップ
        let repo = SqliteRepository::in_memory().unwrap();
        let lot_id = repo.insert_lot(&Lot::new("LOT-001".to_string())).unwrap();

        let provider = MockProvider::new()
            .with_ports(vec![f9p_port("/dev/ttyACM0")])
            .with_responses(all_pass_responses());

        let mut manager = DeviceManager::new(provider);
        manager.list_devices().unwrap();
        manager.connect("/dev/ttyACM0").unwrap();

        // 検査サービス実行
        let service = InspectionService::new(&repo);
        let report = service.run_and_save(&mut manager, Some(lot_id)).unwrap();

        // 検証: レポート
        assert!(report.inspection_id > 0);
        assert!(report.device_id > 0);
        assert_eq!(report.overall_result, "Pass");
        assert_eq!(report.item_results.len(), 5);

        // 検証: DB上の装置
        let device = repo.get_device(report.device_id).unwrap();
        assert_eq!(device.serial_number, "ABCDEF1234"); // 0xAB, 0xCD, 0xEF, 0x12, 0x34
        assert_eq!(device.lot_id, Some(lot_id));
        assert_eq!(device.fw_version, Some("HPG 1.32".to_string()));

        // 検証: DB上の検査
        let inspection = repo.get_inspection(report.inspection_id).unwrap();
        assert_eq!(inspection.device_id, report.device_id);
        assert_eq!(inspection.overall_result, Some("Pass".to_string()));

        // 検証: DB上の項目結果
        let results = repo
            .get_item_results_by_inspection(report.inspection_id)
            .unwrap();
        assert_eq!(results.len(), 5);
    }

    #[test]
    fn test_s2_existing_device_is_reused() {
        // セットアップ: 既存装置を作成
        let repo = SqliteRepository::in_memory().unwrap();
        let existing_device = Device::new("ABCDEF1234".to_string());
        let existing_id = repo.insert_device(&existing_device).unwrap();

        let provider = MockProvider::new()
            .with_ports(vec![f9p_port("/dev/ttyACM0")])
            .with_responses(all_pass_responses());

        let mut manager = DeviceManager::new(provider);
        manager.list_devices().unwrap();
        manager.connect("/dev/ttyACM0").unwrap();

        // 検査サービス実行
        let service = InspectionService::new(&repo);
        let report = service.run_and_save(&mut manager, None).unwrap();

        // 検証: 既存装置が再利用された
        assert_eq!(report.device_id, existing_id);

        // 検証: FWバージョンが更新された
        let device = repo.get_device(report.device_id).unwrap();
        assert_eq!(device.fw_version, Some("HPG 1.32".to_string()));
    }

    #[test]
    fn test_s3_partial_failure_results_in_fail() {
        // セットアップ: 3項目目でタイムアウト
        let repo = SqliteRepository::in_memory().unwrap();

        let provider = MockProvider::new()
            .with_ports(vec![f9p_port("/dev/ttyACM0")])
            .with_responses(vec![
                nav_status_response(),
                mon_ver_response("HPG 1.32"),
                // 3項目目以降は応答なし → タイムアウト
            ]);

        let mut manager = DeviceManager::new(provider);
        manager.list_devices().unwrap();
        manager.connect("/dev/ttyACM0").unwrap();

        // 検査サービス実行
        let service = InspectionService::new(&repo);
        let result = service.run_and_save(&mut manager, None);

        // シリアル番号取得失敗でエラー
        assert!(matches!(result, Err(ServiceError::SerialNumberNotFound)));
    }

    #[test]
    fn test_s4_without_lot_id() {
        // セットアップ: lot_id なしで実行
        let repo = SqliteRepository::in_memory().unwrap();

        let provider = MockProvider::new()
            .with_ports(vec![f9p_port("/dev/ttyACM0")])
            .with_responses(all_pass_responses());

        let mut manager = DeviceManager::new(provider);
        manager.list_devices().unwrap();
        manager.connect("/dev/ttyACM0").unwrap();

        // 検査サービス実行（lot_id = None）
        let service = InspectionService::new(&repo);
        let report = service.run_and_save(&mut manager, None).unwrap();

        // 検証: 装置は作成されたがlot_idはNone
        let device = repo.get_device(report.device_id).unwrap();
        assert!(device.lot_id.is_none());
    }
}
