# Periodic Output 実装設計

**作成**: Session 140 (2026-03-12)
**出典**: u-blox F9 HPG 1.32 Interface Description (UBX-22008968)

---

## 背景

Session 139で特定した問題:
- 複数のAPIが同時にF9Pをポーリング
- シリアルポートは1チャネル → 応答が混在
- パースエラー多発 → 500エラー

**解決策**: Periodic Output（定期出力）を採用

---

## 仕様（PDF抽出結果）

### 1. メッセージタイプ

UBXメッセージには `Type: Periodic/polled` のものがある（p.150-154）:
- `Periodic`: CFG-MSGOUTでレート設定すると自動出力
- `polled`: Pollリクエストで応答を返す
- 両方対応のメッセージは `Periodic/polled`

### 2. CFG-MSGOUT 設定キー

USBポート用の設定キー（PDF p.234-251）:

| メッセージ | キー名 | Key ID | 型 |
|-----------|--------|--------|-----|
| NAV-PVT | CFG-MSGOUT-UBX_NAV_PVT_USB | 0x20910009 | U1 |
| NAV-STATUS | CFG-MSGOUT-UBX_NAV_STATUS_USB | 0x2091001d | U1 |
| NAV-SAT | CFG-MSGOUT-UBX_NAV_SAT_USB | 0x20910018 | U1 |
| NAV-SIG | CFG-MSGOUT-UBX_NAV_SIG_USB | 0x20910348 | U1 |
| MON-SPAN | CFG-MSGOUT-UBX_MON_SPAN_USB | 0x2091038e | U1 |
| MON-RF | CFG-MSGOUT-UBX_MON_RF_USB | 0x2091035c | U1 |

**値の意味**:
- `0`: 出力しない（Pollのみ）
- `1`: 毎エポック出力
- `N`: N エポックごとに出力

### 3. UBX-CFG-VALSET（p.96-98）

設定を書き込むコマンド:

```
Header: 0xB5 0x62
Class: 0x06
ID: 0x8A
Payload:
  [0] version: 0x00 (transactionless)
  [1] layers: bit flags
      bit 0 (0x01): RAM
      bit 1 (0x02): BBR
      bit 2 (0x04): Flash
  [2-3] reserved: 0x00 0x00
  [4+] cfgData: キーと値のペア
        Key: 4バイト (Little Endian)
        Value: 型に応じたサイズ（U1=1バイト）
```

**応答**: UBX-ACK-ACK (0x05 0x01) または UBX-ACK-NAK (0x05 0x00)

---

## アーキテクチャ設計

### 現状

```
[FE] --setInterval(1s)--> [API] --Poll--> [F9P]
[FE] --setInterval(2s)--> [API] --Poll--> [F9P]   // 並行で競合
[FE] --setInterval(5s)--> [API] --Poll--> [F9P]
```

### 新アーキテクチャ

```
                          ┌─────────────────┐
[F9P] ───(定期出力)────> │ 受信スレッド    │
                          │ (バックグラウンド)│
                          └────────┬────────┘
                                   │ 更新
                                   ▼
                          ┌─────────────────┐
                          │ 内部状態        │
                          │ (Arc<Mutex<T>>) │
                          └────────┬────────┘
                                   │ 参照
                                   ▼
[FE] --setInterval(1s)--> [API] ──(状態返却のみ)
```

**ポイント**:
1. F9Pからの定期出力を受信スレッドで継続的に受信
2. メッセージタイプごとに最新状態を保持
3. APIは最新状態を返すだけ（ポーリングしない）

---

## 実装タスク

### Phase 1: 設定送信

1. `UbxConfigBuilder` 構造体を作成
2. CFG-VALSET メッセージ生成関数
3. 接続時に定期出力設定を送信

### Phase 2: 受信スレッド

1. `PeriodicReceiver` スレッドを作成
2. シリアルポートから継続的に読み取り
3. UBXメッセージをパース
4. メッセージタイプに応じて状態を更新

### Phase 3: API修正

1. ポーリング削除
2. 内部状態を参照して返却

---

## 実装詳細

### 設定送信

```rust
// CFG-VALSET: 定期出力を有効化（毎エポック）
fn build_cfg_valset_periodic_output() -> Vec<u8> {
    let mut payload = vec![
        0x00,       // version: transactionless
        0x01,       // layers: RAM only
        0x00, 0x00, // reserved
    ];

    // キーと値のペア
    let configs = [
        (0x20910009u32, 1u8),  // NAV-PVT
        (0x2091001du32, 1u8),  // NAV-STATUS
        (0x20910018u32, 1u8),  // NAV-SAT
        (0x20910348u32, 1u8),  // NAV-SIG
        (0x2091038eu32, 1u8),  // MON-SPAN
        (0x2091035cu32, 1u8),  // MON-RF
    ];

    for (key, value) in configs {
        payload.extend_from_slice(&key.to_le_bytes());
        payload.push(value);
    }

    build_ubx_frame(0x06, 0x8A, &payload)
}
```

### 内部状態

```rust
pub struct GnssState {
    pub nav_pvt: Option<NavPvtData>,
    pub nav_status: Option<NavStatusData>,
    pub nav_sat: Option<NavSatData>,
    pub nav_sig: Option<NavSigData>,
    pub mon_span: Option<MonSpanData>,
    pub mon_rf: Option<MonRfData>,
    pub last_update: Instant,
}
```

### 受信ループ

```rust
fn receiver_loop(port: Box<dyn SerialPort>, state: Arc<Mutex<GnssState>>) {
    let mut buffer = Vec::new();

    loop {
        // 読み取り
        let mut chunk = [0u8; 256];
        match port.read(&mut chunk) {
            Ok(n) => buffer.extend_from_slice(&chunk[..n]),
            Err(e) if e.kind() == TimedOut => continue,
            Err(_) => break,
        }

        // UBXメッセージを抽出・パース
        while let Some((msg, remaining)) = extract_ubx_message(&buffer) {
            buffer = remaining;

            // メッセージタイプに応じて状態更新
            match (msg.class, msg.id) {
                (0x01, 0x07) => {
                    if let Ok(pvt) = nav_pvt::parse(&msg.raw) {
                        state.lock().unwrap().nav_pvt = Some(pvt);
                    }
                }
                // 他のメッセージも同様
                _ => {}
            }
        }
    }
}
```

---

## 考慮事項

### 1. シリアルポートの排他制御

- 受信スレッドがポートを占有
- RTCM転送（NTRIP）との共存が必要
- 解決策: write_data() は受信スレッド経由でキューイング

### 2. 更新頻度の調整

- NAV-PVT, NAV-STATUS: 毎エポック（1Hz）
- NAV-SAT, NAV-SIG: 5エポックに1回
- MON-SPAN: 10エポックに1回

### 3. 接続時の初期化

1. 接続成功
2. CFG-VALSET送信
3. ACK確認
4. 受信スレッド開始

---

## 次のステップ

1. `ubx/config.rs` に CFG-VALSET 生成関数を追加
2. `device/manager.rs` に受信スレッドを追加
3. `web/device_api.rs` に GnssState を追加
4. 各API（nav_status, nav_sat等）を修正
