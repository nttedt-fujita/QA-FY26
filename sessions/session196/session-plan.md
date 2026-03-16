# Session 196 計画

**前提**: Session 195 で MultiDeviceManager の実装完了。12テストすべてパス。

---

## やること

### MultiDeviceManager の統合（Phase 3 続き）

Session 195 で作成した `MultiDeviceManager` を API に統合する。

**タスク**:

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | AppState を MultiDeviceManager に変更 | web/device_api.rs |
| 2 | list_devices API を移行 | device/multi_manager.rs |
| 3 | connect/disconnect API を移行 | device/multi_manager.rs |
| 4 | 複数接続テスト追加 | - |

---

## 詳細

### 1. AppState の変更

```rust
// 変更前
pub struct AppState {
    pub device_manager: Mutex<DeviceManager<RealSerialPortProvider>>,
    ...
}

// 変更後
pub struct AppState {
    pub multi_device_manager: Mutex<MultiDeviceManager<RealSerialPortProvider>>,
    ...
}
```

### 2. RealSerialPortProvider に Clone を追加

`MultiDeviceManager` は `P: SerialPortProvider + Clone` を要求。
`RealSerialPortProvider` は状態を持たないので `#[derive(Clone)]` で対応可能。

### 3. API 移行

既存 API の振る舞いを維持しつつ、内部実装を MultiDeviceManager に変更。

---

## 参照

| ドキュメント | 内容 |
|-------------|------|
| [session195/session-summary.md](../session195/session-summary.md) | 前セッションの成果 |
| [session194/multi-device-manager-design.md](../session194/multi-device-manager-design.md) | 設計書 |
| [ADR-014](../../docs/adr/m1/ADR-014-multi-device-manager.md) | 実装方式の決定 |
