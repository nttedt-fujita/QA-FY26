//! CFG-VALDEL メッセージ生成
//!
//! UBX-CFG-VALDEL: 設定値の削除（デフォルトに戻す）
//! Class: 0x06, ID: 0x8C
//!
//! 主な用途:
//! - BBRレイヤーから設定を削除（Flashが参照されるようになる）
//! - 設定を工場出荷状態に戻す
//!
//! 出典: u-blox F9 HPG 1.32 Interface Description (UBX-22008968) p.93-94

use super::common::{calculate_checksum, UBX_SYNC_1, UBX_SYNC_2};

/// CFG-VALDEL メッセージ識別子
const CFG_CLASS: u8 = 0x06;
const CFG_VALDEL_ID: u8 = 0x8C;

/// 削除対象レイヤー
///
/// 注意: RAMからは削除できない（CFG-VALSETで値を変更するのみ）
/// BBRまたはFlashから削除可能
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum DeleteLayer {
    /// BBR（バッテリバックアップRAM）から削除
    Bbr = 0x02,
    /// Flash（不揮発性）から削除
    Flash = 0x04,
    /// BBR + Flash 両方から削除
    BbrAndFlash = 0x06,
}

/// CFG-VALDELメッセージを生成
///
/// 指定したレイヤーから設定キーを削除する。
/// 削除後、上位レイヤーまたはデフォルト値が参照されるようになる。
///
/// # Arguments
/// * `layer` - 削除対象レイヤー（BBR/Flash/両方）
/// * `keys` - 削除するキーIDのリスト
///
/// # Returns
/// 完全なUBXフレーム（ヘッダー + ペイロード + チェックサム）
///
/// # 参照
/// u-blox レイヤー優先順位: RAM > BBR > Flash > Default
/// BBRを削除すると、Flashまたはデフォルト値が参照される
pub fn delete_config_keys(layer: DeleteLayer, keys: &[u32]) -> Vec<u8> {
    // ペイロード構成:
    // - version (1 byte): 0x00
    // - layers (1 byte): レイヤービットマスク
    // - reserved0 (2 bytes): 0x00, 0x00
    // - keys (N * 4 bytes): 削除するキーID

    let mut payload = vec![
        0x00,           // version
        layer as u8,    // layers
        0x00, 0x00,     // reserved0
    ];

    // キーを追加（little-endian）
    for key in keys {
        payload.extend_from_slice(&key.to_le_bytes());
    }

    build_ubx_frame(CFG_CLASS, CFG_VALDEL_ID, &payload)
}

