//! CFG-VALSET メッセージ生成
//!
//! UBX-CFG-VALSET: 設定値の変更
//! Class: 0x06, ID: 0x8A
//!
//! 主な用途: NMEA出力のON/OFF制御

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
}
