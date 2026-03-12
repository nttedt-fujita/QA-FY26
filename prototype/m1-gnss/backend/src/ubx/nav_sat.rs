//! NAV-SAT メッセージパーサー
//!
//! UBX-NAV-SAT: Satellite Information
//! Class: 0x01, ID: 0x35
//! ペイロード: 8 + numSvs × 12 バイト
//!
//! 衛星単位の情報を提供。スカイプロット表示に使用。
//! 参照: docs/missions/m1-sensor-evaluation/gnss/ubx-nav-messages.md

use std::fmt;

/// UBXヘッダー
const UBX_SYNC_1: u8 = 0xB5;
const UBX_SYNC_2: u8 = 0x62;

/// NAV-SAT メッセージ識別子
const NAV_CLASS: u8 = 0x01;
const NAV_SAT_ID: u8 = 0x35;

/// NAV-SAT ヘッダー長
const NAV_SAT_HEADER_LEN: usize = 8;
/// NAV-SAT 繰り返しブロック長
const NAV_SAT_BLOCK_LEN: usize = 12;

/// パースエラー
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    /// ヘッダーが不正（0xB5 0x62 で始まらない）
    InvalidHeader,
    /// Class/IDが不一致（NAV-SATではない）
    MessageMismatch { class: u8, id: u8 },
    /// チェックサムが不正
    ChecksumError { expected: (u8, u8), actual: (u8, u8) },
    /// ペイロード長不足
    InsufficientLength { expected: usize, actual: usize },
    /// ペイロード長が仕様と不一致
    PayloadLengthMismatch { expected: usize, actual: usize },
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
            ParseError::PayloadLengthMismatch { expected, actual } => {
                write!(f, "Payload length mismatch: expected={}, actual={}", expected, actual)
            }
        }
    }
}

impl std::error::Error for ParseError {}

/// 衛星情報（1衛星分）
#[derive(Debug, Clone, PartialEq)]
pub struct SatelliteInfo {
    /// GNSS識別子（0=GPS, 1=SBAS, 2=Galileo, 3=BeiDou, 5=QZSS, 6=GLONASS）
    pub gnss_id: u8,
    /// 衛星番号
    pub sv_id: u8,
    /// C/N0 [dBHz]（衛星代表値）
    pub cno: u8,
    /// 仰角 [deg]（-90〜+90、天頂が+90）
    pub elev: i8,
    /// 方位角 [deg]（0〜360、北が0）
    pub azim: i16,
    /// 疑似距離残差 [0.1m]
    pub pr_res: i16,
    /// フラグ
    pub flags: u32,
}

impl SatelliteInfo {
    /// 品質指標（flags bits 0-2）
    pub fn quality_ind(&self) -> u8 {
        (self.flags & 0x07) as u8
    }

    /// ナビゲーションに使用中か（flags bit 3）
    pub fn is_used(&self) -> bool {
        (self.flags & 0x08) != 0
    }

    /// 健全性（flags bits 4-5）: 0=unknown, 1=healthy, 2=unhealthy
    pub fn health(&self) -> u8 {
        ((self.flags >> 4) & 0x03) as u8
    }

    /// GNSS名を取得
    pub fn gnss_name(&self) -> &'static str {
        match self.gnss_id {
            0 => "GPS",
            1 => "SBAS",
            2 => "Galileo",
            3 => "BeiDou",
            5 => "QZSS",
            6 => "GLONASS",
            _ => "Unknown",
        }
    }
}

/// NAV-SAT パース結果
#[derive(Debug, Clone, PartialEq)]
pub struct NavSat {
    /// GPS Time of Week (ms)
    pub itow: u32,
    /// メッセージバージョン
    pub version: u8,
    /// 衛星リスト
    pub satellites: Vec<SatelliteInfo>,
}

impl NavSat {
    /// ナビゲーションに使用中の衛星のみを取得
    pub fn used_satellites(&self) -> Vec<&SatelliteInfo> {
        self.satellites.iter().filter(|s| s.is_used()).collect()
    }

    /// 指定GNSSの衛星のみを取得
    pub fn satellites_by_gnss(&self, gnss_id: u8) -> Vec<&SatelliteInfo> {
        self.satellites.iter().filter(|s| s.gnss_id == gnss_id).collect()
    }

    /// スカイプロット用データ（仰角、方位角、C/N0）を取得
    pub fn sky_plot_data(&self) -> Vec<(i8, i16, u8, &'static str, u8)> {
        self.satellites
            .iter()
            .map(|s| (s.elev, s.azim, s.cno, s.gnss_name(), s.sv_id))
            .collect()
    }
}

