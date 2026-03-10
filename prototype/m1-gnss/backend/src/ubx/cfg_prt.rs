//! CFG-PRT メッセージパーサー（USBポートのみ）
//!
//! UBX-CFG-PRT: ポート設定
//! Class: 0x06, ID: 0x00
//! Payload: 20 bytes (固定長)
//!
//! Note: GNSS評価ツールではUSBポート（portID=3）のみをサポート

use std::fmt;

use super::common::{calculate_checksum, UBX_SYNC_1, UBX_SYNC_2};

/// CFG-PRT メッセージ識別子
const CFG_CLASS: u8 = 0x06;
const CFG_PRT_ID: u8 = 0x00;

/// CFG-PRT ペイロード長
const CFG_PRT_PAYLOAD_LEN: usize = 20;

/// USBポートID
const USB_PORT_ID: u8 = 3;

/// パースエラー
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    /// ヘッダーが不正（0xB5 0x62 で始まらない）
    InvalidHeader,
    /// Class/IDが不一致（CFG-PRTではない）
    MessageMismatch { class: u8, id: u8 },
    /// チェックサムが不正
    ChecksumError { expected: (u8, u8), actual: (u8, u8) },
    /// ペイロード長不足（20バイト未満）
    InsufficientLength { expected: usize, actual: usize },
    /// サポート外のポート（USB以外）
    UnsupportedPort { port_id: u8 },
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidHeader => write!(f, "Invalid UBX header"),
            ParseError::MessageMismatch { class, id } => {
                write!(f, "Message mismatch: class=0x{:02X}, id=0x{:02X}", class, id)
            }
            ParseError::ChecksumError { expected, actual } => {
                write!(
                    f,
                    "Checksum error: expected=({:02X},{:02X}), actual=({:02X},{:02X})",
                    expected.0, expected.1, actual.0, actual.1
                )
            }
            ParseError::InsufficientLength { expected, actual } => {
                write!(f, "Insufficient length: expected={}, actual={}", expected, actual)
            }
            ParseError::UnsupportedPort { port_id } => {
                write!(f, "Unsupported port: portID={} (only USB portID=3 is supported)", port_id)
            }
        }
    }
}

impl std::error::Error for ParseError {}

/// プロトコルマスク
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ProtoMask {
    /// UBXプロトコル (bit 0)
    pub ubx: bool,
    /// NMEAプロトコル (bit 1)
    pub nmea: bool,
    /// RTCM3プロトコル (bit 5)
    pub rtcm3: bool,
}

impl From<u16> for ProtoMask {
    fn from(value: u16) -> Self {
        ProtoMask {
            ubx: (value & 0x01) != 0,
            nmea: (value & 0x02) != 0,
            rtcm3: (value & 0x20) != 0,
        }
    }
}

/// CFG-PRT パース結果（USBポート）
#[derive(Debug, Clone, PartialEq)]
pub struct CfgPrtUsb {
    /// ポートID（常に3）
    pub port_id: u8,
    /// 入力プロトコルマスク
    pub in_proto_mask: ProtoMask,
    /// 出力プロトコルマスク
    pub out_proto_mask: ProtoMask,
}

