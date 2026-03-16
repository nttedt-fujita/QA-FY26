//! NAV-SIG API
//!
//! - GET /api/nav-sig - NAV-SIG取得（L1/L2別C/N0、L2受信率）

use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;
use std::time::Duration;

use crate::ubx::nav_sig::{self, SignalInfo};
use super::device_api::{AppState, ErrorResponse};

// ===========================================
// レスポンス型
// ===========================================

/// 信号情報（API用）
#[derive(Debug, Clone, Serialize)]
pub struct SignalInfoResponse {
    /// GNSS識別子（0=GPS, 1=SBAS, 2=Galileo, 3=BeiDou, 5=QZSS, 6=GLONASS）
    pub gnss_id: u8,
    /// 衛星番号
    pub sv_id: u8,
    /// 信号識別子
    pub sig_id: u8,
    /// C/N0 [dBHz]
    pub cno: u8,
    /// 品質指標（0-7）
    pub quality_ind: u8,
    /// L1帯か
    pub is_l1: bool,
    /// L2帯か
    pub is_l2: bool,
}

impl From<&SignalInfo> for SignalInfoResponse {
    fn from(s: &SignalInfo) -> Self {
        Self {
            gnss_id: s.gnss_id,
            sv_id: s.sv_id,
            sig_id: s.sig_id,
            cno: s.cno,
            quality_ind: s.quality_ind,
            is_l1: s.is_l1(),
            is_l2: s.is_l2(),
        }
    }
}

/// 信号統計情報
#[derive(Debug, Clone, Serialize)]
pub struct SignalStatsResponse {
    /// GPS L1可視衛星数
    pub gps_visible_count: usize,
    /// GPS L2受信中衛星数
    pub gps_l2_reception_count: usize,
    /// GPS L2受信率（0.0〜1.0）
    pub gps_l2_reception_rate: f64,
}

/// NAV-SIGレスポンス
#[derive(Debug, Clone, Serialize)]
pub struct NavSigResponse {
    /// 信号情報リスト
    pub signals: Vec<SignalInfoResponse>,
    /// 統計情報
    pub stats: SignalStatsResponse,
}

// ===========================================
// APIハンドラー
// ===========================================

/// NAV-SIG poll リクエストを構築
fn build_nav_sig_poll() -> Vec<u8> {
    // NAV-SIG poll: class=0x01, id=0x43, payload=empty
    let class: u8 = 0x01;
    let id: u8 = 0x43;
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

/// GET /api/nav-sig - NAV-SIG取得
pub async fn get_nav_sig(data: web::Data<AppState>) -> impl Responder {
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

    // NAV-SIG poll送信
    let poll_msg = build_nav_sig_poll();
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
    let nav_sig_result = match nav_sig::parse(&response) {
        Ok(s) => s,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: format!("パースエラー: {}", e),
                code: "PARSE_ERROR".to_string(),
            });
        }
    };

    // 統計計算
    let stats = SignalStatsResponse {
        gps_visible_count: nav_sig::gps_visible_count(&nav_sig_result.signals),
        gps_l2_reception_count: nav_sig::gps_l2_reception_count(&nav_sig_result.signals),
        gps_l2_reception_rate: nav_sig::gps_l2_reception_rate(&nav_sig_result.signals),
    };

    // レスポンス構築
    let signal_responses: Vec<SignalInfoResponse> = nav_sig_result.signals.iter().map(|s| s.into()).collect();

    HttpResponse::Ok().json(NavSigResponse {
        signals: signal_responses,
        stats,
    })
}

