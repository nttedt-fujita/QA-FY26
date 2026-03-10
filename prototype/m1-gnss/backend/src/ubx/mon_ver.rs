//! MON-VER メッセージパーサー
//!
//! UBX-MON-VER: Receiver/Software/Hardware Version
//! Class: 0x0A, ID: 0x04
//! Payload: 40 + N×30 bytes (可変長)

use std::fmt;

use super::common::{calculate_checksum, UBX_SYNC_1, UBX_SYNC_2};

/// MON-VER メッセージ識別子
const MON_CLASS: u8 = 0x0A;
const MON_VER_ID: u8 = 0x04;

/// MON-VER 固定部分のペイロード長（swVersion + hwVersion）
const MON_VER_BASE_PAYLOAD_LEN: usize = 40;

/// extension 1個のサイズ
const EXTENSION_SIZE: usize = 30;

/// パースエラー
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    /// ヘッダーが不正（0xB5 0x62 で始まらない）
    InvalidHeader,
    /// Class/IDが不一致（MON-VERではない）
    MessageMismatch { class: u8, id: u8 },
    /// チェックサムが不正
    ChecksumError { expected: (u8, u8), actual: (u8, u8) },
    /// ペイロード長不足（40バイト未満）
    InsufficientLength { expected: usize, actual: usize },
    /// ペイロード長が不正（40 + N×30 の形式でない）
    InvalidPayloadLength { actual: usize },
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
            ParseError::InvalidPayloadLength { actual } => {
                write!(f, "Invalid payload length: {} (must be 40 + N×30)", actual)
            }
        }
    }
}

impl std::error::Error for ParseError {}

/// MON-VER パース結果
#[derive(Debug, Clone, PartialEq)]
pub struct MonVer {
    /// ソフトウェアバージョン（Null終端文字列）
    pub sw_version: String,
    /// ハードウェアバージョン（Null終端文字列）
    pub hw_version: String,
    /// 拡張情報（Null終端文字列のリスト）
    pub extensions: Vec<String>,
}

