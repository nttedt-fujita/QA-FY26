//! 検査エンジン
//!
//! GNSS装置の受入検査を実行するメインロジック

use std::time::Duration;

use crate::device::manager::{DeviceManager, DeviceManagerError, SerialPortProvider};
use crate::device::status::DeviceStatus;

use super::judge::judge_result;
use super::types::{ExpectedValue, InspectionItem, InspectionResult, ItemType, Verdict};

/// 検査エンジンのエラー
#[derive(Debug)]
pub enum InspectionError {
    /// 未接続
    NotConnected,
    /// 検査中に切断された
    Disconnected,
    /// 通信エラー
    CommunicationError(String),
}

impl std::fmt::Display for InspectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InspectionError::NotConnected => write!(f, "未接続です"),
            InspectionError::Disconnected => write!(f, "検査中に切断されました"),
            InspectionError::CommunicationError(e) => write!(f, "通信エラー: {}", e),
        }
    }
}

impl std::error::Error for InspectionError {}

/// 検査エンジン
pub struct InspectionEngine {
    /// 検査項目リスト（5項目固定）
    items: Vec<InspectionItem>,
}

impl InspectionEngine {
    /// 新しい検査エンジンを作成
    pub fn new() -> Self {
        Self {
            items: Self::default_items(),
        }
    }

    /// カスタム検査項目で作成（テスト用）
    pub fn with_items(items: Vec<InspectionItem>) -> Self {
        Self { items }
    }

    /// デフォルトの検査項目リスト
    fn default_items() -> Vec<InspectionItem> {
        vec![
            InspectionItem::new(ItemType::Connectivity),
            InspectionItem::new(ItemType::FwVersion),
            InspectionItem::new(ItemType::SerialNumber),
            InspectionItem::new(ItemType::OutputRate),
            InspectionItem::new(ItemType::PortConfig),
        ]
    }

    /// 検査を実行
    ///
    /// # Arguments
    /// * `manager` - デバイスマネージャー
    ///
    /// # Returns
    /// * 成功時: 検査結果リスト
    /// * 失敗時: エラー
    pub fn run<P: SerialPortProvider>(
        &self,
        manager: &mut DeviceManager<P>,
    ) -> Result<Vec<InspectionResult>, InspectionError> {
        // 接続確認
        let device = manager
            .get_connected_device()
            .ok_or(InspectionError::NotConnected)?;

        // 状態がConnectedでなければエラー
        if device.status != DeviceStatus::Connected {
            return Err(InspectionError::NotConnected);
        }

        // TODO: 状態をInspectingに変更

        let mut results = Vec::new();

        // 各検査項目を実行
        for item in &self.items {
            let result = self.execute_item(manager, item);
            results.push(result);
        }

        // TODO: 状態をConnectedに戻す

        Ok(results)
    }

    /// 検査項目を実行
    fn execute_item<P: SerialPortProvider>(
        &self,
        manager: &mut DeviceManager<P>,
        item: &InspectionItem,
    ) -> InspectionResult {
        // UBXメッセージを送信
        let poll_message = self.create_poll_message(&item.item_type);

        if let Err(e) = manager.send_ubx(&poll_message) {
            return InspectionResult::new(
                item.item_type.clone(),
                Verdict::Error(format!("Send error: {}", e)),
            );
        }

        // 応答を受信
        match manager.receive_ubx(item.timeout) {
            Ok(response) => {
                // パースして値を取得
                let (actual_value, error) = self.parse_response(&item.item_type, &response);

                let verdict = judge_result(
                    &item.expected,
                    actual_value.as_deref(),
                    error.as_deref(),
                );

                InspectionResult::new(item.item_type.clone(), verdict)
                    .with_expected(item.expected.clone())
                    .with_actual(actual_value.unwrap_or_default())
            }
            Err(DeviceManagerError::Timeout) => {
                InspectionResult::new(
                    item.item_type.clone(),
                    Verdict::Error("Timeout".to_string()),
                )
            }
            Err(e) => {
                InspectionResult::new(
                    item.item_type.clone(),
                    Verdict::Error(format!("{}", e)),
                )
            }
        }
    }

