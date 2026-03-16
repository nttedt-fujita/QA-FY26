//! MON-SPAN API
//!
//! - GET /api/mon-span - MON-SPAN取得（スペクトラム、PGA）

use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;
use std::time::Duration;

use crate::ubx::mon_span::{self, SpanBlock};
use super::device_api::{AppState, ErrorResponse};

// ===========================================
// レスポンス型
// ===========================================

/// スペクトラムブロック情報（API用）
#[derive(Debug, Clone, Serialize)]
pub struct SpanBlockResponse {
    /// スペクトラムデータ（256点、dB）
    pub spectrum: Vec<u8>,
    /// 周波数スパン（Hz）
    pub span: u32,
    /// 周波数分解能（Hz）
    pub res: u32,
    /// 中心周波数（Hz）
    pub center: u32,
    /// PGAゲイン（dB）
    pub pga: u8,
    /// スペクトラム最大値（dB）
    pub max_amplitude: u8,
    /// スペクトラム平均値（dB）
    pub avg_amplitude: f32,
}

impl From<&SpanBlock> for SpanBlockResponse {
    fn from(b: &SpanBlock) -> Self {
        Self {
            spectrum: b.spectrum.to_vec(),
            span: b.span,
            res: b.res,
            center: b.center,
            pga: b.pga,
            max_amplitude: b.max_amplitude(),
            avg_amplitude: b.avg_amplitude(),
        }
    }
}

/// MON-SPANレスポンス
#[derive(Debug, Clone, Serialize)]
pub struct MonSpanResponse {
    /// スペクトラムブロックリスト
    pub blocks: Vec<SpanBlockResponse>,
}

// ===========================================
// APIハンドラー
// ===========================================

/// MON-SPAN poll リクエストを構築
fn build_mon_span_poll() -> Vec<u8> {
    // MON-SPAN poll: class=0x0A, id=0x31, payload=empty
    let class: u8 = 0x0A;
    let id: u8 = 0x31;
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

/// GET /api/mon-span - MON-SPAN取得
pub async fn get_mon_span(data: web::Data<AppState>) -> impl Responder {
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

    // MON-SPAN poll送信
    let poll_msg = build_mon_span_poll();
    if let Err(e) = manager.send_ubx(&poll_msg) {
        return HttpResponse::InternalServerError().json(ErrorResponse {
            error: format!("送信エラー: {}", e),
            code: "SEND_ERROR".to_string(),
        });
    }

    // 少し待機
    std::thread::sleep(Duration::from_millis(50));

    // 応答受信（MON-SPANは大きいので3秒待つ）
    let response = match manager.receive_ubx(Duration::from_secs(3)) {
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
    let mon_span_result = match mon_span::parse(&response) {
        Ok(s) => s,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: format!("パースエラー: {}", e),
                code: "PARSE_ERROR".to_string(),
            });
        }
    };

    // レスポンス構築
    let block_responses: Vec<SpanBlockResponse> = mon_span_result.blocks.iter().map(|b| b.into()).collect();

    HttpResponse::Ok().json(MonSpanResponse {
        blocks: block_responses,
    })
}

