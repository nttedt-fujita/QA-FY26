//! SEC-UNIQID メッセージパーサー
//!
//! UBX-SEC-UNIQID: Unique Chip ID
//! Class: 0x27, ID: 0x03
//! Payload: 9 bytes (固定)

use std::fmt;

use super::common::{calculate_checksum, UBX_SYNC_1, UBX_SYNC_2};

/// SEC-UNIQID メッセージ識別子
const SEC_CLASS: u8 = 0x27;
const SEC_UNIQID_ID: u8 = 0x03;

/// SEC-UNIQID ペイロード長
const SEC_UNIQID_PAYLOAD_LEN: usize = 9;

/// パースエラー
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    /// ヘッダーが不正（0xB5 0x62 で始まらない）
    InvalidHeader,
    /// Class/IDが不一致（SEC-UNIQIDではない）
    MessageMismatch { class: u8, id: u8 },
    /// チェックサムが不正
    ChecksumError { expected: (u8, u8), actual: (u8, u8) },
    /// ペイロード長が不正（9バイトでない）
    InvalidPayloadLength { expected: usize, actual: usize },
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
            ParseError::InvalidPayloadLength { expected, actual } => {
                write!(f, "Invalid payload length: expected={}, actual={}", expected, actual)
            }
        }
    }
}

impl std::error::Error for ParseError {}

/// SEC-UNIQID パース結果
#[derive(Debug, Clone, PartialEq)]
pub struct SecUniqid {
    /// メッセージバージョン
    pub version: u8,
    /// ユニークチップID（5バイト、40ビット）
    pub unique_id: [u8; 5],
}

impl SecUniqid {
    /// ユニークIDを16進数文字列で取得（10文字）
    pub fn unique_id_hex(&self) -> String {
        self.unique_id
            .iter()
            .map(|b| format!("{:02X}", b))
            .collect()
    }
}

/// SEC-UNIQIDメッセージをパースする
pub fn parse(data: &[u8]) -> Result<SecUniqid, ParseError> {
    // 最小長チェック: header(2) + class(1) + id(1) + len(2) + payload(9) + checksum(2) = 17
    let min_len = 2 + 1 + 1 + 2 + SEC_UNIQID_PAYLOAD_LEN + 2;
    if data.len() < min_len {
        return Err(ParseError::InvalidPayloadLength {
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
    if class != SEC_CLASS || id != SEC_UNIQID_ID {
        return Err(ParseError::MessageMismatch { class, id });
    }

    // ペイロード長チェック
    let payload_len = u16::from_le_bytes([data[4], data[5]]) as usize;
    if payload_len != SEC_UNIQID_PAYLOAD_LEN {
        return Err(ParseError::InvalidPayloadLength {
            expected: SEC_UNIQID_PAYLOAD_LEN,
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

    // version (offset 0)
    let version = payload[0];

    // reserved0 (offset 1-3) - スキップ

    // uniqueId (offset 4-8)
    let mut unique_id = [0u8; 5];
    unique_id.copy_from_slice(&payload[4..9]);

    Ok(SecUniqid { version, unique_id })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// テスト用のSEC-UNIQIDメッセージを生成
    fn build_sec_uniqid_message(version: u8, unique_id: [u8; 5]) -> Vec<u8> {
        let mut payload = vec![0u8; SEC_UNIQID_PAYLOAD_LEN];

        // version (offset 0)
        payload[0] = version;

        // reserved0 (offset 1-3) - 0で埋める

        // uniqueId (offset 4-8)
        payload[4..9].copy_from_slice(&unique_id);

        // UBXフレーム構築
        let mut frame = vec![UBX_SYNC_1, UBX_SYNC_2, SEC_CLASS, SEC_UNIQID_ID];
        let len = SEC_UNIQID_PAYLOAD_LEN as u16;
        frame.extend_from_slice(&len.to_le_bytes());
        frame.extend_from_slice(&payload);

        // チェックサム計算
        let (ck_a, ck_b) = calculate_checksum(&frame[2..]);
        frame.push(ck_a);
        frame.push(ck_b);

        frame
    }

    /// テストケース: 正常系
    #[test]
    fn test_parse_success_cases() {
        struct TestCase {
            name: &'static str,
            version: u8,
            unique_id: [u8; 5],
            expected_hex: &'static str,
            should_succeed: bool,
        }

        let cases = vec![
            // 正常系: 有効なSEC-UNIQID
            TestCase {
                name: "Valid SEC-UNIQID",
                version: 0x01,
                unique_id: [0x01, 0x23, 0x45, 0x67, 0x89],
                expected_hex: "0123456789",
                should_succeed: true,
            },
            // 正常系: 全ゼロID
            TestCase {
                name: "All zero ID",
                version: 0x01,
                unique_id: [0x00, 0x00, 0x00, 0x00, 0x00],
                expected_hex: "0000000000",
                should_succeed: true,
            },
            // 正常系: 全FF ID
            TestCase {
                name: "All FF ID",
                version: 0x01,
                unique_id: [0xFF, 0xFF, 0xFF, 0xFF, 0xFF],
                expected_hex: "FFFFFFFFFF",
                should_succeed: true,
            },
        ];

        for tc in cases {
            let data = build_sec_uniqid_message(tc.version, tc.unique_id);
            let result = parse(&data);

            if tc.should_succeed {
                let sec = result.unwrap_or_else(|e| panic!("{}: unexpected error: {}", tc.name, e));
                assert_eq!(sec.version, tc.version, "{}: version mismatch", tc.name);
                assert_eq!(sec.unique_id, tc.unique_id, "{}: unique_id mismatch", tc.name);
                assert_eq!(sec.unique_id_hex(), tc.expected_hex, "{}: unique_id_hex mismatch", tc.name);
            } else {
                assert!(result.is_err(), "{}: expected error", tc.name);
            }
        }
    }

    /// テストケース: エラー系
    #[test]
    fn test_parse_error_cases() {
        let valid_msg = build_sec_uniqid_message(0x01, [0x01, 0x23, 0x45, 0x67, 0x89]);

        struct TestCase {
            name: &'static str,
            data: Vec<u8>,
            should_succeed: bool,
        }

        let cases = vec![
            // 異常系: ヘッダー不正
            TestCase {
                name: "Invalid header (wrong sync1)",
                data: {
                    let mut d = valid_msg.clone();
                    d[0] = 0x00;
                    d
                },
                should_succeed: false,
            },
            // 異常系: Class/ID不一致
            TestCase {
                name: "Message mismatch (wrong class)",
                data: {
                    let mut d = valid_msg.clone();
                    d[2] = 0x01; // NAV class
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
            // 異常系: ペイロード長8バイト（9未満）
            TestCase {
                name: "Invalid payload length (8 bytes)",
                data: {
                    let mut frame = vec![UBX_SYNC_1, UBX_SYNC_2, SEC_CLASS, SEC_UNIQID_ID];
                    let len: u16 = 8;
                    frame.extend_from_slice(&len.to_le_bytes());
                    frame.extend(vec![0u8; 8]);
                    let (ck_a, ck_b) = calculate_checksum(&frame[2..]);
                    frame.push(ck_a);
                    frame.push(ck_b);
                    frame
                },
                should_succeed: false,
            },
            // 異常系: ペイロード長10バイト（9超）
            TestCase {
                name: "Invalid payload length (10 bytes)",
                data: {
                    let mut frame = vec![UBX_SYNC_1, UBX_SYNC_2, SEC_CLASS, SEC_UNIQID_ID];
                    let len: u16 = 10;
                    frame.extend_from_slice(&len.to_le_bytes());
                    frame.extend(vec![0u8; 10]);
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