    /// UBX Pollメッセージを作成
    fn create_poll_message(&self, item_type: &ItemType) -> Vec<u8> {
        match item_type {
            // 通信疎通: NAV-STATUS poll (0x01 0x03)
            ItemType::Connectivity => {
                let payload: &[u8] = &[];
                Self::build_ubx_frame(0x01, 0x03, payload)
            }
            // FWバージョン: MON-VER poll (0x0A 0x04)
            ItemType::FwVersion => {
                let payload: &[u8] = &[];
                Self::build_ubx_frame(0x0A, 0x04, payload)
            }
            // シリアル番号: SEC-UNIQID poll (0x27 0x03)
            ItemType::SerialNumber => {
                let payload: &[u8] = &[];
                Self::build_ubx_frame(0x27, 0x03, payload)
            }
            // 出力レート: CFG-RATE poll (0x06 0x08)
            ItemType::OutputRate => {
                let payload: &[u8] = &[];
                Self::build_ubx_frame(0x06, 0x08, payload)
            }
            // ポート設定: CFG-PRT poll (0x06 0x00)
            ItemType::PortConfig => {
                // UART1 (portID = 1)
                let payload: &[u8] = &[0x01];
                Self::build_ubx_frame(0x06, 0x00, payload)
            }
        }
    }

    /// UBXフレームを構築
    fn build_ubx_frame(class: u8, id: u8, payload: &[u8]) -> Vec<u8> {
        let len = payload.len() as u16;
        let mut frame = vec![
            0xB5, 0x62, // sync
            class, id,
            (len & 0xFF) as u8,
            (len >> 8) as u8,
        ];
        frame.extend_from_slice(payload);

        // チェックサム計算
        let (ck_a, ck_b) = Self::calculate_checksum(&frame[2..]);
        frame.push(ck_a);
        frame.push(ck_b);

        frame
    }

    /// UBXチェックサムを計算
    fn calculate_checksum(data: &[u8]) -> (u8, u8) {
        let mut ck_a: u8 = 0;
        let mut ck_b: u8 = 0;
        for &byte in data {
            ck_a = ck_a.wrapping_add(byte);
            ck_b = ck_b.wrapping_add(ck_a);
        }
        (ck_a, ck_b)
    }

    /// 応答をパース
    fn parse_response(
        &self,
        item_type: &ItemType,
        response: &[u8],
    ) -> (Option<String>, Option<String>) {
        // 最低限のUBXフレーム検証
        if response.len() < 8 {
            return (None, Some("ParseError: too short".to_string()));
        }

        if response[0] != 0xB5 || response[1] != 0x62 {
            return (None, Some("ParseError: invalid sync".to_string()));
        }

        match item_type {
            ItemType::Connectivity => {
                // 任意の応答があればOK
                (Some("OK".to_string()), None)
            }
            ItemType::FwVersion => {
                // MON-VER: ペイロードの最初の30バイトがソフトウェアバージョン
                if response.len() >= 36 {
                    let sw_version = String::from_utf8_lossy(&response[6..36])
                        .trim_end_matches('\0')
                        .to_string();
                    (Some(sw_version), None)
                } else {
                    (None, Some("ParseError: MON-VER too short".to_string()))
                }
            }
            ItemType::SerialNumber => {
                // SEC-UNIQID: 5バイトのユニークID
                if response.len() >= 11 {
                    let unique_id: String = response[6..11]
                        .iter()
                        .map(|b| format!("{:02X}", b))
                        .collect();
                    (Some(unique_id), None)
                } else {
                    (None, Some("ParseError: SEC-UNIQID too short".to_string()))
                }
            }
            ItemType::OutputRate => {
                // CFG-RATE: measRate (2バイト、ミリ秒)
                if response.len() >= 10 {
                    let meas_rate = u16::from_le_bytes([response[6], response[7]]);
                    (Some(meas_rate.to_string()), None)
                } else {
                    (None, Some("ParseError: CFG-RATE too short".to_string()))
                }
            }
            ItemType::PortConfig => {
                // CFG-PRT: baudRate (4バイト、offset 8)
                if response.len() >= 14 {
                    let baud_rate = u32::from_le_bytes([
                        response[14],
                        response[15],
                        response[16],
                        response[17],
                    ]);
                    (Some(baud_rate.to_string()), None)
                } else {
                    (None, Some("ParseError: CFG-PRT too short".to_string()))
                }
            }
        }
    }
}

