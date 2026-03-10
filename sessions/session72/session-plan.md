# Session 72 計画

**目的**: GNSS評価ツール UBXパーサー実装

---

## やること

### 1. UBXパーサー実装（TDD）

**対象メッセージ**:
- NAV-STATUS
- NAV-DOP
- MON-RF

**参照資料**:
- [session64/ubx-messages-spec.md](../session64/ubx-messages-spec.md) — 整理済みUBX仕様書

**手順**:
1. TDDスキル読み込み
2. テストケース作成（テーブルテスト形式）
3. パーサー実装
4. リファクタリング

### 2. DevContainer内でのテスト実行確認

- `cargo test` で全テストがパスすることを確認

---

## 参照

- [session71/session-summary.md](../session71/session-summary.md)
- [prototype/m1-gnss/backend/src/ubx/nav_pvt.rs](../../prototype/m1-gnss/backend/src/ubx/nav_pvt.rs) — 既存のNAV-PVTパーサー（参考）

---

*計画作成: 2026-03-10 Session 71終了時*
