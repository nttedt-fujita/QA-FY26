//! CFG-CFG メッセージ生成
//!
//! UBX-CFG-CFG: レイヤー操作（クリア/保存/読み込み）
//! Class: 0x06, ID: 0x09
//!
//! 出典: u-blox F9 HPG 1.32 Interface Description (UBX-22008968) p.64
//!
//! 主な用途:
//! - 設定のクリア（BBR/Flashからの削除）
//! - 設定の保存（RAM → BBR/Flash）
//! - 設定の読み込み（BBR/Flash/ROM → RAM）
//! - デフォルト設定への復元

use super::common::{calculate_checksum, UBX_SYNC_1, UBX_SYNC_2};

/// CFG-CFG メッセージ識別子
const CFG_CLASS: u8 = 0x06;
const CFG_CFG_ID: u8 = 0x09;

/// 設定マスク
/// どの設定グループを対象にするかを指定
///
/// 出典: u-blox F9 HPG 1.32 Interface Description p.65
///
/// プロトコル23.01以降:
/// clearMask/saveMask/loadMaskのいずれかに1ビットでも立っていれば、
/// 全設定が対象になる。個別グループの指定はレガシー互換のためのみ。
#[derive(Debug, Clone, Copy)]
pub struct ConfigMask(pub u32);

impl ConfigMask {
    /// 空（何も指定しない）
    pub const NONE: Self = Self(0x0000_0000);

    /// 全設定を対象
    /// プロトコル23.01以降では、いずれか1ビットでも立っていれば全設定が対象
    pub const ALL: Self = Self(0x0000_FFFF);

    /// ioPort - I/O ports configuration
    pub const IO_PORT: Self = Self(0x0000_0001);
    /// msgConf - Message configuration
    pub const MSG_CONF: Self = Self(0x0000_0002);
    /// infMsg - INF message configuration
    pub const INF_MSG: Self = Self(0x0000_0004);
    /// navConf - Navigation configuration
    pub const NAV_CONF: Self = Self(0x0000_0008);
    /// rxmConf - RXM configuration
    pub const RXM_CONF: Self = Self(0x0000_0010);
    /// senConf - Sensor interface configuration
    pub const SEN_CONF: Self = Self(0x0000_0100);
    /// rinvConf - Remote inventory configuration
    pub const RINV_CONF: Self = Self(0x0000_0200);
    /// antConf - Antenna configuration
    pub const ANT_CONF: Self = Self(0x0000_0400);
    /// logConf - Logging configuration
    pub const LOG_CONF: Self = Self(0x0000_0800);
    /// ftsConf - FTS configuration
    pub const FTS_CONF: Self = Self(0x0000_1000);
}

/// デバイスマスク
/// どのメモリレイヤーを対象にするかを指定
///
/// 出典: u-blox F9 HPG 1.32 Interface Description p.65
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum DeviceMask {
    /// BBR（Battery-Backed RAM）
    Bbr = 0x01,
    /// Flash
    Flash = 0x02,
    /// BBR + Flash
    BbrAndFlash = 0x03,
}

/// BBR/Flashをクリアしてデフォルト設定に復元するCFG-CFGメッセージを生成
///
/// 動作:
/// 1. clearMask: BBR + Flashの全設定をクリア
/// 2. loadMask: ROM（デフォルト）からRAMに読み込み
///
/// # Returns
/// 完全なUBXフレーム（ヘッダー + ペイロード + チェックサム）
///
/// # 注意
/// - この操作後、レシーバーは工場出荷時の設定で動作する
/// - RTK基準局設定など、BBR/Flashに保存された設定は全て消える
///
/// # 仕様書参照
/// - deviceMaskはclearMask/saveMaskにのみ適用される（loadMaskには適用されない）
/// - loadMaskは残っている下位レイヤー（Flash→BBR→ROM）から再構築
/// - 出典: u-blox F9 HPG 1.32 Interface Description p.64
pub fn reset_config_to_default() -> Vec<u8> {
    // ペイロード構成（13バイト版）:
    // - clearMask (4 bytes, X4): クリアする設定
    // - saveMask (4 bytes, X4): 保存する設定
    // - loadMask (4 bytes, X4): ロードする設定
    // - deviceMask (1 byte, X1): 対象メモリ（clearMask/saveMaskにのみ適用）

    let clear_mask = ConfigMask::ALL.0;
    let save_mask = ConfigMask::NONE.0;
    // loadMaskは使わない（ボーレートがROMデフォルトに戻ってしまうため）
    // 設定はBBR/Flashからクリアされ、次回電源投入時にROMデフォルトが適用される
    let load_mask = ConfigMask::NONE.0;
    // BBR + Flash両方をクリアする（Flashに設定が保存されている場合に対応）
    let device_mask = DeviceMask::BbrAndFlash as u8;

    let mut payload = Vec::with_capacity(13);
    payload.extend_from_slice(&clear_mask.to_le_bytes());
    payload.extend_from_slice(&save_mask.to_le_bytes());
    payload.extend_from_slice(&load_mask.to_le_bytes());
    payload.push(device_mask);

    build_ubx_frame(CFG_CLASS, CFG_CFG_ID, &payload)
}