/// APIルートを設定
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/nav-sig")
            .route("", web::get().to(get_nav_sig)),
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

    impl io::Read for MockSerialPort {
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
    }

    impl io::Write for MockSerialPort {
        fn write(&mut self, _data: &[u8]) -> Result<usize, io::Error> {
            Ok(_data.len())
        }

        fn flush(&mut self) -> Result<(), io::Error> {
            Ok(())
        }
    }

    impl SerialPort for MockSerialPort {
        fn name(&self) -> Option<String> { Some("mock".to_string()) }
        fn baud_rate(&self) -> serialport::Result<u32> { Ok(115200) }
        fn data_bits(&self) -> serialport::Result<serialport::DataBits> { Ok(serialport::DataBits::Eight) }
        fn flow_control(&self) -> serialport::Result<serialport::FlowControl> { Ok(serialport::FlowControl::None) }
        fn parity(&self) -> serialport::Result<serialport::Parity> { Ok(serialport::Parity::None) }
        fn stop_bits(&self) -> serialport::Result<serialport::StopBits> { Ok(serialport::StopBits::One) }
        fn timeout(&self) -> Duration { Duration::from_millis(*self.current_timeout_ms.lock().unwrap()) }
        fn set_baud_rate(&mut self, _: u32) -> serialport::Result<()> { Ok(()) }
        fn set_data_bits(&mut self, _: serialport::DataBits) -> serialport::Result<()> { Ok(()) }
        fn set_flow_control(&mut self, _: serialport::FlowControl) -> serialport::Result<()> { Ok(()) }
        fn set_parity(&mut self, _: serialport::Parity) -> serialport::Result<()> { Ok(()) }
        fn set_stop_bits(&mut self, _: serialport::StopBits) -> serialport::Result<()> { Ok(()) }
        fn set_timeout(&mut self, timeout: Duration) -> serialport::Result<()> {
            *self.current_timeout_ms.lock().unwrap() = timeout.as_millis() as u64;
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
        fn try_clone(&self) -> serialport::Result<Box<dyn SerialPort>> {
            Err(serialport::Error::new(serialport::ErrorKind::Unknown, "clone not supported"))
        }
        fn set_break(&self) -> serialport::Result<()> { Ok(()) }
        fn clear_break(&self) -> serialport::Result<()> { Ok(()) }
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

    /// NAV-SIG応答を生成
    fn build_nav_sig_response(signals: &[SignalInfo]) -> Vec<u8> {
        let num_sigs = signals.len();
        let payload_len = 8 + num_sigs * 16;
        let mut payload = vec![0u8; payload_len];

        // ヘッダー部分
        let itow: u32 = 123456000;
        payload[0..4].copy_from_slice(&itow.to_le_bytes());
        payload[4] = 0x00; // version
        payload[5] = num_sigs as u8;
        // reserved[2] = 0

        // 各信号ブロック
        for (i, sig) in signals.iter().enumerate() {
            let offset = 8 + i * 16;
            payload[offset] = sig.gnss_id;
            payload[offset + 1] = sig.sv_id;
            payload[offset + 2] = sig.sig_id;
            payload[offset + 3] = sig.freq_id;
            payload[offset + 4..offset + 6].copy_from_slice(&sig.pr_res.to_le_bytes());
            payload[offset + 6] = sig.cno;
            payload[offset + 7] = sig.quality_ind;
            payload[offset + 8] = sig.corr_source;
            payload[offset + 9] = sig.iono_model;
            payload[offset + 10..offset + 12].copy_from_slice(&sig.sig_flags.to_le_bytes());
            // reserved[4] = 0
        }

        // UBXフレーム構築
        let class: u8 = 0x01;
        let id: u8 = 0x43;
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

    /// GPS L1信号を作成
    fn gps_l1_signal(sv_id: u8, cno: u8, quality_ind: u8) -> SignalInfo {
        SignalInfo {
            gnss_id: 0,  // GPS
            sv_id,
            sig_id: 0,   // L1C/A
            freq_id: 0,
            pr_res: 0,
            cno,
            quality_ind,
            corr_source: 0,
            iono_model: 0,
            sig_flags: 0,
        }
    }

    /// GPS L2信号を作成
    fn gps_l2_signal(sv_id: u8, cno: u8, quality_ind: u8) -> SignalInfo {
        SignalInfo {
            gnss_id: 0,  // GPS
            sv_id,
            sig_id: 4,   // L2CL
            freq_id: 0,
            pr_res: 0,
            cno,
            quality_ind,
            corr_source: 0,
            iono_model: 0,
            sig_flags: 0,
        }
    }

    /// Galileo E1信号を作成
    fn galileo_e1_signal(sv_id: u8, cno: u8, quality_ind: u8) -> SignalInfo {
        SignalInfo {
            gnss_id: 2,  // Galileo
            sv_id,
            sig_id: 0,   // E1-C
            freq_id: 0,
            pr_res: 0,
            cno,
            quality_ind,
            corr_source: 0,
            iono_model: 0,
            sig_flags: 0,
        }
    }

    // ===========================================
    // テスト
    // ===========================================

    /// T1: 接続済み・正常応答 → 200 + signals + stats
    #[test]
    fn test_t1_connected_normal_response() {
        // 3衛星: GPS L1×2 + GPS L2×1（L2受信中）
        let signals = vec![
            gps_l1_signal(1, 40, 7),   // SV1 L1
            gps_l1_signal(2, 35, 6),   // SV2 L1
            gps_l2_signal(1, 30, 5),   // SV1 L2 (quality>=5で受信中)
        ];
        let response = build_nav_sig_response(&signals);

        let provider = MockProvider::new()
            .with_ports(vec![f9p_port("/dev/ttyACM0")])
            .with_responses(vec![response]);

        let mut manager = DeviceManager::new(provider);
        manager.list_devices().unwrap();
        manager.connect("/dev/ttyACM0").unwrap();

        // パースと統計計算を直接テスト（APIハンドラーはActixランタイムが必要なため）
        let poll_msg = build_nav_sig_poll();
        manager.send_ubx(&poll_msg).unwrap();
        std::thread::sleep(Duration::from_millis(50));
        let raw_response = manager.receive_ubx(Duration::from_secs(1)).unwrap();
        let parsed = nav_sig::parse(&raw_response).unwrap();

        // 検証
        assert_eq!(parsed.signals.len(), 3);
        assert_eq!(nav_sig::gps_visible_count(&parsed.signals), 2);      // L1を持つ衛星数
        assert_eq!(nav_sig::gps_l2_reception_count(&parsed.signals), 1); // L2受信中
        assert!((nav_sig::gps_l2_reception_rate(&parsed.signals) - 0.5).abs() < 0.01); // 1/2 = 50%
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

        let poll_msg = build_nav_sig_poll();
        manager.send_ubx(&poll_msg).unwrap();
        std::thread::sleep(Duration::from_millis(50));
        let result = manager.receive_ubx(Duration::from_millis(100));

        assert!(matches!(result, Err(DeviceManagerError::Timeout)));
    }

    /// T4: パースエラー（不正UBX）
    #[test]
    fn test_t4_parse_error() {
        // 不正なUBXメッセージ
        let invalid_response = vec![0xB5, 0x62, 0x01, 0x43, 0x00, 0x00, 0x00, 0x00];

        let provider = MockProvider::new()
            .with_ports(vec![f9p_port("/dev/ttyACM0")])
            .with_responses(vec![invalid_response]);

        let mut manager = DeviceManager::new(provider);
        manager.list_devices().unwrap();
        manager.connect("/dev/ttyACM0").unwrap();

        let poll_msg = build_nav_sig_poll();
        manager.send_ubx(&poll_msg).unwrap();
        std::thread::sleep(Duration::from_millis(50));
        let raw_response = manager.receive_ubx(Duration::from_secs(1)).unwrap();
        let result = nav_sig::parse(&raw_response);

        assert!(result.is_err());
    }

    /// T5: 空の信号リスト → stats全て0
    #[test]
    fn test_t5_empty_signals() {
        let signals: Vec<SignalInfo> = vec![];
        let response = build_nav_sig_response(&signals);

        let provider = MockProvider::new()
            .with_ports(vec![f9p_port("/dev/ttyACM0")])
            .with_responses(vec![response]);

        let mut manager = DeviceManager::new(provider);
        manager.list_devices().unwrap();
        manager.connect("/dev/ttyACM0").unwrap();

        let poll_msg = build_nav_sig_poll();
        manager.send_ubx(&poll_msg).unwrap();
        std::thread::sleep(Duration::from_millis(50));
        let raw_response = manager.receive_ubx(Duration::from_secs(1)).unwrap();
        let parsed = nav_sig::parse(&raw_response).unwrap();

        assert_eq!(parsed.signals.len(), 0);
        assert_eq!(nav_sig::gps_visible_count(&parsed.signals), 0);
        assert_eq!(nav_sig::gps_l2_reception_count(&parsed.signals), 0);
        assert!((nav_sig::gps_l2_reception_rate(&parsed.signals) - 0.0).abs() < 0.01);
    }

    /// T6: GPS以外（Galileo等）混在 → statsはGPSのみカウント
    #[test]
    fn test_t6_mixed_gnss() {
        let signals = vec![
            gps_l1_signal(1, 40, 7),       // GPS SV1 L1
            gps_l2_signal(1, 30, 5),       // GPS SV1 L2
            galileo_e1_signal(11, 38, 7),  // Galileo E11 E1（GPSではない）
        ];
        let response = build_nav_sig_response(&signals);

        let provider = MockProvider::new()
            .with_ports(vec![f9p_port("/dev/ttyACM0")])
            .with_responses(vec![response]);

        let mut manager = DeviceManager::new(provider);
        manager.list_devices().unwrap();
        manager.connect("/dev/ttyACM0").unwrap();

        let poll_msg = build_nav_sig_poll();
        manager.send_ubx(&poll_msg).unwrap();
        std::thread::sleep(Duration::from_millis(50));
        let raw_response = manager.receive_ubx(Duration::from_secs(1)).unwrap();
        let parsed = nav_sig::parse(&raw_response).unwrap();

        // 全信号は3つ
        assert_eq!(parsed.signals.len(), 3);
        // GPSのみカウント
        assert_eq!(nav_sig::gps_visible_count(&parsed.signals), 1);      // GPS L1は1衛星のみ
        assert_eq!(nav_sig::gps_l2_reception_count(&parsed.signals), 1); // GPS L2受信中は1衛星
        assert!((nav_sig::gps_l2_reception_rate(&parsed.signals) - 1.0).abs() < 0.01); // 1/1 = 100%
    }
}
