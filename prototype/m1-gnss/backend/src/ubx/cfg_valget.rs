//! CFG-VALGET メッセージ生成・パース
//!
//! UBX-CFG-VALGET: 設定値の取得
//! Class: 0x06, ID: 0x8B
//!
//! 主な用途:
//! - Flashレイヤーから設定値を読み取り、Flash搭載確認

use super::common::{calculate_checksum, UBX_SYNC_1, UBX_SYNC_2};

/// CFG-VALGET メッセージ識別子
const CFG_CLASS: u8 = 0x06;
const CFG_VALGET_ID: u8 = 0x8B;

/// 読み取り対象レイヤー（CFG-VALGET用）
///
/// CFG-VALSETとは異なり、単一レイヤーを指定する（ビットフィールドではない）
/// 出典: u-blox F9 HPG 1.32 Interface Description (UBX-22008968) p.95
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum ReadLayer {
    /// RAM（現在有効な設定）
    Ram = 0,
    /// BBR（バッテリバックアップRAM）
    Bbr = 1,
    /// Flash（不揮発性）
    Flash = 2,
    /// Default（ROMデフォルト）
    Default = 7,
}

impl std::fmt::Display for ReadLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReadLayer::Ram => write!(f, "RAM"),
            ReadLayer::Bbr => write!(f, "BBR"),
            ReadLayer::Flash => write!(f, "Flash"),
            ReadLayer::Default => write!(f, "Default"),
        }
    }
}

impl TryFrom<u8> for ReadLayer {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ReadLayer::Ram),
            1 => Ok(ReadLayer::Bbr),
            2 => Ok(ReadLayer::Flash),
            7 => Ok(ReadLayer::Default),
            _ => Err(format!("Invalid layer: {}", value)),
        }
    }
}

/// レイヤー名文字列からReadLayerに変換
pub fn parse_layer_name(name: &str) -> Result<ReadLayer, String> {
    match name.to_lowercase().as_str() {
        "ram" => Ok(ReadLayer::Ram),
        "bbr" => Ok(ReadLayer::Bbr),
        "flash" => Ok(ReadLayer::Flash),
        "default" => Ok(ReadLayer::Default),
        _ => Err(format!("Invalid layer name: {}. Valid values: ram, bbr, flash, default", name)),
    }
}

/// CFG-VALGET Pollメッセージを生成
///
/// # Arguments
/// * `layer` - 読み取り対象レイヤー
/// * `keys` - 取得したい設定キーのリスト（最大64個）
///
/// # Returns
/// 完全なUBXフレーム（ヘッダー + ペイロード + チェックサム）
pub fn build_cfg_valget_poll(layer: ReadLayer, keys: &[u32]) -> Vec<u8> {
    // ペイロード構成（出典: IF p.95）:
    // - version (1 byte): 0x00
    // - layer (1 byte): 読み取りレイヤー
    // - position (2 bytes): スキップするキー数（通常0）
    // - keys (4 bytes x N): 取得したいキーID

    let mut payload = vec![
        0x00,               // version
        layer as u8,        // layer
        0x00, 0x00,         // position (skip 0)
    ];

    for key in keys {
        payload.extend_from_slice(&key.to_le_bytes());
    }

    build_ubx_frame(CFG_CLASS, CFG_VALGET_ID, &payload)
}

/// CFG-VALGETレスポンスの解析結果
#[derive(Debug, Clone)]
pub struct CfgValgetResponse {
    /// レスポンスのレイヤー
    pub layer: ReadLayer,
    /// キーと値のペア
    pub values: Vec<(u32, Vec<u8>)>,
}

