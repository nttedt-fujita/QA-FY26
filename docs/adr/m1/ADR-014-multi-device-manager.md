# ADR-014: 複数装置同時対応の実装方式

**ステータス**: 承認済み
**作成日**: 2026-03-16
**作成元**: Session 194
**影響範囲**: M1 Phase 3（複数台同時対応）

---

## コンテキスト

Phase 3 で複数台（2〜5台）の GNSS 装置を同時に接続・検査する機能を実装する。
現在の `DeviceManager` は1台専用設計（`connected_port: Option<...>`）である。

### 検討した選択肢

| 案 | 概要 | メリット | デメリット |
|----|------|---------|-----------|
| A | DeviceManager を拡張 | 既存コードの延長線上 | 並行処理でロック競合の懸念 |
| **B** | **MultiDeviceManager を新設** | **既存コード変更なし、並行処理シンプル** | **新規コードが必要** |
| C | Device に接続を持たせる | 直感的 | ライフタイム管理が複雑 |

---

## 決定

**案B: MultiDeviceManager を新設する**

既存の `DeviceManager` は変更せず、`MultiDeviceManager` を新設して複数の `DeviceManager` インスタンスを管理する。

---

## 根拠

1. **既存テストへの影響なし**: `DeviceManager` を変更しないため、既存の単体テストがそのまま動作する

2. **並行処理がシンプル**: 各デバイスが独立した `Arc<Mutex<DeviceManager>>` を持つため、ロック競合が発生しない

3. **段階的移行が可能**: `AppState` の `device_manager` を `multi_device_manager` に置き換えるだけで移行完了

4. **検証済みの実現可能性**:
   - `RealSerialPortProvider` は状態を持たないため、各 `DeviceManager` に新規インスタンスを渡せる
   - `list_devices()` は `provider.available_ports()` で全ポートを取得可能
   - 並行検査は `tokio::spawn` で各デバイスを独立処理可能

---

## 実装方針

### 構造

```rust
pub struct MultiDeviceManager {
    managers: HashMap<String, Arc<Mutex<DeviceManager<RealSerialPortProvider>>>>,
    provider: RealSerialPortProvider,
}
```

### API

```rust
impl MultiDeviceManager {
    pub fn new() -> Self;
    pub fn list_devices(&self) -> Result<Vec<DeviceInfo>, Error>;
    pub fn connect(&mut self, path: &str) -> Result<u32, Error>;
    pub fn disconnect(&mut self, path: &str) -> Result<(), Error>;
    pub fn get_manager(&self, path: &str) -> Option<Arc<Mutex<DeviceManager<...>>>>;
}
```

### 並行検査

```rust
pub async fn start_parallel_inspection(&self) -> HashMap<String, InspectionResult> {
    let handles: Vec<_> = self.managers.iter().map(|(path, manager)| {
        let manager = Arc::clone(manager);
        tokio::spawn(async move {
            let mut m = manager.lock().await;
            run_inspection(&mut m).await
        })
    }).collect();
    futures::future::join_all(handles).await
}
```

---

## 結果

- `DeviceManager` は変更不要
- `MultiDeviceManager` を `device/multi_manager.rs` に新設
- `AppState` の `device_manager` を `multi_device_manager` に変更
- API は `device_id` を明示的に指定する形式に統一

---

## 参照

- [multi-device-manager-design.md](../../../sessions/session194/multi-device-manager-design.md) - 詳細設計
- [17-gnss-tool-implementation-plan.md](../../missions/m1-sensor-evaluation/gnss/17-gnss-tool-implementation-plan.md) - Phase 3 計画
