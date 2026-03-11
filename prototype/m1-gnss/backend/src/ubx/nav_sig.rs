//! NAV-SIG メッセージパーサー
//!
//! UBX-NAV-SIG: Signal Information
//! Class: 0x01, ID: 0x43
//! ペイロード: 8 + numSigs × 16 バイト
//!
//! 信号単位の情報を提供。L1/L2別のC/N0比較に使用。

use std::fmt;

/// UBXヘッダー
const UBX_SYNC_1: u8 = 0xB5;
const UBX_SYNC_2: u8 = 0x62;

/// NAV-SIG メッセージ識別子
const NAV_CLASS: u8 = 0x01;
const NAV_SIG_ID: u8 = 0x43;

/// NAV-SIG ヘッダー長
const NAV_SIG_HEADER_LEN: usize = 8;
/// NAV-SIG 繰り返しブロック長
const NAV_SIG_BLOCK_LEN: usize = 16;

/// パースエラー
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    /// ヘッダーが不正（0xB5 0x62 で始まらない）
    InvalidHeader,
    /// Class/IDが不一致（NAV-SIGではない）
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

/// 信号情報（1信号分）
#[derive(Debug, Clone, PartialEq)]
pub struct SignalInfo {
    /// GNSS識別子（0=GPS, 1=SBAS, 2=Galileo, 3=BeiDou, 5=QZSS, 6=GLONASS）
    pub gnss_id: u8,
    /// 衛星番号
    pub sv_id: u8,
    /// 信号識別子（L1/L2/L5等を区別）
    pub sig_id: u8,
    /// GLONASS周波数スロット+7（GLONASS以外は0）
    pub freq_id: u8,
    /// 疑似距離残差 [0.1m]
    pub pr_res: i16,
    /// C/N0 [dBHz]
    pub cno: u8,
    /// 品質指標（0-7）
    pub quality_ind: u8,
    /// 補正ソース
    pub corr_source: u8,
    /// 電離層モデル
    pub iono_model: u8,
    /// フラグ
    pub sig_flags: u16,
}

impl SignalInfo {
    /// 信号がL1帯かどうか（GPS/QZSS/Galileo/BeiDou）
    pub fn is_l1(&self) -> bool {
        match self.gnss_id {
            0 | 5 => self.sig_id == 0 || self.sig_id == 3, // GPS/QZSS: L1C/A(0), L1C(3)
            2 => self.sig_id == 0 || self.sig_id == 1,      // Galileo: E1-C(0), E1-B(1)
            3 => self.sig_id == 0 || self.sig_id == 1,      // BeiDou: B1I(0), B1C(1)
            6 => self.sig_id == 0,                          // GLONASS: L1 OF(0)
            _ => false,
        }
    }

    /// 信号がL2帯かどうか（GPS/QZSS/GLONASS）
    pub fn is_l2(&self) -> bool {
        match self.gnss_id {
            0 | 5 => self.sig_id == 4, // GPS/QZSS: L2 CL(4)
            6 => self.sig_id == 2,     // GLONASS: L2 OF(2)
            _ => false,
        }
    }

    /// ナビゲーションに使用されているか（sigFlags bit 3）
    pub fn is_used(&self) -> bool {
        (self.sig_flags & 0x0008) != 0
    }
}

/// NAV-SIG パース結果
#[derive(Debug, Clone, PartialEq)]
pub struct NavSig {
    /// GPS Time of Week (ms)
    pub itow: u32,
    /// メッセージバージョン
    pub version: u8,
    /// 信号リスト
    pub signals: Vec<SignalInfo>,
}

impl NavSig {
    /// L1帯の信号のみを取得
    pub fn l1_signals(&self) -> Vec<&SignalInfo> {
        self.signals.iter().filter(|s| s.is_l1()).collect()
    }

    /// L2帯の信号のみを取得
    pub fn l2_signals(&self) -> Vec<&SignalInfo> {
        self.signals.iter().filter(|s| s.is_l2()).collect()
    }

    /// 指定した衛星のL1/L2 C/N0を取得
    pub fn get_cno_pair(&self, gnss_id: u8, sv_id: u8) -> Option<(u8, u8)> {
        let l1_cno = self.signals.iter()
            .find(|s| s.gnss_id == gnss_id && s.sv_id == sv_id && s.is_l1())
            .map(|s| s.cno);
        let l2_cno = self.signals.iter()
            .find(|s| s.gnss_id == gnss_id && s.sv_id == sv_id && s.is_l2())
            .map(|s| s.cno);

        match (l1_cno, l2_cno) {
            (Some(l1), Some(l2)) => Some((l1, l2)),
            _ => None,
        }
    }
}