/// 現在の設定をBBRに保存するCFG-CFGメッセージを生成
///
/// 動作:
/// - saveMask: RAMの全設定をBBRに保存
///
/// # Returns
/// 完全なUBXフレーム（ヘッダー + ペイロード + チェックサム）
pub fn save_config_to_bbr() -> Vec<u8> {
    let clear_mask = ConfigMask::NONE.0;
    let save_mask = ConfigMask::ALL.0;
    let load_mask = ConfigMask::NONE.0;
    let device_mask = DeviceMask::Bbr as u8;

    let mut payload = Vec::with_capacity(13);
    payload.extend_from_slice(&clear_mask.to_le_bytes());
    payload.extend_from_slice(&save_mask.to_le_bytes());
    payload.extend_from_slice(&load_mask.to_le_bytes());
    payload.push(device_mask);

    build_ubx_frame(CFG_CLASS, CFG_CFG_ID, &payload)
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

    /// UBXフレームから各フィールドを抽出
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

        /// CFG-CFGのclearMaskを取得
        fn clear_mask(&self) -> u32 {
            u32::from_le_bytes([self.raw[6], self.raw[7], self.raw[8], self.raw[9]])
        }

        /// CFG-CFGのsaveMaskを取得
        fn save_mask(&self) -> u32 {
            u32::from_le_bytes([self.raw[10], self.raw[11], self.raw[12], self.raw[13]])
        }

        /// CFG-CFGのloadMaskを取得
        fn load_mask(&self) -> u32 {
            u32::from_le_bytes([self.raw[14], self.raw[15], self.raw[16], self.raw[17]])
        }

        /// CFG-CFGのdeviceMaskを取得
        fn device_mask(&self) -> u8 { self.raw[18] }

        /// チェックサムが正しいか検証
        fn checksum_valid(&self) -> bool {
            let (expected_a, expected_b) = calculate_checksum(&self.raw[2..self.raw.len()-2]);
            self.raw[self.raw.len()-2] == expected_a && self.raw[self.raw.len()-1] == expected_b
        }

        fn total_len(&self) -> usize { self.raw.len() }
    }

    // ===========================================
    // reset_config_to_default テスト
    // ===========================================

    /// デフォルト復元メッセージの構造が正しい
    #[rstest]
    #[case::正常系(true)]
    fn test_reset_config_メッセージ構造が正しい(
        #[case] should_succeed: bool,
    ) {
        let msg = reset_config_to_default();
        let frame = UbxFrame::new(&msg);

        if should_succeed {
            // UBXヘッダー
            assert_eq!(frame.sync1(), 0xB5, "sync1");
            assert_eq!(frame.sync2(), 0x62, "sync2");
            assert_eq!(frame.class(), 0x06, "class = CFG");
            assert_eq!(frame.id(), 0x09, "id = CFG");

            // ペイロード長: clearMask(4) + saveMask(4) + loadMask(4) + deviceMask(1) = 13
            assert_eq!(frame.payload_len(), 13, "payload length");

            // 全体長: sync(2) + class(1) + id(1) + len(2) + payload(13) + checksum(2) = 21
            assert_eq!(frame.total_len(), 21, "total length");

            // チェックサム
            assert!(frame.checksum_valid(), "checksum");
        }
    }

    /// デフォルト復元時のマスク値が正しい
    #[rstest]
    #[case::正常系(true)]
    fn test_reset_config_マスク値が正しい(
        #[case] should_succeed: bool,
    ) {
        let msg = reset_config_to_default();
        let frame = UbxFrame::new(&msg);

        if should_succeed {
            // clearMask = 0xFFFF（全設定をクリア）
            assert_eq!(frame.clear_mask(), 0x0000_FFFF, "clearMask = ALL");
            // saveMask = 0（保存しない）
            assert_eq!(frame.save_mask(), 0x0000_0000, "saveMask = NONE");
            // loadMask = 0（ロードしない = ボーレート維持）
            assert_eq!(frame.load_mask(), 0x0000_0000, "loadMask = NONE");
            // deviceMask = 0x03（BBR + Flash）
            // Flashに保存された設定もクリアするため、両方を指定
            assert_eq!(frame.device_mask(), 0x03, "deviceMask = BBR + Flash");
        }
    }

    // ===========================================
    // save_config_to_bbr テスト
    // ===========================================

    /// BBR保存メッセージの構造が正しい
    #[rstest]
    #[case::正常系(true)]
    fn test_save_config_メッセージ構造が正しい(
        #[case] should_succeed: bool,
    ) {
        let msg = save_config_to_bbr();
        let frame = UbxFrame::new(&msg);

        if should_succeed {
            // UBXヘッダー
            assert_eq!(frame.sync1(), 0xB5, "sync1");
            assert_eq!(frame.sync2(), 0x62, "sync2");
            assert_eq!(frame.class(), 0x06, "class = CFG");
            assert_eq!(frame.id(), 0x09, "id = CFG");

            // ペイロード長: 13
            assert_eq!(frame.payload_len(), 13, "payload length");

            // チェックサム
            assert!(frame.checksum_valid(), "checksum");
        }
    }

    /// BBR保存時のマスク値が正しい
    #[rstest]
    #[case::正常系(true)]
    fn test_save_config_マスク値が正しい(
        #[case] should_succeed: bool,
    ) {
        let msg = save_config_to_bbr();
        let frame = UbxFrame::new(&msg);

        if should_succeed {
            // clearMask = 0（クリアしない）
            assert_eq!(frame.clear_mask(), 0x0000_0000, "clearMask = NONE");
            // saveMask = 0xFFFF（全設定を保存）
            assert_eq!(frame.save_mask(), 0x0000_FFFF, "saveMask = ALL");
            // loadMask = 0（読み込まない）
            assert_eq!(frame.load_mask(), 0x0000_0000, "loadMask = NONE");
            // deviceMask = 0x01（BBR）
            assert_eq!(frame.device_mask(), 0x01, "deviceMask = BBR");
        }
    }
}
