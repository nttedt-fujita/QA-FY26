//! CFG-VALSET メッセージ生成
//!
//! UBX-CFG-VALSET: 設定値の変更
//! Class: 0x06, ID: 0x8A
//!
//! 主な用途:
//! - NMEA出力のON/OFF制御
//! - 定期出力（Periodic Output）の設定

use super::common::{calculate_checksum, UBX_SYNC_1, UBX_SYNC_2};

/// CFG-VALSET メッセージ識別子
const CFG_CLASS: u8 = 0x06;
const CFG_VALSET_ID: u8 = 0x8A;

/// 設定レイヤー
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Layer {
    /// RAM（揮発性、電源OFFで消える）
    Ram = 0x01,
    /// BBR（バッテリバックアップRAM）
    Bbr = 0x02,
    /// Flash（不揮発性）
    Flash = 0x04,
}

/// CFG-UART1OUTPROT-NMEA のキーID
/// NMEA出力プロトコルのON/OFF
const CFG_UART1OUTPROT_NMEA: u32 = 0x10740002;

// ===========================================
// CFG-MSGOUT キーID（USB用）
// 出典: u-blox F9 HPG 1.32 Interface Description (UBX-22008968) p.234-251
// ===========================================

/// CFG-MSGOUT-UBX_NAV_PVT_USB
pub const CFG_MSGOUT_NAV_PVT_USB: u32 = 0x20910009;

/// CFG-MSGOUT-UBX_NAV_STATUS_USB
pub const CFG_MSGOUT_NAV_STATUS_USB: u32 = 0x2091001d;

/// CFG-MSGOUT-UBX_NAV_SAT_USB
pub const CFG_MSGOUT_NAV_SAT_USB: u32 = 0x20910018;

/// CFG-MSGOUT-UBX_NAV_SIG_USB
pub const CFG_MSGOUT_NAV_SIG_USB: u32 = 0x20910348;

/// CFG-MSGOUT-UBX_MON_SPAN_USB
pub const CFG_MSGOUT_MON_SPAN_USB: u32 = 0x2091038e;

/// CFG-MSGOUT-UBX_MON_RF_USB
pub const CFG_MSGOUT_MON_RF_USB: u32 = 0x2091035c;

/// NMEA出力を制御するCFG-VALSETメッセージを生成
///
/// # Arguments
/// * `enable` - true: NMEA出力ON, false: NMEA出力OFF
/// * `layer` - 設定レイヤー（RAM/BBR/Flash）
///
/// # Returns
/// 完全なUBXフレーム（ヘッダー + ペイロード + チェックサム）
pub fn set_uart1_nmea_output(enable: bool, layer: Layer) -> Vec<u8> {
    // ペイロード構成:
    // - version (1 byte): 0x00
    // - layers (1 byte): レイヤービットマスク
    // - reserved (2 bytes): 0x00, 0x00
    // - cfgData (N bytes): キー(4bytes) + 値(1byte for boolean)

    let value: u8 = if enable { 1 } else { 0 };

    // ペイロード
    let mut payload = vec![
        0x00,           // version
        layer as u8,    // layers
        0x00, 0x00,     // reserved
    ];

    // キー (little-endian)
    payload.extend_from_slice(&CFG_UART1OUTPROT_NMEA.to_le_bytes());

    // 値 (1 byte for L type)
    payload.push(value);

    // フレーム構築
    build_ubx_frame(CFG_CLASS, CFG_VALSET_ID, &payload)
}

/// UBXフレームを構築する
fn build_ubx_frame(class: u8, id: u8, payload: &[u8]) -> Vec<u8> {
    let payload_len = payload.len() as u16;

    let mut frame = vec![UBX_SYNC_1, UBX_SYNC_2, class, id];
    frame.extend_from_slice(&payload_len.to_le_bytes());
    frame.extend_from_slice(payload);

    // チェックサム計算（class〜payloadまで）
    let (ck_a, ck_b) = calculate_checksum(&frame[2..]);
    frame.push(ck_a);
    frame.push(ck_b);

    frame
}

// ===========================================
// 定期出力（Periodic Output）設定
// ===========================================

/// 定期出力設定
/// 出力レート（エポックごと）
#[derive(Debug, Clone, Copy)]
pub struct PeriodicOutputConfig {
    /// NAV-PVT出力レート（0=無効, 1=毎エポック）
    pub nav_pvt: u8,
    /// NAV-STATUS出力レート
    pub nav_status: u8,
    /// NAV-SAT出力レート
    pub nav_sat: u8,
    /// NAV-SIG出力レート
    pub nav_sig: u8,
    /// MON-SPAN出力レート
    pub mon_span: u8,
    /// MON-RF出力レート
    pub mon_rf: u8,
}

