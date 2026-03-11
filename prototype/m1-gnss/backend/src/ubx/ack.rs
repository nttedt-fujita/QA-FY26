//! ACK/NAK メッセージ関連
//!
//! UBX-ACK-ACK (0x05 0x01): 設定変更などの成功応答
//! UBX-ACK-NAK (0x05 0x00): 設定変更などの失敗応答

use super::common::{UBX_SYNC_1, UBX_SYNC_2};

/// ACKクラス
pub const ACK_CLASS: u8 = 0x05;

/// ACK-ACK ID（成功）
pub const ACK_ACK_ID: u8 = 0x01;

/// ACK-NAK ID（失敗）
pub const ACK_NAK_ID: u8 = 0x00;

/// ACK判定結果
#[derive(Debug, Clone, PartialEq)]
pub enum AckResult {
    /// ACK-ACK（成功）
    Ack { class: u8, id: u8 },
    /// ACK-NAK（失敗）
    Nak { class: u8, id: u8 },
    /// ACK/NAKではない（別のメッセージ）
    NotAck,
    /// フレームが短すぎる
    TooShort,
    /// 同期バイト不一致
    InvalidSync,
}

/// UBXフレームがACK/NAKか判定する
///
/// # Arguments
/// * `frame` - 受信したUBXフレーム
///
/// # Returns
/// * `AckResult` - 判定結果
pub fn parse_ack(frame: &[u8]) -> AckResult {
    // 最低限のフレーム長チェック
    // Sync(2) + Class(1) + ID(1) + Length(2) + Payload(2) + Checksum(2) = 10
    if frame.len() < 10 {
        return AckResult::TooShort;
    }

    // 同期バイトチェック
    if frame[0] != UBX_SYNC_1 || frame[1] != UBX_SYNC_2 {
        return AckResult::InvalidSync;
    }

    // クラスチェック
    if frame[2] != ACK_CLASS {
        return AckResult::NotAck;
    }

    // ペイロードから対象メッセージのclass/idを取得
    let target_class = frame[6];
    let target_id = frame[7];

    match frame[3] {
        ACK_ACK_ID => AckResult::Ack {
            class: target_class,
            id: target_id,
        },
        ACK_NAK_ID => AckResult::Nak {
            class: target_class,
            id: target_id,
        },
        _ => AckResult::NotAck,
    }
}

/// 指定したclass/idのACK-ACKかどうか判定
///
/// # Arguments
/// * `frame` - 受信したUBXフレーム
/// * `expected_class` - 期待するメッセージクラス
/// * `expected_id` - 期待するメッセージID
///
/// # Returns
/// * `true` - 指定したメッセージに対するACK-ACK
/// * `false` - それ以外
pub fn is_ack_for(frame: &[u8], expected_class: u8, expected_id: u8) -> bool {
    match parse_ack(frame) {
        AckResult::Ack { class, id } => class == expected_class && id == expected_id,
        _ => false,
    }
}

/// CFG-VALSETに対するACK-ACKかどうか判定
pub fn is_cfg_valset_ack(frame: &[u8]) -> bool {
    is_ack_for(frame, 0x06, 0x8A)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 有効なACK-ACKフレームを生成（テスト用）
    fn build_ack_ack(target_class: u8, target_id: u8) -> Vec<u8> {
        use super::super::common::calculate_checksum;

        let mut frame = vec![
            UBX_SYNC_1, UBX_SYNC_2,
            ACK_CLASS, ACK_ACK_ID,
            0x02, 0x00,  // length = 2
            target_class, target_id,
        ];
        let (ck_a, ck_b) = calculate_checksum(&frame[2..]);
        frame.push(ck_a);
        frame.push(ck_b);
        frame
    }

    /// 有効なACK-NAKフレームを生成（テスト用）
    fn build_ack_nak(target_class: u8, target_id: u8) -> Vec<u8> {
        use super::super::common::calculate_checksum;

        let mut frame = vec![
            UBX_SYNC_1, UBX_SYNC_2,
            ACK_CLASS, ACK_NAK_ID,
            0x02, 0x00,  // length = 2
            target_class, target_id,
        ];
        let (ck_a, ck_b) = calculate_checksum(&frame[2..]);
        frame.push(ck_a);
        frame.push(ck_b);
        frame
    }

    /// A1: ACK-ACKの正常パース
    #[test]
    fn test_a1_parse_ack_ack() {
        let frame = build_ack_ack(0x06, 0x8A);
        let result = parse_ack(&frame);

        assert_eq!(result, AckResult::Ack { class: 0x06, id: 0x8A });
    }

    /// A2: ACK-NAKの正常パース
    #[test]
    fn test_a2_parse_ack_nak() {
        let frame = build_ack_nak(0x06, 0x8A);
        let result = parse_ack(&frame);

        assert_eq!(result, AckResult::Nak { class: 0x06, id: 0x8A });
    }

    /// A3: ACKクラスでないメッセージ
    #[test]
    fn test_a3_not_ack_class() {
        // MON-VER応答など、ACKクラスでないメッセージ
        let frame = vec![
            0xB5, 0x62,
            0x0A, 0x04,  // class=0x0A (MON), id=0x04 (VER)
            0x04, 0x00,
            0x01, 0x02, 0x03, 0x04,
            0x00, 0x00,
        ];
        let result = parse_ack(&frame);

        assert_eq!(result, AckResult::NotAck);
    }

    /// A4: フレームが短すぎる
    #[test]
    fn test_a4_too_short() {
        let frame = vec![0xB5, 0x62, 0x05, 0x01];
        let result = parse_ack(&frame);

        assert_eq!(result, AckResult::TooShort);
    }

    /// A5: 同期バイト不一致
    #[test]
    fn test_a5_invalid_sync() {
        let frame = vec![0x00, 0x00, 0x05, 0x01, 0x02, 0x00, 0x06, 0x8A, 0x00, 0x00];
        let result = parse_ack(&frame);

        assert_eq!(result, AckResult::InvalidSync);
    }

    /// A6: is_ack_for 正常判定
    #[test]
    fn test_a6_is_ack_for_correct() {
        let frame = build_ack_ack(0x06, 0x8A);

        assert!(is_ack_for(&frame, 0x06, 0x8A));
        assert!(!is_ack_for(&frame, 0x06, 0x00)); // 異なるID
        assert!(!is_ack_for(&frame, 0x0A, 0x8A)); // 異なるClass
    }

    /// A7: is_cfg_valset_ack 正常判定
    #[test]
    fn test_a7_is_cfg_valset_ack() {
        let ack_frame = build_ack_ack(0x06, 0x8A);
        assert!(is_cfg_valset_ack(&ack_frame));

        let nak_frame = build_ack_nak(0x06, 0x8A);
        assert!(!is_cfg_valset_ack(&nak_frame));

        let other_frame = build_ack_ack(0x06, 0x00);
        assert!(!is_cfg_valset_ack(&other_frame));
    }
}
