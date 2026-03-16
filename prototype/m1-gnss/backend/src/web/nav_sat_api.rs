//! NAV-SAT API
//!
//! - GET /api/nav-sat - NAV-SAT取得（スカイプロット用衛星情報）

use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;
use std::time::Duration;

use crate::ubx::nav_sat::{self, SatelliteInfo};
use super::device_api::{AppState, ErrorResponse};

// ===========================================
// レスポンス型
// ===========================================

/// 衛星情報（API用）
#[derive(Debug, Clone, Serialize)]
pub struct SatelliteInfoResponse {
    /// GNSS識別子（0=GPS, 1=SBAS, 2=Galileo, 3=BeiDou, 5=QZSS, 6=GLONASS）
    pub gnss_id: u8,
    /// GNSS名
    pub gnss_name: &'static str,
    /// 衛星番号
    pub sv_id: u8,
    /// C/N0 [dBHz]
    pub cno: u8,
    /// 仰角 [deg]（-90〜+90）
    pub elev: i8,
    /// 方位角 [deg]（0〜360）
    pub azim: i16,
    /// ナビゲーションに使用中か
    pub is_used: bool,
    /// 品質指標（0-7）
    pub quality_ind: u8,
    /// 健全性（0=unknown, 1=healthy, 2=unhealthy）
    pub health: u8,
}

impl From<&SatelliteInfo> for SatelliteInfoResponse {
    fn from(s: &SatelliteInfo) -> Self {
        Self {
            gnss_id: s.gnss_id,
            gnss_name: s.gnss_name(),
            sv_id: s.sv_id,
            cno: s.cno,
            elev: s.elev,
            azim: s.azim,
            is_used: s.is_used(),
            quality_ind: s.quality_ind(),
            health: s.health(),
        }
    }
}

/// 衛星統計情報
#[derive(Debug, Clone, Serialize)]
pub struct SatelliteStatsResponse {
    /// 全衛星数
    pub total_count: usize,
    /// ナビゲーションに使用中の衛星数
    pub used_count: usize,
    /// GNSS別衛星数
    pub gnss_counts: GnssCountsResponse,
}

/// GNSS別衛星数
#[derive(Debug, Clone, Serialize)]
pub struct GnssCountsResponse {
    pub gps: usize,
    pub sbas: usize,
    pub galileo: usize,
    pub beidou: usize,
    pub qzss: usize,
    pub glonass: usize,
}

/// NAV-SATレスポンス
#[derive(Debug, Clone, Serialize)]
pub struct NavSatResponse {
    /// 衛星情報リスト
    pub satellites: Vec<SatelliteInfoResponse>,
    /// 統計情報
    pub stats: SatelliteStatsResponse,
}

// ===========================================
// APIハンドラー
// ===========================================

/// NAV-SAT poll リクエストを構築
fn build_nav_sat_poll() -> Vec<u8> {
    // NAV-SAT poll: class=0x01, id=0x35, payload=empty
    let class: u8 = 0x01;
    let id: u8 = 0x35;
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

/// GET /api/nav-sat - NAV-SAT取得
pub async fn get_nav_sat(data: web::Data<AppState>) -> impl Responder {
    // Phase 3: MultiDeviceManager経由で最初の接続デバイスを取得
    let device_manager_arc = match data.get_first_device_manager() {
        Ok(Some(arc)) => arc,
        Ok(None) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "装置が接続されていません".to_string(),
                code: "DEVICE_NOT_CONNECTED".to_string(),
            });
        }
        Err(_) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "内部エラー: ロック取得に失敗".to_string(),
                code: "LOCK_ERROR".to_string(),
            });
        }
    };

    let mut manager = match device_manager_arc.lock() {
        Ok(m) => m,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "内部エラー: デバイスロック取得に失敗".to_string(),
                code: "LOCK_ERROR".to_string(),
            });
        }
    };

    // バッファをクリア
    if let Err(e) = manager.drain_buffer() {
        return HttpResponse::InternalServerError().json(ErrorResponse {
            error: format!("バッファクリアエラー: {}", e),
            code: "DRAIN_ERROR".to_string(),
        });
    }

    // NAV-SAT poll送信
    let poll_msg = build_nav_sat_poll();
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
    let nav_sat_result = match nav_sat::parse(&response) {
        Ok(s) => s,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: format!("パースエラー: {}", e),
                code: "PARSE_ERROR".to_string(),
            });
        }
    };

    // 統計計算
    let used_count = nav_sat_result.used_satellites().len();
    let gnss_counts = GnssCountsResponse {
        gps: nav_sat_result.satellites_by_gnss(0).len(),
        sbas: nav_sat_result.satellites_by_gnss(1).len(),
        galileo: nav_sat_result.satellites_by_gnss(2).len(),
        beidou: nav_sat_result.satellites_by_gnss(3).len(),
        qzss: nav_sat_result.satellites_by_gnss(5).len(),
        glonass: nav_sat_result.satellites_by_gnss(6).len(),
    };

    let stats = SatelliteStatsResponse {
        total_count: nav_sat_result.satellites.len(),
        used_count,
        gnss_counts,
    };

    // レスポンス構築
    let satellite_responses: Vec<SatelliteInfoResponse> =
        nav_sat_result.satellites.iter().map(|s| s.into()).collect();

    HttpResponse::Ok().json(NavSatResponse {
        satellites: satellite_responses,
        stats,
    })
}

