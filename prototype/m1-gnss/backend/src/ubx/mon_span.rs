//! MON-SPAN メッセージパーサー
//!
//! UBX-MON-SPAN: Signal characteristics (スペクトラムアナライザ)
//! Class: 0x0A, ID: 0x31
//! Payload: 4 + numRfBlocks × 272 bytes

use std::fmt;

/// UBXヘッダー
const UBX_SYNC_1: u8 = 0xB5;
const UBX_SYNC_2: u8 = 0x62;

/// MON-SPAN メッセージ識別子
const MON_CLASS: u8 = 0x0A;
const MON_SPAN_ID: u8 = 0x31;

/// MON-SPAN ヘッダー長
const MON_SPAN_HEADER_LEN: usize = 4;

/// MON-SPAN ブロックサイズ
const MON_SPAN_BLOCK_SIZE: usize = 272;

/// スペクトラムビン数
const SPECTRUM_BINS: usize = 256;

/// パースエラー
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    /// ヘッダーが不正（0xB5 0x62 で始まらない）
    InvalidHeader,
    /// Class/IDが不一致（MON-SPANではない）
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

/// スペクトラムブロック情報
#[derive(Debug, Clone, PartialEq)]
pub struct SpanBlock {
    /// スペクトラムデータ（256点、dB）
    pub spectrum: [u8; SPECTRUM_BINS],
    /// 周波数スパン（Hz）
    pub span: u32,
    /// 周波数分解能（Hz）
    pub res: u32,
    /// 中心周波数（Hz）
    pub center: u32,
    /// PGAゲイン（dB）
    /// No.5異常機では38dB、正常機では54dB程度
    pub pga: u8,
}

impl SpanBlock {
    /// ビンインデックスから周波数を計算（Hz）
    /// f(i) = center + span × (i - 127) / 256
    pub fn frequency_at_bin(&self, bin_index: u8) -> f64 {
        let i = bin_index as f64;
        let center = self.center as f64;
        let span = self.span as f64;
        center + span * (i - 127.0) / 256.0
    }

    /// スペクトラムの最大値（dB）
    pub fn max_amplitude(&self) -> u8 {
        self.spectrum.iter().copied().max().unwrap_or(0)
    }

    /// スペクトラムの平均値（dB）
    pub fn avg_amplitude(&self) -> f32 {
        let sum: u32 = self.spectrum.iter().map(|&x| x as u32).sum();
        sum as f32 / SPECTRUM_BINS as f32
    }
}

/// MON-SPAN パース結果
#[derive(Debug, Clone, PartialEq)]
pub struct MonSpan {
    /// メッセージバージョン
    pub version: u8,
    /// RFブロック数
    pub n_blocks: u8,
    /// スペクトラムブロック情報
    pub blocks: Vec<SpanBlock>,
}

impl MonSpan {
    /// L1ブロック（block_id=0に相当）を取得
    /// 注: MON-SPANにはblock_idがないため、インデックス0をL1として扱う
    pub fn l1_block(&self) -> Option<&SpanBlock> {
        self.blocks.first()
    }

    /// L2ブロック（block_id=1に相当）を取得
    pub fn l2_block(&self) -> Option<&SpanBlock> {
        self.blocks.get(1)
    }
}

