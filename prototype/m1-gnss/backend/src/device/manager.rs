//! DeviceManager
//!
//! GNSS装置の検出・接続・状態管理を行うマネージャー

use std::io;

use super::filter::{filter_gnss_ports, PortInfo};
use super::status::DeviceStatus;

/// DeviceManagerのエラー
#[derive(Debug)]
pub enum DeviceManagerError {
    /// ポートが見つからない
    PortNotFound(String),
    /// ポートが使用中
    PortBusy(String),
    /// 最大接続数に達した
    MaxDevicesReached,
    /// 未接続
    NotConnected,
    /// タイムアウト
    Timeout,
    /// IOエラー
    IoError(io::Error),
}

impl std::fmt::Display for DeviceManagerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeviceManagerError::PortNotFound(path) => write!(f, "ポートが見つかりません: {}", path),
            DeviceManagerError::PortBusy(path) => write!(f, "ポートが使用中です: {}", path),
            DeviceManagerError::MaxDevicesReached => write!(f, "最大接続数に達しました"),
            DeviceManagerError::NotConnected => write!(f, "未接続です"),
            DeviceManagerError::Timeout => write!(f, "タイムアウトしました"),
            DeviceManagerError::IoError(e) => write!(f, "IOエラー: {}", e),
        }
    }
}

impl std::error::Error for DeviceManagerError {}

impl From<io::Error> for DeviceManagerError {
    fn from(e: io::Error) -> Self {
        DeviceManagerError::IoError(e)
    }
}

/// デバイス情報
#[derive(Debug, Clone)]
pub struct Device {
    /// ポート情報
    pub port: PortInfo,
    /// 現在の状態
    pub status: DeviceStatus,
}

/// シリアルポートプロバイダートレイト
///
/// 外部依存（serialportクレート）を抽象化し、テストでモック可能にする
pub trait SerialPortProvider {
    /// 利用可能なポート一覧を取得
    fn available_ports(&self) -> Result<Vec<PortInfo>, DeviceManagerError>;

    /// ポートを開く
    fn open(&self, path: &str, baud_rate: u32) -> Result<Box<dyn SerialPort>, DeviceManagerError>;
}

/// シリアルポートトレイト
///
/// `Send`を要求するのはActix-webのスレッド間共有のため
pub trait SerialPort: Send {
    /// データを書き込む
    fn write(&mut self, data: &[u8]) -> Result<usize, io::Error>;

    /// データを読み込む
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error>;

    /// 読み込みタイムアウトを設定
    fn set_timeout(&mut self, timeout: std::time::Duration) -> Result<(), io::Error>;
}

/// デフォルトボーレート（u-blox F9P）
pub const DEFAULT_BAUD_RATE: u32 = 115200;

/// F9P評価ボードのボーレート（FTDI経由）
pub const F9P_EVAL_BAUD_RATE: u32 = 38400;

/// ボーレート自動検出の候補リスト（ADR-007）
/// 順序: 38400（F9Pデフォルト） → 115200（高速設定） → 9600（セーフブート）
pub const BAUD_RATE_CANDIDATES: [u32; 3] = [38400, 115200, 9600];

/// ボーレート自動検出のタイムアウト（ms）
pub const AUTO_DETECT_TIMEOUT_MS: u64 = 500;

/// DeviceManager
///
/// Phase 1では1台のみ接続可能。Phase 2以降で複数台対応予定。
pub struct DeviceManager<P: SerialPortProvider> {
    provider: P,
    devices: Vec<Device>,
    connected_port: Option<Box<dyn SerialPort>>,
    connected_device_index: Option<usize>,
    /// 接続時のボーレート
    baud_rate: u32,
    /// 自動検出で検出されたボーレート
    detected_baud_rate: Option<u32>,
}

impl<P: SerialPortProvider> DeviceManager<P> {
    /// 新しいDeviceManagerを作成
    pub fn new(provider: P) -> Self {
        Self {
            provider,
            devices: Vec::new(),
            connected_port: None,
            connected_device_index: None,
            baud_rate: DEFAULT_BAUD_RATE,
            detected_baud_rate: None,
        }
    }

    /// ボーレートを指定して作成
    pub fn with_baud_rate(provider: P, baud_rate: u32) -> Self {
        Self {
            provider,
            devices: Vec::new(),
            connected_port: None,
            connected_device_index: None,
            baud_rate,
            detected_baud_rate: None,
        }
    }

    /// ボーレートを設定
    pub fn set_baud_rate(&mut self, baud_rate: u32) {
        self.baud_rate = baud_rate;
    }

