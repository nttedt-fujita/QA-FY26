//! CFG-RATE メッセージパーサー
//!
//! UBX-CFG-RATE: ナビゲーション/測定レート設定
//! Class: 0x06, ID: 0x08
//! Payload: 6 bytes (固定長)

use std::fmt;

use super::common::{calculate_checksum, UBX_SYNC_1, UBX_SYNC_2};

/// CFG-RATE メッセージ識別子
const CFG_CLASS: u8 = 0x06;
const CFG_RATE_ID: u8 = 0x08;

/// CFG-RATE ペイロード長
const CFG_RATE_PAYLOAD_LEN: usize = 6;

/// パースエラー
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    /// ヘッダーが不正（0xB5 0x62 で始まらない）
    InvalidHeader,
    /// Class/IDが不一致（CFG-RATEではない）
    MessageMismatch { class: u8, id: u8 },
    /// チェックサムが不正
    ChecksumError { expected: (u8, u8), actual: (u8, u8) },
    /// ペイロード長不足（6バイト未満）
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

/// 時刻基準
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimeRef {
    Utc,
    Gps,
    Glonass,
    BeiDou,
    Galileo,
    NavIc,
    /// 未知の値（将来の仕様拡張に備える）
    Unknown(u16),
}

impl From<u16> for TimeRef {
    fn from(value: u16) -> Self {
        match value {
            0 => TimeRef::Utc,
            1 => TimeRef::Gps,
            2 => TimeRef::Glonass,
            3 => TimeRef::BeiDou,
            4 => TimeRef::Galileo,
            5 => TimeRef::NavIc,
            v => TimeRef::Unknown(v),
        }
    }
}

/// CFG-RATE パース結果
#[derive(Debug, Clone, PartialEq)]
pub struct CfgRate {
    /// 測定間隔 (ms)。最小25ms。
    pub meas_rate: u16,
    /// ナビゲーション比率。最大127。
    pub nav_rate: u16,
    /// 時刻基準
    pub time_ref: TimeRef,
}

