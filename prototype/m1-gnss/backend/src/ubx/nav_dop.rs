//! NAV-DOP メッセージパーサー
//!
//! UBX-NAV-DOP: Dilution of Precision
//! Class: 0x01, ID: 0x04
//! Payload: 18 bytes

use std::fmt;

/// UBXヘッダー
const UBX_SYNC_1: u8 = 0xB5;
const UBX_SYNC_2: u8 = 0x62;

/// NAV-DOP メッセージ識別子
const NAV_CLASS: u8 = 0x01;
const NAV_DOP_ID: u8 = 0x04;

/// NAV-DOP ペイロード長
const NAV_DOP_PAYLOAD_LEN: usize = 18;

/// DOPスケールファクター
const DOP_SCALE: f64 = 0.01;

/// パースエラー
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    /// ヘッダーが不正（0xB5 0x62 で始まらない）
    InvalidHeader,
    /// Class/IDが不一致（NAV-DOPではない）
    MessageMismatch { class: u8, id: u8 },
    /// チェックサムが不正
    ChecksumError { expected: (u8, u8), actual: (u8, u8) },
    /// ペイロード長不足
    InsufficientLength { expected: usize, actual: usize },
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
        }
    }
}

impl std::error::Error for ParseError {}

/// NAV-DOP パース結果
#[derive(Debug, Clone, PartialEq)]
pub struct NavDop {
    /// GPS Time of Week (ms)
    pub itow: u32,
    /// Geometric DOP
    pub g_dop: f64,
    /// Position DOP
    pub p_dop: f64,
    /// Time DOP
    pub t_dop: f64,
    /// Vertical DOP
    pub v_dop: f64,
    /// Horizontal DOP
    pub h_dop: f64,
    /// Northing DOP
    pub n_dop: f64,
    /// Easting DOP
    pub e_dop: f64,
}

impl NavDop {
    /// 位置精度が良好か（pDOP < 2.0）
    pub fn is_good_position_accuracy(&self) -> bool {
        self.p_dop < 2.0
    }

    /// 位置精度が許容範囲か（pDOP < 5.0）
    pub fn is_acceptable_position_accuracy(&self) -> bool {
        self.p_dop < 5.0
    }
}

/// NAV-DOPメッセージをパースする
pub fn parse(data: &[u8]) -> Result<NavDop, ParseError> {
    // 最小長チェック: header(2) + class(1) + id(1) + len(2) + payload(18) + checksum(2) = 26
    let min_len = 2 + 1 + 1 + 2 + NAV_DOP_PAYLOAD_LEN + 2;
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
    if class != NAV_CLASS || id != NAV_DOP_ID {
        return Err(ParseError::MessageMismatch { class, id });
    }

    // ペイロード長チェック
    let payload_len = u16::from_le_bytes([data[4], data[5]]) as usize;
    if payload_len != NAV_DOP_PAYLOAD_LEN {
        return Err(ParseError::InsufficientLength {
            expected: NAV_DOP_PAYLOAD_LEN,
            actual: payload_len,
        });
    }

    // チェックサム計算（Class, ID, Length, Payload に対して）
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

    // フィールド抽出
    let itow = u32::from_le_bytes([payload[0], payload[1], payload[2], payload[3]]);
    let g_dop = u16::from_le_bytes([payload[4], payload[5]]) as f64 * DOP_SCALE;
    let p_dop = u16::from_le_bytes([payload[6], payload[7]]) as f64 * DOP_SCALE;
    let t_dop = u16::from_le_bytes([payload[8], payload[9]]) as f64 * DOP_SCALE;
    let v_dop = u16::from_le_bytes([payload[10], payload[11]]) as f64 * DOP_SCALE;
    let h_dop = u16::from_le_bytes([payload[12], payload[13]]) as f64 * DOP_SCALE;
    let n_dop = u16::from_le_bytes([payload[14], payload[15]]) as f64 * DOP_SCALE;
    let e_dop = u16::from_le_bytes([payload[16], payload[17]]) as f64 * DOP_SCALE;

    Ok(NavDop {
        itow,
        g_dop,
        p_dop,
        t_dop,
        v_dop,
        h_dop,
        n_dop,
        e_dop,
    })
}

