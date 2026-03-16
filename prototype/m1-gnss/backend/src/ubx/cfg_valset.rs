//! CFG-VALSET メッセージ生成
//!
//! UBX-CFG-VALSET: 設定値の変更
//! Class: 0x06, ID: 0x8A
//!
//! 主な用途:
//! - NMEA出力のON/OFF制御
//! - 定期出力（Periodic Output）の設定

use super::common::{calculate_checksum, UBX_SYNC_1, UBX_SYNC_2};

/// CFG-VALSET メッセージ識別子
const CFG_CLASS: u8 = 0x06;
const CFG_VALSET_ID: u8 = 0x8A;

/// 設定レイヤー
///
/// u-blox F9P の設定レイヤーは以下の通り:
/// - RAM: 即座に有効。電源OFFで消える
/// - BBR: 電源再投入時に有効になる。バッテリバックアップで保持
/// - Flash: 完全不揮発性
///
/// **重要**: BBRのみへの書き込みは電源再投入まで有効にならない。
/// 即座に有効にしたい場合は `RamAndBbr` を使用すること。
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Layer {
    /// RAM（揮発性、電源OFFで消える）- 即座に有効
    Ram = 0x01,
    /// BBR（バッテリバックアップRAM）- 電源再投入時に有効
    Bbr = 0x02,
    /// Flash（不揮発性）
    Flash = 0x04,
    /// RAM + BBR（即座に有効 + 再起動後も有効）
    /// Session 151: BBRのみでは即座に有効にならないため追加
    RamAndBbr = 0x03,
}

/// CFG-UART1OUTPROT-NMEA のキーID
/// NMEA出力プロトコルのON/OFF
const CFG_UART1OUTPROT_NMEA: u32 = 0x10740002;

/// CFG-UART1-BAUDRATE のキーID
/// UART1のボーレート設定（U4型: 4バイト）
/// 出典: u-blox F9 HPG 1.32 Interface Description (UBX-22008968) p.270
pub const CFG_UART1_BAUDRATE: u32 = 0x40520001;

// ===========================================
// CFG-MSGOUT キーID（USB用）
// 出典: u-blox F9 HPG 1.32 Interface Description (UBX-22008968) p.234-251
// ===========================================

/// CFG-MSGOUT-UBX_NAV_PVT_USB
pub const CFG_MSGOUT_NAV_PVT_USB: u32 = 0x20910009;

/// CFG-MSGOUT-UBX_NAV_STATUS_USB
pub const CFG_MSGOUT_NAV_STATUS_USB: u32 = 0x2091001d;

/// CFG-MSGOUT-UBX_NAV_SAT_USB
pub const CFG_MSGOUT_NAV_SAT_USB: u32 = 0x20910018;

/// CFG-MSGOUT-UBX_NAV_SIG_USB
pub const CFG_MSGOUT_NAV_SIG_USB: u32 = 0x20910348;

/// CFG-MSGOUT-UBX_MON_SPAN_USB
pub const CFG_MSGOUT_MON_SPAN_USB: u32 = 0x2091038e;

/// CFG-MSGOUT-UBX_MON_RF_USB
pub const CFG_MSGOUT_MON_RF_USB: u32 = 0x2091035c;

// ===========================================
// CFG-MSGOUT キーID（UART1用）
// 出典: u-blox F9 HPG 1.32 Interface Description (UBX-22008968) p.234-251
// 実機はUSBハブ経由でUART1として接続されるため、UART1用キーが必要
// ===========================================

/// CFG-MSGOUT-UBX_NAV_PVT_UART1
pub const CFG_MSGOUT_NAV_PVT_UART1: u32 = 0x20910007;

/// CFG-MSGOUT-UBX_NAV_STATUS_UART1
pub const CFG_MSGOUT_NAV_STATUS_UART1: u32 = 0x2091001b;

/// CFG-MSGOUT-UBX_NAV_SAT_UART1
pub const CFG_MSGOUT_NAV_SAT_UART1: u32 = 0x20910016;

/// CFG-MSGOUT-UBX_NAV_SIG_UART1
pub const CFG_MSGOUT_NAV_SIG_UART1: u32 = 0x20910346;

