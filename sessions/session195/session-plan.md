# Session 195 計画

**前提**: Session 194で複数装置同時対応（Phase 3）の実装方針を決定。案B（MultiDeviceManager新設）で進める。

---

## やること

### MultiDeviceManager の実装（TDD）

ADR-014 に基づき、`MultiDeviceManager` を実装する。

**タスク**:

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | `device/multi_manager.rs` 新規作成 | session194/multi-device-manager-design.md |
| 2 | `list_devices()` 実装 | device/manager.rs（既存パターン参照） |
| 3 | `connect()` / `disconnect()` 実装 | device/manager.rs |
| 4 | `get_manager()` 実装 | - |
| 5 | 単体テスト追加 | device/manager.rs のテストパターン参照 |

---

## 実装順序

1. **テスト先行**: `MultiDeviceManager` のテストケースを先に書く
2. **list_devices()**: 全ポートスキャン + 接続状態表示
3. **connect()**: 新しい `DeviceManager` インスタンス作成 + 接続
4. **disconnect()**: 指定デバイスのマネージャーを削除
5. **get_manager()**: 並行処理用のマネージャー取得

---

## 参照

| ドキュメント | 内容 |
|-------------|------|
| [session194/multi-device-manager-design.md](../session194/multi-device-manager-design.md) | MultiDeviceManager 設計書 |
| [ADR-014](../../docs/adr/m1/ADR-014-multi-device-manager.md) | 実装方式の決定 |
| [device/manager.rs](../../prototype/m1-gnss/backend/src/device/manager.rs) | 既存 DeviceManager 実装 |