/// UBXフレームを構築する
fn build_ubx_frame(class: u8, id: u8, payload: &[u8]) -> Vec<u8> {
    let payload_len = payload.len() as u16;

    let mut frame = vec![UBX_SYNC_1, UBX_SYNC_2, class, id];
    frame.extend_from_slice(&payload_len.to_le_bytes());
    frame.extend_from_slice(payload);

    // チェックサム計算（class〜payloadまで）
    let (ck_a, ck_b) = calculate_checksum(&frame[2..]);
    frame.push(ck_a);
    frame.push(ck_b);

    frame
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    // ===========================================
    // ヘルパー関数
    // ===========================================

    struct UbxFrame<'a> {
        raw: &'a [u8],
    }

    impl<'a> UbxFrame<'a> {
        fn new(raw: &'a [u8]) -> Self {
            Self { raw }
        }

        fn sync1(&self) -> u8 { self.raw[0] }
        fn sync2(&self) -> u8 { self.raw[1] }
        fn class(&self) -> u8 { self.raw[2] }
        fn id(&self) -> u8 { self.raw[3] }
        fn payload_len(&self) -> u16 {
            u16::from_le_bytes([self.raw[4], self.raw[5]])
        }
        fn version(&self) -> u8 { self.raw[6] }
        fn layers(&self) -> u8 { self.raw[7] }

        /// n番目のキーを取得（0-indexed）
        fn nth_key(&self, n: usize) -> u32 {
            let offset = 10 + n * 4;
            u32::from_le_bytes([
                self.raw[offset],
                self.raw[offset + 1],
                self.raw[offset + 2],
                self.raw[offset + 3],
            ])
        }

        fn checksum_valid(&self) -> bool {
            let (expected_a, expected_b) = calculate_checksum(&self.raw[2..self.raw.len()-2]);
            self.raw[self.raw.len()-2] == expected_a && self.raw[self.raw.len()-1] == expected_b
        }

        fn total_len(&self) -> usize { self.raw.len() }
    }

    // ===========================================
    // CFG-VALDELメッセージ構造テスト
    // ===========================================

    /// CFG-VALDELメッセージの基本構造が正しい
    #[rstest]
    #[case::正常系_単一キー(DeleteLayer::Bbr, vec![0x20910009], true)]
    fn test_メッセージ構造が正しい(
        #[case] layer: DeleteLayer,
        #[case] keys: Vec<u32>,
        #[case] should_succeed: bool,
    ) {
        let msg = delete_config_keys(layer, &keys);
        let frame = UbxFrame::new(&msg);

        if should_succeed {
            // UBXヘッダー
            assert_eq!(frame.sync1(), 0xB5, "sync1");
            assert_eq!(frame.sync2(), 0x62, "sync2");
            assert_eq!(frame.class(), 0x06, "class = CFG");
            assert_eq!(frame.id(), 0x8C, "id = VALDEL");
            // version
            assert_eq!(frame.version(), 0x00, "version");
            // チェックサム
            assert!(frame.checksum_valid(), "checksum");
        }
    }

    /// 各レイヤー指定時に正しいレイヤー値が設定される
    #[rstest]
    #[case::bbr(DeleteLayer::Bbr, 0x02, true)]
    #[case::flash(DeleteLayer::Flash, 0x04, true)]
    #[case::bbr_and_flash(DeleteLayer::BbrAndFlash, 0x06, true)]
    fn test_レイヤー指定で正しい値が設定される(
        #[case] layer: DeleteLayer,
        #[case] expected_layer: u8,
        #[case] should_succeed: bool,
    ) {
        let msg = delete_config_keys(layer, &[0x20910009]);
        let frame = UbxFrame::new(&msg);

        if should_succeed {
            assert_eq!(frame.layers(), expected_layer);
        }
    }

    /// 単一キー削除時のペイロード長が正しい
    #[rstest]
    #[case::単一キー(vec![0x20910009], 8, true)]  // version(1) + layers(1) + reserved(2) + key(4) = 8
    #[case::複数キー(vec![0x20910009, 0x20910007], 12, true)]  // + key(4) = 12
    fn test_ペイロード長が正しい(
        #[case] keys: Vec<u32>,
        #[case] expected_len: u16,
        #[case] should_succeed: bool,
    ) {
        let msg = delete_config_keys(DeleteLayer::Bbr, &keys);
        let frame = UbxFrame::new(&msg);

        if should_succeed {
            assert_eq!(frame.payload_len(), expected_len);
        }
    }

    /// キーが正しくエンコードされる
    #[rstest]
    #[case::nav_pvt_usb(0, 0x20910009_u32, true)]
    #[case::nav_pvt_uart1(1, 0x20910007_u32, true)]
    fn test_キーが正しくエンコードされる(
        #[case] index: usize,
        #[case] expected_key: u32,
        #[case] should_succeed: bool,
    ) {
        let keys = vec![0x20910009_u32, 0x20910007_u32];
        let msg = delete_config_keys(DeleteLayer::Bbr, &keys);
        let frame = UbxFrame::new(&msg);

        if should_succeed {
            assert_eq!(frame.nth_key(index), expected_key);
        }
    }

    /// 全体長が正しい
    #[rstest]
    #[case::単一キー(vec![0x20910009], 16, true)]  // sync(2) + class(1) + id(1) + len(2) + payload(8) + checksum(2) = 16
    fn test_全体長が正しい(
        #[case] keys: Vec<u32>,
        #[case] expected_len: usize,
        #[case] should_succeed: bool,
    ) {
        let msg = delete_config_keys(DeleteLayer::Bbr, &keys);
        let frame = UbxFrame::new(&msg);

        if should_succeed {
            assert_eq!(frame.total_len(), expected_len);
        }
    }
}
