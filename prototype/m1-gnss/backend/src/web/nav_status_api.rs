//! NAV-STATUS API
//!
//! - GET /api/nav-status - NAV-STATUS取得（TTFF、Fix状態）

use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;
use std::time::Duration;

use crate::ubx::nav_status;
use super::device_api::{AppState, ErrorResponse};

// ===========================================
// レスポンス型
// ===========================================

/// NAV-STATUSレスポンス
#[derive(Debug, Clone, Serialize)]
pub struct NavStatusResponse {
    /// Time to First Fix (ms)
    pub ttff: u32,
    /// 起動/リセットからの経過時間 (ms)
    pub msss: u32,
    /// FIXタイプ（0=No fix, 2=2D, 3=3D, etc.）
    pub gps_fix: u8,
    /// 位置・速度が有効か
    pub gps_fix_ok: bool,
    /// FIXが有効か（gps_fix >= 2 && gps_fix_ok）
    pub is_fix_valid: bool,
    /// RTK状態（0=なし, 1=Float, 2=Fixed）
    pub carr_soln: u8,
    /// RTK Fixed状態か
    pub is_rtk_fixed: bool,
    /// RTK Float状態か
    pub is_rtk_float: bool,
}

// ===========================================
// APIハンドラー
// ===========================================

/// NAV-STATUS poll リクエストを構築
fn build_nav_status_poll() -> Vec<u8> {
    // NAV-STATUS poll: class=0x01, id=0x03, payload=empty
    let class: u8 = 0x01;
    let id: u8 = 0x03;
    let payload: &[u8] = &[];
    let len = payload.len() as u16;

    let mut frame = vec![
        0xB5, 0x62, // sync
        class, id,
        (len & 0xFF) as u8,
        (len >> 8) as u8,
    ];
    frame.extend_from_slice(payload);

    // チェックサム計算
    let (ck_a, ck_b) = calculate_checksum(&frame[2..]);
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

/// GET /api/nav-status - NAV-STATUS取得
pub async fn get_nav_status(data: web::Data<AppState>) -> impl Responder {
    let mut manager = match data.device_manager.lock() {
        Ok(m) => m,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "内部エラー: ロック取得に失敗".to_string(),
                code: "LOCK_ERROR".to_string(),
            })
        }
    };

    // 接続確認
    if manager.get_connected_device().is_none() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "装置が接続されていません".to_string(),
            code: "DEVICE_NOT_CONNECTED".to_string(),
        });
    }

    // バッファをクリア
    if let Err(e) = manager.drain_buffer() {
        return HttpResponse::InternalServerError().json(ErrorResponse {
            error: format!("バッファクリアエラー: {}", e),
            code: "DRAIN_ERROR".to_string(),
        });
    }

    // NAV-STATUS poll送信
    let poll_msg = build_nav_status_poll();
    if let Err(e) = manager.send_ubx(&poll_msg) {
        return HttpResponse::InternalServerError().json(ErrorResponse {
            error: format!("送信エラー: {}", e),
            code: "SEND_ERROR".to_string(),
        });
    }

    // 少し待機
    std::thread::sleep(Duration::from_millis(50));

    // 応答受信
    let response = match manager.receive_ubx(Duration::from_secs(1)) {
        Ok(r) => r,
        Err(crate::device::manager::DeviceManagerError::Timeout) => {
            return HttpResponse::GatewayTimeout().json(ErrorResponse {
                error: "タイムアウト".to_string(),
                code: "TIMEOUT".to_string(),
            });
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: format!("受信エラー: {}", e),
                code: "RECEIVE_ERROR".to_string(),
            });
        }
    };

    // パース
    let nav_status_result = match nav_status::parse(&response) {
        Ok(s) => s,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: format!("パースエラー: {}", e),
                code: "PARSE_ERROR".to_string(),
            });
        }
    };

    // レスポンス構築
    HttpResponse::Ok().json(NavStatusResponse {
        ttff: nav_status_result.ttff,
        msss: nav_status_result.msss,
        gps_fix: nav_status_result.gps_fix,
        gps_fix_ok: nav_status_result.gps_fix_ok,
        is_fix_valid: nav_status_result.is_fix_valid(),
        carr_soln: nav_status_result.carr_soln,
        is_rtk_fixed: nav_status_result.is_rtk_fixed(),
        is_rtk_float: nav_status_result.is_rtk_float(),
    })
}