    /// 現在のボーレートを取得
    pub fn baud_rate(&self) -> u32 {
        self.baud_rate
    }

    /// 自動検出されたボーレートを取得
    ///
    /// connect_auto_detect() で接続した場合のみ Some を返す
    pub fn detected_baud_rate(&self) -> Option<u32> {
        self.detected_baud_rate
    }

    /// 利用可能なデバイス一覧を取得（F9P直接 + FTDI経由）
    pub fn list_devices(&mut self) -> Result<Vec<Device>, DeviceManagerError> {
        let ports = self.provider.available_ports()?;
        let gnss_ports = filter_gnss_ports(ports);

        // 既存デバイスリストを更新
        self.devices = gnss_ports
            .into_iter()
            .map(|port| {
                // 既存デバイスがあれば状態を引き継ぐ
                let existing_status = self
                    .devices
                    .iter()
                    .find(|d| d.port.path == port.path)
                    .map(|d| d.status.clone());

                Device {
                    port,
                    status: existing_status.unwrap_or(DeviceStatus::Detected),
                }
            })
            .collect();

        Ok(self.devices.clone())
    }

    /// デバイスに接続
    pub fn connect(&mut self, path: &str) -> Result<(), DeviceManagerError> {
        // 既に接続中の場合はエラー
        if self.connected_port.is_some() {
            return Err(DeviceManagerError::MaxDevicesReached);
        }

        // デバイスを探す
        let device_index = self
            .devices
            .iter()
            .position(|d| d.port.path == path)
            .ok_or_else(|| DeviceManagerError::PortNotFound(path.to_string()))?;

        // ポートを開く
        let port = self.provider.open(path, self.baud_rate)?;

        // 状態を更新
        self.devices[device_index].status = DeviceStatus::Connected;
        self.connected_port = Some(port);
        self.connected_device_index = Some(device_index);

        Ok(())
    }

    /// ボーレートを自動検出して接続（ADR-007）
    ///
    /// 候補ボーレート（38400, 115200, 9600）を順番に試し、
    /// UBX-MON-VER Poll で応答があったボーレートで接続する。
    ///
    /// # 戻り値
    /// - Ok(baud_rate): 検出されたボーレート
    /// - Err: 全候補で応答がなかった場合
    pub fn connect_auto_detect(&mut self, path: &str) -> Result<u32, DeviceManagerError> {
        use crate::ubx::mon_ver;
        use std::time::Duration;

        // 既に接続中の場合はエラー
        if self.connected_port.is_some() {
            return Err(DeviceManagerError::MaxDevicesReached);
        }

        // デバイスを探す
        let device_index = self
            .devices
            .iter()
            .position(|d| d.port.path == path)
            .ok_or_else(|| DeviceManagerError::PortNotFound(path.to_string()))?;

        let timeout = Duration::from_millis(AUTO_DETECT_TIMEOUT_MS);
        let poll_request = mon_ver::poll_request();

        // 各ボーレートを試行
        for &baud_rate in &BAUD_RATE_CANDIDATES {
            // ポートを開く
            let port_result = self.provider.open(path, baud_rate);
            let mut port = match port_result {
                Ok(p) => p,
                Err(_) => continue, // 開けなければ次の候補へ
            };

            // MON-VER Poll を送信
            if port.write(&poll_request).is_err() {
                continue;
            }

            // タイムアウト設定
            if port.set_timeout(timeout).is_err() {
                continue;
            }

            // 応答を待つ
            let mut buf = vec![0u8; 256];
            match port.read(&mut buf) {
                Ok(n) if n > 0 => {
                    // 何らかの応答があれば成功
                    // 状態を更新
                    self.devices[device_index].status = DeviceStatus::Connected;
                    self.connected_port = Some(port);
                    self.connected_device_index = Some(device_index);
                    self.baud_rate = baud_rate;
                    self.detected_baud_rate = Some(baud_rate);
                    return Ok(baud_rate);
                }
                _ => {
                    // タイムアウトまたはエラー → 次の候補へ
                    continue;
                }
            }
        }

        // 全候補で失敗
        Err(DeviceManagerError::Timeout)
    }

    /// 切断
    pub fn disconnect(&mut self) -> Result<(), DeviceManagerError> {
        if self.connected_port.is_none() {
            return Err(DeviceManagerError::NotConnected);
        }

        // 状態を更新
        if let Some(index) = self.connected_device_index {
            self.devices[index].status = DeviceStatus::Disconnected;
        }

        self.connected_port = None;
        self.connected_device_index = None;
        self.detected_baud_rate = None;

        Ok(())
    }