/// CFG-MSGOUT-UBX_MON_SPAN_UART1
pub const CFG_MSGOUT_MON_SPAN_UART1: u32 = 0x2091038c;

/// CFG-MSGOUT-UBX_MON_RF_UART1
pub const CFG_MSGOUT_MON_RF_UART1: u32 = 0x2091035a;

// ===========================================
// CFG-MSGOUT キーID（追加メッセージ）
// Session 199で発見: USB1から流れていた追加メッセージを無効化するため追加
// Session 200で仕様書から正確なKey IDを確認
// 出典: u-blox F9 HPG 1.32 Interface Description (UBX-22008968) p.248-252
// ===========================================

/// CFG-MSGOUT-UBX_NAV_CLOCK_USB
pub const CFG_MSGOUT_NAV_CLOCK_USB: u32 = 0x20910068;

/// CFG-MSGOUT-UBX_NAV_CLOCK_UART1
pub const CFG_MSGOUT_NAV_CLOCK_UART1: u32 = 0x20910066;

/// CFG-MSGOUT-UBX_NAV_POSLLH_USB
pub const CFG_MSGOUT_NAV_POSLLH_USB: u32 = 0x2091002c;

/// CFG-MSGOUT-UBX_NAV_POSLLH_UART1
pub const CFG_MSGOUT_NAV_POSLLH_UART1: u32 = 0x2091002a;

/// CFG-MSGOUT-UBX_NAV_HPPOSECEF_USB
pub const CFG_MSGOUT_NAV_HPPOSECEF_USB: u32 = 0x20910031;

/// CFG-MSGOUT-UBX_NAV_HPPOSECEF_UART1
pub const CFG_MSGOUT_NAV_HPPOSECEF_UART1: u32 = 0x2091002f;

/// CFG-MSGOUT-UBX_NAV_TIMEGPS_USB
pub const CFG_MSGOUT_NAV_TIMEGPS_USB: u32 = 0x2091004a;

/// CFG-MSGOUT-UBX_NAV_TIMEGPS_UART1
pub const CFG_MSGOUT_NAV_TIMEGPS_UART1: u32 = 0x20910048;

/// CFG-MSGOUT-UBX_NAV_SBAS_USB
pub const CFG_MSGOUT_NAV_SBAS_USB: u32 = 0x2091006d;

/// CFG-MSGOUT-UBX_NAV_SBAS_UART1
pub const CFG_MSGOUT_NAV_SBAS_UART1: u32 = 0x2091006b;

/// CFG-MSGOUT-UBX_NAV_TIMEQZSS_USB
/// Session 200で特定: 0x01 0x27 はNAV-TIMEGPS(0x01 0x20)ではなくNAV-TIMEQZSS
pub const CFG_MSGOUT_NAV_TIMEQZSS_USB: u32 = 0x20910389;

/// CFG-MSGOUT-UBX_NAV_TIMEQZSS_UART1
pub const CFG_MSGOUT_NAV_TIMEQZSS_UART1: u32 = 0x20910387;

/// NMEA出力を制御するCFG-VALSETメッセージを生成
///
/// # Arguments
/// * `enable` - true: NMEA出力ON, false: NMEA出力OFF
/// * `layer` - 設定レイヤー（RAM/BBR/Flash）
///
/// # Returns
/// 完全なUBXフレーム（ヘッダー + ペイロード + チェックサム）
pub fn set_uart1_nmea_output(enable: bool, layer: Layer) -> Vec<u8> {
    // ペイロード構成:
    // - version (1 byte): 0x00
    // - layers (1 byte): レイヤービットマスク
    // - reserved (2 bytes): 0x00, 0x00
    // - cfgData (N bytes): キー(4bytes) + 値(1byte for boolean)

    let value: u8 = if enable { 1 } else { 0 };

    // ペイロード
    let mut payload = vec![
        0x00,           // version
        layer as u8,    // layers
        0x00, 0x00,     // reserved
    ];

    // キー (little-endian)
    payload.extend_from_slice(&CFG_UART1OUTPROT_NMEA.to_le_bytes());

    // 値 (1 byte for L type)
    payload.push(value);

    // フレーム構築
    build_ubx_frame(CFG_CLASS, CFG_VALSET_ID, &payload)
}

