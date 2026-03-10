//! MON-RF メッセージパーサー
//!
//! UBX-MON-RF: RF Information
//! Class: 0x0A, ID: 0x38
//! Payload: 4 + nBlocks × 24 bytes

use std::fmt;

/// UBXヘッダー
const UBX_SYNC_1: u8 = 0xB5;
const UBX_SYNC_2: u8 = 0x62;

/// MON-RF メッセージ識別子
const MON_CLASS: u8 = 0x0A;
const MON_RF_ID: u8 = 0x38;

/// MON-RF ヘッダー長
const MON_RF_HEADER_LEN: usize = 4;

/// MON-RF ブロックサイズ
const MON_RF_BLOCK_SIZE: usize = 24;

/// パースエラー
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    /// ヘッダーが不正（0xB5 0x62 で始まらない）
    InvalidHeader,
    /// Class/IDが不一致（MON-RFではない）
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

/// RFブロック情報
#[derive(Debug, Clone, PartialEq)]
pub struct RfBlock {
    /// RFブロックID (0=L1, 1=L2/L5)
    pub block_id: u8,
    /// ジャミング状態 (0=不明, 1=OK, 2=警告, 3=危険)
    pub jamming_state: u8,
    /// アンテナ状態 (0=INIT, 1=DONTKNOW, 2=OK, 3=SHORT, 4=OPEN)
    pub ant_status: u8,
    /// アンテナ電源状態 (0=OFF, 1=ON, 2=DONTKNOW)
    pub ant_power: u8,
    /// ノイズレベル
    pub noise_per_ms: u16,
    /// AGCモニタ (0-8191)
    pub agc_cnt: u16,
    /// CW干渉抑制レベル (0=なし, 255=強)
    pub cw_suppression: u8,
}

impl RfBlock {
    /// ジャミングが検出されているか
    pub fn is_jamming_detected(&self) -> bool {
        self.jamming_state >= 2
    }

    /// ジャミングが危険レベルか
    pub fn is_jamming_critical(&self) -> bool {
        self.jamming_state == 3
    }

    /// アンテナが正常か
    pub fn is_antenna_ok(&self) -> bool {
        self.ant_status == 2
    }
}

/// MON-RF パース結果
#[derive(Debug, Clone, PartialEq)]
pub struct MonRf {
    /// メッセージバージョン
    pub version: u8,
    /// RFブロック数
    pub n_blocks: u8,
    /// RFブロック情報
    pub blocks: Vec<RfBlock>,
}

impl MonRf {
    /// いずれかのブロックでジャミングが検出されているか
    pub fn has_jamming(&self) -> bool {
        self.blocks.iter().any(|b| b.is_jamming_detected())
    }

    /// いずれかのブロックでジャミングが危険レベルか
    pub fn has_critical_jamming(&self) -> bool {
        self.blocks.iter().any(|b| b.is_jamming_critical())
    }
}

