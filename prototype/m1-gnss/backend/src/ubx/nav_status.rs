//! NAV-STATUS メッセージパーサー
//!
//! UBX-NAV-STATUS: Receiver Navigation Status
//! Class: 0x01, ID: 0x03
//! Payload: 16 bytes

use std::fmt;

/// UBXヘッダー
const UBX_SYNC_1: u8 = 0xB5;
const UBX_SYNC_2: u8 = 0x62;

/// NAV-STATUS メッセージ識別子
const NAV_CLASS: u8 = 0x01;
const NAV_STATUS_ID: u8 = 0x03;

/// NAV-STATUS ペイロード長
const NAV_STATUS_PAYLOAD_LEN: usize = 16;

/// パースエラー
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    /// ヘッダーが不正（0xB5 0x62 で始まらない）
    InvalidHeader,
    /// Class/IDが不一致（NAV-STATUSではない）
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

/// NAV-STATUS パース結果
#[derive(Debug, Clone, PartialEq)]
pub struct NavStatus {
    /// GPS Time of Week (ms)
    pub itow: u32,
    /// FIXタイプ（0=No fix, 2=2D, 3=3D, etc.）
    pub gps_fix: u8,
    /// 位置・速度が有効か（flags bit 0）
    pub gps_fix_ok: bool,
    /// 差分補正適用済みか（flags bit 1）
    pub diff_soln: bool,
    /// RTK状態（0=なし, 1=Float, 2=Fixed）（flags2 bit 6-7）
    pub carr_soln: u8,
    /// スプーフィング検出状態（0=不明, 1=なし, 2=検出, 3=複数検出）（flags2 bit 4-3）
    pub spoof_det_state: u8,
    /// パワーセーブモード状態（flags2 bit 1-0）
    pub psm_state: u8,
    /// Time to First Fix (ms)
    pub ttff: u32,
    /// 起動/リセットからの経過時間 (ms)
    pub msss: u32,
}

impl NavStatus {
    /// RTK Fixed状態かどうか
    pub fn is_rtk_fixed(&self) -> bool {
        self.carr_soln == 2
    }

    /// RTK Float状態かどうか
    pub fn is_rtk_float(&self) -> bool {
        self.carr_soln == 1
    }

    /// FIXが有効かどうか
    pub fn is_fix_valid(&self) -> bool {
        self.gps_fix_ok && self.gps_fix >= 2
    }
}