/// NAV-SIGメッセージをパースする
pub fn parse(data: &[u8]) -> Result<NavSig, ParseError> {
    // 最小長チェック: header(2) + class(1) + id(1) + len(2) + payload(8) + checksum(2) = 16
    let min_len = 2 + 1 + 1 + 2 + NAV_SIG_HEADER_LEN + 2;
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
    if class != NAV_CLASS || id != NAV_SIG_ID {
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
    let num_sigs = payload[5] as usize;

    // ペイロード長の検証
    let expected_payload_len = NAV_SIG_HEADER_LEN + num_sigs * NAV_SIG_BLOCK_LEN;
    if payload_len != expected_payload_len {
        return Err(ParseError::PayloadLengthMismatch {
            expected: expected_payload_len,
            actual: payload_len,
        });
    }

    // 信号情報のパース
    let mut signals = Vec::with_capacity(num_sigs);
    for i in 0..num_sigs {
        let offset = NAV_SIG_HEADER_LEN + i * NAV_SIG_BLOCK_LEN;
        let block = &payload[offset..offset + NAV_SIG_BLOCK_LEN];

        let signal = SignalInfo {
            gnss_id: block[0],
            sv_id: block[1],
            sig_id: block[2],
            freq_id: block[3],
            pr_res: i16::from_le_bytes([block[4], block[5]]),
            cno: block[6],
            quality_ind: block[7],
            corr_source: block[8],
            iono_model: block[9],
            sig_flags: u16::from_le_bytes([block[10], block[11]]),
            // block[12..16] は reserved
        };
        signals.push(signal);
    }

    Ok(NavSig {
        itow,
        version,
        signals,
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

    /// テスト用のNAV-SIGメッセージを生成
    fn build_nav_sig_message(signals: &[SignalInfo]) -> Vec<u8> {
        let num_sigs = signals.len();
        let payload_len = NAV_SIG_HEADER_LEN + num_sigs * NAV_SIG_BLOCK_LEN;
        let mut payload = vec![0u8; payload_len];

        // ヘッダー部分
        let itow: u32 = 123456000;
        payload[0..4].copy_from_slice(&itow.to_le_bytes());
        payload[4] = 0x00; // version
        payload[5] = num_sigs as u8;
        // payload[6..8] = reserved

        // 信号ブロック
        for (i, sig) in signals.iter().enumerate() {
            let offset = NAV_SIG_HEADER_LEN + i * NAV_SIG_BLOCK_LEN;
            payload[offset] = sig.gnss_id;
            payload[offset + 1] = sig.sv_id;
            payload[offset + 2] = sig.sig_id;
            payload[offset + 3] = sig.freq_id;
            payload[offset + 4..offset + 6].copy_from_slice(&sig.pr_res.to_le_bytes());
            payload[offset + 6] = sig.cno;
            payload[offset + 7] = sig.quality_ind;
            payload[offset + 8] = sig.corr_source;
            payload[offset + 9] = sig.iono_model;
            payload[offset + 10..offset + 12].copy_from_slice(&sig.sig_flags.to_le_bytes());
            // offset + 12..16 = reserved
        }

        // UBXフレーム構築
        let mut frame = vec![UBX_SYNC_1, UBX_SYNC_2, NAV_CLASS, NAV_SIG_ID];
        let len = payload_len as u16;
        frame.extend_from_slice(&len.to_le_bytes());
        frame.extend_from_slice(&payload);

        // チェックサム計算
        let (ck_a, ck_b) = calculate_checksum(&frame[2..]);
        frame.push(ck_a);
        frame.push(ck_b);

        frame
    }

    /// テスト用SignalInfo生成ヘルパー
    fn make_signal(gnss_id: u8, sv_id: u8, sig_id: u8, cno: u8) -> SignalInfo {
        SignalInfo {
            gnss_id,
            sv_id,
            sig_id,
            freq_id: 0,
            pr_res: 0,
            cno,
            quality_ind: 5,
            corr_source: 0,
            iono_model: 0,
            sig_flags: 0x0008, // used flag set
        }
    }

    /// テストケース: 正常系パース
    #[test]
    fn test_parse_success_cases() {
        #[derive(Debug)]
        struct TestCase {
            name: &'static str,
            signals: Vec<SignalInfo>,
            should_succeed: bool,
        }

        let cases = vec![
            // 正常系: GPS L1信号1つ
            TestCase {
                name: "Single GPS L1 signal",
                signals: vec![make_signal(0, 1, 0, 42)], // GPS, SV1, L1C/A, C/N0=42
                should_succeed: true,
            },
            // 正常系: GPS L1 + L2（2信号）
            TestCase {
                name: "GPS L1 + L2",
                signals: vec![
                    make_signal(0, 1, 0, 45), // GPS SV1 L1
                    make_signal(0, 1, 4, 38), // GPS SV1 L2
                ],
                should_succeed: true,
            },
            // 正常系: 複数GNSS
            TestCase {
                name: "Multiple GNSS (GPS + GLONASS)",
                signals: vec![
                    make_signal(0, 1, 0, 44),  // GPS SV1 L1
                    make_signal(6, 10, 0, 40), // GLONASS SV10 L1
                ],
                should_succeed: true,
            },
            // 境界値: 信号なし
            TestCase {
                name: "No signals (numSigs=0)",
                signals: vec![],
                should_succeed: true,
            },
        ];

        for tc in cases {
            let data = build_nav_sig_message(&tc.signals);
            let result = parse(&data);

            if tc.should_succeed {
                let nav_sig = result.unwrap_or_else(|e| panic!("{}: unexpected error: {}", tc.name, e));
                assert_eq!(nav_sig.signals.len(), tc.signals.len(), "{}: signal count mismatch", tc.name);

                for (i, (actual, expected)) in nav_sig.signals.iter().zip(tc.signals.iter()).enumerate() {
                    assert_eq!(actual.gnss_id, expected.gnss_id, "{}: signal[{}] gnss_id mismatch", tc.name, i);
                    assert_eq!(actual.sv_id, expected.sv_id, "{}: signal[{}] sv_id mismatch", tc.name, i);
                    assert_eq!(actual.sig_id, expected.sig_id, "{}: signal[{}] sig_id mismatch", tc.name, i);
                    assert_eq!(actual.cno, expected.cno, "{}: signal[{}] cno mismatch", tc.name, i);
                }
            } else {
                assert!(result.is_err(), "{}: expected error", tc.name);
            }
        }
    }

    /// テストケース: L1/L2判定
    #[test]
    fn test_l1_l2_detection() {
        #[derive(Debug)]
        struct TestCase {
            name: &'static str,
            gnss_id: u8,
            sig_id: u8,
            expected_l1: bool,
            expected_l2: bool,
            should_succeed: bool,
        }

        let cases = vec![
            // GPS
            TestCase { name: "GPS L1C/A", gnss_id: 0, sig_id: 0, expected_l1: true, expected_l2: false, should_succeed: true },
            TestCase { name: "GPS L1C", gnss_id: 0, sig_id: 3, expected_l1: true, expected_l2: false, should_succeed: true },
            TestCase { name: "GPS L2 CL", gnss_id: 0, sig_id: 4, expected_l1: false, expected_l2: true, should_succeed: true },
            // GLONASS
            TestCase { name: "GLONASS L1", gnss_id: 6, sig_id: 0, expected_l1: true, expected_l2: false, should_succeed: true },
            TestCase { name: "GLONASS L2", gnss_id: 6, sig_id: 2, expected_l1: false, expected_l2: true, should_succeed: true },
            // Galileo
            TestCase { name: "Galileo E1-C", gnss_id: 2, sig_id: 0, expected_l1: true, expected_l2: false, should_succeed: true },
            // その他
            TestCase { name: "Unknown signal", gnss_id: 0, sig_id: 99, expected_l1: false, expected_l2: false, should_succeed: true },
        ];

        for tc in cases {
            if tc.should_succeed {
                let signal = make_signal(tc.gnss_id, 1, tc.sig_id, 40);
                assert_eq!(signal.is_l1(), tc.expected_l1, "{}: is_l1 mismatch", tc.name);
                assert_eq!(signal.is_l2(), tc.expected_l2, "{}: is_l2 mismatch", tc.name);
            }
        }
    }

    /// テストケース: C/N0ペア取得
    #[test]
    fn test_get_cno_pair() {
        #[derive(Debug)]
        struct TestCase {
            name: &'static str,
            signals: Vec<SignalInfo>,
            query_gnss_id: u8,
            query_sv_id: u8,
            expected: Option<(u8, u8)>,
            should_succeed: bool,
        }

        let cases = vec![
            // 正常系: L1/L2両方あり
            TestCase {
                name: "Both L1 and L2 present",
                signals: vec![
                    make_signal(0, 1, 0, 45), // GPS SV1 L1 C/N0=45
                    make_signal(0, 1, 4, 38), // GPS SV1 L2 C/N0=38
                ],
                query_gnss_id: 0,
                query_sv_id: 1,
                expected: Some((45, 38)),
                should_succeed: true,
            },
            // L1のみ
            TestCase {
                name: "Only L1 present",
                signals: vec![
                    make_signal(0, 1, 0, 45),
                ],
                query_gnss_id: 0,
                query_sv_id: 1,
                expected: None,
                should_succeed: true,
            },
            // 該当衛星なし
            TestCase {
                name: "Satellite not found",
                signals: vec![
                    make_signal(0, 1, 0, 45),
                ],
                query_gnss_id: 0,
                query_sv_id: 99,
                expected: None,
                should_succeed: true,
            },
        ];

        for tc in cases {
            if tc.should_succeed {
                let data = build_nav_sig_message(&tc.signals);
                let nav_sig = parse(&data).unwrap();
                let result = nav_sig.get_cno_pair(tc.query_gnss_id, tc.query_sv_id);
                assert_eq!(result, tc.expected, "{}: get_cno_pair mismatch", tc.name);
            }
        }
    }

    /// テストケース: エラー系
    #[test]
    fn test_parse_error_cases() {
        let valid_signals = vec![make_signal(0, 1, 0, 42)];
        let valid_msg = build_nav_sig_message(&valid_signals);

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
                    d[3] = 0x07;
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
                data: vec![0xB5, 0x62, 0x01, 0x43, 0x00, 0x00],
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