/// UART1のボーレートを変更するCFG-VALSETメッセージを生成
///
/// # Arguments
/// * `baud_rate` - ボーレート（例: 115200, 38400, 9600）
/// * `layer` - 設定レイヤー（RAM/BBR/Flash）
///
/// # Returns
/// 完全なUBXフレーム（ヘッダー + ペイロード + チェックサム）
///
/// # 注意
/// - RAMに書き込むと即座に反映される
/// - ホスト側も同じボーレートに変更してから再接続が必要
pub fn set_uart1_baudrate(baud_rate: u32, layer: Layer) -> Vec<u8> {
    // ペイロード構成:
    // - version (1 byte): 0x00
    // - layers (1 byte): レイヤービットマスク
    // - reserved (2 bytes): 0x00, 0x00
    // - cfgData (N bytes): キー(4bytes) + 値(4bytes for U4)

    let mut payload = vec![
        0x00,           // version
        layer as u8,    // layers
        0x00, 0x00,     // reserved
    ];

    // キー (little-endian)
    payload.extend_from_slice(&CFG_UART1_BAUDRATE.to_le_bytes());

    // 値 (4 bytes for U4 type)
    payload.extend_from_slice(&baud_rate.to_le_bytes());

    // フレーム構築
    build_ubx_frame(CFG_CLASS, CFG_VALSET_ID, &payload)
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

// ===========================================
// 定期出力（Periodic Output）設定
// ===========================================

/// 定期出力設定
/// 出力レート（エポックごと）
#[derive(Debug, Clone, Copy)]
pub struct PeriodicOutputConfig {
    /// NAV-PVT出力レート（0=無効, 1=毎エポック）
    pub nav_pvt: u8,
    /// NAV-STATUS出力レート
    pub nav_status: u8,
    /// NAV-SAT出力レート
    pub nav_sat: u8,
    /// NAV-SIG出力レート
    pub nav_sig: u8,
    /// MON-SPAN出力レート
    pub mon_span: u8,
    /// MON-RF出力レート
    pub mon_rf: u8,
}

impl Default for PeriodicOutputConfig {
    fn default() -> Self {
        Self {
            nav_pvt: 1,      // 毎エポック
            nav_status: 1,   // 毎エポック
            nav_sat: 5,      // 5エポックに1回
            nav_sig: 5,      // 5エポックに1回
            mon_span: 10,    // 10エポックに1回
            mon_rf: 10,      // 10エポックに1回
        }
    }
}

/// 定期出力を有効化するCFG-VALSETメッセージを生成
///
/// # Arguments
/// * `config` - 定期出力設定（各メッセージの出力レート）
/// * `layer` - 設定レイヤー（RAM/BBR/Flash）
///
/// # Returns
/// 完全なUBXフレーム（ヘッダー + ペイロード + チェックサム）
pub fn set_periodic_output(config: &PeriodicOutputConfig, layer: Layer) -> Vec<u8> {
    // ペイロード構成:
    // - version (1 byte): 0x00
    // - layers (1 byte): レイヤービットマスク
    // - reserved (2 bytes): 0x00, 0x00
    // - cfgData (N bytes): 各キー(4bytes) + 値(1byte)

    let mut payload = vec![
        0x00,           // version
        layer as u8,    // layers
        0x00, 0x00,     // reserved
    ];

    // 各メッセージの設定を追加
    let configs: [(u32, u8); 6] = [
        (CFG_MSGOUT_NAV_PVT_USB, config.nav_pvt),
        (CFG_MSGOUT_NAV_STATUS_USB, config.nav_status),
        (CFG_MSGOUT_NAV_SAT_USB, config.nav_sat),
        (CFG_MSGOUT_NAV_SIG_USB, config.nav_sig),
        (CFG_MSGOUT_MON_SPAN_USB, config.mon_span),
        (CFG_MSGOUT_MON_RF_USB, config.mon_rf),
    ];

    for (key, value) in configs {
        payload.extend_from_slice(&key.to_le_bytes());
        payload.push(value);
    }

    build_ubx_frame(CFG_CLASS, CFG_VALSET_ID, &payload)
}

/// 定期出力を無効化するCFG-VALSETメッセージを生成
///
/// 全てのメッセージを出力レート0（ポーリングのみ）に設定
/// USB用とUART1用の両方のキーを設定（実機はUART1接続のため）
pub fn disable_periodic_output(layer: Layer) -> Vec<u8> {
    // ペイロード構成:
    // - version (1 byte): 0x00
    // - layers (1 byte): レイヤービットマスク
    // - reserved (2 bytes): 0x00, 0x00
    // - cfgData (N bytes): 各キー(4bytes) + 値(1byte)

    let mut payload = vec![
        0x00,           // version
        layer as u8,    // layers
        0x00, 0x00,     // reserved
    ];

    // USB用とUART1用の両方のキーを0に設定
    // Session 199-200: 追加メッセージ（NAV-CLOCK, NAV-POSLLH, NAV-HPPOSECEF, NAV-TIMEGPS, NAV-TIMEQZSS, NAV-SBAS）を追加
    // Session 201: NAV-TIMEQZSS追加（合計24キー）
    let configs: [(u32, u8); 24] = [
        // USB用（元の6メッセージ）
        (CFG_MSGOUT_NAV_PVT_USB, 0),
        (CFG_MSGOUT_NAV_STATUS_USB, 0),
        (CFG_MSGOUT_NAV_SAT_USB, 0),
        (CFG_MSGOUT_NAV_SIG_USB, 0),
        (CFG_MSGOUT_MON_SPAN_USB, 0),
        (CFG_MSGOUT_MON_RF_USB, 0),
        // USB用（追加6メッセージ）
        (CFG_MSGOUT_NAV_CLOCK_USB, 0),
        (CFG_MSGOUT_NAV_POSLLH_USB, 0),
        (CFG_MSGOUT_NAV_HPPOSECEF_USB, 0),
        (CFG_MSGOUT_NAV_TIMEGPS_USB, 0),
        (CFG_MSGOUT_NAV_TIMEQZSS_USB, 0),
        (CFG_MSGOUT_NAV_SBAS_USB, 0),
        // UART1用（元の6メッセージ）
        (CFG_MSGOUT_NAV_PVT_UART1, 0),
        (CFG_MSGOUT_NAV_STATUS_UART1, 0),
        (CFG_MSGOUT_NAV_SAT_UART1, 0),
        (CFG_MSGOUT_NAV_SIG_UART1, 0),
        (CFG_MSGOUT_MON_SPAN_UART1, 0),
        (CFG_MSGOUT_MON_RF_UART1, 0),
        // UART1用（追加6メッセージ）
        (CFG_MSGOUT_NAV_CLOCK_UART1, 0),
        (CFG_MSGOUT_NAV_POSLLH_UART1, 0),
        (CFG_MSGOUT_NAV_HPPOSECEF_UART1, 0),
        (CFG_MSGOUT_NAV_TIMEGPS_UART1, 0),
        (CFG_MSGOUT_NAV_TIMEQZSS_UART1, 0),
        (CFG_MSGOUT_NAV_SBAS_UART1, 0),
    ];

    for (key, value) in configs {
        payload.extend_from_slice(&key.to_le_bytes());
        payload.push(value);
    }

    build_ubx_frame(CFG_CLASS, CFG_VALSET_ID, &payload)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    // ===========================================
    // ヘルパー関数: オフセット直接アクセスを避けるため
    // ===========================================

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
        fn version(&self) -> u8 { self.raw[6] }
        fn layers(&self) -> u8 { self.raw[7] }

        /// CFG-VALSETの最初のキーを取得
        fn first_key(&self) -> u32 {
            u32::from_le_bytes([self.raw[10], self.raw[11], self.raw[12], self.raw[13]])
        }

        /// CFG-VALSETの最初の値を取得
        fn first_value(&self) -> u8 { self.raw[14] }

        /// n番目の設定値を取得（0-indexed, 定期出力用）
        fn nth_value(&self, n: usize) -> u8 {
            let offset = 14 + n * 5;
            self.raw[offset]
        }

        /// チェックサムが正しいか検証
        fn checksum_valid(&self) -> bool {
            let (expected_a, expected_b) = calculate_checksum(&self.raw[2..self.raw.len()-2]);
            self.raw[self.raw.len()-2] == expected_a && self.raw[self.raw.len()-1] == expected_b
        }

        fn total_len(&self) -> usize { self.raw.len() }
    }

    // ===========================================
    // NMEA出力設定テスト
    // ===========================================

    /// NMEA出力ON/OFF設定時に正しい値が設定される
    #[rstest]
    #[case::nmea_off(false, 0x00, true)]
    #[case::nmea_on(true, 0x01, true)]
    fn test_nmea出力設定で正しい値が設定される(
        #[case] enable: bool,
        #[case] expected_value: u8,
        #[case] should_succeed: bool,
    ) {
        let msg = set_uart1_nmea_output(enable, Layer::Ram);
        let frame = UbxFrame::new(&msg);

        if should_succeed {
            // 振る舞い: 指定した値が設定される
            assert_eq!(frame.first_value(), expected_value);
            // 振る舞い: 正しいキーが使われる
            assert_eq!(frame.first_key(), CFG_UART1OUTPROT_NMEA);
        }
    }

    /// 各レイヤー指定時に正しいレイヤー値が設定される
    #[rstest]
    #[case::ram(Layer::Ram, 0x01, true)]
    #[case::bbr(Layer::Bbr, 0x02, true)]
    #[case::flash(Layer::Flash, 0x04, true)]
    fn test_レイヤー指定で正しい値が設定される(
        #[case] layer: Layer,
        #[case] expected_layer: u8,
        #[case] should_succeed: bool,
    ) {
        let msg = set_uart1_nmea_output(false, layer);
        let frame = UbxFrame::new(&msg);

        if should_succeed {
            assert_eq!(frame.layers(), expected_layer);
        }
    }

    /// NMEA出力設定メッセージの構造が正しい
    #[rstest]
    #[case::正常系(false, Layer::Ram, true)]
    fn test_nmea出力メッセージ構造が正しい(
        #[case] enable: bool,
        #[case] layer: Layer,
        #[case] should_succeed: bool,
    ) {
        let msg = set_uart1_nmea_output(enable, layer);
        let frame = UbxFrame::new(&msg);

        if should_succeed {
            // UBXヘッダー
            assert_eq!(frame.sync1(), 0xB5, "sync1");
            assert_eq!(frame.sync2(), 0x62, "sync2");
            assert_eq!(frame.class(), 0x06, "class = CFG");
            assert_eq!(frame.id(), 0x8A, "id = VALSET");
            // ペイロード長: version(1) + layers(1) + reserved(2) + key(4) + value(1) = 9
            assert_eq!(frame.payload_len(), 9, "payload length");
            // CFG-VALSET固定フィールド
            assert_eq!(frame.version(), 0x00, "version");
            // 全体長: sync(2) + class(1) + id(1) + len(2) + payload(9) + checksum(2) = 17
            assert_eq!(frame.total_len(), 17, "total length");
            // チェックサム
            assert!(frame.checksum_valid(), "checksum");
        }
    }

    // ===========================================
    // 定期出力設定テスト
    // ===========================================

    /// 定期出力設定時に各メッセージのレートが正しく設定される
    #[rstest]
    #[case::nav_pvt(0, 1, true)]      // NAV-PVT: 毎エポック
    #[case::nav_status(1, 1, true)]   // NAV-STATUS: 毎エポック
    #[case::nav_sat(2, 5, true)]      // NAV-SAT: 5エポックに1回
    #[case::nav_sig(3, 5, true)]      // NAV-SIG: 5エポックに1回
    #[case::mon_span(4, 10, true)]    // MON-SPAN: 10エポックに1回
    #[case::mon_rf(5, 10, true)]      // MON-RF: 10エポックに1回
    fn test_定期出力デフォルト設定で正しいレートが設定される(
        #[case] index: usize,
        #[case] expected_rate: u8,
        #[case] should_succeed: bool,
    ) {
        let config = PeriodicOutputConfig::default();
        let msg = set_periodic_output(&config, Layer::Ram);
        let frame = UbxFrame::new(&msg);

        if should_succeed {
            assert_eq!(frame.nth_value(index), expected_rate);
        }
    }

    /// 定期出力無効化時に全メッセージのレートが0になる（USB + UART1の24キー）
    #[rstest]
    // USB用（index 0-11）
    #[case::usb_nav_pvt(0, 0, true)]
    #[case::usb_nav_status(1, 0, true)]
    #[case::usb_nav_sat(2, 0, true)]
    #[case::usb_nav_sig(3, 0, true)]
    #[case::usb_mon_span(4, 0, true)]
    #[case::usb_mon_rf(5, 0, true)]
    #[case::usb_nav_clock(6, 0, true)]
    #[case::usb_nav_posllh(7, 0, true)]
    #[case::usb_nav_hpposecef(8, 0, true)]
    #[case::usb_nav_timegps(9, 0, true)]
    #[case::usb_nav_timeqzss(10, 0, true)]
    #[case::usb_nav_sbas(11, 0, true)]
    // UART1用（index 12-23）
    #[case::uart1_nav_pvt(12, 0, true)]
    #[case::uart1_nav_status(13, 0, true)]
    #[case::uart1_nav_sat(14, 0, true)]
    #[case::uart1_nav_sig(15, 0, true)]
    #[case::uart1_mon_span(16, 0, true)]
    #[case::uart1_mon_rf(17, 0, true)]
    #[case::uart1_nav_clock(18, 0, true)]
    #[case::uart1_nav_posllh(19, 0, true)]
    #[case::uart1_nav_hpposecef(20, 0, true)]
    #[case::uart1_nav_timegps(21, 0, true)]
    #[case::uart1_nav_timeqzss(22, 0, true)]
    #[case::uart1_nav_sbas(23, 0, true)]
    fn test_定期出力無効化で全レートが0になる(
        #[case] index: usize,
        #[case] expected_rate: u8,
        #[case] should_succeed: bool,
    ) {
        let msg = disable_periodic_output(Layer::Ram);
        let frame = UbxFrame::new(&msg);

        if should_succeed {
            assert_eq!(frame.nth_value(index), expected_rate);
        }
    }

    /// 定期出力設定メッセージの構造が正しい
    #[rstest]
    #[case::正常系(Layer::Ram, true)]
    fn test_定期出力メッセージ構造が正しい(
        #[case] layer: Layer,
        #[case] should_succeed: bool,
    ) {
        let config = PeriodicOutputConfig::default();
        let msg = set_periodic_output(&config, layer);
        let frame = UbxFrame::new(&msg);

        if should_succeed {
            // UBXヘッダー
            assert_eq!(frame.sync1(), 0xB5, "sync1");
            assert_eq!(frame.sync2(), 0x62, "sync2");
            assert_eq!(frame.class(), 0x06, "class = CFG");
            assert_eq!(frame.id(), 0x8A, "id = VALSET");
            // ペイロード長: version(1) + layers(1) + reserved(2) + 6*(key(4) + value(1)) = 34
            assert_eq!(frame.payload_len(), 34, "payload length");
            // 最初のキーがNAV-PVT
            assert_eq!(frame.first_key(), CFG_MSGOUT_NAV_PVT_USB, "first key = NAV-PVT");
            // チェックサム
            assert!(frame.checksum_valid(), "checksum");
        }
    }

    /// 定期出力無効化メッセージの構造が正しい（USB + UART1の24キー）
    #[rstest]
    #[case::正常系(Layer::Ram, true)]
    fn test_定期出力無効化メッセージ構造が正しい(
        #[case] layer: Layer,
        #[case] should_succeed: bool,
    ) {
        let msg = disable_periodic_output(layer);
        let frame = UbxFrame::new(&msg);

        if should_succeed {
            // UBXヘッダー
            assert_eq!(frame.sync1(), 0xB5, "sync1");
            assert_eq!(frame.sync2(), 0x62, "sync2");
            assert_eq!(frame.class(), 0x06, "class = CFG");
            assert_eq!(frame.id(), 0x8A, "id = VALSET");
            // ペイロード長: version(1) + layers(1) + reserved(2) + 24*(key(4) + value(1)) = 124
            assert_eq!(frame.payload_len(), 124, "payload length (24 keys)");
            // 最初のキーがNAV-PVT USB
            assert_eq!(frame.first_key(), CFG_MSGOUT_NAV_PVT_USB, "first key = NAV-PVT USB");
            // チェックサム
            assert!(frame.checksum_valid(), "checksum");
        }
    }
}
