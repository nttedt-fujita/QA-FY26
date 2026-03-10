//! NAV-PVT メッセージパーサー
//!
//! UBX-NAV-PVT: Navigation Position Velocity Time Solution
//! Class: 0x01, ID: 0x07
//! Payload: 92 bytes

use std::fmt;

/// UBXヘッダー
const UBX_SYNC_1: u8 = 0xB5;
const UBX_SYNC_2: u8 = 0x62;

/// NAV-PVT メッセージ識別子
const NAV_CLASS: u8 = 0x01;
const NAV_PVT_ID: u8 = 0x07;

/// NAV-PVT ペイロード長
const NAV_PVT_PAYLOAD_LEN: usize = 92;

/// パースエラー
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    /// ヘッダーが不正（0xB5 0x62 で始まらない）
    InvalidHeader,
    /// Class/IDが不一致（NAV-PVTではない）
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

/// NAV-PVT パース結果
#[derive(Debug, Clone, PartialEq)]
pub struct NavPvt {
    /// GPS Time of Week (ms)
    pub itow: u32,
    /// FIXタイプ（0=No fix, 2=2D, 3=3D, etc.）
    pub fix_type: u8,
    /// RTK状態（0=なし, 1=Float, 2=Fixed）
    pub carr_soln: u8,
    /// 使用衛星数
    pub num_sv: u8,
    /// 緯度（度）
    pub lat: f64,
    /// 経度（度）
    pub lon: f64,
    /// 水平精度（mm）
    pub h_acc: u32,
    /// 垂直精度（mm）
    pub v_acc: u32,
}

impl NavPvt {
    /// RTK Fixed状態かどうか
    pub fn is_rtk_fixed(&self) -> bool {
        self.carr_soln == 2
    }

    /// RTK Float状態かどうか
    pub fn is_rtk_float(&self) -> bool {
        self.carr_soln == 1
    }
}