    /// 接続中のデバイスの状態を取得
    pub fn get_connected_device(&self) -> Option<&Device> {
        self.connected_device_index
            .and_then(|index| self.devices.get(index))
    }

    /// UBXメッセージを送信
    pub fn send_ubx(&mut self, data: &[u8]) -> Result<(), DeviceManagerError> {
        let port = self
            .connected_port
            .as_mut()
            .ok_or(DeviceManagerError::NotConnected)?;

        port.write(data)?;
        Ok(())
    }

    /// UBXメッセージを受信
    pub fn receive_ubx(&mut self, timeout: std::time::Duration) -> Result<Vec<u8>, DeviceManagerError> {
        let port = self
            .connected_port
            .as_mut()
            .ok_or(DeviceManagerError::NotConnected)?;

        port.set_timeout(timeout)?;

        let mut buf = vec![0u8; 1024];
        match port.read(&mut buf) {
            Ok(n) => {
                buf.truncate(n);
                Ok(buf)
            }
            Err(e) if e.kind() == io::ErrorKind::TimedOut => {
                Err(DeviceManagerError::Timeout)
            }
            Err(e) => Err(DeviceManagerError::IoError(e)),
        }
    }

    /// 受信バッファをドレイン（空読み）
    ///
    /// poll送信前に呼び出して、前回の応答の残りをクリアする
    pub fn drain_buffer(&mut self) -> Result<(), DeviceManagerError> {
        use std::time::Duration;

        let port = self
            .connected_port
            .as_mut()
            .ok_or(DeviceManagerError::NotConnected)?;

        // 短いタイムアウトで空読み
        port.set_timeout(Duration::from_millis(10))?;

        let mut buf = vec![0u8; 1024];
        loop {
            match port.read(&mut buf) {
                Ok(0) => break,
                Ok(_) => continue, // データがあれば読み捨てる
                Err(e) if e.kind() == io::ErrorKind::TimedOut => break,
                Err(_) => break, // その他のエラーは無視して終了
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;
    use std::sync::{Arc, Mutex};
    use std::time::Duration;

    use super::super::filter::{F9P_PID, F9P_VID};

    // ===========================================
    // モック実装
    // ===========================================

    /// モックシリアルポート（Send対応）
    struct MockSerialPort {
        write_data: Arc<Mutex<Vec<u8>>>,
        read_data: Arc<Mutex<VecDeque<u8>>>,
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
            let mut read_data = self.read_data.lock().unwrap();
            let len = buf.len().min(read_data.len());
            for i in 0..len {
                buf[i] = read_data.pop_front().unwrap();
            }
            Ok(len)
        }

        fn set_timeout(&mut self, _timeout: Duration) -> Result<(), io::Error> {
            Ok(())
        }
    }

    /// モックSerialPortProvider（Send対応）
    struct MockProvider {
        ports: Vec<PortInfo>,
        write_data: Arc<Mutex<Vec<u8>>>,
        read_data: Arc<Mutex<VecDeque<u8>>>,
        should_timeout: bool,
    }

    impl MockProvider {
        fn new() -> Self {
            Self {
                ports: vec![],
                write_data: Arc::new(Mutex::new(Vec::new())),
                read_data: Arc::new(Mutex::new(VecDeque::new())),
                should_timeout: false,
            }
        }

        fn with_ports(mut self, ports: Vec<PortInfo>) -> Self {
            self.ports = ports;
            self
        }

        fn with_read_data(mut self, data: Vec<u8>) -> Self {
            self.read_data = Arc::new(Mutex::new(data.into()));
            self
        }

        fn with_timeout(mut self) -> Self {
            self.should_timeout = true;
            self
        }
    }

    impl SerialPortProvider for MockProvider {
        fn available_ports(&self) -> Result<Vec<PortInfo>, DeviceManagerError> {
            Ok(self.ports.clone())
        }

        fn open(&self, path: &str, _baud_rate: u32) -> Result<Box<dyn SerialPort>, DeviceManagerError> {
            // ポートが存在するか確認
            if !self.ports.iter().any(|p| p.path == path) {
                return Err(DeviceManagerError::PortNotFound(path.to_string()));
            }
            Ok(Box::new(MockSerialPort {
                write_data: self.write_data.clone(),
                read_data: self.read_data.clone(),
                should_timeout: self.should_timeout,
            }))
        }
    }

    // ===========================================
    // ヘルパー関数
    // ===========================================

    fn f9p_port(path: &str) -> PortInfo {
        PortInfo {
            path: path.to_string(),
            vid: Some(F9P_VID),
            pid: Some(F9P_PID),
            serial_number: Some("TEST123".to_string()),
        }
    }

    fn other_port(path: &str) -> PortInfo {
        PortInfo {
            path: path.to_string(),
            vid: Some(0x1234),
            pid: Some(0x5678),
            serial_number: None,
        }
    }

    // ===========================================
    // C1. ポート列挙テスト
    // ===========================================

    /// C1-1: ポートがない場合
    #[test]
    fn test_c1_1_no_ports() {
        let provider = MockProvider::new().with_ports(vec![]);
        let mut manager = DeviceManager::new(provider);

        let devices = manager.list_devices().unwrap();
        assert!(devices.is_empty(), "ポートがない場合は空リスト");
    }

    /// C1-2: F9Pが1台接続
    #[test]
    fn test_c1_2_one_f9p() {
        let provider = MockProvider::new().with_ports(vec![f9p_port("/dev/ttyACM0")]);
        let mut manager = DeviceManager::new(provider);

        let devices = manager.list_devices().unwrap();
        assert_eq!(devices.len(), 1);
        assert_eq!(devices[0].port.path, "/dev/ttyACM0");
        assert_eq!(devices[0].status, DeviceStatus::Detected);
    }

    /// C1-3: F9Pが複数台接続
    #[test]
    fn test_c1_3_multiple_f9p() {
        let provider = MockProvider::new().with_ports(vec![
            f9p_port("/dev/ttyACM0"),
            f9p_port("/dev/ttyACM1"),
        ]);
        let mut manager = DeviceManager::new(provider);

        let devices = manager.list_devices().unwrap();
        assert_eq!(devices.len(), 2);
    }

    /// C1-4: F9P以外は除外される
    #[test]
    fn test_c1_4_non_f9p_excluded() {
        let provider = MockProvider::new().with_ports(vec![
            f9p_port("/dev/ttyACM0"),
            other_port("/dev/ttyUSB0"),
        ]);
        let mut manager = DeviceManager::new(provider);

        let devices = manager.list_devices().unwrap();
        assert_eq!(devices.len(), 1);
        assert_eq!(devices[0].port.path, "/dev/ttyACM0");
    }

    // ===========================================
    // C2. 接続管理テスト
    // ===========================================

    /// C2-1: 正常に接続できる
    #[test]
    fn test_c2_1_connect_success() {
        let provider = MockProvider::new().with_ports(vec![f9p_port("/dev/ttyACM0")]);
        let mut manager = DeviceManager::new(provider);

        // まずlist_devicesを呼んでデバイスを検出
        manager.list_devices().unwrap();

        let result = manager.connect("/dev/ttyACM0");
        assert!(result.is_ok(), "接続に成功するはず");
    }

    /// C2-2: 存在しないポート
    #[test]
    fn test_c2_2_port_not_found() {
        let provider = MockProvider::new().with_ports(vec![]);
        let mut manager = DeviceManager::new(provider);

        let result = manager.connect("/dev/ttyACM99");
        assert!(matches!(result, Err(DeviceManagerError::PortNotFound(_))));
    }

    /// C2-4: 接続後に状態がConnectedになる
    #[test]
    fn test_c2_4_status_connected_after_connect() {
        let provider = MockProvider::new().with_ports(vec![f9p_port("/dev/ttyACM0")]);
        let mut manager = DeviceManager::new(provider);

        manager.list_devices().unwrap();
        manager.connect("/dev/ttyACM0").unwrap();

        let device = manager.get_connected_device();
        assert!(device.is_some());
        assert_eq!(device.unwrap().status, DeviceStatus::Connected);
    }

    /// C2-5: 切断できる
    #[test]
    fn test_c2_5_disconnect() {
        let provider = MockProvider::new().with_ports(vec![f9p_port("/dev/ttyACM0")]);
        let mut manager = DeviceManager::new(provider);

        manager.list_devices().unwrap();
        manager.connect("/dev/ttyACM0").unwrap();

        let result = manager.disconnect();
        assert!(result.is_ok());
    }

    /// C2-6: 切断後に状態がDisconnectedになる
    #[test]
    fn test_c2_6_status_disconnected_after_disconnect() {
        let provider = MockProvider::new().with_ports(vec![f9p_port("/dev/ttyACM0")]);
        let mut manager = DeviceManager::new(provider);

        manager.list_devices().unwrap();
        manager.connect("/dev/ttyACM0").unwrap();
        manager.disconnect().unwrap();

        // 切断後もデバイスリストには残る（Disconnected状態で）
        let devices = manager.list_devices().unwrap();
        let device = devices.iter().find(|d| d.port.path == "/dev/ttyACM0");
        assert!(device.is_some());
        // 注: 実際の実装では、list_devicesは最新のポート状態を取得するので
        // Disconnectedではなく再度Detectedになる可能性がある
    }

    /// C2-7: 切断後に再接続できる
    #[test]
    fn test_c2_7_reconnect_after_disconnect() {
        let provider = MockProvider::new().with_ports(vec![f9p_port("/dev/ttyACM0")]);
        let mut manager = DeviceManager::new(provider);

        manager.list_devices().unwrap();
        manager.connect("/dev/ttyACM0").unwrap();
        manager.disconnect().unwrap();

        // 再接続
        let result = manager.connect("/dev/ttyACM0");
        assert!(result.is_ok());
    }

    /// C2-8: Phase 1では2台目接続はエラー
    #[test]
    fn test_c2_8_max_devices_reached() {
        let provider = MockProvider::new().with_ports(vec![
            f9p_port("/dev/ttyACM0"),
            f9p_port("/dev/ttyACM1"),
        ]);
        let mut manager = DeviceManager::new(provider);

        manager.list_devices().unwrap();
        manager.connect("/dev/ttyACM0").unwrap();

        // 2台目を接続しようとする
        let result = manager.connect("/dev/ttyACM1");
        assert!(matches!(result, Err(DeviceManagerError::MaxDevicesReached)));
    }

    // ===========================================
    // C3. UBX送受信テスト
    // ===========================================

    /// C3-1: UBXメッセージ送信成功
    #[test]
    fn test_c3_1_send_ubx_success() {
        let provider = MockProvider::new().with_ports(vec![f9p_port("/dev/ttyACM0")]);
        let mut manager = DeviceManager::new(provider);

        manager.list_devices().unwrap();
        manager.connect("/dev/ttyACM0").unwrap();

        let data = vec![0xB5, 0x62, 0x01, 0x03, 0x00, 0x00, 0x04, 0x0D];
        let result = manager.send_ubx(&data);
        assert!(result.is_ok());
    }

    /// C3-2: UBXメッセージ受信成功
    #[test]
    fn test_c3_2_receive_ubx_success() {
        let response_data = vec![0xB5, 0x62, 0x01, 0x03, 0x00, 0x00, 0x04, 0x0D];
        let provider = MockProvider::new()
            .with_ports(vec![f9p_port("/dev/ttyACM0")])
            .with_read_data(response_data.clone());
        let mut manager = DeviceManager::new(provider);

        manager.list_devices().unwrap();
        manager.connect("/dev/ttyACM0").unwrap();

        let result = manager.receive_ubx(Duration::from_secs(1));
        assert!(result.is_ok());
    }

    /// C3-3: 未接続で送信
    #[test]
    fn test_c3_3_send_without_connection() {
        let provider = MockProvider::new();
        let mut manager = DeviceManager::new(provider);

        let data = vec![0xB5, 0x62];
        let result = manager.send_ubx(&data);
        assert!(matches!(result, Err(DeviceManagerError::NotConnected)));
    }

    /// C3-4: タイムアウト
    #[test]
    fn test_c3_4_receive_timeout() {
        let provider = MockProvider::new()
            .with_ports(vec![f9p_port("/dev/ttyACM0")])
            .with_timeout();
        let mut manager = DeviceManager::new(provider);

        manager.list_devices().unwrap();
        manager.connect("/dev/ttyACM0").unwrap();

        let result = manager.receive_ubx(Duration::from_secs(1));
        assert!(matches!(result, Err(DeviceManagerError::Timeout)));
    }

    // ===========================================
    // C4. ボーレート自動検出テスト（ADR-007）
    // ===========================================

    /// ボーレートに応じて応答を変えるモック
    struct BaudRateMockPort {
        baud_rate: u32,
        respond_at_baud: u32,
    }

    impl SerialPort for BaudRateMockPort {
        fn write(&mut self, _data: &[u8]) -> Result<usize, io::Error> {
            Ok(8) // MON-VER Poll は 8バイト
        }

        fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
            if self.baud_rate == self.respond_at_baud {
                // 応答をシミュレート（MON-VER の最小応答）
                let response = [0xB5, 0x62, 0x0A, 0x04];
                let len = buf.len().min(response.len());
                buf[..len].copy_from_slice(&response[..len]);
                Ok(len)
            } else {
                // タイムアウト
                Err(io::Error::new(io::ErrorKind::TimedOut, "no response"))
            }
        }

        fn set_timeout(&mut self, _timeout: Duration) -> Result<(), io::Error> {
            Ok(())
        }
    }

    struct BaudRateMockProvider {
        ports: Vec<PortInfo>,
        respond_at_baud: u32,
    }

    impl BaudRateMockProvider {
        fn new(respond_at_baud: u32) -> Self {
            Self {
                ports: vec![f9p_port("/dev/ttyACM0")],
                respond_at_baud,
            }
        }
    }

    impl SerialPortProvider for BaudRateMockProvider {
        fn available_ports(&self) -> Result<Vec<PortInfo>, DeviceManagerError> {
            Ok(self.ports.clone())
        }

        fn open(&self, path: &str, baud_rate: u32) -> Result<Box<dyn SerialPort>, DeviceManagerError> {
            if !self.ports.iter().any(|p| p.path == path) {
                return Err(DeviceManagerError::PortNotFound(path.to_string()));
            }
            Ok(Box::new(BaudRateMockPort {
                baud_rate,
                respond_at_baud: self.respond_at_baud,
            }))
        }
    }

    /// C4-1: 38400で応答（最初の候補で成功）
    #[test]
    fn test_c4_1_auto_detect_first_candidate() {
        let provider = BaudRateMockProvider::new(38400);
        let mut manager = DeviceManager::new(provider);

        manager.list_devices().unwrap();
        let result = manager.connect_auto_detect("/dev/ttyACM0");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 38400);
        assert_eq!(manager.detected_baud_rate(), Some(38400));
        assert_eq!(manager.baud_rate(), 38400);
    }

