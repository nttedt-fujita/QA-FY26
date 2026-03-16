# Session 198 計画

**前提**: Session 197 で2台同時接続の実機テスト完了。APIがパス指定に未対応であることが判明。

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | パス指定APIの追加 | web/nav_status_api.rs, web/gnss_state_api.rs |
| 2 | FEデバイス選択UI実装 | frontend/src/App.tsx |
| 3 | USBハブ経由テスト（任意） | - |

---

## 詳細

### 1. パス指定APIの追加

現在の `/api/nav-status` を `/api/devices/{path}/nav-status` 形式に拡張。

対象API:
- nav-status
- nav-sat
- nav-sig
- mon-span
- gnss-state

### 2. FEデバイス選択UI

- デバイス一覧からの選択UI
- 選択したデバイスのデータを表示

---

## 参照

| ドキュメント | 内容 |
|-------------|------|
| [session197/session-summary.md](../session197/session-summary.md) | 実機テスト結果 |
| [ADR-014](../../docs/adr/m1/ADR-014-multi-device-manager.md) | 複数装置同時対応 |