/// APIルートを設定
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/nav-sat")
            .route("", web::get().to(get_nav_sat)),
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

    /// NAV-SAT応答を生成
    fn build_nav_sat_response(satellites: &[SatelliteInfo]) -> Vec<u8> {
        let num_svs = satellites.len();
        let payload_len = 8 + num_svs * 12;
        let mut payload = vec![0u8; payload_len];

        // ヘッダー部分
        let itow: u32 = 123456000;
        payload[0..4].copy_from_slice(&itow.to_le_bytes());
        payload[4] = 0x01; // version
        payload[5] = num_svs as u8;
        // reserved[2] = 0

        // 各衛星ブロック
        for (i, sat) in satellites.iter().enumerate() {
            let offset = 8 + i * 12;
            payload[offset] = sat.gnss_id;
            payload[offset + 1] = sat.sv_id;
            payload[offset + 2] = sat.cno;
            payload[offset + 3] = sat.elev as u8;
            payload[offset + 4..offset + 6].copy_from_slice(&sat.azim.to_le_bytes());
            payload[offset + 6..offset + 8].copy_from_slice(&sat.pr_res.to_le_bytes());
            payload[offset + 8..offset + 12].copy_from_slice(&sat.flags.to_le_bytes());
        }

        // UBXフレーム構築
        let class: u8 = 0x01;
        let id: u8 = 0x35;
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

    /// 衛星を作成
    fn make_satellite(gnss_id: u8, sv_id: u8, cno: u8, elev: i8, azim: i16, flags: u32) -> SatelliteInfo {
        SatelliteInfo {
            gnss_id,
            sv_id,
            cno,
            elev,
            azim,
            pr_res: 0,
            flags,
        }
    }

    // ===========================================
    // テスト
    // ===========================================

    /// T1: 接続済み・正常応答 → 200 + satellites + stats
    #[test]
    fn test_t1_connected_normal_response() {
        // 3衛星: GPS×2 + GLONASS×1（2つがused）
        let satellites = vec![
            make_satellite(0, 1, 45, 60, 90, 0x19),    // GPS SV1, used, healthy
            make_satellite(0, 5, 38, 30, 270, 0x08),   // GPS SV5, used
            make_satellite(6, 10, 40, 45, 180, 0x00),  // GLONASS SV10, not used
        ];
        let response = build_nav_sat_response(&satellites);

        let provider = MockProvider::new()
            .with_ports(vec![f9p_port("/dev/ttyACM0")])
            .with_responses(vec![response]);

        let mut manager = DeviceManager::new(provider);
        manager.list_devices().unwrap();
        manager.connect("/dev/ttyACM0").unwrap();

        let poll_msg = build_nav_sat_poll();
        manager.send_ubx(&poll_msg).unwrap();
        std::thread::sleep(Duration::from_millis(50));
        let raw_response = manager.receive_ubx(Duration::from_secs(1)).unwrap();
        let parsed = nav_sat::parse(&raw_response).unwrap();

        // 検証
        assert_eq!(parsed.satellites.len(), 3);
        assert_eq!(parsed.used_satellites().len(), 2);
        assert_eq!(parsed.satellites_by_gnss(0).len(), 2); // GPS
        assert_eq!(parsed.satellites_by_gnss(6).len(), 1); // GLONASS

        // 1つ目の衛星の詳細確認
        let first = &parsed.satellites[0];
        assert_eq!(first.gnss_id, 0);
        assert_eq!(first.sv_id, 1);
        assert_eq!(first.cno, 45);
        assert_eq!(first.elev, 60);
        assert_eq!(first.azim, 90);
        assert!(first.is_used());
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

        let poll_msg = build_nav_sat_poll();
        manager.send_ubx(&poll_msg).unwrap();
        std::thread::sleep(Duration::from_millis(50));
        let result = manager.receive_ubx(Duration::from_millis(100));

        assert!(matches!(result, Err(DeviceManagerError::Timeout)));
    }

    /// T4: パースエラー（不正UBX）
    #[test]
    fn test_t4_parse_error() {
        // 不正なUBXメッセージ
        let invalid_response = vec![0xB5, 0x62, 0x01, 0x35, 0x00, 0x00, 0x00, 0x00];

        let provider = MockProvider::new()
            .with_ports(vec![f9p_port("/dev/ttyACM0")])
            .with_responses(vec![invalid_response]);

        let mut manager = DeviceManager::new(provider);
        manager.list_devices().unwrap();
        manager.connect("/dev/ttyACM0").unwrap();

        let poll_msg = build_nav_sat_poll();
        manager.send_ubx(&poll_msg).unwrap();
        std::thread::sleep(Duration::from_millis(50));
        let raw_response = manager.receive_ubx(Duration::from_secs(1)).unwrap();
        let result = nav_sat::parse(&raw_response);

        assert!(result.is_err());
    }

    /// T5: 空の衛星リスト → stats全て0
    #[test]
    fn test_t5_empty_satellites() {
        let satellites: Vec<SatelliteInfo> = vec![];
        let response = build_nav_sat_response(&satellites);

        let provider = MockProvider::new()
            .with_ports(vec![f9p_port("/dev/ttyACM0")])
            .with_responses(vec![response]);

        let mut manager = DeviceManager::new(provider);
        manager.list_devices().unwrap();
        manager.connect("/dev/ttyACM0").unwrap();

        let poll_msg = build_nav_sat_poll();
        manager.send_ubx(&poll_msg).unwrap();
        std::thread::sleep(Duration::from_millis(50));
        let raw_response = manager.receive_ubx(Duration::from_secs(1)).unwrap();
        let parsed = nav_sat::parse(&raw_response).unwrap();

        assert_eq!(parsed.satellites.len(), 0);
        assert_eq!(parsed.used_satellites().len(), 0);
    }

    /// T6: 仰角・方位角境界値（スカイプロット用）
    #[test]
    fn test_t6_sky_plot_boundary_values() {
        let satellites = vec![
            make_satellite(0, 1, 45, 90, 0, 0x08),     // 天頂（elev=90°）
            make_satellite(0, 2, 40, 0, 90, 0x08),     // 水平線・東（elev=0°, azim=90°）
            make_satellite(0, 3, 35, -10, 180, 0x08),  // 地平線下（elev=-10°）
            make_satellite(0, 4, 30, 45, 359, 0x08),   // 方位角最大付近（azim=359°）
        ];
        let response = build_nav_sat_response(&satellites);

        let provider = MockProvider::new()
            .with_ports(vec![f9p_port("/dev/ttyACM0")])
            .with_responses(vec![response]);

        let mut manager = DeviceManager::new(provider);
        manager.list_devices().unwrap();
        manager.connect("/dev/ttyACM0").unwrap();

        let poll_msg = build_nav_sat_poll();
        manager.send_ubx(&poll_msg).unwrap();
        std::thread::sleep(Duration::from_millis(50));
        let raw_response = manager.receive_ubx(Duration::from_secs(1)).unwrap();
        let parsed = nav_sat::parse(&raw_response).unwrap();

        let sky_plot = parsed.sky_plot_data();
        assert_eq!(sky_plot.len(), 4);
        assert_eq!(sky_plot[0], (90, 0, 45, "GPS", 1));   // 天頂
        assert_eq!(sky_plot[1], (0, 90, 40, "GPS", 2));   // 水平線・東
        assert_eq!(sky_plot[2], (-10, 180, 35, "GPS", 3)); // 地平線下
        assert_eq!(sky_plot[3], (45, 359, 30, "GPS", 4)); // 方位角最大付近
    }
}