/// MON-VERメッセージをパースする
pub fn parse(data: &[u8]) -> Result<MonVer, ParseError> {
    // 最小長チェック: header(2) + class(1) + id(1) + len(2) + payload(40) + checksum(2) = 48
    let min_len = 2 + 1 + 1 + 2 + MON_VER_BASE_PAYLOAD_LEN + 2;
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
    if class != MON_CLASS || id != MON_VER_ID {
        return Err(ParseError::MessageMismatch { class, id });
    }

    // ペイロード長チェック
    let payload_len = u16::from_le_bytes([data[4], data[5]]) as usize;
    if payload_len < MON_VER_BASE_PAYLOAD_LEN {
        return Err(ParseError::InsufficientLength {
            expected: MON_VER_BASE_PAYLOAD_LEN,
            actual: payload_len,
        });
    }

    // ペイロード長が (40 + N×30) の形式かチェック
    let ext_part = payload_len - MON_VER_BASE_PAYLOAD_LEN;
    if ext_part % EXTENSION_SIZE != 0 {
        return Err(ParseError::InvalidPayloadLength { actual: payload_len });
    }

    // フレーム全体の長さチェック
    let frame_len = 6 + payload_len + 2;
    if data.len() < frame_len {
        return Err(ParseError::InsufficientLength {
            expected: frame_len,
            actual: data.len(),
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

    // swVersion (offset 0-29, Null終端)
    let sw_version = extract_null_terminated_string(&payload[0..30]);

    // hwVersion (offset 30-39, Null終端)
    let hw_version = extract_null_terminated_string(&payload[30..40]);

    // extensions (offset 40+, 30バイトごと)
    let num_extensions = ext_part / EXTENSION_SIZE;
    let mut extensions = Vec::with_capacity(num_extensions);
    for i in 0..num_extensions {
        let start = 40 + i * EXTENSION_SIZE;
        let end = start + EXTENSION_SIZE;
        let ext = extract_null_terminated_string(&payload[start..end]);
        extensions.push(ext);
    }

    Ok(MonVer {
        sw_version,
        hw_version,
        extensions,
    })
}

/// Null終端文字列を抽出する
fn extract_null_terminated_string(data: &[u8]) -> String {
    let end = data.iter().position(|&b| b == 0).unwrap_or(data.len());
    String::from_utf8_lossy(&data[0..end]).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// テスト用のMON-VERメッセージを生成
    fn build_mon_ver_message(sw_version: &str, hw_version: &str, extensions: &[&str]) -> Vec<u8> {
        let num_ext = extensions.len();
        let payload_len = MON_VER_BASE_PAYLOAD_LEN + num_ext * EXTENSION_SIZE;
        let mut payload = vec![0u8; payload_len];

        // swVersion (offset 0-29)
        let sw_bytes = sw_version.as_bytes();
        let sw_len = sw_bytes.len().min(29);
        payload[0..sw_len].copy_from_slice(&sw_bytes[0..sw_len]);

        // hwVersion (offset 30-39)
        let hw_bytes = hw_version.as_bytes();
        let hw_len = hw_bytes.len().min(9);
        payload[30..30 + hw_len].copy_from_slice(&hw_bytes[0..hw_len]);

        // extensions (offset 40+)
        for (i, ext) in extensions.iter().enumerate() {
            let start = 40 + i * EXTENSION_SIZE;
            let ext_bytes = ext.as_bytes();
            let ext_len = ext_bytes.len().min(29);
            payload[start..start + ext_len].copy_from_slice(&ext_bytes[0..ext_len]);
        }

        // UBXフレーム構築
        let mut frame = vec![UBX_SYNC_1, UBX_SYNC_2, MON_CLASS, MON_VER_ID];
        let len = payload_len as u16;
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
            sw_version: &'static str,
            hw_version: &'static str,
            extensions: Vec<&'static str>,
            should_succeed: bool,
        }

        let cases = vec![
            // 正常系: extension 2個
            TestCase {
                name: "2 extensions",
                sw_version: "EXT CORE 1.00 (3fda8e)",
                hw_version: "00190000",
                extensions: vec!["ROM BASE 0x118B2060", "FWVER=HPG 1.32"],
                should_succeed: true,
            },
            // 正常系: extension 3個
            TestCase {
                name: "3 extensions",
                sw_version: "EXT CORE 1.00",
                hw_version: "00190000",
                extensions: vec!["ROM BASE", "FWVER=HPG 1.32", "PROTVER=27.31"],
                should_succeed: true,
            },
            // 境界値: extension 0個
            TestCase {
                name: "0 extensions",
                sw_version: "EXT CORE 1.00",
                hw_version: "00190000",
                extensions: vec![],
                should_succeed: true,
            },
        ];

        for tc in cases {
            let data = build_mon_ver_message(tc.sw_version, tc.hw_version, &tc.extensions);
            let result = parse(&data);

            if tc.should_succeed {
                let mon_ver = result.unwrap_or_else(|e| panic!("{}: unexpected error: {}", tc.name, e));
                assert_eq!(mon_ver.sw_version, tc.sw_version, "{}: sw_version mismatch", tc.name);
                assert_eq!(mon_ver.hw_version, tc.hw_version, "{}: hw_version mismatch", tc.name);
                assert_eq!(mon_ver.extensions.len(), tc.extensions.len(), "{}: extensions count mismatch", tc.name);
                for (i, ext) in tc.extensions.iter().enumerate() {
                    assert_eq!(mon_ver.extensions[i], *ext, "{}: extension[{}] mismatch", tc.name, i);
                }
            } else {
                assert!(result.is_err(), "{}: expected error", tc.name);
            }
        }
    }

    /// テストケース: エラー系
    #[test]
    fn test_parse_error_cases() {
        let valid_msg = build_mon_ver_message("EXT CORE 1.00", "00190000", &["FWVER=HPG 1.32"]);

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
            // 異常系: ペイロード長不足（39バイト）
            TestCase {
                name: "Insufficient payload length (39 bytes)",
                data: {
                    // 手動で39バイトペイロードのフレームを構築
                    let mut frame = vec![UBX_SYNC_1, UBX_SYNC_2, MON_CLASS, MON_VER_ID];
                    let len: u16 = 39;
                    frame.extend_from_slice(&len.to_le_bytes());
                    frame.extend(vec![0u8; 39]);
                    let (ck_a, ck_b) = calculate_checksum(&frame[2..]);
                    frame.push(ck_a);
                    frame.push(ck_b);
                    frame
                },
                should_succeed: false,
            },
            // 異常系: ペイロード長が (40 + N×30) でない（65バイト）
            TestCase {
                name: "Invalid payload length (65 bytes, not 40+N*30)",
                data: {
                    let mut frame = vec![UBX_SYNC_1, UBX_SYNC_2, MON_CLASS, MON_VER_ID];
                    let len: u16 = 65; // 40 + 25 (30の倍数でない)
                    frame.extend_from_slice(&len.to_le_bytes());
                    frame.extend(vec![0u8; 65]);
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