/// MON-RFメッセージをパースする
pub fn parse(data: &[u8]) -> Result<MonRf, ParseError> {
    // 最小長チェック: header(2) + class(1) + id(1) + len(2) + header_payload(4) + checksum(2) = 12
    let min_len = 2 + 1 + 1 + 2 + MON_RF_HEADER_LEN + 2;
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
    if class != MON_CLASS || id != MON_RF_ID {
        return Err(ParseError::MessageMismatch { class, id });
    }

    // ペイロード長取得
    let payload_len = u16::from_le_bytes([data[4], data[5]]) as usize;

    // ペイロード長が4未満（ヘッダー部すらない）
    if payload_len < MON_RF_HEADER_LEN {
        return Err(ParseError::InsufficientLength {
            expected: MON_RF_HEADER_LEN,
            actual: payload_len,
        });
    }

    // nBlocksを読んで、期待されるペイロード長を計算
    let n_blocks = data[6 + 1]; // offset 1 in payload
    let expected_payload_len = MON_RF_HEADER_LEN + (n_blocks as usize) * MON_RF_BLOCK_SIZE;

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

    // RFブロック抽出
    let mut blocks = Vec::with_capacity(n_blocks as usize);
    for i in 0..n_blocks as usize {
        let block_offset = MON_RF_HEADER_LEN + i * MON_RF_BLOCK_SIZE;
        let block = &payload[block_offset..block_offset + MON_RF_BLOCK_SIZE];

        let block_id = block[0];
        let flags = block[1];
        let jamming_state = flags & 0x03;
        let ant_status = block[2];
        let ant_power = block[3];
        let noise_per_ms = u16::from_le_bytes([block[12], block[13]]);
        let agc_cnt = u16::from_le_bytes([block[14], block[15]]);
        let cw_suppression = block[16];

        blocks.push(RfBlock {
            block_id,
            jamming_state,
            ant_status,
            ant_power,
            noise_per_ms,
            agc_cnt,
            cw_suppression,
        });
    }

    Ok(MonRf {
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

    /// テスト用のRFブロックを生成
    fn build_rf_block(
        block_id: u8,
        jamming_state: u8,
        ant_status: u8,
        ant_power: u8,
        noise_per_ms: u16,
        agc_cnt: u16,
        cw_suppression: u8,
    ) -> Vec<u8> {
        let mut block = vec![0u8; MON_RF_BLOCK_SIZE];
        block[0] = block_id;
        block[1] = jamming_state & 0x03; // flags (jamming_state in bit 0-1)
        block[2] = ant_status;
        block[3] = ant_power;
        // offset 4-7: postStatus (unused)
        // offset 8-11: reserved1 (unused)
        block[12..14].copy_from_slice(&noise_per_ms.to_le_bytes());
        block[14..16].copy_from_slice(&agc_cnt.to_le_bytes());
        block[16] = cw_suppression;
        // offset 17-20: ofsI, magI, ofsQ, magQ (unused)
        // offset 21-23: reserved2 (unused)
        block
    }

    /// テスト用のMON-RFメッセージを生成
    fn build_mon_rf_message(blocks: &[Vec<u8>]) -> Vec<u8> {
        let n_blocks = blocks.len() as u8;
        let payload_len = MON_RF_HEADER_LEN + blocks.len() * MON_RF_BLOCK_SIZE;

        let mut payload = vec![0u8; payload_len];
        // ヘッダー部
        payload[0] = 0x00; // version
        payload[1] = n_blocks;
        // offset 2-3: reserved0

        // ブロック部
        for (i, block) in blocks.iter().enumerate() {
            let offset = MON_RF_HEADER_LEN + i * MON_RF_BLOCK_SIZE;
            payload[offset..offset + MON_RF_BLOCK_SIZE].copy_from_slice(block);
        }

        // UBXフレーム構築
        let mut frame = vec![UBX_SYNC_1, UBX_SYNC_2, MON_CLASS, MON_RF_ID];
        let len = payload_len as u16;
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
            blocks: Vec<(u8, u8, u8, u8)>, // (block_id, jamming_state, ant_status, agc_cnt)
            expected_n_blocks: usize,
            should_succeed: bool,
        }

        let cases = vec![
            // 正常系: L1のみ、ジャミングなし
            TestCase {
                name: "L1 only, no jamming",
                blocks: vec![(0, 1, 2, 0)], // block_id=0(L1), jamming=1(OK), ant=2(OK)
                expected_n_blocks: 1,
                should_succeed: true,
            },
            // 正常系: L1+L2、ジャミング警告
            TestCase {
                name: "L1+L2, jamming warning on L2",
                blocks: vec![(0, 1, 2, 0), (1, 2, 2, 0)], // L2にjamming=2(Warning)
                expected_n_blocks: 2,
                should_succeed: true,
            },
            // 正常系: ジャミング危険
            TestCase {
                name: "Critical jamming",
                blocks: vec![(0, 3, 2, 0)], // jamming=3(Critical)
                expected_n_blocks: 1,
                should_succeed: true,
            },
            // 境界値: nBlocks=0
            TestCase {
                name: "No blocks",
                blocks: vec![],
                expected_n_blocks: 0,
                should_succeed: true,
            },
        ];

        for tc in cases {
            let block_data: Vec<Vec<u8>> = tc
                .blocks
                .iter()
                .map(|(block_id, jamming, ant, agc)| {
                    build_rf_block(*block_id, *jamming, *ant, 1, 100, *agc as u16, 0)
                })
                .collect();
            let data = build_mon_rf_message(&block_data);
            let result = parse(&data);

            if tc.should_succeed {
                let rf = result.unwrap_or_else(|e| panic!("{}: unexpected error: {}", tc.name, e));
                assert_eq!(rf.blocks.len(), tc.expected_n_blocks, "{}: n_blocks mismatch", tc.name);

                // 各ブロックの値を検証
                for (i, (block_id, jamming, ant, _agc)) in tc.blocks.iter().enumerate() {
                    assert_eq!(rf.blocks[i].block_id, *block_id, "{}: block_id mismatch at {}", tc.name, i);
                    assert_eq!(rf.blocks[i].jamming_state, *jamming, "{}: jamming_state mismatch at {}", tc.name, i);
                    assert_eq!(rf.blocks[i].ant_status, *ant, "{}: ant_status mismatch at {}", tc.name, i);
                }
            } else {
                assert!(result.is_err(), "{}: expected error", tc.name);
            }
        }
    }

    /// テストケース: アンテナ状態
    #[test]
    fn test_antenna_status() {
        struct TestCase {
            name: &'static str,
            ant_status: u8,
            is_ok: bool,
            should_succeed: bool,
        }

        let cases = vec![
            TestCase {
                name: "Antenna OK",
                ant_status: 2,
                is_ok: true,
                should_succeed: true,
            },
            TestCase {
                name: "Antenna SHORT",
                ant_status: 3,
                is_ok: false,
                should_succeed: true,
            },
            TestCase {
                name: "Antenna OPEN",
                ant_status: 4,
                is_ok: false,
                should_succeed: true,
            },
            TestCase {
                name: "Antenna INIT",
                ant_status: 0,
                is_ok: false,
                should_succeed: true,
            },
        ];

        for tc in cases {
            let block = build_rf_block(0, 1, tc.ant_status, 1, 100, 4000, 0);
            let data = build_mon_rf_message(&[block]);
            let result = parse(&data);

            if tc.should_succeed {
                let rf = result.unwrap_or_else(|e| panic!("{}: unexpected error: {}", tc.name, e));
                assert_eq!(rf.blocks[0].is_antenna_ok(), tc.is_ok, "{}: is_antenna_ok mismatch", tc.name);
            }
        }
    }

    /// テストケース: ジャミング判定ヘルパー
    #[test]
    fn test_jamming_helpers() {
        struct TestCase {
            name: &'static str,
            jamming_states: Vec<u8>,
            has_jamming: bool,
            has_critical: bool,
            should_succeed: bool,
        }

        let cases = vec![
            TestCase {
                name: "No jamming",
                jamming_states: vec![1, 1],
                has_jamming: false,
                has_critical: false,
                should_succeed: true,
            },
            TestCase {
                name: "Warning on one block",
                jamming_states: vec![1, 2],
                has_jamming: true,
                has_critical: false,
                should_succeed: true,
            },
            TestCase {
                name: "Critical on one block",
                jamming_states: vec![1, 3],
                has_jamming: true,
                has_critical: true,
                should_succeed: true,
            },
        ];

        for tc in cases {
            let blocks: Vec<Vec<u8>> = tc
                .jamming_states
                .iter()
                .enumerate()
                .map(|(i, &j)| build_rf_block(i as u8, j, 2, 1, 100, 4000, 0))
                .collect();
            let data = build_mon_rf_message(&blocks);
            let result = parse(&data);

            if tc.should_succeed {
                let rf = result.unwrap_or_else(|e| panic!("{}: unexpected error: {}", tc.name, e));
                assert_eq!(rf.has_jamming(), tc.has_jamming, "{}: has_jamming mismatch", tc.name);
                assert_eq!(rf.has_critical_jamming(), tc.has_critical, "{}: has_critical_jamming mismatch", tc.name);
            }
        }
    }

    /// テストケース: AGC/CW抑制値の抽出
    #[test]
    fn test_agc_and_cw_suppression() {
        struct TestCase {
            name: &'static str,
            agc_cnt: u16,
            cw_suppression: u8,
            should_succeed: bool,
        }

        let cases = vec![
            TestCase {
                name: "Normal AGC",
                agc_cnt: 4000,
                cw_suppression: 0,
                should_succeed: true,
            },
            TestCase {
                name: "High CW suppression",
                agc_cnt: 3000,
                cw_suppression: 128,
                should_succeed: true,
            },
            TestCase {
                name: "Max values",
                agc_cnt: 8191,
                cw_suppression: 255,
                should_succeed: true,
            },
        ];

        for tc in cases {
            let block = build_rf_block(0, 1, 2, 1, 100, tc.agc_cnt, tc.cw_suppression);
            let data = build_mon_rf_message(&[block]);
            let result = parse(&data);

            if tc.should_succeed {
                let rf = result.unwrap_or_else(|e| panic!("{}: unexpected error: {}", tc.name, e));
                assert_eq!(rf.blocks[0].agc_cnt, tc.agc_cnt, "{}: agc_cnt mismatch", tc.name);
                assert_eq!(rf.blocks[0].cw_suppression, tc.cw_suppression, "{}: cw_suppression mismatch", tc.name);
            }
        }
    }

    /// テストケース: エラー系
    #[test]
    fn test_parse_error_cases() {
        let block = build_rf_block(0, 1, 2, 1, 100, 4000, 0);
        let valid_msg = build_mon_rf_message(&[block]);

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
            // 異常系: ペイロード長不足（nBlocks=2だがデータは1ブロック分）
            TestCase {
                name: "Insufficient length (blocks mismatch)",
                data: {
                    // nBlocks=2と宣言しているが、実際には1ブロック分しかないデータを作る
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