/// NAV-SATメッセージをパースする
pub fn parse(data: &[u8]) -> Result<NavSat, ParseError> {
    // 最小長チェック: header(2) + class(1) + id(1) + len(2) + payload(8) + checksum(2) = 16
    let min_len = 2 + 1 + 1 + 2 + NAV_SAT_HEADER_LEN + 2;
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
    if class != NAV_CLASS || id != NAV_SAT_ID {
        return Err(ParseError::MessageMismatch { class, id });
    }

    // ペイロード長取得
    let payload_len = u16::from_le_bytes([data[4], data[5]]) as usize;

    // 全体長チェック
    let total_len = 6 + payload_len + 2; // header(6) + payload + checksum(2)
    if data.len() < total_len {
        return Err(ParseError::InsufficientLength {
            expected: total_len,
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

    // ヘッダー部分のパース
    let itow = u32::from_le_bytes([payload[0], payload[1], payload[2], payload[3]]);
    let version = payload[4];
    let num_svs = payload[5] as usize;

    // ペイロード長の検証
    let expected_payload_len = NAV_SAT_HEADER_LEN + num_svs * NAV_SAT_BLOCK_LEN;
    if payload_len != expected_payload_len {
        return Err(ParseError::PayloadLengthMismatch {
            expected: expected_payload_len,
            actual: payload_len,
        });
    }

    // 衛星情報のパース
    let mut satellites = Vec::with_capacity(num_svs);
    for i in 0..num_svs {
        let offset = NAV_SAT_HEADER_LEN + i * NAV_SAT_BLOCK_LEN;
        let block = &payload[offset..offset + NAV_SAT_BLOCK_LEN];

        let satellite = SatelliteInfo {
            gnss_id: block[0],
            sv_id: block[1],
            cno: block[2],
            elev: block[3] as i8,
            azim: i16::from_le_bytes([block[4], block[5]]),
            pr_res: i16::from_le_bytes([block[6], block[7]]),
            flags: u32::from_le_bytes([block[8], block[9], block[10], block[11]]),
        };
        satellites.push(satellite);
    }

    Ok(NavSat {
        itow,
        version,
        satellites,
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

    /// テスト用のNAV-SATメッセージを生成
    fn build_nav_sat_message(satellites: &[SatelliteInfo]) -> Vec<u8> {
        let num_svs = satellites.len();
        let payload_len = NAV_SAT_HEADER_LEN + num_svs * NAV_SAT_BLOCK_LEN;
        let mut payload = vec![0u8; payload_len];

        // ヘッダー部分
        let itow: u32 = 123456000;
        payload[0..4].copy_from_slice(&itow.to_le_bytes());
        payload[4] = 0x01; // version
        payload[5] = num_svs as u8;
        // payload[6..8] = reserved

        // 衛星ブロック
        for (i, sat) in satellites.iter().enumerate() {
            let offset = NAV_SAT_HEADER_LEN + i * NAV_SAT_BLOCK_LEN;
            payload[offset] = sat.gnss_id;
            payload[offset + 1] = sat.sv_id;
            payload[offset + 2] = sat.cno;
            payload[offset + 3] = sat.elev as u8;
            payload[offset + 4..offset + 6].copy_from_slice(&sat.azim.to_le_bytes());
            payload[offset + 6..offset + 8].copy_from_slice(&sat.pr_res.to_le_bytes());
            payload[offset + 8..offset + 12].copy_from_slice(&sat.flags.to_le_bytes());
        }

        // UBXフレーム構築
        let mut frame = vec![UBX_SYNC_1, UBX_SYNC_2, NAV_CLASS, NAV_SAT_ID];
        let len = payload_len as u16;
        frame.extend_from_slice(&len.to_le_bytes());
        frame.extend_from_slice(&payload);

        // チェックサム計算
        let (ck_a, ck_b) = calculate_checksum(&frame[2..]);
        frame.push(ck_a);
        frame.push(ck_b);

        frame
    }

    /// テスト用SatelliteInfo生成ヘルパー
    fn make_satellite(gnss_id: u8, sv_id: u8, cno: u8, elev: i8, azim: i16) -> SatelliteInfo {
        SatelliteInfo {
            gnss_id,
            sv_id,
            cno,
            elev,
            azim,
            pr_res: 0,
            flags: 0x08, // used flag set
        }
    }

    /// テスト用SatelliteInfo生成ヘルパー（flags指定可能）
    fn make_satellite_with_flags(
        gnss_id: u8,
        sv_id: u8,
        cno: u8,
        elev: i8,
        azim: i16,
        flags: u32,
    ) -> SatelliteInfo {
        SatelliteInfo {
            gnss_id,
            sv_id,
            cno,
            elev,
            azim,
            pr_res: 0,
            flags,
        }
    }

    /// テストケース: 正常系パース
    #[test]
    fn test_parse_success_cases() {
        #[derive(Debug)]
        struct TestCase {
            name: &'static str,
            satellites: Vec<SatelliteInfo>,
            should_succeed: bool,
        }

        let cases = vec![
            // 正常系: GPS衛星1つ
            TestCase {
                name: "Single GPS satellite",
                satellites: vec![make_satellite(0, 1, 42, 45, 180)], // GPS SV1, C/N0=42, elev=45°, azim=180°
                should_succeed: true,
            },
            // 正常系: 複数衛星（GPS + GLONASS）
            TestCase {
                name: "Multiple satellites (GPS + GLONASS)",
                satellites: vec![
                    make_satellite(0, 1, 45, 60, 90),   // GPS SV1
                    make_satellite(0, 5, 38, 30, 270),  // GPS SV5
                    make_satellite(6, 10, 40, 45, 45),  // GLONASS SV10
                ],
                should_succeed: true,
            },
            // 境界値: 衛星なし
            TestCase {
                name: "No satellites (numSvs=0)",
                satellites: vec![],
                should_succeed: true,
            },
            // 境界値: 仰角最大値（+90°）
            TestCase {
                name: "Max elevation (+90)",
                satellites: vec![make_satellite(0, 1, 45, 90, 0)],
                should_succeed: true,
            },
            // 境界値: 仰角最小値（-90°）
            TestCase {
                name: "Min elevation (-90)",
                satellites: vec![make_satellite(0, 1, 45, -90, 0)],
                should_succeed: true,
            },
            // 境界値: 方位角最大値（359°）
            TestCase {
                name: "Max azimuth (359)",
                satellites: vec![make_satellite(0, 1, 45, 45, 359)],
                should_succeed: true,
            },
        ];

        for tc in cases {
            let data = build_nav_sat_message(&tc.satellites);
            let result = parse(&data);

            if tc.should_succeed {
                let nav_sat = result.unwrap_or_else(|e| panic!("{}: unexpected error: {}", tc.name, e));
                assert_eq!(nav_sat.satellites.len(), tc.satellites.len(), "{}: satellite count mismatch", tc.name);

                for (i, (actual, expected)) in nav_sat.satellites.iter().zip(tc.satellites.iter()).enumerate() {
                    assert_eq!(actual.gnss_id, expected.gnss_id, "{}: sat[{}] gnss_id mismatch", tc.name, i);
                    assert_eq!(actual.sv_id, expected.sv_id, "{}: sat[{}] sv_id mismatch", tc.name, i);
                    assert_eq!(actual.cno, expected.cno, "{}: sat[{}] cno mismatch", tc.name, i);
                    assert_eq!(actual.elev, expected.elev, "{}: sat[{}] elev mismatch", tc.name, i);
                    assert_eq!(actual.azim, expected.azim, "{}: sat[{}] azim mismatch", tc.name, i);
                }
            } else {
                assert!(result.is_err(), "{}: expected error", tc.name);
            }
        }
    }

    /// テストケース: flags解釈
    #[test]
    fn test_flags_interpretation() {
        #[derive(Debug)]
        struct TestCase {
            name: &'static str,
            flags: u32,
            expected_quality_ind: u8,
            expected_is_used: bool,
            expected_health: u8,
            should_succeed: bool,
        }

        let cases = vec![
            // 正常系: 全フラグOFF
            TestCase {
                name: "All flags off",
                flags: 0x00,
                expected_quality_ind: 0,
                expected_is_used: false,
                expected_health: 0,
                should_succeed: true,
            },
            // 正常系: usedフラグのみON
            TestCase {
                name: "Used flag on",
                flags: 0x08,
                expected_quality_ind: 0,
                expected_is_used: true,
                expected_health: 0,
                should_succeed: true,
            },
            // 正常系: qualityInd=7（最大）
            TestCase {
                name: "QualityInd=7 (max)",
                flags: 0x07,
                expected_quality_ind: 7,
                expected_is_used: false,
                expected_health: 0,
                should_succeed: true,
            },
            // 正常系: health=1（healthy）
            TestCase {
                name: "Health=1 (healthy)",
                flags: 0x10,
                expected_quality_ind: 0,
                expected_is_used: false,
                expected_health: 1,
                should_succeed: true,
            },
            // 正常系: health=2（unhealthy）
            TestCase {
                name: "Health=2 (unhealthy)",
                flags: 0x20,
                expected_quality_ind: 0,
                expected_is_used: false,
                expected_health: 2,
                should_succeed: true,
            },
            // 正常系: 複合フラグ（qualityInd=5, used, health=1）
            TestCase {
                name: "Combined flags (quality=5, used, healthy)",
                flags: 0x1D, // 0b00011101 = qualityInd=5 | used | health=1
                expected_quality_ind: 5,
                expected_is_used: true,
                expected_health: 1,
                should_succeed: true,
            },
        ];

        for tc in cases {
            if tc.should_succeed {
                let sat = make_satellite_with_flags(0, 1, 40, 45, 180, tc.flags);
                assert_eq!(sat.quality_ind(), tc.expected_quality_ind, "{}: quality_ind mismatch", tc.name);
                assert_eq!(sat.is_used(), tc.expected_is_used, "{}: is_used mismatch", tc.name);
                assert_eq!(sat.health(), tc.expected_health, "{}: health mismatch", tc.name);
            }
        }
    }

    /// テストケース: GNSS名
    #[test]
    fn test_gnss_name() {
        #[derive(Debug)]
        struct TestCase {
            name: &'static str,
            gnss_id: u8,
            expected_name: &'static str,
            should_succeed: bool,
        }

        let cases = vec![
            TestCase { name: "GPS", gnss_id: 0, expected_name: "GPS", should_succeed: true },
            TestCase { name: "SBAS", gnss_id: 1, expected_name: "SBAS", should_succeed: true },
            TestCase { name: "Galileo", gnss_id: 2, expected_name: "Galileo", should_succeed: true },
            TestCase { name: "BeiDou", gnss_id: 3, expected_name: "BeiDou", should_succeed: true },
            TestCase { name: "QZSS", gnss_id: 5, expected_name: "QZSS", should_succeed: true },
            TestCase { name: "GLONASS", gnss_id: 6, expected_name: "GLONASS", should_succeed: true },
            TestCase { name: "Unknown", gnss_id: 99, expected_name: "Unknown", should_succeed: true },
        ];

        for tc in cases {
            if tc.should_succeed {
                let sat = make_satellite(tc.gnss_id, 1, 40, 45, 180);
                assert_eq!(sat.gnss_name(), tc.expected_name, "{}: gnss_name mismatch", tc.name);
            }
        }
    }

    /// テストケース: used_satellites フィルタ
    #[test]
    fn test_used_satellites_filter() {
        #[derive(Debug)]
        struct TestCase {
            name: &'static str,
            satellites: Vec<SatelliteInfo>,
            expected_used_count: usize,
            should_succeed: bool,
        }

        let cases = vec![
            // 正常系: 2/3がused
            TestCase {
                name: "2 of 3 used",
                satellites: vec![
                    make_satellite_with_flags(0, 1, 45, 60, 90, 0x08),   // used
                    make_satellite_with_flags(0, 5, 38, 30, 270, 0x00), // not used
                    make_satellite_with_flags(6, 10, 40, 45, 45, 0x08), // used
                ],
                expected_used_count: 2,
                should_succeed: true,
            },
            // 境界値: 全てused
            TestCase {
                name: "All used",
                satellites: vec![
                    make_satellite_with_flags(0, 1, 45, 60, 90, 0x08),
                    make_satellite_with_flags(0, 5, 38, 30, 270, 0x08),
                ],
                expected_used_count: 2,
                should_succeed: true,
            },
            // 境界値: none used
            TestCase {
                name: "None used",
                satellites: vec![
                    make_satellite_with_flags(0, 1, 45, 60, 90, 0x00),
                    make_satellite_with_flags(0, 5, 38, 30, 270, 0x00),
                ],
                expected_used_count: 0,
                should_succeed: true,
            },
        ];

        for tc in cases {
            if tc.should_succeed {
                let data = build_nav_sat_message(&tc.satellites);
                let nav_sat = parse(&data).unwrap();
                let used = nav_sat.used_satellites();
                assert_eq!(used.len(), tc.expected_used_count, "{}: used_satellites count mismatch", tc.name);
            }
        }
    }

    /// テストケース: エラー系
    #[test]
    fn test_parse_error_cases() {
        let valid_satellites = vec![make_satellite(0, 1, 42, 45, 180)];
        let valid_msg = build_nav_sat_message(&valid_satellites);

        #[derive(Debug)]
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
                    d[3] = 0x43; // NAV-SIG ID
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
            // 異常系: データ長不足
            TestCase {
                name: "Insufficient length",
                data: vec![0xB5, 0x62, 0x01, 0x35, 0x00, 0x00],
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

    /// テストケース: sky_plot_data
    #[test]
    fn test_sky_plot_data() {
        let satellites = vec![
            make_satellite(0, 1, 45, 60, 90),   // GPS SV1, elev=60°, azim=90°
            make_satellite(6, 10, 40, 30, 180), // GLONASS SV10, elev=30°, azim=180°
        ];
        let data = build_nav_sat_message(&satellites);
        let nav_sat = parse(&data).unwrap();

        let plot_data = nav_sat.sky_plot_data();
        assert_eq!(plot_data.len(), 2);
        assert_eq!(plot_data[0], (60, 90, 45, "GPS", 1));
        assert_eq!(plot_data[1], (30, 180, 40, "GLONASS", 10));
    }
}
