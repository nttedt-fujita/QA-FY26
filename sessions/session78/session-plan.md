# Session 78 計画

**目的**: UBXパーサー追加実装（CFG-RATE, CFG-PRT）または DeviceManager実装開始

---

## 背景

Session 77でMON-VER, SEC-UNIQIDパーサーを実装完了。
Phase 1 Step 1（UBXパーサー）は5/7完了。

---

## 選択肢

### A: CFG-RATE, CFG-PRTを先に実装

- Step 1を完了させてからStep 2に進む
- TDDで実装（Phase 1から）

### B: DeviceManagerに着手

- MON-VER/SEC-UNIQIDで装置識別は可能
- CFG-RATE/CFG-PRTは「設定確認」用なので後回しにできる

---

## やること（Aの場合）

### 1. CFG-RATE仕様抽出

- PDF（u-blox F9 HPG 1.32）からCFG-RATEの仕様を確認
- 振る舞い記述（TDD Phase 1）

### 2. CFG-PRT仕様抽出

- PDF（u-blox F9 HPG 1.32）からCFG-PRTの仕様を確認
- 振る舞い記述（TDD Phase 1）

### 3. TDD Phase 2〜5

- テストシナリオ → テストコード → 実装 → リファクタリング

---

## 参照資料

- [session77/session-summary.md](../session77/session-summary.md) — 前セッションサマリー
- [session64/ubx-messages-spec.md](../session64/ubx-messages-spec.md) — 既存UBX仕様
- [17-gnss-tool-implementation-plan.md](../../docs/missions/m1-sensor-evaluation/gnss/17-gnss-tool-implementation-plan.md) — 実装計画

---

*計画作成: 2026-03-10 Session 77終了時*
