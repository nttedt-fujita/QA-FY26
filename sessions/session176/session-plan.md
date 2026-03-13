# Session 176 計画

**目的**: NTRIP/UBX競合問題の解決（KISS原則に従った順序）

**前提**: Session 175で競合問題を特定、解決方針を検討済み

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | 案D（タイムアウト短縮）の検討 | UBX仕様書、gnss_state_api.rs |
| 2 | 案Dで不十分なら案A（try_clone）検討 | serialport docs、manager.rs |
| 3 | 必要なら実装 | - |

---

## 詳細

### 1. 案D（タイムアウト短縮）- 最もシンプル

**確認事項**:
- タイムアウト2秒の根拠をUBX仕様書で確認
- F9Pの応答時間の仕様値は何秒か
- 短縮しても問題ないか

**現状**: [gnss_state_api.rs:63](../../prototype/m1-gnss/backend/src/web/gnss_state_api.rs#L63)
```rust
let raw = match $manager.receive_ubx(std::time::Duration::from_millis(2000)) {
```

### 2. 案A（try_clone）

**検討事項**:
- try_clone()の実装箇所特定
- DeviceManagerの変更範囲
- 注意: try_cloneドキュメントに「真の非同期ならmio-serial/tokio-serialを検討」と記載あり

### 3. async化は最後の手段

**理由**:
- 複雑度が大幅に上がる
- KISS原則から外れる
- 本当に必要か慎重に判断

---

## 参照

- [session175/session-summary.md](../session175/session-summary.md) - 前セッションサマリー
- [ADR-012](../../docs/adr/m1/ADR-012-periodic-output-and-unified-api.md) - 統合API採用経緯
- [gnss_state_api.rs](../../prototype/m1-gnss/backend/src/web/gnss_state_api.rs) - タイムアウト設定箇所
- [ntrip_api.rs](../../prototype/m1-gnss/backend/src/web/ntrip_api.rs) - 競合箇所
