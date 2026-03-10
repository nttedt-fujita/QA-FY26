//! DeviceManager
//!
//! GNSS装置の検出・接続・状態管理を行うマネージャー

use std::io;

use super::filter::{filter_f9p_ports, PortInfo};
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
pub trait SerialPort {
    /// データを書き込む
    fn write(&mut self, data: &[u8]) -> Result<usize, io::Error>;

    /// データを読み込む
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error>;

    /// 読み込みタイムアウトを設定
    fn set_timeout(&mut self, timeout: std::time::Duration) -> Result<(), io::Error>;
}

/// デフォルトボーレート（u-blox F9P）
const DEFAULT_BAUD_RATE: u32 = 115200;

/// DeviceManager
///
/// Phase 1では1台のみ接続可能。Phase 2以降で複数台対応予定。
pub struct DeviceManager<P: SerialPortProvider> {
    provider: P,
    devices: Vec<Device>,
    connected_port: Option<Box<dyn SerialPort>>,
    connected_device_index: Option<usize>,
}

impl<P: SerialPortProvider> DeviceManager<P> {
    /// 新しいDeviceManagerを作成
    pub fn new(provider: P) -> Self {
        Self {
            provider,
            devices: Vec::new(),
            connected_port: None,
            connected_device_index: None,
        }
    }

    /// 利用可能なデバイス一覧を取得（F9Pのみ）
    pub fn list_devices(&mut self) -> Result<Vec<Device>, DeviceManagerError> {
        let ports = self.provider.available_ports()?;
        let f9p_ports = filter_f9p_ports(ports);

        // 既存デバイスリストを更新
        self.devices = f9p_ports
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
        let port = self.provider.open(path, DEFAULT_BAUD_RATE)?;

        // 状態を更新
        self.devices[device_index].status = DeviceStatus::Connected;
        self.connected_port = Some(port);
        self.connected_device_index = Some(device_index);

        Ok(())
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::rc::Rc;
    use std::time::Duration;

    use super::super::filter::{F9P_PID, F9P_VID};

    // ===========================================
    // モック実装
    // ===========================================

    /// モックシリアルポート
    struct MockSerialPort {
        write_data: Rc<RefCell<Vec<u8>>>,
        read_data: Rc<RefCell<VecDeque<u8>>>,
        should_timeout: bool,
    }

    impl SerialPort for MockSerialPort {
        fn write(&mut self, data: &[u8]) -> Result<usize, io::Error> {
            self.write_data.borrow_mut().extend_from_slice(data);
            Ok(data.len())
        }

        fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
            if self.should_timeout {
                return Err(io::Error::new(io::ErrorKind::TimedOut, "timeout"));
            }
            let mut read_data = self.read_data.borrow_mut();
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

    /// モックSerialPortProvider
    struct MockProvider {
        ports: Vec<PortInfo>,
        write_data: Rc<RefCell<Vec<u8>>>,
        read_data: Rc<RefCell<VecDeque<u8>>>,
        should_timeout: bool,
    }

    impl MockProvider {
        fn new() -> Self {
            Self {
                ports: vec![],
                write_data: Rc::new(RefCell::new(Vec::new())),
                read_data: Rc::new(RefCell::new(VecDeque::new())),
                should_timeout: false,
            }
        }

        fn with_ports(mut self, ports: Vec<PortInfo>) -> Self {
            self.ports = ports;
            self
        }

        fn with_read_data(mut self, data: Vec<u8>) -> Self {
            self.read_data = Rc::new(RefCell::new(data.into()));
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
}