/// CFG-RATEメッセージをパースする
pub fn parse(data: &[u8]) -> Result<CfgRate, ParseError> {
    // 最小長チェック: header(2) + class(1) + id(1) + len(2) + payload(6) + checksum(2) = 14
    let min_len = 2 + 1 + 1 + 2 + CFG_RATE_PAYLOAD_LEN + 2;
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
    if class != CFG_CLASS || id != CFG_RATE_ID {
        return Err(ParseError::MessageMismatch { class, id });
    }

    // ペイロード長チェック
    let payload_len = u16::from_le_bytes([data[4], data[5]]) as usize;
    if payload_len < CFG_RATE_PAYLOAD_LEN {
        return Err(ParseError::InsufficientLength {
            expected: CFG_RATE_PAYLOAD_LEN,
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

    // フィールド抽出
    let meas_rate = u16::from_le_bytes([payload[0], payload[1]]);
    let nav_rate = u16::from_le_bytes([payload[2], payload[3]]);
    let time_ref_raw = u16::from_le_bytes([payload[4], payload[5]]);

    Ok(CfgRate {
        meas_rate,
        nav_rate,
        time_ref: TimeRef::from(time_ref_raw),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// テスト用のCFG-RATEメッセージを生成
    fn build_cfg_rate_message(meas_rate: u16, nav_rate: u16, time_ref: u16) -> Vec<u8> {
        let payload_len = CFG_RATE_PAYLOAD_LEN as u16;
        let mut frame = vec![UBX_SYNC_1, UBX_SYNC_2, CFG_CLASS, CFG_RATE_ID];
        frame.extend_from_slice(&payload_len.to_le_bytes());

        // payload
        frame.extend_from_slice(&meas_rate.to_le_bytes());
        frame.extend_from_slice(&nav_rate.to_le_bytes());
        frame.extend_from_slice(&time_ref.to_le_bytes());

        // チェックサム計算
        let (ck_a, ck_b) = calculate_checksum(&frame[2..]);
        frame.push(ck_a);
        frame.push(ck_b);

        frame
    }

    /// R1-R7: 正常系テスト
    #[test]
    fn test_parse_success_cases() {
        struct TestCase {
            name: &'static str,
            meas_rate: u16,
            nav_rate: u16,
            time_ref: u16,
            expected_time_ref: TimeRef,
            should_succeed: bool,
        }

        let cases = vec![
            // R1: 基本パース成功
            TestCase {
                name: "R1: basic parse success",
                meas_rate: 1000,
                nav_rate: 1,
                time_ref: 0,
                expected_time_ref: TimeRef::Utc,
                should_succeed: true,
            },
            // R2: 1Hz (1000ms)
            TestCase {
                name: "R2: 1Hz (1000ms)",
                meas_rate: 1000,
                nav_rate: 1,
                time_ref: 1,
                expected_time_ref: TimeRef::Gps,
                should_succeed: true,
            },
            // R3: 最小値 40Hz (25ms)
            TestCase {
                name: "R3: min 40Hz (25ms)",
                meas_rate: 25,
                nav_rate: 1,
                time_ref: 0,
                expected_time_ref: TimeRef::Utc,
                should_succeed: true,
            },
            // R4: navRate=1
            TestCase {
                name: "R4: navRate=1",
                meas_rate: 100,
                nav_rate: 1,
                time_ref: 0,
                expected_time_ref: TimeRef::Utc,
                should_succeed: true,
            },
            // R5: navRate最大値 (127)
            TestCase {
                name: "R5: navRate max (127)",
                meas_rate: 100,
                nav_rate: 127,
                time_ref: 0,
                expected_time_ref: TimeRef::Utc,
                should_succeed: true,
            },
            // R7: timeRef範囲外 → Unknown
            TestCase {
                name: "R7: timeRef out of range (6) -> Unknown",
                meas_rate: 1000,
                nav_rate: 1,
                time_ref: 6,
                expected_time_ref: TimeRef::Unknown(6),
                should_succeed: true,
            },
        ];

        for tc in cases {
            let data = build_cfg_rate_message(tc.meas_rate, tc.nav_rate, tc.time_ref);
            let result = parse(&data);

            if tc.should_succeed {
                let cfg = result.unwrap_or_else(|e| panic!("{}: unexpected error: {}", tc.name, e));
                assert_eq!(cfg.meas_rate, tc.meas_rate, "{}: meas_rate mismatch", tc.name);
                assert_eq!(cfg.nav_rate, tc.nav_rate, "{}: nav_rate mismatch", tc.name);
                assert_eq!(cfg.time_ref, tc.expected_time_ref, "{}: time_ref mismatch", tc.name);
            } else {
                assert!(result.is_err(), "{}: expected error", tc.name);
            }
        }
    }

    /// R6: timeRef全値 (0〜5)
    #[test]
    fn test_time_ref_all_values() {
        struct TestCase {
            time_ref: u16,
            expected: TimeRef,
            should_succeed: bool,
        }

        let cases = vec![
            TestCase { time_ref: 0, expected: TimeRef::Utc, should_succeed: true },
            TestCase { time_ref: 1, expected: TimeRef::Gps, should_succeed: true },
            TestCase { time_ref: 2, expected: TimeRef::Glonass, should_succeed: true },
            TestCase { time_ref: 3, expected: TimeRef::BeiDou, should_succeed: true },
            TestCase { time_ref: 4, expected: TimeRef::Galileo, should_succeed: true },
            TestCase { time_ref: 5, expected: TimeRef::NavIc, should_succeed: true },
        ];

        for tc in cases {
            let data = build_cfg_rate_message(1000, 1, tc.time_ref);
            let result = parse(&data);

            if tc.should_succeed {
                let cfg = result.unwrap_or_else(|e| {
                    panic!("timeRef={}: unexpected error: {}", tc.time_ref, e)
                });
                assert_eq!(cfg.time_ref, tc.expected, "timeRef={}: mismatch", tc.time_ref);
            }
        }
    }

    /// R8-R11: エラー系テスト
    #[test]
    fn test_parse_error_cases() {
        let valid_msg = build_cfg_rate_message(1000, 1, 0);

        struct TestCase {
            name: &'static str,
            data: Vec<u8>,
            should_succeed: bool,
        }

        let cases = vec![
            // R8: ヘッダー不正
            TestCase {
                name: "R8: Invalid header",
                data: {
                    let mut d = valid_msg.clone();
                    d[0] = 0x00;
                    d
                },
                should_succeed: false,
            },
            // R9: Class/ID不一致
            TestCase {
                name: "R9: Message mismatch",
                data: {
                    let mut d = valid_msg.clone();
                    d[3] = 0x09; // wrong ID
                    d
                },
                should_succeed: false,
            },
            // R10: チェックサム不正
            TestCase {
                name: "R10: Checksum error",
                data: {
                    let mut d = valid_msg.clone();
                    let len = d.len();
                    d[len - 1] = 0xFF;
                    d
                },
                should_succeed: false,
            },
            // R11: ペイロード長不足 (5バイト)
            TestCase {
                name: "R11: Insufficient payload length",
                data: {
                    let mut frame = vec![UBX_SYNC_1, UBX_SYNC_2, CFG_CLASS, CFG_RATE_ID];
                    let len: u16 = 5;
                    frame.extend_from_slice(&len.to_le_bytes());
                    frame.extend(vec![0u8; 5]);
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