/// MON-SPANメッセージをパースする
pub fn parse(data: &[u8]) -> Result<MonSpan, ParseError> {
    // 最小長チェック: header(2) + class(1) + id(1) + len(2) + header_payload(4) + checksum(2) = 12
    let min_len = 2 + 1 + 1 + 2 + MON_SPAN_HEADER_LEN + 2;
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
    if class != MON_CLASS || id != MON_SPAN_ID {
        return Err(ParseError::MessageMismatch { class, id });
    }

    // ペイロード長取得
    let payload_len = u16::from_le_bytes([data[4], data[5]]) as usize;

    // ペイロード長が4未満（ヘッダー部すらない）
    if payload_len < MON_SPAN_HEADER_LEN {
        return Err(ParseError::InsufficientLength {
            expected: MON_SPAN_HEADER_LEN,
            actual: payload_len,
        });
    }

    // nBlocksを読んで、期待されるペイロード長を計算
    let n_blocks = data[6 + 1]; // offset 1 in payload
    let expected_payload_len = MON_SPAN_HEADER_LEN + (n_blocks as usize) * MON_SPAN_BLOCK_SIZE;

    if payload_len < expected_payload_len {
        return Err(ParseError::InsufficientLength {
            expected: expected_payload_len,
            actual: payload_len,
        });
    }

    // データ長チェック
    let total_len = 6 + payload_len + 2;
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

    // ヘッダー部フィールド抽出
    let version = payload[0];
    // n_blocks は既に取得済み

    // スペクトラムブロック抽出
    let mut blocks = Vec::with_capacity(n_blocks as usize);
    for i in 0..n_blocks as usize {
        let block_offset = MON_SPAN_HEADER_LEN + i * MON_SPAN_BLOCK_SIZE;
        let block = &payload[block_offset..block_offset + MON_SPAN_BLOCK_SIZE];

        // spectrum: offset 0-255 (256バイト)
        let mut spectrum = [0u8; SPECTRUM_BINS];
        spectrum.copy_from_slice(&block[0..SPECTRUM_BINS]);

        // span: offset 256-259 (4バイト)
        let span = u32::from_le_bytes([block[256], block[257], block[258], block[259]]);

        // res: offset 260-263 (4バイト)
        let res = u32::from_le_bytes([block[260], block[261], block[262], block[263]]);

        // center: offset 264-267 (4バイト)
        let center = u32::from_le_bytes([block[264], block[265], block[266], block[267]]);

        // pga: offset 268 (1バイト)
        let pga = block[268];

        blocks.push(SpanBlock {
            spectrum,
            span,
            res,
            center,
            pga,
        });
    }

    Ok(MonSpan {
        version,
        n_blocks,
        blocks,
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

    /// テスト用のスペクトラムブロックを生成
    fn build_span_block(
        spectrum_fill: u8,
        span: u32,
        res: u32,
        center: u32,
        pga: u8,
    ) -> Vec<u8> {
        let mut block = vec![0u8; MON_SPAN_BLOCK_SIZE];

        // spectrum: offset 0-255
        for i in 0..SPECTRUM_BINS {
            block[i] = spectrum_fill;
        }

        // span: offset 256-259
        block[256..260].copy_from_slice(&span.to_le_bytes());

        // res: offset 260-263
        block[260..264].copy_from_slice(&res.to_le_bytes());

        // center: offset 264-267
        block[264..268].copy_from_slice(&center.to_le_bytes());

        // pga: offset 268
        block[268] = pga;

        // reserved1: offset 269-271 (そのまま0)

        block
    }

    /// テスト用のMON-SPANメッセージを生成
    fn build_mon_span_message(blocks: &[Vec<u8>]) -> Vec<u8> {
        let n_blocks = blocks.len() as u8;
        let payload_len = MON_SPAN_HEADER_LEN + blocks.len() * MON_SPAN_BLOCK_SIZE;

        let mut payload = vec![0u8; payload_len];
        // ヘッダー部
        payload[0] = 0x00; // version
        payload[1] = n_blocks;
        // offset 2-3: reserved0

        // ブロック部
        for (i, block) in blocks.iter().enumerate() {
            let offset = MON_SPAN_HEADER_LEN + i * MON_SPAN_BLOCK_SIZE;
            payload[offset..offset + MON_SPAN_BLOCK_SIZE].copy_from_slice(block);
        }

        // UBXフレーム構築
        let mut frame = vec![UBX_SYNC_1, UBX_SYNC_2, MON_CLASS, MON_SPAN_ID];
        let len = payload_len as u16;
        frame.extend_from_slice(&len.to_le_bytes());
        frame.extend_from_slice(&payload);

        // チェックサム計算
        let (ck_a, ck_b) = calculate_checksum(&frame[2..]);
        frame.push(ck_a);
        frame.push(ck_b);

        frame
    }

    /// テストケース: パース成功（1ブロック、2ブロック、0ブロック）
    #[test]
    fn test_parse_success_cases() {
        struct TestCase {
            name: &'static str,
            n_blocks: usize,
            pga_values: Vec<u8>,
            should_succeed: bool,
        }

        let cases = vec![
            // 正常系: L1のみ1ブロック
            TestCase {
                name: "L1 only (1 block)",
                n_blocks: 1,
                pga_values: vec![54], // 正常機のPGA
                should_succeed: true,
            },
            // 正常系: L1+L2の2ブロック
            TestCase {
                name: "L1+L2 (2 blocks)",
                n_blocks: 2,
                pga_values: vec![54, 54],
                should_succeed: true,
            },
            // 境界値: nBlocks=0
            TestCase {
                name: "No blocks",
                n_blocks: 0,
                pga_values: vec![],
                should_succeed: true,
            },
        ];

        for tc in cases {
            let blocks: Vec<Vec<u8>> = tc
                .pga_values
                .iter()
                .map(|&pga| {
                    build_span_block(
                        50,           // spectrum_fill
                        128_000_000,  // span: 128MHz
                        500_000,      // res: 500kHz
                        1_575_420_000, // center: L1周波数
                        pga,
                    )
                })
                .collect();
            let data = build_mon_span_message(&blocks);
            let result = parse(&data);

            if tc.should_succeed {
                let span = result.unwrap_or_else(|e| panic!("{}: unexpected error: {}", tc.name, e));
                assert_eq!(span.blocks.len(), tc.n_blocks, "{}: n_blocks mismatch", tc.name);

                // PGA値を検証
                for (i, &expected_pga) in tc.pga_values.iter().enumerate() {
                    assert_eq!(span.blocks[i].pga, expected_pga, "{}: pga mismatch at {}", tc.name, i);
                }
            } else {
                assert!(result.is_err(), "{}: expected error", tc.name);
            }
        }
    }

    /// テストケース: スペクトラムデータの抽出
    #[test]
    fn test_spectrum_extraction() {
        struct TestCase {
            name: &'static str,
            spectrum_fill: u8,
            expected_max: u8,
            should_succeed: bool,
        }

        let cases = vec![
            // 正常系: 一様なスペクトラム
            TestCase {
                name: "Uniform spectrum at 50dB",
                spectrum_fill: 50,
                expected_max: 50,
                should_succeed: true,
            },
            // 正常系: 最大値
            TestCase {
                name: "Max amplitude 255",
                spectrum_fill: 255,
                expected_max: 255,
                should_succeed: true,
            },
            // 正常系: 最小値
            TestCase {
                name: "Min amplitude 0",
                spectrum_fill: 0,
                expected_max: 0,
                should_succeed: true,
            },
        ];

        for tc in cases {
            let block = build_span_block(tc.spectrum_fill, 128_000_000, 500_000, 1_575_420_000, 54);
            let data = build_mon_span_message(&[block]);
            let result = parse(&data);

            if tc.should_succeed {
                let span = result.unwrap_or_else(|e| panic!("{}: unexpected error: {}", tc.name, e));
                let max = span.blocks[0].max_amplitude();
                assert_eq!(max, tc.expected_max, "{}: max_amplitude mismatch", tc.name);

                // 全ビンが同じ値か確認
                for (i, &val) in span.blocks[0].spectrum.iter().enumerate() {
                    assert_eq!(val, tc.spectrum_fill, "{}: spectrum[{}] mismatch", tc.name, i);
                }
            }
        }
    }

    /// テストケース: PGAゲインの抽出（異常検出用）
    /// 参考: 38dBは異常（No.5異常機）、54dBは正常
    #[test]
    fn test_pga_extraction() {
        struct TestCase {
            name: &'static str,
            pga: u8,
            should_succeed: bool,
        }

        let cases = vec![
            // 正常系: 正常機のPGA
            TestCase {
                name: "Normal PGA (54dB)",
                pga: 54,
                should_succeed: true,
            },
            // 正常系: No.5異常機のPGA
            TestCase {
                name: "Anomaly PGA (38dB)",
                pga: 38,
                should_succeed: true,
            },
            // 境界値: 最大値
            TestCase {
                name: "Max PGA (255)",
                pga: 255,
                should_succeed: true,
            },
        ];

        for tc in cases {
            let block = build_span_block(50, 128_000_000, 500_000, 1_575_420_000, tc.pga);
            let data = build_mon_span_message(&[block]);
            let result = parse(&data);

            if tc.should_succeed {
                let span = result.unwrap_or_else(|e| panic!("{}: unexpected error: {}", tc.name, e));
                assert_eq!(span.blocks[0].pga, tc.pga, "{}: pga mismatch", tc.name);
            }
        }
    }

    /// テストケース: 周波数計算ヘルパー
    #[test]
    fn test_frequency_calculation() {
        struct TestCase {
            name: &'static str,
            center: u32,
            span: u32,
            bin_index: u8,
            expected_freq: f64,
            should_succeed: bool,
        }

        let cases = vec![
            // 正常系: 中央ビン（i=127）→ center - span/256
            TestCase {
                name: "Center bin (i=127)",
                center: 1_575_420_000,
                span: 128_000_000,
                bin_index: 127,
                expected_freq: 1_575_420_000.0 + 128_000_000.0 * (127.0 - 127.0) / 256.0,
                should_succeed: true,
            },
            // 正常系: 最初のビン（i=0）
            TestCase {
                name: "First bin (i=0)",
                center: 1_575_420_000,
                span: 128_000_000,
                bin_index: 0,
                expected_freq: 1_575_420_000.0 + 128_000_000.0 * (0.0 - 127.0) / 256.0,
                should_succeed: true,
            },
            // 正常系: 最後のビン（i=255）
            TestCase {
                name: "Last bin (i=255)",
                center: 1_575_420_000,
                span: 128_000_000,
                bin_index: 255,
                expected_freq: 1_575_420_000.0 + 128_000_000.0 * (255.0 - 127.0) / 256.0,
                should_succeed: true,
            },
        ];

        for tc in cases {
            let block = build_span_block(50, tc.span, 500_000, tc.center, 54);
            let data = build_mon_span_message(&[block]);
            let result = parse(&data);

            if tc.should_succeed {
                let span = result.unwrap_or_else(|e| panic!("{}: unexpected error: {}", tc.name, e));
                let freq = span.blocks[0].frequency_at_bin(tc.bin_index);
                let diff = (freq - tc.expected_freq).abs();
                assert!(diff < 1.0, "{}: frequency mismatch, got {}, expected {}", tc.name, freq, tc.expected_freq);
            }
        }
    }

    /// テストケース: エラー系
    #[test]
    fn test_parse_error_cases() {
        let block = build_span_block(50, 128_000_000, 500_000, 1_575_420_000, 54);
        let valid_msg = build_mon_span_message(&[block]);

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
            // 異常系: Class/ID不一致
            TestCase {
                name: "Message mismatch (wrong ID)",
                data: {
                    let mut d = valid_msg.clone();
                    d[3] = 0x38; // MON-RF ID
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
            // 異常系: ペイロード長不足（nBlocks=2だがデータは1ブロック分）
            TestCase {
                name: "Insufficient length (blocks mismatch)",
                data: {
                    let mut d = valid_msg.clone();
                    // nBlocksを2に変更（offset 6+1 = 7）
                    d[7] = 2;
                    // チェックサムを再計算
                    let payload_len = u16::from_le_bytes([d[4], d[5]]) as usize;
                    let (ck_a, ck_b) = calculate_checksum(&d[2..6 + payload_len]);
                    d[6 + payload_len] = ck_a;
                    d[6 + payload_len + 1] = ck_b;
                    d
                },
                should_succeed: false,
            },
            // 異常系: データが短すぎる
            TestCase {
                name: "Data too short",
                data: vec![0xB5, 0x62, 0x0A, 0x31],
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

    /// テストケース: L1/L2ブロックアクセスヘルパー
    #[test]
    fn test_block_access_helpers() {
        struct TestCase {
            name: &'static str,
            n_blocks: usize,
            has_l1: bool,
            has_l2: bool,
            should_succeed: bool,
        }

        let cases = vec![
            TestCase {
                name: "2 blocks (L1+L2)",
                n_blocks: 2,
                has_l1: true,
                has_l2: true,
                should_succeed: true,
            },
            TestCase {
                name: "1 block (L1 only)",
                n_blocks: 1,
                has_l1: true,
                has_l2: false,
                should_succeed: true,
            },
            TestCase {
                name: "0 blocks",
                n_blocks: 0,
                has_l1: false,
                has_l2: false,
                should_succeed: true,
            },
        ];

        for tc in cases {
            let blocks: Vec<Vec<u8>> = (0..tc.n_blocks)
                .map(|_| build_span_block(50, 128_000_000, 500_000, 1_575_420_000, 54))
                .collect();
            let data = build_mon_span_message(&blocks);
            let result = parse(&data);

            if tc.should_succeed {
                let span = result.unwrap_or_else(|e| panic!("{}: unexpected error: {}", tc.name, e));
                assert_eq!(span.l1_block().is_some(), tc.has_l1, "{}: l1_block mismatch", tc.name);
                assert_eq!(span.l2_block().is_some(), tc.has_l2, "{}: l2_block mismatch", tc.name);
            }
        }
    }

    /// テストケース: span/res/center の抽出検証
    #[test]
    fn test_span_res_center_extraction() {
        struct TestCase {
            name: &'static str,
            span: u32,
            res: u32,
            center: u32,
            should_succeed: bool,
        }

        let cases = vec![
            // 正常系: L1帯の典型値
            TestCase {
                name: "L1 band typical values",
                span: 128_000_000,      // 128MHz
                res: 500_000,           // 500kHz
                center: 1_575_420_000,  // L1周波数
                should_succeed: true,
            },
            // 正常系: L2帯の典型値
            TestCase {
                name: "L2 band typical values",
                span: 128_000_000,
                res: 500_000,
                center: 1_227_600_000,  // L2周波数
                should_succeed: true,
            },
            // 境界値: 最小値
            TestCase {
                name: "Minimum values",
                span: 0,
                res: 0,
                center: 0,
                should_succeed: true,
            },
        ];

        for tc in cases {
            let block = build_span_block(50, tc.span, tc.res, tc.center, 54);
            let data = build_mon_span_message(&[block]);
            let result = parse(&data);

            if tc.should_succeed {
                let span_msg = result.unwrap_or_else(|e| panic!("{}: unexpected error: {}", tc.name, e));
                let blk = &span_msg.blocks[0];
                assert_eq!(blk.span, tc.span, "{}: span mismatch", tc.name);
                assert_eq!(blk.res, tc.res, "{}: res mismatch", tc.name);
                assert_eq!(blk.center, tc.center, "{}: center mismatch", tc.name);
            }
        }
    }

    /// テストケース: avg_amplitude() ヘルパー
    #[test]
    fn test_avg_amplitude() {
        struct TestCase {
            name: &'static str,
            spectrum_fill: u8,
            expected_avg: f32,
            should_succeed: bool,
        }

        let cases = vec![
            // 正常系: 一様なスペクトラム
            TestCase {
                name: "Uniform at 50",
                spectrum_fill: 50,
                expected_avg: 50.0,
                should_succeed: true,
            },
            // 正常系: 一様なスペクトラム（最大値）
            TestCase {
                name: "Uniform at 255",
                spectrum_fill: 255,
                expected_avg: 255.0,
                should_succeed: true,
            },
            // 正常系: 一様なスペクトラム（最小値）
            TestCase {
                name: "Uniform at 0",
                spectrum_fill: 0,
                expected_avg: 0.0,
                should_succeed: true,
            },
        ];

        for tc in cases {
            let block = build_span_block(tc.spectrum_fill, 128_000_000, 500_000, 1_575_420_000, 54);
            let data = build_mon_span_message(&[block]);
            let result = parse(&data);

            if tc.should_succeed {
                let span = result.unwrap_or_else(|e| panic!("{}: unexpected error: {}", tc.name, e));
                let avg = span.blocks[0].avg_amplitude();
                let diff = (avg - tc.expected_avg).abs();
                assert!(diff < 0.01, "{}: avg_amplitude mismatch, got {}, expected {}", tc.name, avg, tc.expected_avg);
            }
        }
    }
}