/// NAV-PVTメッセージをパースする
pub fn parse(data: &[u8]) -> Result<NavPvt, ParseError> {
    // 最小長チェック: header(2) + class(1) + id(1) + len(2) + payload(92) + checksum(2) = 100
    let min_len = 2 + 1 + 1 + 2 + NAV_PVT_PAYLOAD_LEN + 2;
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
    if class != NAV_CLASS || id != NAV_PVT_ID {
        return Err(ParseError::MessageMismatch { class, id });
    }

    // ペイロード長チェック
    let payload_len = u16::from_le_bytes([data[4], data[5]]) as usize;
    if payload_len != NAV_PVT_PAYLOAD_LEN {
        return Err(ParseError::InsufficientLength {
            expected: NAV_PVT_PAYLOAD_LEN,
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
    let fix_type = payload[20];
    let flags = payload[21];
    let carr_soln = (flags >> 6) & 0x03; // bit 6-7
    let num_sv = payload[23];

    // 経度・緯度（offset 24, 28）
    let lon_raw = i32::from_le_bytes([payload[24], payload[25], payload[26], payload[27]]);
    let lat_raw = i32::from_le_bytes([payload[28], payload[29], payload[30], payload[31]]);
    let lon = lon_raw as f64 * 1e-7;
    let lat = lat_raw as f64 * 1e-7;

    // 精度（offset 40, 44）
    let h_acc = u32::from_le_bytes([payload[40], payload[41], payload[42], payload[43]]);
    let v_acc = u32::from_le_bytes([payload[44], payload[45], payload[46], payload[47]]);

    Ok(NavPvt {
        itow,
        fix_type,
        carr_soln,
        num_sv,
        lat,
        lon,
        h_acc,
        v_acc,
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

    /// テスト用のNAV-PVTメッセージを生成
    fn build_nav_pvt_message(
        fix_type: u8,
        carr_soln: u8,
        num_sv: u8,
        lat_deg: f64,
        lon_deg: f64,
        h_acc_mm: u32,
        v_acc_mm: u32,
    ) -> Vec<u8> {
        let mut payload = vec![0u8; NAV_PVT_PAYLOAD_LEN];

        // iTOW (offset 0-3)
        let itow: u32 = 123456000;
        payload[0..4].copy_from_slice(&itow.to_le_bytes());

        // fixType (offset 20)
        payload[20] = fix_type;

        // flags (offset 21) - carrSolnはbit 6-7
        payload[21] = (carr_soln & 0x03) << 6;

        // numSV (offset 23)
        payload[23] = num_sv;

        // lon (offset 24-27)
        let lon_raw = (lon_deg * 1e7) as i32;
        payload[24..28].copy_from_slice(&lon_raw.to_le_bytes());

        // lat (offset 28-31)
        let lat_raw = (lat_deg * 1e7) as i32;
        payload[28..32].copy_from_slice(&lat_raw.to_le_bytes());

        // hAcc (offset 40-43)
        payload[40..44].copy_from_slice(&h_acc_mm.to_le_bytes());

        // vAcc (offset 44-47)
        payload[44..48].copy_from_slice(&v_acc_mm.to_le_bytes());

        // UBXフレーム構築
        let mut frame = vec![UBX_SYNC_1, UBX_SYNC_2, NAV_CLASS, NAV_PVT_ID];
        let len = NAV_PVT_PAYLOAD_LEN as u16;
        frame.extend_from_slice(&len.to_le_bytes());
        frame.extend_from_slice(&payload);

        // チェックサム計算（Class, ID, Length, Payload）
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
            fix_type: u8,
            carr_soln: u8,
            num_sv: u8,
            lat: f64,
            lon: f64,
            h_acc: u32,
            v_acc: u32,
            should_succeed: bool,
        }

        let cases = vec![
            // 正常系: 3D Fix
            TestCase {
                name: "3D Fix, RTK Fixed",
                fix_type: 3,
                carr_soln: 2,
                num_sv: 15,
                lat: 35.6812,
                lon: 139.7671,
                h_acc: 10,
                v_acc: 15,
                should_succeed: true,
            },
            // 正常系: RTK Float
            TestCase {
                name: "3D Fix, RTK Float",
                fix_type: 3,
                carr_soln: 1,
                num_sv: 12,
                lat: 35.0,
                lon: 135.0,
                h_acc: 500,
                v_acc: 800,
                should_succeed: true,
            },
            // 正常系: No RTK
            TestCase {
                name: "3D Fix, No RTK",
                fix_type: 3,
                carr_soln: 0,
                num_sv: 8,
                lat: 0.0,
                lon: 0.0,
                h_acc: 5000,
                v_acc: 8000,
                should_succeed: true,
            },
            // 正常系: No Fix
            TestCase {
                name: "No Fix",
                fix_type: 0,
                carr_soln: 0,
                num_sv: 0,
                lat: 0.0,
                lon: 0.0,
                h_acc: 0,
                v_acc: 0,
                should_succeed: true,
            },
        ];

        for tc in cases {
            let data = build_nav_pvt_message(
                tc.fix_type,
                tc.carr_soln,
                tc.num_sv,
                tc.lat,
                tc.lon,
                tc.h_acc,
                tc.v_acc,
            );
            let result = parse(&data);

            if tc.should_succeed {
                let pvt = result.unwrap_or_else(|e| panic!("{}: unexpected error: {}", tc.name, e));
                assert_eq!(pvt.fix_type, tc.fix_type, "{}: fix_type mismatch", tc.name);
                assert_eq!(pvt.carr_soln, tc.carr_soln, "{}: carr_soln mismatch", tc.name);
                assert_eq!(pvt.num_sv, tc.num_sv, "{}: num_sv mismatch", tc.name);
                assert!((pvt.lat - tc.lat).abs() < 1e-6, "{}: lat mismatch", tc.name);
                assert!((pvt.lon - tc.lon).abs() < 1e-6, "{}: lon mismatch", tc.name);
                assert_eq!(pvt.h_acc, tc.h_acc, "{}: h_acc mismatch", tc.name);
                assert_eq!(pvt.v_acc, tc.v_acc, "{}: v_acc mismatch", tc.name);
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
            // 正常系: RTK Fixed
            TestCase {
                name: "RTK Fixed",
                carr_soln: 2,
                is_fixed: true,
                is_float: false,
                should_succeed: true,
            },
            // 正常系: RTK Float
            TestCase {
                name: "RTK Float",
                carr_soln: 1,
                is_fixed: false,
                is_float: true,
                should_succeed: true,
            },
            // 正常系: No RTK
            TestCase {
                name: "No RTK",
                carr_soln: 0,
                is_fixed: false,
                is_float: false,
                should_succeed: true,
            },
        ];

        for tc in cases {
            let data = build_nav_pvt_message(3, tc.carr_soln, 10, 35.0, 139.0, 100, 150);
            let result = parse(&data);

            if tc.should_succeed {
                let pvt = result.unwrap_or_else(|e| panic!("{}: unexpected error: {}", tc.name, e));
                assert_eq!(
                    pvt.is_rtk_fixed(),
                    tc.is_fixed,
                    "{}: is_rtk_fixed mismatch",
                    tc.name
                );
                assert_eq!(
                    pvt.is_rtk_float(),
                    tc.is_float,
                    "{}: is_rtk_float mismatch",
                    tc.name
                );
            }
        }
    }

    /// テストケース: エラー系
    #[test]
    fn test_parse_error_cases() {
        struct TestCase {
            name: &'static str,
            data: Vec<u8>,
            expected_error: ParseError,
            should_succeed: bool,
        }

        // 正常なメッセージを基に異常データを作成
        let valid_msg = build_nav_pvt_message(3, 2, 15, 35.6812, 139.7671, 10, 15);

        let cases = vec![
            // 異常系: ヘッダー不正
            TestCase {
                name: "Invalid header (wrong sync1)",
                data: {
                    let mut d = valid_msg.clone();
                    d[0] = 0x00;
                    d
                },
                expected_error: ParseError::InvalidHeader,
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
                expected_error: ParseError::InvalidHeader,
                should_succeed: false,
            },
            // 異常系: Class/ID不一致
            TestCase {
                name: "Message mismatch (wrong class)",
                data: {
                    let mut d = valid_msg.clone();
                    d[2] = 0x05; // NAV(0x01)ではない
                    d
                },
                expected_error: ParseError::MessageMismatch { class: 0x05, id: 0x07 },
                should_succeed: false,
            },
            // 異常系: Class/ID不一致
            TestCase {
                name: "Message mismatch (wrong id)",
                data: {
                    let mut d = valid_msg.clone();
                    d[3] = 0x03; // PVT(0x07)ではない
                    d
                },
                expected_error: ParseError::MessageMismatch { class: 0x01, id: 0x03 },
                should_succeed: false,
            },
            // 異常系: チェックサム不正
            TestCase {
                name: "Checksum error",
                data: {
                    let mut d = valid_msg.clone();
                    let len = d.len();
                    d[len - 1] = 0xFF; // CK_Bを壊す
                    d
                },
                expected_error: ParseError::ChecksumError {
                    expected: (0xFF, valid_msg[valid_msg.len() - 1]),
                    actual: (valid_msg[valid_msg.len() - 2], valid_msg[valid_msg.len() - 1]),
                },
                should_succeed: false,
            },
            // 異常系: ペイロード長不足
            TestCase {
                name: "Insufficient length (too short)",
                data: vec![0xB5, 0x62, 0x01, 0x07, 0x00, 0x00],
                expected_error: ParseError::InsufficientLength {
                    expected: 100,
                    actual: 6,
                },
                should_succeed: false,
            },
        ];

        for tc in cases {
            let result = parse(&tc.data);

            if tc.should_succeed {
                assert!(result.is_ok(), "{}: expected success", tc.name);
            } else {
                let err = result.unwrap_err();
                // エラーの種類だけ確認（詳細値は異なる場合がある）
                match (&err, &tc.expected_error) {
                    (ParseError::InvalidHeader, ParseError::InvalidHeader) => {}
                    (
                        ParseError::MessageMismatch { class: c1, id: i1 },
                        ParseError::MessageMismatch { class: c2, id: i2 },
                    ) => {
                        assert_eq!(c1, c2, "{}: class mismatch", tc.name);
                        assert_eq!(i1, i2, "{}: id mismatch", tc.name);
                    }
                    (ParseError::ChecksumError { .. }, ParseError::ChecksumError { .. }) => {}
                    (
                        ParseError::InsufficientLength { actual: a1, .. },
                        ParseError::InsufficientLength { actual: a2, .. },
                    ) => {
                        assert_eq!(a1, a2, "{}: actual length mismatch", tc.name);
                    }
                    _ => panic!("{}: unexpected error type: {:?}", tc.name, err),
                }
            }
        }
    }
}