/// CFG-VALGETレスポンスをパース
///
/// # Arguments
/// * `payload` - UBXペイロード（ヘッダー・チェックサムを除く）
///
/// # Returns
/// パース結果（レイヤーとキー・値ペアのリスト）
pub fn parse_cfg_valget_response(payload: &[u8]) -> Result<CfgValgetResponse, String> {
    // 最小ペイロード長: version(1) + layer(1) + position(2) = 4
    if payload.len() < 4 {
        return Err(format!("Payload too short: {} bytes", payload.len()));
    }

    let version = payload[0];
    if version != 0x01 {
        return Err(format!("Unexpected version: {}", version));
    }

    let layer = ReadLayer::try_from(payload[1])?;
    let _position = u16::from_le_bytes([payload[2], payload[3]]);

    // cfgDataのパース（offset 4から）
    // キーと値のペア。キーのサイズ部分から値のサイズを判定する。
    // キーのbit 28-30がサイズ情報を含む（出典: IF p.207）:
    // - 0x01: L (1 byte)
    // - 0x02: U1 (1 byte)
    // - 0x03: I2/U2 (2 bytes)
    // - 0x04: I4/U4 (4 bytes)
    // - 0x05: X8/R8/U8/I8 (8 bytes)
    let mut values = Vec::new();
    let mut offset = 4usize;

    while offset + 4 <= payload.len() {
        let key = u32::from_le_bytes([
            payload[offset],
            payload[offset + 1],
            payload[offset + 2],
            payload[offset + 3],
        ]);
        offset += 4;

        // キーからサイズを抽出
        let size_id = ((key >> 28) & 0x07) as u8;
        let value_size = match size_id {
            0x01 | 0x02 => 1,  // L, U1
            0x03 => 2,         // I2, U2
            0x04 => 4,         // I4, U4
            0x05 => 8,         // X8, R8, U8, I8
            _ => {
                return Err(format!("Unknown key size ID: {} for key 0x{:08X}", size_id, key));
            }
        };

        if offset + value_size > payload.len() {
            return Err(format!("Payload truncated at key 0x{:08X}", key));
        }

        let value = payload[offset..offset + value_size].to_vec();
        offset += value_size;

        values.push((key, value));
    }

    Ok(CfgValgetResponse { layer, values })
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

    // NAV-PVT USBキー（テスト用）
    const CFG_MSGOUT_NAV_PVT_USB: u32 = 0x20910009;

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
        fn layer(&self) -> u8 { self.raw[7] }

        fn checksum_valid(&self) -> bool {
            let (expected_a, expected_b) = calculate_checksum(&self.raw[2..self.raw.len()-2]);
            self.raw[self.raw.len()-2] == expected_a && self.raw[self.raw.len()-1] == expected_b
        }
    }

    // ===========================================
    // CFG-VALGET Poll生成テスト
    // ===========================================

    #[rstest]
    #[case::ram(ReadLayer::Ram, 0, true)]
    #[case::bbr(ReadLayer::Bbr, 1, true)]
    #[case::flash(ReadLayer::Flash, 2, true)]
    #[case::default(ReadLayer::Default, 7, true)]
    fn test_レイヤー指定で正しい値が設定される(
        #[case] layer: ReadLayer,
        #[case] expected_layer: u8,
        #[case] should_succeed: bool,
    ) {
        let msg = build_cfg_valget_poll(layer, &[CFG_MSGOUT_NAV_PVT_USB]);
        let frame = UbxFrame::new(&msg);

        if should_succeed {
            assert_eq!(frame.layer(), expected_layer);
        }
    }

    #[rstest]
    #[case::正常系(ReadLayer::Flash, true)]
    fn test_cfg_valget_poll構造が正しい(
        #[case] layer: ReadLayer,
        #[case] should_succeed: bool,
    ) {
        let msg = build_cfg_valget_poll(layer, &[CFG_MSGOUT_NAV_PVT_USB]);
        let frame = UbxFrame::new(&msg);

        if should_succeed {
            assert_eq!(frame.sync1(), 0xB5, "sync1");
            assert_eq!(frame.sync2(), 0x62, "sync2");
            assert_eq!(frame.class(), 0x06, "class = CFG");
            assert_eq!(frame.id(), 0x8B, "id = VALGET");
            // ペイロード長: version(1) + layer(1) + position(2) + key(4) = 8
            assert_eq!(frame.payload_len(), 8, "payload length");
            assert_eq!(frame.version(), 0x00, "version");
            assert!(frame.checksum_valid(), "checksum");
        }
    }

    // ===========================================
    // CFG-VALGETレスポンスパーステスト
    // ===========================================

    #[rstest]
    #[case::正常系_u1値(&[
        0x01,       // version
        0x02,       // layer = Flash
        0x00, 0x00, // position
        0x09, 0x00, 0x91, 0x20, // key = 0x20910009 (NAV-PVT USB)
        0x01,       // value = 1
    ], ReadLayer::Flash, 1, true)]
    fn test_cfg_valget_response_パース(
        #[case] payload: &[u8],
        #[case] expected_layer: ReadLayer,
        #[case] expected_value: u8,
        #[case] should_succeed: bool,
    ) {
        let result = parse_cfg_valget_response(payload);

        if should_succeed {
            let response = result.expect("parse should succeed");
            assert_eq!(response.layer, expected_layer);
            assert_eq!(response.values.len(), 1);
            assert_eq!(response.values[0].0, CFG_MSGOUT_NAV_PVT_USB);
            assert_eq!(response.values[0].1, vec![expected_value]);
        }
    }

    #[rstest]
    #[case::短すぎる(&[0x01, 0x00], false)]
    fn test_cfg_valget_response_エラー(
        #[case] payload: &[u8],
        #[case] should_succeed: bool,
    ) {
        let result = parse_cfg_valget_response(payload);
        assert_eq!(result.is_ok(), should_succeed);
    }

    // ===========================================
    // レイヤー名パーステスト
    // ===========================================

    #[rstest]
    #[case::ram("ram", ReadLayer::Ram, true)]
    #[case::bbr("bbr", ReadLayer::Bbr, true)]
    #[case::flash("flash", ReadLayer::Flash, true)]
    #[case::default("default", ReadLayer::Default, true)]
    #[case::大文字("FLASH", ReadLayer::Flash, true)]
    #[case::混合("Flash", ReadLayer::Flash, true)]
    #[case::無効("invalid", ReadLayer::Ram, false)]
    fn test_parse_layer_name(
        #[case] name: &str,
        #[case] expected: ReadLayer,
        #[case] should_succeed: bool,
    ) {
        let result = parse_layer_name(name);
        if should_succeed {
            assert_eq!(result.unwrap(), expected);
        } else {
            assert!(result.is_err());
        }
    }
}
