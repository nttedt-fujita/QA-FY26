# Session 164 計画

**目的**: 送信タイムアウトの根本原因調査

**前提**: Session 163でデバッグログ（bytes_to_read/bytes_to_write）を追加済み

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | 実機テストでデバッグログ確認 | バックエンドログ |
| 2 | 失敗時のバッファ状態を分析 | - |
| 3 | 根本原因に応じた対策を検討 | - |

---

## 確認ポイント

失敗時の以下の値を確認：
- `bytes_to_read`: 受信バッファに溜まっているバイト数
- `bytes_to_write`: 送信バッファに溜まっているバイト数

### 仮説

| 仮説 | bytes_to_write | 対策 |
|------|----------------|------|
| 送信バッファがフル | 大きい値 | flush待ち or ボーレート上げる |
| デバイスがビジー | 0に近い | タイムアウト延長 or リトライ |

---

## 参照

- [Session 163 summary](../session163/session-summary.md)
- [gnss_state_api.rs](../../prototype/m1-gnss/backend/src/web/gnss_state_api.rs)
- [manager.rs](../../prototype/m1-gnss/backend/src/device/manager.rs)

---

*作成: Session 163 (2026-03-13)*