/// APIルートを設定
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/mon-span")
            .route("", web::get().to(get_mon_span)),
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
    const SPECTRUM_BINS: usize = 256;
    const MON_SPAN_BLOCK_SIZE: usize = 272;
    const MON_SPAN_HEADER_LEN: usize = 4;

    fn f9p_port(path: &str) -> PortInfo {
        PortInfo {
            path: path.to_string(),
            vid: Some(F9P_VID),
            pid: Some(F9P_PID),
            serial_number: Some("TEST123".to_string()),
        }
    }

    /// テスト用のスペクトラムブロックを生成
    fn build_span_block(
        spectrum_fill: u8,
        span: u32,
        res: u32,
        center: u32,
        pga: u8,
    ) -> Vec<u8> {
        let mut block = vec![0u8; MON_SPAN_BLOCK_SIZE];

        // spectrum: offset 0-255
        for i in 0..SPECTRUM_BINS {
            block[i] = spectrum_fill;
        }

        // span: offset 256-259
        block[256..260].copy_from_slice(&span.to_le_bytes());

        // res: offset 260-263
        block[260..264].copy_from_slice(&res.to_le_bytes());

        // center: offset 264-267
        block[264..268].copy_from_slice(&center.to_le_bytes());

        // pga: offset 268
        block[268] = pga;

        block
    }

    /// テスト用のMON-SPANメッセージを生成
    fn build_mon_span_message(blocks: &[Vec<u8>]) -> Vec<u8> {
        let n_blocks = blocks.len() as u8;
        let payload_len = MON_SPAN_HEADER_LEN + blocks.len() * MON_SPAN_BLOCK_SIZE;

        let mut payload = vec![0u8; payload_len];
        // ヘッダー部
        payload[0] = 0x00; // version
        payload[1] = n_blocks;
        // offset 2-3: reserved0

        // ブロック部
        for (i, block) in blocks.iter().enumerate() {
            let offset = MON_SPAN_HEADER_LEN + i * MON_SPAN_BLOCK_SIZE;
            payload[offset..offset + MON_SPAN_BLOCK_SIZE].copy_from_slice(block);
        }

        // UBXフレーム構築
        let mut frame = vec![0xB5, 0x62, 0x0A, 0x31];
        let len = payload_len as u16;
        frame.extend_from_slice(&len.to_le_bytes());
        frame.extend_from_slice(&payload);

        // チェックサム計算
        let (ck_a, ck_b) = calculate_checksum(&frame[2..]);
        frame.push(ck_a);
        frame.push(ck_b);

        frame
    }

    // ===========================================
    // テスト
    // ===========================================

    /// T1: 接続済み・正常応答（2ブロック） → 200 + 2ブロック
    #[test]
    fn test_t1_connected_normal_response_2_blocks() {
        let blocks = vec![
            build_span_block(50, 128_000_000, 500_000, 1_575_420_000, 54),  // L1
            build_span_block(45, 128_000_000, 500_000, 1_227_600_000, 54),  // L2
        ];
        let response = build_mon_span_message(&blocks);

        let provider = MockProvider::new()
            .with_ports(vec![f9p_port("/dev/ttyACM0")])
            .with_responses(vec![response]);

        let mut manager = DeviceManager::new(provider);
        manager.list_devices().unwrap();
        manager.connect("/dev/ttyACM0").unwrap();

        // パースと変換を直接テスト（APIハンドラーはActixランタイムが必要）
        let poll_msg = build_mon_span_poll();
        manager.send_ubx(&poll_msg).unwrap();
        std::thread::sleep(Duration::from_millis(50));
        let raw_response = manager.receive_ubx(Duration::from_secs(1)).unwrap();
        let parsed = mon_span::parse(&raw_response).unwrap();

        // 検証
        assert_eq!(parsed.blocks.len(), 2);

        // L1ブロック
        assert_eq!(parsed.blocks[0].pga, 54);
        assert_eq!(parsed.blocks[0].center, 1_575_420_000);
        assert_eq!(parsed.blocks[0].max_amplitude(), 50);

        // L2ブロック
        assert_eq!(parsed.blocks[1].pga, 54);
        assert_eq!(parsed.blocks[1].center, 1_227_600_000);
        assert_eq!(parsed.blocks[1].max_amplitude(), 45);
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

        let poll_msg = build_mon_span_poll();
        manager.send_ubx(&poll_msg).unwrap();
        std::thread::sleep(Duration::from_millis(50));
        let result = manager.receive_ubx(Duration::from_millis(100));

        assert!(matches!(result, Err(DeviceManagerError::Timeout)));
    }

    /// T4: パースエラー（不正UBX）
    #[test]
    fn test_t4_parse_error() {
        // 不正なUBXメッセージ（MON-SPANヘッダーだがペイロード不足）
        let invalid_response = vec![0xB5, 0x62, 0x0A, 0x31, 0x00, 0x00, 0x00, 0x00];

        let provider = MockProvider::new()
            .with_ports(vec![f9p_port("/dev/ttyACM0")])
            .with_responses(vec![invalid_response]);

        let mut manager = DeviceManager::new(provider);
        manager.list_devices().unwrap();
        manager.connect("/dev/ttyACM0").unwrap();

        let poll_msg = build_mon_span_poll();
        manager.send_ubx(&poll_msg).unwrap();
        std::thread::sleep(Duration::from_millis(50));
        let raw_response = manager.receive_ubx(Duration::from_secs(1)).unwrap();
        let result = mon_span::parse(&raw_response);

        assert!(result.is_err());
    }

    /// T5: 空のブロック（nBlocks=0） → 200 + 空配列
    #[test]
    fn test_t5_empty_blocks() {
        let blocks: Vec<Vec<u8>> = vec![];
        let response = build_mon_span_message(&blocks);

        let provider = MockProvider::new()
            .with_ports(vec![f9p_port("/dev/ttyACM0")])
            .with_responses(vec![response]);

        let mut manager = DeviceManager::new(provider);
        manager.list_devices().unwrap();
        manager.connect("/dev/ttyACM0").unwrap();

        let poll_msg = build_mon_span_poll();
        manager.send_ubx(&poll_msg).unwrap();
        std::thread::sleep(Duration::from_millis(50));
        let raw_response = manager.receive_ubx(Duration::from_secs(1)).unwrap();
        let parsed = mon_span::parse(&raw_response).unwrap();

        assert_eq!(parsed.blocks.len(), 0);
    }

    /// T6: 1ブロックのみ（L1のみ） → 200 + 1ブロック
    #[test]
    fn test_t6_single_block() {
        let blocks = vec![
            build_span_block(50, 128_000_000, 500_000, 1_575_420_000, 54),  // L1のみ
        ];
        let response = build_mon_span_message(&blocks);

        let provider = MockProvider::new()
            .with_ports(vec![f9p_port("/dev/ttyACM0")])
            .with_responses(vec![response]);

        let mut manager = DeviceManager::new(provider);
        manager.list_devices().unwrap();
        manager.connect("/dev/ttyACM0").unwrap();

        let poll_msg = build_mon_span_poll();
        manager.send_ubx(&poll_msg).unwrap();
        std::thread::sleep(Duration::from_millis(50));
        let raw_response = manager.receive_ubx(Duration::from_secs(1)).unwrap();
        let parsed = mon_span::parse(&raw_response).unwrap();

        // 検証
        assert_eq!(parsed.blocks.len(), 1);
        assert_eq!(parsed.blocks[0].pga, 54);
        assert_eq!(parsed.blocks[0].center, 1_575_420_000);
    }
}