impl Default for PeriodicOutputConfig {
    fn default() -> Self {
        Self {
            nav_pvt: 1,      // 毎エポック
            nav_status: 1,   // 毎エポック
            nav_sat: 5,      // 5エポックに1回
            nav_sig: 5,      // 5エポックに1回
            mon_span: 10,    // 10エポックに1回
            mon_rf: 10,      // 10エポックに1回
        }
    }
}

/// 定期出力を有効化するCFG-VALSETメッセージを生成
///
/// # Arguments
/// * `config` - 定期出力設定（各メッセージの出力レート）
/// * `layer` - 設定レイヤー（RAM/BBR/Flash）
///
/// # Returns
/// 完全なUBXフレーム（ヘッダー + ペイロード + チェックサム）
pub fn set_periodic_output(config: &PeriodicOutputConfig, layer: Layer) -> Vec<u8> {
    // ペイロード構成:
    // - version (1 byte): 0x00
    // - layers (1 byte): レイヤービットマスク
    // - reserved (2 bytes): 0x00, 0x00
    // - cfgData (N bytes): 各キー(4bytes) + 値(1byte)

    let mut payload = vec![
        0x00,           // version
        layer as u8,    // layers
        0x00, 0x00,     // reserved
    ];

    // 各メッセージの設定を追加
    let configs: [(u32, u8); 6] = [
        (CFG_MSGOUT_NAV_PVT_USB, config.nav_pvt),
        (CFG_MSGOUT_NAV_STATUS_USB, config.nav_status),
        (CFG_MSGOUT_NAV_SAT_USB, config.nav_sat),
        (CFG_MSGOUT_NAV_SIG_USB, config.nav_sig),
        (CFG_MSGOUT_MON_SPAN_USB, config.mon_span),
        (CFG_MSGOUT_MON_RF_USB, config.mon_rf),
    ];

    for (key, value) in configs {
        payload.extend_from_slice(&key.to_le_bytes());
        payload.push(value);
    }

    build_ubx_frame(CFG_CLASS, CFG_VALSET_ID, &payload)
}

