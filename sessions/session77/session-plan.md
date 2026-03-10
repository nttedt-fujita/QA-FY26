# Session 77 計画

**目的**: UBXパーサー実装（MON-VER, SEC-UNIQID）— TDD Phase 2〜5

---

## 背景

Session 76でTDD Phase 1（振る舞い記述）を完了。Phase 2以降を実施する。

**振る舞い記述**: [session76/ubx-mon-ver-sec-uniqid-spec.md](../session76/ubx-mon-ver-sec-uniqid-spec.md)

---

## やること

### 1. TDD Phase 2: テストシナリオリスト作成

振る舞い記述をもとにテストシナリオをドラフトし、ユーザー承認を得る。

**MON-VER**: 8件の振る舞い → テストシナリオ
**SEC-UNIQID**: 6件の振る舞い → テストシナリオ

### 2. TDD Phase 3: テストコード作成

承認後、テストコードを作成。
- テーブルテスト形式（`06-test-style.md` 準拠）
- `should_succeed` パラメータ必須

### 3. TDD Phase 4: 実装（Red → Green）

テストを1つずつ通す。

### 4. TDD Phase 5: リファクタリング

全テストグリーン後、コード構造を改善。

---

## 参照資料

- [session76/ubx-mon-ver-sec-uniqid-spec.md](../session76/ubx-mon-ver-sec-uniqid-spec.md) — 仕様 + 振る舞い記述
- [session64/ubx-messages-spec.md](../session64/ubx-messages-spec.md) — NAV-STATUS等の仕様
- [nav_status.rs](../../prototype/m1-gnss/backend/src/ubx/nav_status.rs) — 既存パーサー実装パターン

---

## 注意

- TDD作業前に `tdd-practice` スキルを読む
- テストは `06-test-style.md` のテーブルテスト形式で作成
- 仕様書フィールドのヌケモレチェックを実施（Session 72, 76の反省）

---

*計画作成: 2026-03-10 Session 76終了時*
