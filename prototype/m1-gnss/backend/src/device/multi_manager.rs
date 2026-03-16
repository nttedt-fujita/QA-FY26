//! MultiDeviceManager
//!
//! 複数GNSS装置の同時管理を行うマネージャー
//! Phase 3（複数台同時対応）で使用
//!
//! 設計: ADR-014, session194/multi-device-manager-design.md

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::filter::PortInfo;
use super::manager::{DeviceManager, DeviceManagerError, SerialPortProvider};

/// デバイス情報（API用）
#[derive(Debug, Clone)]
pub struct DeviceInfo {
    /// ポートパス
    pub path: String,
    /// ポート情報
    pub port: PortInfo,
    /// 接続状態
    pub connected: bool,
    /// ボーレート（接続中のみ）
    pub baud_rate: Option<u32>,
}

/// MultiDeviceManager
///
/// 複数のDeviceManagerを管理し、複数台の同時接続を実現する
pub struct MultiDeviceManager<P: SerialPortProvider + Clone> {
    /// デバイスごとのマネージャー（パス -> DeviceManager）
    managers: HashMap<String, Arc<Mutex<DeviceManager<P>>>>,
    /// ポートプロバイダー（複製可能）
    provider: P,
}

impl<P: SerialPortProvider + Clone> MultiDeviceManager<P> {
    /// 新しいMultiDeviceManagerを作成
    pub fn new(provider: P) -> Self {
        Self {
            managers: HashMap::new(),
            provider,
        }
    }

    /// 利用可能なデバイス一覧を取得
    ///
    /// ポートをスキャンし、各ポートの接続状態を付与して返す
    pub fn list_devices(&self) -> Result<Vec<DeviceInfo>, DeviceManagerError> {
        // ポートをスキャンするための一時的なマネージャーを作成
        let mut temp_manager = DeviceManager::new(self.provider.clone());
        let devices = temp_manager.list_devices()?;

        // 各デバイスの接続状態を確認
        let device_infos: Vec<DeviceInfo> = devices
            .iter()
            .map(|d| {
                let connected = self.managers.contains_key(&d.port.path);
                let baud_rate = if connected {
                    self.managers
                        .get(&d.port.path)
                        .and_then(|m| m.lock().ok())
                        .and_then(|m| m.detected_baud_rate())
                } else {
                    None
                };
                DeviceInfo {
                    path: d.port.path.clone(),
                    port: d.port.clone(),
                    connected,
                    baud_rate,
                }
            })
            .collect();

        Ok(device_infos)
    }

    /// 特定デバイスに接続
    ///
    /// 新しいDeviceManagerを作成し、指定ポートに接続する
    ///
    /// # Returns
    /// 検出されたボーレート
    pub fn connect(&mut self, path: &str) -> Result<u32, DeviceManagerError> {
        // 既に接続中なら重複接続エラー
        if self.managers.contains_key(path) {
            return Err(DeviceManagerError::PortBusy(path.to_string()));
        }

        // 新しいDeviceManagerを作成
        let mut manager = DeviceManager::new(self.provider.clone());

        // デバイスをスキャン
        manager.list_devices()?;

        // 自動検出で接続
        let baud_rate = manager.connect_auto_detect(path)?;

        // マネージャーを登録
        self.managers
            .insert(path.to_string(), Arc::new(Mutex::new(manager)));

        Ok(baud_rate)
    }