/// 定期出力を無効化するCFG-VALSETメッセージを生成
///
/// 全てのメッセージを出力レート0（ポーリングのみ）に設定
pub fn disable_periodic_output(layer: Layer) -> Vec<u8> {
    let config = PeriodicOutputConfig {
        nav_pvt: 0,
        nav_status: 0,
        nav_sat: 0,
        nav_sig: 0,
        mon_span: 0,
        mon_rf: 0,
    };
    set_periodic_output(&config, layer)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// V1: NMEA OFF メッセージの正常生成
    #[test]
    fn test_nmea_off_message() {
        let msg = set_uart1_nmea_output(false, Layer::Ram);

        // ヘッダーチェック
        assert_eq!(msg[0], 0xB5, "sync1");
        assert_eq!(msg[1], 0x62, "sync2");
        assert_eq!(msg[2], 0x06, "class");
        assert_eq!(msg[3], 0x8A, "id");

        // ペイロード長 (9 bytes: version(1) + layers(1) + reserved(2) + key(4) + value(1))
        let payload_len = u16::from_le_bytes([msg[4], msg[5]]);
        assert_eq!(payload_len, 9, "payload length");

        // version
        assert_eq!(msg[6], 0x00, "version");

        // layers (RAM = 0x01)
        assert_eq!(msg[7], 0x01, "layers");

        // reserved
        assert_eq!(msg[8], 0x00, "reserved[0]");
        assert_eq!(msg[9], 0x00, "reserved[1]");

        // key (CFG-UART1OUTPROT-NMEA = 0x10740002)
        let key = u32::from_le_bytes([msg[10], msg[11], msg[12], msg[13]]);
        assert_eq!(key, 0x10740002, "key");

        // value (0 = OFF)
        assert_eq!(msg[14], 0x00, "value (OFF)");

        // チェックサム検証
        let (ck_a, ck_b) = calculate_checksum(&msg[2..15]);
        assert_eq!(msg[15], ck_a, "checksum A");
        assert_eq!(msg[16], ck_b, "checksum B");
    }

    /// V2: NMEA ON メッセージの正常生成
    #[test]
    fn test_nmea_on_message() {
        let msg = set_uart1_nmea_output(true, Layer::Ram);

        // value (1 = ON)
        assert_eq!(msg[14], 0x01, "value (ON)");

        // 他のフィールドはV1と同じ構造
        assert_eq!(msg[0], 0xB5);
        assert_eq!(msg[1], 0x62);
        assert_eq!(msg[2], 0x06);
        assert_eq!(msg[3], 0x8A);
    }

    /// V3: 各レイヤーのテスト
    #[test]
    fn test_layers() {
        struct TestCase {
            layer: Layer,
            expected: u8,
            should_succeed: bool,
        }

        let cases = vec![
            TestCase { layer: Layer::Ram, expected: 0x01, should_succeed: true },
            TestCase { layer: Layer::Bbr, expected: 0x02, should_succeed: true },
            TestCase { layer: Layer::Flash, expected: 0x04, should_succeed: true },
        ];

        for tc in cases {
            let msg = set_uart1_nmea_output(false, tc.layer);
            if tc.should_succeed {
                assert_eq!(msg[7], tc.expected, "layer {:?}", tc.layer);
            }
        }
    }

    /// V4: メッセージ長の確認
    #[test]
    fn test_message_length() {
        let msg = set_uart1_nmea_output(false, Layer::Ram);

        // 全体長: sync(2) + class(1) + id(1) + len(2) + payload(9) + checksum(2) = 17
        assert_eq!(msg.len(), 17, "total message length");
    }

    /// V5: チェックサム計算の検証
    #[test]
    fn test_checksum_validity() {
        let msg = set_uart1_nmea_output(true, Layer::Ram);

        // チェックサム範囲: class〜payload (index 2〜14)
        let (expected_a, expected_b) = calculate_checksum(&msg[2..msg.len()-2]);
        assert_eq!(msg[msg.len()-2], expected_a, "checksum A");
        assert_eq!(msg[msg.len()-1], expected_b, "checksum B");
    }

    // ===========================================
    // 定期出力テスト
    // ===========================================

    /// P1: 定期出力設定メッセージの正常生成
    #[test]
    fn test_periodic_output_default() {
        let config = PeriodicOutputConfig::default();
        let msg = set_periodic_output(&config, Layer::Ram);

        // ヘッダーチェック
        assert_eq!(msg[0], 0xB5, "sync1");
        assert_eq!(msg[1], 0x62, "sync2");
        assert_eq!(msg[2], 0x06, "class");
        assert_eq!(msg[3], 0x8A, "id");

        // ペイロード長 (4 + 6*5 = 34 bytes)
        // version(1) + layers(1) + reserved(2) + 6個の(key(4) + value(1))
        let payload_len = u16::from_le_bytes([msg[4], msg[5]]);
        assert_eq!(payload_len, 34, "payload length");

        // version
        assert_eq!(msg[6], 0x00, "version");

        // layers (RAM = 0x01)
        assert_eq!(msg[7], 0x01, "layers");
    }

    /// P2: 最初のキーと値の検証（NAV-PVT）
    #[test]
    fn test_periodic_output_nav_pvt_key() {
        let config = PeriodicOutputConfig::default();
        let msg = set_periodic_output(&config, Layer::Ram);

        // 最初のキー (offset 10-13)
        let key = u32::from_le_bytes([msg[10], msg[11], msg[12], msg[13]]);
        assert_eq!(key, CFG_MSGOUT_NAV_PVT_USB, "NAV-PVT key");

        // 値 (offset 14)
        assert_eq!(msg[14], 1, "NAV-PVT rate = 1");
    }

    /// P3: 定期出力無効化のテスト
    #[test]
    fn test_disable_periodic_output() {
        let msg = disable_periodic_output(Layer::Ram);

        // 全ての値が0であることを確認
        // offset 14から始まる各値（5バイトごと）
        for i in 0..6 {
            let value_offset = 14 + i * 5;
            assert_eq!(msg[value_offset], 0, "value at index {} should be 0", i);
        }
    }

    /// P4: チェックサム検証
    #[test]
    fn test_periodic_output_checksum() {
        let config = PeriodicOutputConfig::default();
        let msg = set_periodic_output(&config, Layer::Ram);

        let (expected_a, expected_b) = calculate_checksum(&msg[2..msg.len()-2]);
        assert_eq!(msg[msg.len()-2], expected_a, "checksum A");
        assert_eq!(msg[msg.len()-1], expected_b, "checksum B");
    }
}
