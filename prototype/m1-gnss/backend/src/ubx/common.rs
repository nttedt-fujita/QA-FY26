//! UBXプロトコル共通定義
//!
//! 各パーサーで共通して使用する定数・関数

/// UBXヘッダー同期バイト1
pub const UBX_SYNC_1: u8 = 0xB5;

/// UBXヘッダー同期バイト2
pub const UBX_SYNC_2: u8 = 0x62;

/// 8-bit Fletcher checksum（UBX仕様）
///
/// Class, ID, Length, Payload に対して計算する
pub fn calculate_checksum(data: &[u8]) -> (u8, u8) {
    let mut ck_a: u8 = 0;
    let mut ck_b: u8 = 0;
    for byte in data {
        ck_a = ck_a.wrapping_add(*byte);
        ck_b = ck_b.wrapping_add(ck_a);
    }
    (ck_a, ck_b)
}

/// UBX Pollコマンドを構築（空のペイロード）
///
/// 指定されたClass/IDに対するPoll（問い合わせ）コマンドを生成する。
/// 例: SEC-UNIQID poll = build_ubx_poll(0x27, 0x03)
pub fn build_ubx_poll(class: u8, id: u8) -> Vec<u8> {
    let payload: &[u8] = &[];
    let payload_len: u16 = 0;

    let mut frame = vec![UBX_SYNC_1, UBX_SYNC_2, class, id];
    frame.extend_from_slice(&payload_len.to_le_bytes());
    frame.extend_from_slice(payload);

    let (ck_a, ck_b) = calculate_checksum(&frame[2..]);
    frame.push(ck_a);
    frame.push(ck_b);

    frame
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checksum_calculation() {
        // 空データ
        assert_eq!(calculate_checksum(&[]), (0, 0));

        // 単一バイト
        assert_eq!(calculate_checksum(&[0x01]), (0x01, 0x01));

        // 複数バイト（手計算で検証）
        // [0x01, 0x02]: ck_a = 0x01+0x02 = 0x03, ck_b = 0x01 + 0x03 = 0x04
        assert_eq!(calculate_checksum(&[0x01, 0x02]), (0x03, 0x04));
    }
}