    /// 特定デバイスを切断
    pub fn disconnect(&mut self, path: &str) -> Result<(), DeviceManagerError> {
        // マネージャーを取得
        let manager_arc = self
            .managers
            .remove(path)
            .ok_or_else(|| DeviceManagerError::NotConnected)?;

        // 切断処理
        let mut manager = manager_arc
            .lock()
            .map_err(|_| DeviceManagerError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                "ロック取得に失敗",
            )))?;

        manager.disconnect()?;
        Ok(())
    }

    /// 特定デバイスのマネージャーを取得
    ///
    /// 並行処理用に Arc<Mutex<DeviceManager>> を返す
    pub fn get_manager(&self, path: &str) -> Option<Arc<Mutex<DeviceManager<P>>>> {
        self.managers.get(path).cloned()
    }

    /// 接続中のデバイスパス一覧を取得
    ///
    /// パス名でソートして返す（HashMapは順序を保証しないため）
    pub fn connected_paths(&self) -> Vec<String> {
        let mut paths: Vec<String> = self.managers.keys().cloned().collect();
        paths.sort();
        paths
    }

    /// 接続中のデバイス数を取得
    pub fn connected_count(&self) -> usize {
        self.managers.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;
    use std::io;
    use std::time::Duration;

    use super::super::filter::{F9P_VID, F9P_PID, FTDI_VID, FTDI_FT232H_PID};

    // ===========================================
    // モック実装
    // ===========================================

    /// モックシリアルポート
    struct MockSerialPort {
        #[allow(dead_code)]
        read_data: Arc<Mutex<VecDeque<u8>>>,
        respond_at_baud: u32,
        current_baud: u32,
        timeout: Duration,
    }

    impl io::Read for MockSerialPort {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
            // ボーレートが一致したら応答
            if self.current_baud == self.respond_at_baud {
                let response = [0xB5, 0x62, 0x0A, 0x04];
                let len = buf.len().min(response.len());
                buf[..len].copy_from_slice(&response[..len]);
                Ok(len)
            } else {
                Err(io::Error::new(io::ErrorKind::TimedOut, "no response"))
            }
        }
    }

    impl io::Write for MockSerialPort {
        fn write(&mut self, _data: &[u8]) -> Result<usize, io::Error> {
            Ok(8)
        }

        fn flush(&mut self) -> Result<(), io::Error> {
            Ok(())
        }
    }

    impl crate::device::manager::SerialPort for MockSerialPort {
        fn name(&self) -> Option<String> { Some("mock".to_string()) }
        fn baud_rate(&self) -> serialport::Result<u32> { Ok(self.current_baud) }
        fn data_bits(&self) -> serialport::Result<serialport::DataBits> { Ok(serialport::DataBits::Eight) }
        fn flow_control(&self) -> serialport::Result<serialport::FlowControl> { Ok(serialport::FlowControl::None) }
        fn parity(&self) -> serialport::Result<serialport::Parity> { Ok(serialport::Parity::None) }
        fn stop_bits(&self) -> serialport::Result<serialport::StopBits> { Ok(serialport::StopBits::One) }
        fn timeout(&self) -> Duration { self.timeout }
        fn set_baud_rate(&mut self, _: u32) -> serialport::Result<()> { Ok(()) }
        fn set_data_bits(&mut self, _: serialport::DataBits) -> serialport::Result<()> { Ok(()) }
        fn set_flow_control(&mut self, _: serialport::FlowControl) -> serialport::Result<()> { Ok(()) }
        fn set_parity(&mut self, _: serialport::Parity) -> serialport::Result<()> { Ok(()) }
        fn set_stop_bits(&mut self, _: serialport::StopBits) -> serialport::Result<()> { Ok(()) }
        fn set_timeout(&mut self, timeout: Duration) -> serialport::Result<()> {
            self.timeout = timeout;
            Ok(())
        }
        fn write_request_to_send(&mut self, _: bool) -> serialport::Result<()> { Ok(()) }
        fn write_data_terminal_ready(&mut self, _: bool) -> serialport::Result<()> { Ok(()) }
        fn read_clear_to_send(&mut self) -> serialport::Result<bool> { Ok(false) }
        fn read_data_set_ready(&mut self) -> serialport::Result<bool> { Ok(false) }
        fn read_ring_indicator(&mut self) -> serialport::Result<bool> { Ok(false) }
        fn read_carrier_detect(&mut self) -> serialport::Result<bool> { Ok(false) }
        fn bytes_to_read(&self) -> serialport::Result<u32> { Ok(0) }
        fn bytes_to_write(&self) -> serialport::Result<u32> { Ok(0) }
        fn clear(&self, _: serialport::ClearBuffer) -> serialport::Result<()> { Ok(()) }
        fn try_clone(&self) -> serialport::Result<Box<dyn crate::device::manager::SerialPort>> {
            Err(serialport::Error::new(serialport::ErrorKind::Unknown, "clone not supported"))
        }
        fn set_break(&self) -> serialport::Result<()> { Ok(()) }
        fn clear_break(&self) -> serialport::Result<()> { Ok(()) }
    }

    /// モックシリアルポートプロバイダー（Clone対応）
    #[derive(Clone)]
    struct MockProvider {
        ports: Vec<PortInfo>,
        respond_at_baud: u32,
    }

    impl MockProvider {
        fn new() -> Self {
            Self {
                ports: vec![],
                respond_at_baud: 38400,
            }
        }

        fn with_ports(mut self, ports: Vec<PortInfo>) -> Self {
            self.ports = ports;
            self
        }

        fn with_respond_at(mut self, baud: u32) -> Self {
            self.respond_at_baud = baud;
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
            baud_rate: u32,
        ) -> Result<Box<dyn crate::device::manager::SerialPort>, DeviceManagerError> {
            if !self.ports.iter().any(|p| p.path == path) {
                return Err(DeviceManagerError::PortNotFound(path.to_string()));
            }
            Ok(Box::new(MockSerialPort {
                read_data: Arc::new(Mutex::new(VecDeque::new())),
                respond_at_baud: self.respond_at_baud,
                current_baud: baud_rate,
                timeout: Duration::from_secs(1),
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

    fn ftdi_port(path: &str) -> PortInfo {
        PortInfo {
            path: path.to_string(),
            vid: Some(FTDI_VID),
            pid: Some(FTDI_FT232H_PID),
            serial_number: None,
        }
    }

    // ===========================================
    // M1. デバイス検出テスト
    // ===========================================

    /// M1-1: 0台検出
    #[test]
    fn test_m1_1_no_devices() {
        let provider = MockProvider::new().with_ports(vec![]);
        let manager = MultiDeviceManager::new(provider);

        let devices = manager.list_devices().unwrap();
        assert!(devices.is_empty(), "0台の場合は空リスト");
    }

    /// M1-2: 複数台検出
    #[test]
    fn test_m1_2_multiple_devices() {
        let provider = MockProvider::new().with_ports(vec![
            f9p_port("/dev/ttyACM0"),
            f9p_port("/dev/ttyACM1"),
            ftdi_port("/dev/ttyUSB0"),
        ]);
        let manager = MultiDeviceManager::new(provider);

        let devices = manager.list_devices().unwrap();
        assert_eq!(devices.len(), 3, "3台検出");
        assert!(!devices[0].connected, "まだ接続していない");
        assert!(!devices[1].connected, "まだ接続していない");
        assert!(!devices[2].connected, "まだ接続していない");
    }

    // ===========================================
    // M2. 接続管理テスト
    // ===========================================

    /// M2-1: 1台接続
    #[test]
    fn test_m2_1_single_connect() {
        let provider = MockProvider::new()
            .with_ports(vec![f9p_port("/dev/ttyACM0")])
            .with_respond_at(38400);
        let mut manager = MultiDeviceManager::new(provider);

        let result = manager.connect("/dev/ttyACM0");
        assert!(result.is_ok(), "1台接続に成功");
        assert_eq!(result.unwrap(), 38400, "ボーレート38400で接続");
        assert_eq!(manager.connected_count(), 1);
    }

    /// M2-2: 複数台接続
    #[test]
    fn test_m2_2_multiple_connect() {
        let provider = MockProvider::new()
            .with_ports(vec![
                f9p_port("/dev/ttyACM0"),
                f9p_port("/dev/ttyACM1"),
            ])
            .with_respond_at(38400);
        let mut manager = MultiDeviceManager::new(provider);

        // 1台目接続
        let result1 = manager.connect("/dev/ttyACM0");
        assert!(result1.is_ok(), "1台目接続に成功");

        // 2台目接続
        let result2 = manager.connect("/dev/ttyACM1");
        assert!(result2.is_ok(), "2台目接続に成功");

        assert_eq!(manager.connected_count(), 2, "2台接続中");
    }

    /// M2-3: 同じポートに二重接続はエラー
    #[test]
    fn test_m2_3_duplicate_connect() {
        let provider = MockProvider::new()
            .with_ports(vec![f9p_port("/dev/ttyACM0")])
            .with_respond_at(38400);
        let mut manager = MultiDeviceManager::new(provider);

        manager.connect("/dev/ttyACM0").unwrap();

        // 同じポートに再接続
        let result = manager.connect("/dev/ttyACM0");
        assert!(
            matches!(result, Err(DeviceManagerError::PortBusy(_))),
            "二重接続はPortBusyエラー"
        );
    }

    /// M2-4: 切断後に再接続
    #[test]
    fn test_m2_4_reconnect_after_disconnect() {
        let provider = MockProvider::new()
            .with_ports(vec![f9p_port("/dev/ttyACM0")])
            .with_respond_at(38400);
        let mut manager = MultiDeviceManager::new(provider);

        // 接続 → 切断
        manager.connect("/dev/ttyACM0").unwrap();
        manager.disconnect("/dev/ttyACM0").unwrap();

        // 再接続
        let result = manager.connect("/dev/ttyACM0");
        assert!(result.is_ok(), "切断後に再接続できる");
    }

    /// M2-5: 存在しないポートに接続
    #[test]
    fn test_m2_5_connect_nonexistent() {
        let provider = MockProvider::new().with_ports(vec![]);
        let mut manager = MultiDeviceManager::new(provider);

        let result = manager.connect("/dev/ttyACM99");
        assert!(
            matches!(result, Err(DeviceManagerError::PortNotFound(_))),
            "存在しないポートはPortNotFoundエラー"
        );
    }

    /// M2-6: 未接続のポートを切断
    #[test]
    fn test_m2_6_disconnect_not_connected() {
        let provider = MockProvider::new()
            .with_ports(vec![f9p_port("/dev/ttyACM0")]);
        let mut manager = MultiDeviceManager::new(provider);

        let result = manager.disconnect("/dev/ttyACM0");
        assert!(
            matches!(result, Err(DeviceManagerError::NotConnected)),
            "未接続のポートを切断するとNotConnectedエラー"
        );
    }

    // ===========================================
    // M3. マネージャー取得テスト
    // ===========================================

    /// M3-1: 接続中のマネージャーを取得
    #[test]
    fn test_m3_1_get_manager() {
        let provider = MockProvider::new()
            .with_ports(vec![f9p_port("/dev/ttyACM0")])
            .with_respond_at(38400);
        let mut manager = MultiDeviceManager::new(provider);

        manager.connect("/dev/ttyACM0").unwrap();

        let device_manager = manager.get_manager("/dev/ttyACM0");
        assert!(device_manager.is_some(), "接続中のマネージャーを取得できる");
    }

    /// M3-2: 未接続のマネージャーはNone
    #[test]
    fn test_m3_2_get_manager_not_connected() {
        let provider = MockProvider::new()
            .with_ports(vec![f9p_port("/dev/ttyACM0")]);
        let manager = MultiDeviceManager::new(provider);

        let device_manager = manager.get_manager("/dev/ttyACM0");
        assert!(device_manager.is_none(), "未接続のマネージャーはNone");
    }

    /// M3-3: 接続中のパス一覧を取得
    #[test]
    fn test_m3_3_connected_paths() {
        let provider = MockProvider::new()
            .with_ports(vec![
                f9p_port("/dev/ttyACM0"),
                f9p_port("/dev/ttyACM1"),
            ])
            .with_respond_at(38400);
        let mut manager = MultiDeviceManager::new(provider);

        manager.connect("/dev/ttyACM0").unwrap();
        manager.connect("/dev/ttyACM1").unwrap();

        let paths = manager.connected_paths();
        assert_eq!(paths.len(), 2);
        assert!(paths.contains(&"/dev/ttyACM0".to_string()));
        assert!(paths.contains(&"/dev/ttyACM1".to_string()));
    }

    // ===========================================
    // M4. list_devicesと接続状態の連携テスト
    // ===========================================

    /// M4-1: 接続後にlist_devicesで接続状態が反映される
    #[test]
    fn test_m4_1_list_shows_connected_status() {
        let provider = MockProvider::new()
            .with_ports(vec![
                f9p_port("/dev/ttyACM0"),
                f9p_port("/dev/ttyACM1"),
            ])
            .with_respond_at(38400);
        let mut manager = MultiDeviceManager::new(provider);

        // 1台だけ接続
        manager.connect("/dev/ttyACM0").unwrap();

        let devices = manager.list_devices().unwrap();
        let dev0 = devices.iter().find(|d| d.path == "/dev/ttyACM0").unwrap();
        let dev1 = devices.iter().find(|d| d.path == "/dev/ttyACM1").unwrap();

        assert!(dev0.connected, "接続したデバイスはconnected=true");
        assert!(!dev1.connected, "接続していないデバイスはconnected=false");
    }
}