/// CFG-PRTメッセージをパースする（USBポートのみ）
pub fn parse(data: &[u8]) -> Result<CfgPrtUsb, ParseError> {
    // 最小長チェック: header(2) + class(1) + id(1) + len(2) + payload(20) + checksum(2) = 28
    let min_len = 2 + 1 + 1 + 2 + CFG_PRT_PAYLOAD_LEN + 2;
    if data.len() < min_len {
        return Err(ParseError::InsufficientLength {
            expected: min_len,
            actual: data.len(),
        });
    }

    // ヘッダーチェック
    if data[0] != UBX_SYNC_1 || data[1] != UBX_SYNC_2 {
        return Err(ParseError::InvalidHeader);
    }

    // Class/IDチェック
    let class = data[2];
    let id = data[3];
    if class != CFG_CLASS || id != CFG_PRT_ID {
        return Err(ParseError::MessageMismatch { class, id });
    }

    // ペイロード長チェック
    let payload_len = u16::from_le_bytes([data[4], data[5]]) as usize;
    if payload_len < CFG_PRT_PAYLOAD_LEN {
        return Err(ParseError::InsufficientLength {
            expected: CFG_PRT_PAYLOAD_LEN,
            actual: payload_len,
        });
    }

    // フレーム全体の長さチェック
    let frame_len = 6 + payload_len + 2;
    if data.len() < frame_len {
        return Err(ParseError::InsufficientLength {
            expected: frame_len,
            actual: data.len(),
        });
    }

    // チェックサム計算
    let checksum_range = &data[2..6 + payload_len];
    let (ck_a, ck_b) = calculate_checksum(checksum_range);
    let expected_ck_a = data[6 + payload_len];
    let expected_ck_b = data[6 + payload_len + 1];
    if ck_a != expected_ck_a || ck_b != expected_ck_b {
        return Err(ParseError::ChecksumError {
            expected: (expected_ck_a, expected_ck_b),
            actual: (ck_a, ck_b),
        });
    }

    // ペイロード開始位置
    let payload = &data[6..6 + payload_len];

    // portIDチェック（USBポートのみサポート）
    let port_id = payload[0];
    if port_id != USB_PORT_ID {
        return Err(ParseError::UnsupportedPort { port_id });
    }

    // フィールド抽出（USB構造）
    // offset 12-13: inProtoMask
    // offset 14-15: outProtoMask
    let in_proto_mask_raw = u16::from_le_bytes([payload[12], payload[13]]);
    let out_proto_mask_raw = u16::from_le_bytes([payload[14], payload[15]]);

    Ok(CfgPrtUsb {
        port_id,
        in_proto_mask: ProtoMask::from(in_proto_mask_raw),
        out_proto_mask: ProtoMask::from(out_proto_mask_raw),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// テスト用のCFG-PRTメッセージを生成（USBポート）
    fn build_cfg_prt_message(port_id: u8, in_proto_mask: u16, out_proto_mask: u16) -> Vec<u8> {
        let payload_len = CFG_PRT_PAYLOAD_LEN as u16;
        let mut frame = vec![UBX_SYNC_1, UBX_SYNC_2, CFG_CLASS, CFG_PRT_ID];
        frame.extend_from_slice(&payload_len.to_le_bytes());

        // payload (20 bytes)
        let mut payload = vec![0u8; CFG_PRT_PAYLOAD_LEN];
        payload[0] = port_id;          // portID
        // reserved0 (1), txReady (2), reserved1 (8) = 11 bytes
        payload[12] = (in_proto_mask & 0xFF) as u8;
        payload[13] = ((in_proto_mask >> 8) & 0xFF) as u8;
        payload[14] = (out_proto_mask & 0xFF) as u8;
        payload[15] = ((out_proto_mask >> 8) & 0xFF) as u8;
        // reserved2 (2), reserved3 (2) = 4 bytes

        frame.extend_from_slice(&payload);

        // チェックサム計算
        let (ck_a, ck_b) = calculate_checksum(&frame[2..]);
        frame.push(ck_a);
        frame.push(ck_b);

        frame
    }

    /// P1: 基本パース成功
    #[test]
    fn test_parse_basic_success() {
        let data = build_cfg_prt_message(USB_PORT_ID, 0x03, 0x03);
        let result = parse(&data);

        let cfg = result.expect("P1: should succeed");
        assert_eq!(cfg.port_id, USB_PORT_ID);
        assert!(cfg.in_proto_mask.ubx);
        assert!(cfg.in_proto_mask.nmea);
        assert!(!cfg.in_proto_mask.rtcm3);
        assert!(cfg.out_proto_mask.ubx);
        assert!(cfg.out_proto_mask.nmea);
        assert!(!cfg.out_proto_mask.rtcm3);
    }

    /// P2-P5: inProtoMask テスト
    #[test]
    fn test_in_proto_mask_cases() {
        struct TestCase {
            name: &'static str,
            in_proto_mask: u16,
            expected_ubx: bool,
            expected_nmea: bool,
            expected_rtcm3: bool,
            should_succeed: bool,
        }

        let cases = vec![
            // P2: UBX単独
            TestCase {
                name: "P2: inProtoMask UBX only",
                in_proto_mask: 0x01,
                expected_ubx: true,
                expected_nmea: false,
                expected_rtcm3: false,
                should_succeed: true,
            },
            // P3: NMEA単独
            TestCase {
                name: "P3: inProtoMask NMEA only",
                in_proto_mask: 0x02,
                expected_ubx: false,
                expected_nmea: true,
                expected_rtcm3: false,
                should_succeed: true,
            },
            // P4: RTCM3単独
            TestCase {
                name: "P4: inProtoMask RTCM3 only",
                in_proto_mask: 0x20,
                expected_ubx: false,
                expected_nmea: false,
                expected_rtcm3: true,
                should_succeed: true,
            },
            // P5: 複数同時 (UBX + NMEA + RTCM3)
            TestCase {
                name: "P5: inProtoMask multiple (UBX+NMEA+RTCM3)",
                in_proto_mask: 0x23,
                expected_ubx: true,
                expected_nmea: true,
                expected_rtcm3: true,
                should_succeed: true,
            },
        ];

        for tc in cases {
            let data = build_cfg_prt_message(USB_PORT_ID, tc.in_proto_mask, 0x00);
            let result = parse(&data);

            if tc.should_succeed {
                let cfg = result.unwrap_or_else(|e| panic!("{}: unexpected error: {}", tc.name, e));
                assert_eq!(cfg.in_proto_mask.ubx, tc.expected_ubx, "{}: ubx mismatch", tc.name);
                assert_eq!(cfg.in_proto_mask.nmea, tc.expected_nmea, "{}: nmea mismatch", tc.name);
                assert_eq!(cfg.in_proto_mask.rtcm3, tc.expected_rtcm3, "{}: rtcm3 mismatch", tc.name);
            } else {
                assert!(result.is_err(), "{}: expected error", tc.name);
            }
        }
    }

    /// P6-P8: outProtoMask テスト
    #[test]
    fn test_out_proto_mask_cases() {
        struct TestCase {
            name: &'static str,
            out_proto_mask: u16,
            expected_ubx: bool,
            expected_nmea: bool,
            expected_rtcm3: bool,
            should_succeed: bool,
        }

        let cases = vec![
            // P6: UBX
            TestCase {
                name: "P6: outProtoMask UBX",
                out_proto_mask: 0x01,
                expected_ubx: true,
                expected_nmea: false,
                expected_rtcm3: false,
                should_succeed: true,
            },
            // P7: NMEA
            TestCase {
                name: "P7: outProtoMask NMEA",
                out_proto_mask: 0x02,
                expected_ubx: false,
                expected_nmea: true,
                expected_rtcm3: false,
                should_succeed: true,
            },
            // P8: RTCM3
            TestCase {
                name: "P8: outProtoMask RTCM3",
                out_proto_mask: 0x20,
                expected_ubx: false,
                expected_nmea: false,
                expected_rtcm3: true,
                should_succeed: true,
            },
        ];

        for tc in cases {
            let data = build_cfg_prt_message(USB_PORT_ID, 0x00, tc.out_proto_mask);
            let result = parse(&data);

            if tc.should_succeed {
                let cfg = result.unwrap_or_else(|e| panic!("{}: unexpected error: {}", tc.name, e));
                assert_eq!(cfg.out_proto_mask.ubx, tc.expected_ubx, "{}: ubx mismatch", tc.name);
                assert_eq!(cfg.out_proto_mask.nmea, tc.expected_nmea, "{}: nmea mismatch", tc.name);
                assert_eq!(cfg.out_proto_mask.rtcm3, tc.expected_rtcm3, "{}: rtcm3 mismatch", tc.name);
            } else {
                assert!(result.is_err(), "{}: expected error", tc.name);
            }
        }
    }

    /// P9: portID≠3 (USB以外)
    #[test]
    fn test_unsupported_port() {
        struct TestCase {
            name: &'static str,
            port_id: u8,
            should_succeed: bool,
        }

        let cases = vec![
            TestCase { name: "P9: UART1 (portID=1)", port_id: 1, should_succeed: false },
            TestCase { name: "P9: UART2 (portID=2)", port_id: 2, should_succeed: false },
            TestCase { name: "P9: I2C (portID=0)", port_id: 0, should_succeed: false },
            TestCase { name: "P9: SPI (portID=4)", port_id: 4, should_succeed: false },
        ];

        for tc in cases {
            let data = build_cfg_prt_message(tc.port_id, 0x03, 0x03);
            let result = parse(&data);

            if tc.should_succeed {
                assert!(result.is_ok(), "{}: expected success", tc.name);
            } else {
                match result {
                    Err(ParseError::UnsupportedPort { port_id }) => {
                        assert_eq!(port_id, tc.port_id, "{}: port_id mismatch", tc.name);
                    }
                    _ => panic!("{}: expected UnsupportedPort error, got {:?}", tc.name, result),
                }
            }
        }
    }

    /// P10-P13: エラー系テスト
    #[test]
    fn test_parse_error_cases() {
        let valid_msg = build_cfg_prt_message(USB_PORT_ID, 0x03, 0x03);

        struct TestCase {
            name: &'static str,
            data: Vec<u8>,
            should_succeed: bool,
        }

        let cases = vec![
            // P10: ヘッダー不正
            TestCase {
                name: "P10: Invalid header",
                data: {
                    let mut d = valid_msg.clone();
                    d[0] = 0x00;
                    d
                },
                should_succeed: false,
            },
            // P11: Class/ID不一致
            TestCase {
                name: "P11: Message mismatch",
                data: {
                    let mut d = valid_msg.clone();
                    d[3] = 0x01; // wrong ID
                    d
                },
                should_succeed: false,
            },
            // P12: チェックサム不正
            TestCase {
                name: "P12: Checksum error",
                data: {
                    let mut d = valid_msg.clone();
                    let len = d.len();
                    d[len - 1] = 0xFF;
                    d
                },
                should_succeed: false,
            },
            // P13: ペイロード長不足 (19バイト)
            TestCase {
                name: "P13: Insufficient payload length",
                data: {
                    let mut frame = vec![UBX_SYNC_1, UBX_SYNC_2, CFG_CLASS, CFG_PRT_ID];
                    let len: u16 = 19;
                    frame.extend_from_slice(&len.to_le_bytes());
                    frame.extend(vec![0u8; 19]);
                    let (ck_a, ck_b) = calculate_checksum(&frame[2..]);
                    frame.push(ck_a);
                    frame.push(ck_b);
                    frame
                },
                should_succeed: false,
            },
        ];

        for tc in cases {
            let result = parse(&tc.data);
            if tc.should_succeed {
                assert!(result.is_ok(), "{}: expected success", tc.name);
            } else {
                assert!(result.is_err(), "{}: expected error, got {:?}", tc.name, result);
            }
        }
    }
}