/// 8-bit Fletcher checksum（UBX仕様）
fn calculate_checksum(data: &[u8]) -> (u8, u8) {
    let mut ck_a: u8 = 0;
    let mut ck_b: u8 = 0;
    for byte in data {
        ck_a = ck_a.wrapping_add(*byte);
        ck_b = ck_b.wrapping_add(ck_a);
    }
    (ck_a, ck_b)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// テスト用のNAV-DOPメッセージを生成
    fn build_nav_dop_message(
        g_dop: u16,
        p_dop: u16,
        t_dop: u16,
        v_dop: u16,
        h_dop: u16,
        n_dop: u16,
        e_dop: u16,
    ) -> Vec<u8> {
        let mut payload = vec![0u8; NAV_DOP_PAYLOAD_LEN];

        // iTOW (offset 0-3)
        let itow: u32 = 123456000;
        payload[0..4].copy_from_slice(&itow.to_le_bytes());

        // DOP値（全てU2、スケール0.01）
        payload[4..6].copy_from_slice(&g_dop.to_le_bytes());
        payload[6..8].copy_from_slice(&p_dop.to_le_bytes());
        payload[8..10].copy_from_slice(&t_dop.to_le_bytes());
        payload[10..12].copy_from_slice(&v_dop.to_le_bytes());
        payload[12..14].copy_from_slice(&h_dop.to_le_bytes());
        payload[14..16].copy_from_slice(&n_dop.to_le_bytes());
        payload[16..18].copy_from_slice(&e_dop.to_le_bytes());

        // UBXフレーム構築
        let mut frame = vec![UBX_SYNC_1, UBX_SYNC_2, NAV_CLASS, NAV_DOP_ID];
        let len = NAV_DOP_PAYLOAD_LEN as u16;
        frame.extend_from_slice(&len.to_le_bytes());
        frame.extend_from_slice(&payload);

        // チェックサム計算
        let (ck_a, ck_b) = calculate_checksum(&frame[2..]);
        frame.push(ck_a);
        frame.push(ck_b);

        frame
    }

    /// テストケース: パース成功・スケール変換
    #[test]
    fn test_parse_success_and_scale() {
        struct TestCase {
            name: &'static str,
            g_dop: u16,
            p_dop: u16,
            t_dop: u16,
            v_dop: u16,
            h_dop: u16,
            n_dop: u16,
            e_dop: u16,
            expected_p_dop: f64,
            expected_h_dop: f64,
            expected_v_dop: f64,
            should_succeed: bool,
        }

        let cases = vec![
            // 正常系: 良好なDOP値
            TestCase {
                name: "Good DOP values",
                g_dop: 200,
                p_dop: 156,
                t_dop: 100,
                v_dop: 100,
                h_dop: 120,
                n_dop: 80,
                e_dop: 90,
                expected_p_dop: 1.56,
                expected_h_dop: 1.20,
                expected_v_dop: 1.00,
                should_succeed: true,
            },
            // 正常系: 高いDOP値（精度悪い）
            TestCase {
                name: "High DOP values",
                g_dop: 3000,
                p_dop: 2500,
                t_dop: 500,
                v_dop: 2000,
                h_dop: 1500,
                n_dop: 1000,
                e_dop: 1200,
                expected_p_dop: 25.0,
                expected_h_dop: 15.0,
                expected_v_dop: 20.0,
                should_succeed: true,
            },
            // 境界値: 最小値
            TestCase {
                name: "Minimum DOP values",
                g_dop: 1,
                p_dop: 1,
                t_dop: 1,
                v_dop: 1,
                h_dop: 1,
                n_dop: 1,
                e_dop: 1,
                expected_p_dop: 0.01,
                expected_h_dop: 0.01,
                expected_v_dop: 0.01,
                should_succeed: true,
            },
            // 境界値: 最大値
            TestCase {
                name: "Maximum DOP values",
                g_dop: 65535,
                p_dop: 65535,
                t_dop: 65535,
                v_dop: 65535,
                h_dop: 65535,
                n_dop: 65535,
                e_dop: 65535,
                expected_p_dop: 655.35,
                expected_h_dop: 655.35,
                expected_v_dop: 655.35,
                should_succeed: true,
            },
        ];

        for tc in cases {
            let data = build_nav_dop_message(
                tc.g_dop, tc.p_dop, tc.t_dop, tc.v_dop, tc.h_dop, tc.n_dop, tc.e_dop,
            );
            let result = parse(&data);

            if tc.should_succeed {
                let dop = result.unwrap_or_else(|e| panic!("{}: unexpected error: {}", tc.name, e));
                assert!(
                    (dop.p_dop - tc.expected_p_dop).abs() < 0.001,
                    "{}: p_dop mismatch: {} vs {}",
                    tc.name,
                    dop.p_dop,
                    tc.expected_p_dop
                );
                assert!(
                    (dop.h_dop - tc.expected_h_dop).abs() < 0.001,
                    "{}: h_dop mismatch: {} vs {}",
                    tc.name,
                    dop.h_dop,
                    tc.expected_h_dop
                );
                assert!(
                    (dop.v_dop - tc.expected_v_dop).abs() < 0.001,
                    "{}: v_dop mismatch: {} vs {}",
                    tc.name,
                    dop.v_dop,
                    tc.expected_v_dop
                );
            } else {
                assert!(result.is_err(), "{}: expected error", tc.name);
            }
        }
    }

    /// テストケース: 精度判定ヘルパー
    #[test]
    fn test_accuracy_helpers() {
        struct TestCase {
            name: &'static str,
            p_dop: u16,
            is_good: bool,
            is_acceptable: bool,
            should_succeed: bool,
        }

        let cases = vec![
            // 良好（pDOP < 2.0）
            TestCase {
                name: "Good accuracy",
                p_dop: 150, // 1.50
                is_good: true,
                is_acceptable: true,
                should_succeed: true,
            },
            // 許容範囲（2.0 <= pDOP < 5.0）
            TestCase {
                name: "Acceptable accuracy",
                p_dop: 350, // 3.50
                is_good: false,
                is_acceptable: true,
                should_succeed: true,
            },
            // 不良（pDOP >= 5.0）
            TestCase {
                name: "Poor accuracy",
                p_dop: 600, // 6.00
                is_good: false,
                is_acceptable: false,
                should_succeed: true,
            },
        ];

        for tc in cases {
            let data = build_nav_dop_message(tc.p_dop, tc.p_dop, 100, 100, 100, 100, 100);
            let result = parse(&data);

            if tc.should_succeed {
                let dop = result.unwrap_or_else(|e| panic!("{}: unexpected error: {}", tc.name, e));
                assert_eq!(
                    dop.is_good_position_accuracy(),
                    tc.is_good,
                    "{}: is_good mismatch",
                    tc.name
                );
                assert_eq!(
                    dop.is_acceptable_position_accuracy(),
                    tc.is_acceptable,
                    "{}: is_acceptable mismatch",
                    tc.name
                );
            }
        }
    }

    /// テストケース: エラー系
    #[test]
    fn test_parse_error_cases() {
        let valid_msg = build_nav_dop_message(200, 156, 100, 100, 120, 80, 90);

        struct TestCase {
            name: &'static str,
            data: Vec<u8>,
            should_succeed: bool,
        }

        let cases = vec![
            // 異常系: ヘッダー不正
            TestCase {
                name: "Invalid header",
                data: {
                    let mut d = valid_msg.clone();
                    d[0] = 0x00;
                    d
                },
                should_succeed: false,
            },
            // 異常系: Class/ID不一致（NAV-PVTのID）
            TestCase {
                name: "Message mismatch (wrong id)",
                data: {
                    let mut d = valid_msg.clone();
                    d[3] = 0x07; // NAV-PVT ID
                    d
                },
                should_succeed: false,
            },
            // 異常系: チェックサム不正
            TestCase {
                name: "Checksum error",
                data: {
                    let mut d = valid_msg.clone();
                    let len = d.len();
                    d[len - 1] = 0xFF;
                    d
                },
                should_succeed: false,
            },
            // 異常系: ペイロード長不足
            TestCase {
                name: "Insufficient length",
                data: vec![0xB5, 0x62, 0x01, 0x04, 0x0A, 0x00], // 10バイトのペイロード長
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
