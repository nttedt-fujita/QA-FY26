# Session 79 計画

**目的**: CFG-RATE/CFG-PRT パーサー実装（TDD Phase 2〜5）

---

## 背景

Session 78でCFG-RATE、CFG-PRTの仕様抽出とTDD Phase 1（振る舞い記述）を完了。
Phase 1 Step 1（UBXパーサー）の残り2メッセージを実装する。

---

## やること

### 1. TDD Phase 2: テストシナリオリスト作成

振る舞い記述（[session78/cfg-rate-prt-behavior.md](../session78/cfg-rate-prt-behavior.md)）を元に、
テストシナリオリストを作成する。

**CFG-RATE**: R1〜R10（10件）
**CFG-PRT**: P1〜P11（11件）

### 2. TDD Phase 3: テストコード作成

テーブルテスト形式で作成（`rules/06-test-style.md`に従う）。

### 3. TDD Phase 4: 実装（Red → Green）

1つずつテストを通す。

### 4. TDD Phase 5: リファクタリング

共通処理があればcommon.rsに抽出。

---

## 完了条件

- [ ] CFG-RATEパーサーが全テストをパス
- [ ] CFG-PRTパーサーが全テストをパス
- [ ] Phase 1 Step 1（UBXパーサー）7/7完了

---

## 参照資料

- [session78/cfg-rate-prt-behavior.md](../session78/cfg-rate-prt-behavior.md) — 振る舞い記述
- [session78/cfg-rate-prt-spec.md](../session78/cfg-rate-prt-spec.md) — 仕様書
- [mon_ver.rs](../../prototype/m1-gnss/backend/src/ubx/mon_ver.rs) — 既存パーサーの参考

---

*計画作成: 2026-03-10 Session 78終了時*