/// APIルートを設定
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/nav-status")
            .route("", web::get().to(get_nav_status)),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::filter::PortInfo;
    use crate::device::manager::{DeviceManager, DeviceManagerError, SerialPort, SerialPortProvider};
    use std::collections::VecDeque;
    use std::io;
    use std::sync::{Arc, Mutex};

    // ===========================================
    // モック実装
    // ===========================================

    struct MockSerialPort {
        read_queue: Arc<Mutex<VecDeque<Vec<u8>>>>,
        should_timeout: bool,
        current_timeout_ms: Arc<Mutex<u64>>,
    }

    impl SerialPort for MockSerialPort {
        fn write(&mut self, _data: &[u8]) -> Result<usize, io::Error> {
            Ok(_data.len())
        }

        fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
            // drain_buffer用の短いタイムアウト
            {
                let timeout_ms = *self.current_timeout_ms.lock().unwrap();
                if timeout_ms <= 20 {
                    return Err(io::Error::new(io::ErrorKind::TimedOut, "drain"));
                }
            }

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

        fn set_timeout(&mut self, timeout: Duration) -> Result<(), io::Error> {
            *self.current_timeout_ms.lock().unwrap() = timeout.as_millis() as u64;
            Ok(())
        }
    }

    struct MockProvider {
        ports: Vec<PortInfo>,
        read_queue: Arc<Mutex<VecDeque<Vec<u8>>>>,
        should_timeout: bool,
        current_timeout_ms: Arc<Mutex<u64>>,
    }

    impl MockProvider {
        fn new() -> Self {
            Self {
                ports: vec![],
                read_queue: Arc::new(Mutex::new(VecDeque::new())),
                should_timeout: false,
                current_timeout_ms: Arc::new(Mutex::new(1000)),
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

        fn with_timeout(mut self) -> Self {
            self.should_timeout = true;
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
                read_queue: self.read_queue.clone(),
                should_timeout: self.should_timeout,
                current_timeout_ms: self.current_timeout_ms.clone(),
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

    /// NAV-STATUS応答を生成
    fn build_nav_status_response(
        gps_fix: u8,
        gps_fix_ok: bool,
        diff_soln: bool,
        carr_soln: u8,
        ttff: u32,
        msss: u32,
    ) -> Vec<u8> {
        let mut payload = vec![0u8; 16];

        // iTOW (offset 0-3)
        let itow: u32 = 123456000;
        payload[0..4].copy_from_slice(&itow.to_le_bytes());

        // gpsFix (offset 4)
        payload[4] = gps_fix;

        // flags (offset 5)
        let mut flags: u8 = 0;
        if gps_fix_ok {
            flags |= 0x01;
        }
        if diff_soln {
            flags |= 0x02;
        }
        payload[5] = flags;

        // fixStat (offset 6) - 未使用
        payload[6] = 0;

        // flags2 (offset 7)
        let flags2 = (carr_soln & 0x03) << 6;
        payload[7] = flags2;

        // ttff (offset 8-11)
        payload[8..12].copy_from_slice(&ttff.to_le_bytes());

        // msss (offset 12-15)
        payload[12..16].copy_from_slice(&msss.to_le_bytes());

        // UBXフレーム構築
        let class: u8 = 0x01;
        let id: u8 = 0x03;
        let len = payload.len() as u16;

        let mut frame = vec![
            0xB5, 0x62,
            class, id,
            (len & 0xFF) as u8,
            (len >> 8) as u8,
        ];
        frame.extend_from_slice(&payload);

        let (ck_a, ck_b) = calculate_checksum(&frame[2..]);
        frame.push(ck_a);
        frame.push(ck_b);

        frame
    }

    // ===========================================
    // テスト
    // ===========================================

    /// T1: 接続済み・3D Fix + RTK Fixed → 正常応答
    #[test]
    fn test_t1_connected_rtk_fixed() {
        let response = build_nav_status_response(
            3,     // gps_fix: 3D
            true,  // gps_fix_ok
            true,  // diff_soln
            2,     // carr_soln: RTK Fixed
            5000,  // ttff: 5秒
            60000, // msss: 60秒
        );

        let provider = MockProvider::new()
            .with_ports(vec![f9p_port("/dev/ttyACM0")])
            .with_responses(vec![response]);

        let mut manager = DeviceManager::new(provider);
        manager.list_devices().unwrap();
        manager.connect("/dev/ttyACM0").unwrap();

        // パースを直接テスト
        let poll_msg = build_nav_status_poll();
        manager.send_ubx(&poll_msg).unwrap();
        std::thread::sleep(Duration::from_millis(50));
        let raw_response = manager.receive_ubx(Duration::from_secs(1)).unwrap();
        let parsed = nav_status::parse(&raw_response).unwrap();

        // 検証
        assert_eq!(parsed.gps_fix, 3);
        assert!(parsed.gps_fix_ok);
        assert!(parsed.is_fix_valid());
        assert_eq!(parsed.carr_soln, 2);
        assert!(parsed.is_rtk_fixed());
        assert!(!parsed.is_rtk_float());
        assert_eq!(parsed.ttff, 5000);
        assert_eq!(parsed.msss, 60000);
    }

    /// T2: 未接続 → エラー
    #[test]
    fn test_t2_not_connected() {
        let provider = MockProvider::new()
            .with_ports(vec![f9p_port("/dev/ttyACM0")]);

        let manager = DeviceManager::new(provider);
        // 接続しない

        assert!(manager.get_connected_device().is_none());
    }

    /// T3: タイムアウト
    #[test]
    fn test_t3_timeout() {
        let provider = MockProvider::new()
            .with_ports(vec![f9p_port("/dev/ttyACM0")])
            .with_timeout();

        let mut manager = DeviceManager::new(provider);
        manager.list_devices().unwrap();
        manager.connect("/dev/ttyACM0").unwrap();

        let poll_msg = build_nav_status_poll();
        manager.send_ubx(&poll_msg).unwrap();
        std::thread::sleep(Duration::from_millis(50));
        let result = manager.receive_ubx(Duration::from_millis(100));

        assert!(matches!(result, Err(DeviceManagerError::Timeout)));
    }

    /// T4: No Fix → is_fix_valid = false
    #[test]
    fn test_t4_no_fix() {
        let response = build_nav_status_response(
            0,     // gps_fix: No fix
            false, // gps_fix_ok
            false, // diff_soln
            0,     // carr_soln: なし
            0,     // ttff: 0
            5000,  // msss: 5秒
        );

        let provider = MockProvider::new()
            .with_ports(vec![f9p_port("/dev/ttyACM0")])
            .with_responses(vec![response]);

        let mut manager = DeviceManager::new(provider);
        manager.list_devices().unwrap();
        manager.connect("/dev/ttyACM0").unwrap();

        let poll_msg = build_nav_status_poll();
        manager.send_ubx(&poll_msg).unwrap();
        std::thread::sleep(Duration::from_millis(50));
        let raw_response = manager.receive_ubx(Duration::from_secs(1)).unwrap();
        let parsed = nav_status::parse(&raw_response).unwrap();

        assert_eq!(parsed.gps_fix, 0);
        assert!(!parsed.gps_fix_ok);
        assert!(!parsed.is_fix_valid());
        assert_eq!(parsed.ttff, 0);
    }

    /// T5: RTK Float → is_rtk_float = true
    #[test]
    fn test_t5_rtk_float() {
        let response = build_nav_status_response(
            3,     // gps_fix: 3D
            true,  // gps_fix_ok
            true,  // diff_soln
            1,     // carr_soln: RTK Float
            8000,  // ttff: 8秒
            30000, // msss: 30秒
        );

        let provider = MockProvider::new()
            .with_ports(vec![f9p_port("/dev/ttyACM0")])
            .with_responses(vec![response]);

        let mut manager = DeviceManager::new(provider);
        manager.list_devices().unwrap();
        manager.connect("/dev/ttyACM0").unwrap();

        let poll_msg = build_nav_status_poll();
        manager.send_ubx(&poll_msg).unwrap();
        std::thread::sleep(Duration::from_millis(50));
        let raw_response = manager.receive_ubx(Duration::from_secs(1)).unwrap();
        let parsed = nav_status::parse(&raw_response).unwrap();

        assert_eq!(parsed.carr_soln, 1);
        assert!(!parsed.is_rtk_fixed());
        assert!(parsed.is_rtk_float());
        assert_eq!(parsed.ttff, 8000);
    }

    /// T6: パースエラー
    #[test]
    fn test_t6_parse_error() {
        // 不正なUBXメッセージ
        let invalid_response = vec![0xB5, 0x62, 0x01, 0x03, 0x00, 0x00, 0x00, 0x00];

        let provider = MockProvider::new()
            .with_ports(vec![f9p_port("/dev/ttyACM0")])
            .with_responses(vec![invalid_response]);

        let mut manager = DeviceManager::new(provider);
        manager.list_devices().unwrap();
        manager.connect("/dev/ttyACM0").unwrap();

        let poll_msg = build_nav_status_poll();
        manager.send_ubx(&poll_msg).unwrap();
        std::thread::sleep(Duration::from_millis(50));
        let raw_response = manager.receive_ubx(Duration::from_secs(1)).unwrap();
        let result = nav_status::parse(&raw_response);

        assert!(result.is_err());
    }
}