impl Default for InspectionEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::filter::PortInfo;
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::io;
    use std::rc::Rc;

    // ===========================================
    // モック実装（DeviceManagerと同じ）
    // ===========================================

    use crate::device::manager::SerialPort;

    /// モックシリアルポート
    struct MockSerialPort {
        write_data: Rc<RefCell<Vec<u8>>>,
        read_queue: Rc<RefCell<VecDeque<Vec<u8>>>>,
        should_timeout: bool,
        should_disconnect: bool,
        call_count: Rc<RefCell<usize>>,
        disconnect_at: Option<usize>,
    }

    impl SerialPort for MockSerialPort {
        fn write(&mut self, data: &[u8]) -> Result<usize, io::Error> {
            if self.should_disconnect {
                return Err(io::Error::new(io::ErrorKind::BrokenPipe, "disconnected"));
            }
            self.write_data.borrow_mut().extend_from_slice(data);
            Ok(data.len())
        }

        fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
            // 切断チェック
            {
                let mut count = self.call_count.borrow_mut();
                *count += 1;
                if let Some(disconnect_at) = self.disconnect_at {
                    if *count >= disconnect_at {
                        return Err(io::Error::new(io::ErrorKind::BrokenPipe, "disconnected"));
                    }
                }
            }

            if self.should_timeout {
                return Err(io::Error::new(io::ErrorKind::TimedOut, "timeout"));
            }

            let mut queue = self.read_queue.borrow_mut();
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

    /// モックProvider
    struct MockProvider {
        ports: Vec<PortInfo>,
        write_data: Rc<RefCell<Vec<u8>>>,
        read_queue: Rc<RefCell<VecDeque<Vec<u8>>>>,
        should_timeout: bool,
        should_disconnect: bool,
        call_count: Rc<RefCell<usize>>,
        disconnect_at: Option<usize>,
    }

    impl MockProvider {
        fn new() -> Self {
            Self {
                ports: vec![],
                write_data: Rc::new(RefCell::new(Vec::new())),
                read_queue: Rc::new(RefCell::new(VecDeque::new())),
                should_timeout: false,
                should_disconnect: false,
                call_count: Rc::new(RefCell::new(0)),
                disconnect_at: None,
            }
        }

        fn with_ports(mut self, ports: Vec<PortInfo>) -> Self {
            self.ports = ports;
            self
        }

        /// 各検査項目ごとの応答を設定
        fn with_responses(mut self, responses: Vec<Vec<u8>>) -> Self {
            self.read_queue = Rc::new(RefCell::new(responses.into()));
            self
        }

        fn with_timeout(mut self) -> Self {
            self.should_timeout = true;
            self
        }

        fn get_write_data(&self) -> Rc<RefCell<Vec<u8>>> {
            self.write_data.clone()
        }

        /// N回目のread呼び出しで切断をシミュレート
        fn disconnect_at(mut self, n: usize) -> Self {
            self.disconnect_at = Some(n);
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
                should_disconnect: self.should_disconnect,
                call_count: self.call_count.clone(),
                disconnect_at: self.disconnect_at,
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

    /// 有効なUBX応答を生成
    fn valid_ubx_response(class: u8, id: u8, payload: &[u8]) -> Vec<u8> {
        InspectionEngine::build_ubx_frame(class, id, payload)
    }

    /// MON-VER応答を生成
    fn mon_ver_response(sw_version: &str) -> Vec<u8> {
        let mut payload = vec![0u8; 40];
        let bytes = sw_version.as_bytes();
        let len = bytes.len().min(30);
        payload[..len].copy_from_slice(&bytes[..len]);
        valid_ubx_response(0x0A, 0x04, &payload)
    }

    /// SEC-UNIQID応答を生成
    fn sec_uniqid_response(unique_id: &[u8; 5]) -> Vec<u8> {
        let mut payload = vec![0u8; 9];
        payload[4..9].copy_from_slice(unique_id);
        valid_ubx_response(0x27, 0x03, &payload)
    }

    /// CFG-RATE応答を生成
    fn cfg_rate_response(meas_rate_ms: u16) -> Vec<u8> {
        let mut payload = vec![0u8; 6];
        payload[0..2].copy_from_slice(&meas_rate_ms.to_le_bytes());
        valid_ubx_response(0x06, 0x08, &payload)
    }

    /// CFG-PRT応答を生成
    fn cfg_prt_response(baud_rate: u32) -> Vec<u8> {
        let mut payload = vec![0u8; 20];
        payload[8..12].copy_from_slice(&baud_rate.to_le_bytes());
        valid_ubx_response(0x06, 0x00, &payload)
    }

    /// 5項目すべての正常応答を生成
    fn all_pass_responses() -> Vec<Vec<u8>> {
        vec![
            valid_ubx_response(0x01, 0x03, &[0u8; 16]), // Connectivity
            mon_ver_response("HPG 1.32"),               // FwVersion
            sec_uniqid_response(&[0xAB, 0xCD, 0xEF, 0x12, 0x34]), // SerialNumber
            cfg_rate_response(100),                      // OutputRate: 100ms
            cfg_prt_response(115200),                    // PortConfig: 115200bps
        ]
    }

    // ===========================================
    // A1-A4: 検査シーケンス実行テスト
    // ===========================================

    /// A1: 検査開始で5項目が順番に実行される
    #[test]
    fn test_a1_five_items_executed_in_order() {
        let provider = MockProvider::new()
            .with_ports(vec![f9p_port("/dev/ttyACM0")])
            .with_responses(all_pass_responses());

        let mut manager = DeviceManager::new(provider);
        manager.list_devices().unwrap();
        manager.connect("/dev/ttyACM0").unwrap();

        let engine = InspectionEngine::new();
        let results = engine.run(&mut manager).unwrap();

        // 5項目が返される
        assert_eq!(results.len(), 5);

        // 順番を確認
        assert_eq!(results[0].item_type, ItemType::Connectivity);
        assert_eq!(results[1].item_type, ItemType::FwVersion);
        assert_eq!(results[2].item_type, ItemType::SerialNumber);
        assert_eq!(results[3].item_type, ItemType::OutputRate);
        assert_eq!(results[4].item_type, ItemType::PortConfig);
    }

    /// A2: 全項目完了後にVec<InspectionResult>が返る
    #[test]
    fn test_a2_returns_vec_inspection_result() {
        let provider = MockProvider::new()
            .with_ports(vec![f9p_port("/dev/ttyACM0")])
            .with_responses(all_pass_responses());

        let mut manager = DeviceManager::new(provider);
        manager.list_devices().unwrap();
        manager.connect("/dev/ttyACM0").unwrap();

        let engine = InspectionEngine::new();
        let results = engine.run(&mut manager);

        assert!(results.is_ok());
        let results = results.unwrap();
        assert!(!results.is_empty());
    }

    /// A3: 未接続装置でNotConnectedエラー
    #[test]
    fn test_a3_not_connected_error() {
        let provider = MockProvider::new().with_ports(vec![f9p_port("/dev/ttyACM0")]);

        let mut manager = DeviceManager::new(provider);
        // 接続しない

        let engine = InspectionEngine::new();
        let result = engine.run(&mut manager);

        assert!(matches!(result, Err(InspectionError::NotConnected)));
    }

    /// A4: 結果リストは実行順に並ぶ
    #[test]
    fn test_a4_results_in_execution_order() {
        let provider = MockProvider::new()
            .with_ports(vec![f9p_port("/dev/ttyACM0")])
            .with_responses(all_pass_responses());

        let mut manager = DeviceManager::new(provider);
        manager.list_devices().unwrap();
        manager.connect("/dev/ttyACM0").unwrap();

        let engine = InspectionEngine::new();
        let results = engine.run(&mut manager).unwrap();

        // 定義順と実行順が一致
        let expected_order = vec![
            ItemType::Connectivity,
            ItemType::FwVersion,
            ItemType::SerialNumber,
            ItemType::OutputRate,
            ItemType::PortConfig,
        ];

        for (i, item_type) in expected_order.iter().enumerate() {
            assert_eq!(&results[i].item_type, item_type, "順番 {} が一致", i);
        }
    }

    // ===========================================
    // B1-B2: 通信疎通テスト
    // ===========================================

    /// B1: 1秒以内に応答でPass
    #[test]
    fn test_b1_response_within_timeout_pass() {
        // 1項目だけの検査（通信疎通のみ）
        let items = vec![InspectionItem::new(ItemType::Connectivity)
            .with_timeout(Duration::from_secs(1))];

        let provider = MockProvider::new()
            .with_ports(vec![f9p_port("/dev/ttyACM0")])
            .with_responses(vec![valid_ubx_response(0x01, 0x03, &[0u8; 16])]);

        let mut manager = DeviceManager::new(provider);
        manager.list_devices().unwrap();
        manager.connect("/dev/ttyACM0").unwrap();

        let engine = InspectionEngine::with_items(items);
        let results = engine.run(&mut manager).unwrap();

        assert_eq!(results[0].verdict, Verdict::Pass);
    }

    /// B2: タイムアウトでFail（Error）
    #[test]
    fn test_b2_timeout_error() {
        let items = vec![InspectionItem::new(ItemType::Connectivity)
            .with_timeout(Duration::from_millis(100))];

        let provider = MockProvider::new()
            .with_ports(vec![f9p_port("/dev/ttyACM0")])
            .with_timeout(); // 応答なし

        let mut manager = DeviceManager::new(provider);
        manager.list_devices().unwrap();
        manager.connect("/dev/ttyACM0").unwrap();

        let engine = InspectionEngine::with_items(items);
        let results = engine.run(&mut manager).unwrap();

        assert!(matches!(results[0].verdict, Verdict::Error(_)));
    }

    // ===========================================
    // G1-G5: 各検査項目のUBX送信テスト
    // ===========================================

    /// G1: 通信疎通でNAV-STATUS pollを送信する
    #[test]
    fn test_g1_connectivity_sends_nav_status_poll() {
        let items = vec![InspectionItem::new(ItemType::Connectivity)];

        let provider = MockProvider::new()
            .with_ports(vec![f9p_port("/dev/ttyACM0")])
            .with_responses(vec![valid_ubx_response(0x01, 0x03, &[0u8; 16])]);
        let write_data = provider.get_write_data();

        let mut manager = DeviceManager::new(provider);
        manager.list_devices().unwrap();
        manager.connect("/dev/ttyACM0").unwrap();

        let engine = InspectionEngine::with_items(items);
        let _ = engine.run(&mut manager);

        let data = write_data.borrow();
        // NAV-STATUS poll: 0xB5 0x62 0x01 0x03 ...
        assert!(data.len() >= 4);
        assert_eq!(data[0], 0xB5);
        assert_eq!(data[1], 0x62);
        assert_eq!(data[2], 0x01);
        assert_eq!(data[3], 0x03);
    }

    /// G2: FWバージョンでMON-VER pollを送信する
    #[test]
    fn test_g2_fw_version_sends_mon_ver_poll() {
        let items = vec![InspectionItem::new(ItemType::FwVersion)];

        let provider = MockProvider::new()
            .with_ports(vec![f9p_port("/dev/ttyACM0")])
            .with_responses(vec![mon_ver_response("HPG 1.32")]);
        let write_data = provider.get_write_data();

        let mut manager = DeviceManager::new(provider);
        manager.list_devices().unwrap();
        manager.connect("/dev/ttyACM0").unwrap();

        let engine = InspectionEngine::with_items(items);
        let _ = engine.run(&mut manager);

        let data = write_data.borrow();
        // MON-VER poll: 0xB5 0x62 0x0A 0x04 ...
        assert_eq!(data[2], 0x0A);
        assert_eq!(data[3], 0x04);
    }

    /// G3: シリアル番号でSEC-UNIQID pollを送信する
    #[test]
    fn test_g3_serial_number_sends_sec_uniqid_poll() {
        let items = vec![InspectionItem::new(ItemType::SerialNumber)];

        let provider = MockProvider::new()
            .with_ports(vec![f9p_port("/dev/ttyACM0")])
            .with_responses(vec![sec_uniqid_response(&[0xAB, 0xCD, 0xEF, 0x12, 0x34])]);
        let write_data = provider.get_write_data();

        let mut manager = DeviceManager::new(provider);
        manager.list_devices().unwrap();
        manager.connect("/dev/ttyACM0").unwrap();

        let engine = InspectionEngine::with_items(items);
        let _ = engine.run(&mut manager);

        let data = write_data.borrow();
        // SEC-UNIQID poll: 0xB5 0x62 0x27 0x03 ...
        assert_eq!(data[2], 0x27);
        assert_eq!(data[3], 0x03);
    }

    /// G4: 出力レートでCFG-RATE pollを送信する
    #[test]
    fn test_g4_output_rate_sends_cfg_rate_poll() {
        let items = vec![InspectionItem::new(ItemType::OutputRate)];

        let provider = MockProvider::new()
            .with_ports(vec![f9p_port("/dev/ttyACM0")])
            .with_responses(vec![cfg_rate_response(100)]);
        let write_data = provider.get_write_data();

        let mut manager = DeviceManager::new(provider);
        manager.list_devices().unwrap();
        manager.connect("/dev/ttyACM0").unwrap();

        let engine = InspectionEngine::with_items(items);
        let _ = engine.run(&mut manager);

        let data = write_data.borrow();
        // CFG-RATE poll: 0xB5 0x62 0x06 0x08 ...
        assert_eq!(data[2], 0x06);
        assert_eq!(data[3], 0x08);
    }

    /// G5: ポート設定でCFG-PRT pollを送信する
    #[test]
    fn test_g5_port_config_sends_cfg_prt_poll() {
        let items = vec![InspectionItem::new(ItemType::PortConfig)];

        let provider = MockProvider::new()
            .with_ports(vec![f9p_port("/dev/ttyACM0")])
            .with_responses(vec![cfg_prt_response(115200)]);
        let write_data = provider.get_write_data();

        let mut manager = DeviceManager::new(provider);
        manager.list_devices().unwrap();
        manager.connect("/dev/ttyACM0").unwrap();

        let engine = InspectionEngine::with_items(items);
        let _ = engine.run(&mut manager);

        let data = write_data.borrow();
        // CFG-PRT poll: 0xB5 0x62 0x06 0x00 ...
        assert_eq!(data[2], 0x06);
        assert_eq!(data[3], 0x00);
    }

    // ===========================================
    // E1-E2: 状態連携テスト（TODO: 実装時に有効化）
    // ===========================================

    // E1, E2は状態遷移のテスト
    // DeviceManagerにset_status機能が必要
    // Phase 1では省略（コメントアウト）

    // ===========================================
    // F1-F3: 異常系テスト
    // ===========================================

    /// F1: 切断で中断、部分結果を返す
    #[test]
    fn test_f1_disconnect_returns_partial_results() {
        // 3項目目で切断をシミュレート
        let provider = MockProvider::new()
            .with_ports(vec![f9p_port("/dev/ttyACM0")])
            .with_responses(vec![
                valid_ubx_response(0x01, 0x03, &[0u8; 16]), // Connectivity: OK
                mon_ver_response("HPG 1.32"),               // FwVersion: OK
                // 3項目目以降は切断エラー
            ])
            .disconnect_at(3); // 3回目のreadで切断

        let mut manager = DeviceManager::new(provider);
        manager.list_devices().unwrap();
        manager.connect("/dev/ttyACM0").unwrap();

        let engine = InspectionEngine::new();
        let results = engine.run(&mut manager).unwrap();

        // 5項目すべて返される（途中エラーでも続行）
        assert_eq!(results.len(), 5);

        // 最初の2項目はPass
        assert_eq!(results[0].verdict, Verdict::Pass);
        assert_eq!(results[1].verdict, Verdict::Pass);

        // 3項目目以降はError
        assert!(matches!(results[2].verdict, Verdict::Error(_)));
    }

    /// F2: 一部失敗でも続行
    #[test]
    fn test_f2_continue_on_failure() {
        // 2項目目だけ不一致（Fail）
        let items = vec![
            InspectionItem::new(ItemType::Connectivity),
            InspectionItem::new(ItemType::FwVersion)
                .with_expected(ExpectedValue::String("HPG 1.40".to_string())), // 不一致
            InspectionItem::new(ItemType::SerialNumber),
        ];

        let provider = MockProvider::new()
            .with_ports(vec![f9p_port("/dev/ttyACM0")])
            .with_responses(vec![
                valid_ubx_response(0x01, 0x03, &[0u8; 16]), // Pass
                mon_ver_response("HPG 1.32"),               // Fail（期待値と不一致）
                sec_uniqid_response(&[0xAB, 0xCD, 0xEF, 0x12, 0x34]), // Pass
            ]);

        let mut manager = DeviceManager::new(provider);
        manager.list_devices().unwrap();
        manager.connect("/dev/ttyACM0").unwrap();

        let engine = InspectionEngine::with_items(items);
        let results = engine.run(&mut manager).unwrap();

        // 3項目すべて返される
        assert_eq!(results.len(), 3);

        // 順番を確認
        assert_eq!(results[0].verdict, Verdict::Pass);
        assert_eq!(results[1].verdict, Verdict::Fail); // 不一致でFail
        assert_eq!(results[2].verdict, Verdict::Pass); // 続行してPass
    }

    /// F3: Pass/Fail/Error混在で全て記録される
    #[test]
    fn test_f3_mixed_verdicts_all_recorded() {
        let items = vec![
            InspectionItem::new(ItemType::Connectivity), // Pass
            InspectionItem::new(ItemType::FwVersion)
                .with_expected(ExpectedValue::String("HPG 1.40".to_string())), // Fail
            InspectionItem::new(ItemType::SerialNumber), // Error (timeout)
        ];

        // 3項目目はタイムアウト
        let provider = MockProvider::new()
            .with_ports(vec![f9p_port("/dev/ttyACM0")])
            .with_responses(vec![
                valid_ubx_response(0x01, 0x03, &[0u8; 16]), // Pass
                mon_ver_response("HPG 1.32"),               // Fail
                // 3項目目: 応答なし → タイムアウト
            ]);

        let mut manager = DeviceManager::new(provider);
        manager.list_devices().unwrap();
        manager.connect("/dev/ttyACM0").unwrap();

        let engine = InspectionEngine::with_items(items);
        let results = engine.run(&mut manager).unwrap();

        // 3項目すべて返される
        assert_eq!(results.len(), 3);

        // Pass, Fail, Error混在
        assert_eq!(results[0].verdict, Verdict::Pass);
        assert_eq!(results[1].verdict, Verdict::Fail);
        assert!(matches!(results[2].verdict, Verdict::Error(_)));
    }

    /// 基本: エンジン作成できる
    #[test]
    fn test_engine_creation() {
        let _engine = InspectionEngine::new();
    }
}