/// NAV-STATUSメッセージをパースする
pub fn parse(data: &[u8]) -> Result<NavStatus, ParseError> {
    // 最小長チェック: header(2) + class(1) + id(1) + len(2) + payload(16) + checksum(2) = 24
    let min_len = 2 + 1 + 1 + 2 + NAV_STATUS_PAYLOAD_LEN + 2;
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
    if class != NAV_CLASS || id != NAV_STATUS_ID {
        return Err(ParseError::MessageMismatch { class, id });
    }

    // ペイロード長チェック
    let payload_len = u16::from_le_bytes([data[4], data[5]]) as usize;
    if payload_len != NAV_STATUS_PAYLOAD_LEN {
        return Err(ParseError::InsufficientLength {
            expected: NAV_STATUS_PAYLOAD_LEN,
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
    let gps_fix = payload[4];
    let flags = payload[5];
    let gps_fix_ok = (flags & 0x01) != 0;
    let diff_soln = (flags & 0x02) != 0;
    let flags2 = payload[7];
    let psm_state = flags2 & 0x03;
    let spoof_det_state = (flags2 >> 3) & 0x03;
    let carr_soln = (flags2 >> 6) & 0x03;
    let ttff = u32::from_le_bytes([payload[8], payload[9], payload[10], payload[11]]);
    let msss = u32::from_le_bytes([payload[12], payload[13], payload[14], payload[15]]);

    Ok(NavStatus {
        itow,
        gps_fix,
        gps_fix_ok,
        diff_soln,
        carr_soln,
        spoof_det_state,
        psm_state,
        ttff,
        msss,
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

    /// テスト用のNAV-STATUSメッセージを生成
    fn build_nav_status_message(
        gps_fix: u8,
        gps_fix_ok: bool,
        diff_soln: bool,
        carr_soln: u8,
        spoof_det_state: u8,
        psm_state: u8,
        ttff: u32,
        msss: u32,
    ) -> Vec<u8> {
        let mut payload = vec![0u8; NAV_STATUS_PAYLOAD_LEN];

        // iTOW (offset 0-3)
        let itow: u32 = 123456000;
        payload[0..4].copy_from_slice(&itow.to_le_bytes());

        // gpsFix (offset 4)
        payload[4] = gps_fix;

        // flags (offset 5)
        let mut flags: u8 = 0;
        if gps_fix_ok {
            flags |= 0x01;
        }
        if diff_soln {
            flags |= 0x02;
        }
        payload[5] = flags;

        // fixStat (offset 6) - 未使用
        payload[6] = 0;

        // flags2 (offset 7)
        let flags2 = (psm_state & 0x03) | ((spoof_det_state & 0x03) << 3) | ((carr_soln & 0x03) << 6);
        payload[7] = flags2;

        // ttff (offset 8-11)
        payload[8..12].copy_from_slice(&ttff.to_le_bytes());

        // msss (offset 12-15)
        payload[12..16].copy_from_slice(&msss.to_le_bytes());

        // UBXフレーム構築
        let mut frame = vec![UBX_SYNC_1, UBX_SYNC_2, NAV_CLASS, NAV_STATUS_ID];
        let len = NAV_STATUS_PAYLOAD_LEN as u16;
        frame.extend_from_slice(&len.to_le_bytes());
        frame.extend_from_slice(&payload);

        // チェックサム計算
        let (ck_a, ck_b) = calculate_checksum(&frame[2..]);
        frame.push(ck_a);
        frame.push(ck_b);

        frame
    }

    /// テストケース: パース成功
    #[test]
    fn test_parse_success_cases() {
        struct TestCase {
            name: &'static str,
            gps_fix: u8,
            gps_fix_ok: bool,
            diff_soln: bool,
            carr_soln: u8,
            spoof_det_state: u8,
            ttff: u32,
            should_succeed: bool,
        }

        let cases = vec![
            // 正常系: 3D Fix + RTK Fixed
            TestCase {
                name: "3D Fix, RTK Fixed",
                gps_fix: 3,
                gps_fix_ok: true,
                diff_soln: true,
                carr_soln: 2,
                spoof_det_state: 1,
                ttff: 5000,
                should_succeed: true,
            },
            // 正常系: 3D Fix + RTK Float
            TestCase {
                name: "3D Fix, RTK Float",
                gps_fix: 3,
                gps_fix_ok: true,
                diff_soln: true,
                carr_soln: 1,
                spoof_det_state: 1,
                ttff: 8000,
                should_succeed: true,
            },
            // 正常系: No Fix
            TestCase {
                name: "No Fix",
                gps_fix: 0,
                gps_fix_ok: false,
                diff_soln: false,
                carr_soln: 0,
                spoof_det_state: 0,
                ttff: 0,
                should_succeed: true,
            },
            // 正常系: 2D Fix
            TestCase {
                name: "2D Fix",
                gps_fix: 2,
                gps_fix_ok: true,
                diff_soln: false,
                carr_soln: 0,
                spoof_det_state: 1,
                ttff: 3000,
                should_succeed: true,
            },
            // 正常系: スプーフィング検出
            TestCase {
                name: "Spoofing detected",
                gps_fix: 3,
                gps_fix_ok: true,
                diff_soln: false,
                carr_soln: 0,
                spoof_det_state: 2,
                ttff: 5000,
                should_succeed: true,
            },
        ];

        for tc in cases {
            let data = build_nav_status_message(
                tc.gps_fix,
                tc.gps_fix_ok,
                tc.diff_soln,
                tc.carr_soln,
                tc.spoof_det_state,
                0, // psm_state
                tc.ttff,
                1000, // msss
            );
            let result = parse(&data);

            if tc.should_succeed {
                let status = result.unwrap_or_else(|e| panic!("{}: unexpected error: {}", tc.name, e));
                assert_eq!(status.gps_fix, tc.gps_fix, "{}: gps_fix mismatch", tc.name);
                assert_eq!(status.gps_fix_ok, tc.gps_fix_ok, "{}: gps_fix_ok mismatch", tc.name);
                assert_eq!(status.diff_soln, tc.diff_soln, "{}: diff_soln mismatch", tc.name);
                assert_eq!(status.carr_soln, tc.carr_soln, "{}: carr_soln mismatch", tc.name);
                assert_eq!(status.spoof_det_state, tc.spoof_det_state, "{}: spoof_det_state mismatch", tc.name);
                assert_eq!(status.ttff, tc.ttff, "{}: ttff mismatch", tc.name);
            } else {
                assert!(result.is_err(), "{}: expected error", tc.name);
            }
        }
    }

    /// テストケース: RTK状態判定
    #[test]
    fn test_rtk_status() {
        struct TestCase {
            name: &'static str,
            carr_soln: u8,
            is_fixed: bool,
            is_float: bool,
            should_succeed: bool,
        }

        let cases = vec![
            TestCase {
                name: "RTK Fixed",
                carr_soln: 2,
                is_fixed: true,
                is_float: false,
                should_succeed: true,
            },
            TestCase {
                name: "RTK Float",
                carr_soln: 1,
                is_fixed: false,
                is_float: true,
                should_succeed: true,
            },
            TestCase {
                name: "No RTK",
                carr_soln: 0,
                is_fixed: false,
                is_float: false,
                should_succeed: true,
            },
        ];

        for tc in cases {
            let data = build_nav_status_message(3, true, false, tc.carr_soln, 1, 0, 5000, 1000);
            let result = parse(&data);

            if tc.should_succeed {
                let status = result.unwrap_or_else(|e| panic!("{}: unexpected error: {}", tc.name, e));
                assert_eq!(status.is_rtk_fixed(), tc.is_fixed, "{}: is_rtk_fixed mismatch", tc.name);
                assert_eq!(status.is_rtk_float(), tc.is_float, "{}: is_rtk_float mismatch", tc.name);
            }
        }
    }

    /// テストケース: FIX有効性判定
    #[test]
    fn test_fix_validity() {
        struct TestCase {
            name: &'static str,
            gps_fix: u8,
            gps_fix_ok: bool,
            expected_valid: bool,
            should_succeed: bool,
        }

        let cases = vec![
            TestCase {
                name: "3D Fix, OK",
                gps_fix: 3,
                gps_fix_ok: true,
                expected_valid: true,
                should_succeed: true,
            },
            TestCase {
                name: "2D Fix, OK",
                gps_fix: 2,
                gps_fix_ok: true,
                expected_valid: true,
                should_succeed: true,
            },
            TestCase {
                name: "3D Fix, NOT OK",
                gps_fix: 3,
                gps_fix_ok: false,
                expected_valid: false,
                should_succeed: true,
            },
            TestCase {
                name: "No Fix",
                gps_fix: 0,
                gps_fix_ok: false,
                expected_valid: false,
                should_succeed: true,
            },
        ];

        for tc in cases {
            let data = build_nav_status_message(tc.gps_fix, tc.gps_fix_ok, false, 0, 1, 0, 5000, 1000);
            let result = parse(&data);

            if tc.should_succeed {
                let status = result.unwrap_or_else(|e| panic!("{}: unexpected error: {}", tc.name, e));
                assert_eq!(status.is_fix_valid(), tc.expected_valid, "{}: is_fix_valid mismatch", tc.name);
            }
        }
    }

    /// テストケース: エラー系
    #[test]
    fn test_parse_error_cases() {
        let valid_msg = build_nav_status_message(3, true, true, 2, 1, 0, 5000, 1000);

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
            // 異常系: ヘッダー不正
            TestCase {
                name: "Invalid header (wrong sync2)",
                data: {
                    let mut d = valid_msg.clone();
                    d[1] = 0x00;
                    d
                },
                should_succeed: false,
            },
            // 異常系: Class/ID不一致
            TestCase {
                name: "Message mismatch (wrong class)",
                data: {
                    let mut d = valid_msg.clone();
                    d[2] = 0x05;
                    d
                },
                should_succeed: false,
            },
            // 異常系: Class/ID不一致
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
                data: vec![0xB5, 0x62, 0x01, 0x03, 0x00, 0x00],
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
