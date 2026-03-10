# Session 82 計画

**目的**: InspectionEngine実装（TDD）

---

## 背景

Session 81でDeviceManagerが完成。次はInspectionEngine（検査シーケンス制御）を実装する。

---

## やること

### 1. TDD Phase 1: 振る舞い記述

InspectionEngineの責務を定義:
- 検査シーケンスの制御（開始・停止・状態遷移）
- UBXメッセージの送受信とパース
- 検査結果の構造化

### 2. TDD Phase 2: テストシナリオリスト

振る舞いからテストシナリオを導出。

### 3. TDD Phase 3-5: テストコード → 実装 → リファクタリング

時間が許せば実装まで進める。

---

## 完了条件

- [ ] InspectionEngineの振る舞い記述が完了
- [ ] テストシナリオリストが承認される
- [ ] （オプション）テストコードと実装が完了

---

## 参照資料

- [device-manager-behavior.md](../session80/device-manager-behavior.md) — DeviceManager振る舞い記述
- [16-gnss-tool-architecture.md](../../docs/missions/m1-sensor-evaluation/gnss/16-gnss-tool-architecture.md) — アーキテクチャ設計
- [17-gnss-tool-implementation-plan.md](../../docs/missions/m1-sensor-evaluation/gnss/17-gnss-tool-implementation-plan.md) — 実装計画

---

*計画作成: 2026-03-10 Session 81終了時*