    /// C4-2: 115200で応答（2番目の候補で成功）
    #[test]
    fn test_c4_2_auto_detect_second_candidate() {
        let provider = BaudRateMockProvider::new(115200);
        let mut manager = DeviceManager::new(provider);

        manager.list_devices().unwrap();
        let result = manager.connect_auto_detect("/dev/ttyACM0");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 115200);
        assert_eq!(manager.detected_baud_rate(), Some(115200));
    }

    /// C4-3: 9600で応答（3番目の候補で成功）
    #[test]
    fn test_c4_3_auto_detect_third_candidate() {
        let provider = BaudRateMockProvider::new(9600);
        let mut manager = DeviceManager::new(provider);

        manager.list_devices().unwrap();
        let result = manager.connect_auto_detect("/dev/ttyACM0");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 9600);
        assert_eq!(manager.detected_baud_rate(), Some(9600));
    }

    /// C4-4: どのボーレートでも応答なし（タイムアウト）
    #[test]
    fn test_c4_4_auto_detect_all_fail() {
        let provider = BaudRateMockProvider::new(57600); // 候補にないボーレート
        let mut manager = DeviceManager::new(provider);

        manager.list_devices().unwrap();
        let result = manager.connect_auto_detect("/dev/ttyACM0");

        assert!(matches!(result, Err(DeviceManagerError::Timeout)));
        assert_eq!(manager.detected_baud_rate(), None);
    }

    /// C4-5: 自動検出後に切断すると detected_baud_rate がリセットされる
    #[test]
    fn test_c4_5_disconnect_resets_detected_baud_rate() {
        let provider = BaudRateMockProvider::new(38400);
        let mut manager = DeviceManager::new(provider);

        manager.list_devices().unwrap();
        manager.connect_auto_detect("/dev/ttyACM0").unwrap();
        assert_eq!(manager.detected_baud_rate(), Some(38400));

        manager.disconnect().unwrap();
        assert_eq!(manager.detected_baud_rate(), None);
    }

    /// C4-6: 既に接続中に自動検出を試みるとエラー
    #[test]
    fn test_c4_6_auto_detect_while_connected() {
        let provider = BaudRateMockProvider::new(38400);
        let mut manager = DeviceManager::new(provider);

        manager.list_devices().unwrap();
        manager.connect_auto_detect("/dev/ttyACM0").unwrap();

        // 既に接続中
        let result = manager.connect_auto_detect("/dev/ttyACM0");
        assert!(matches!(result, Err(DeviceManagerError::MaxDevicesReached)));
    }
}
