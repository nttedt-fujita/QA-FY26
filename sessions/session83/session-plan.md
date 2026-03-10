# Session 83 計画

**目的**: InspectionEngine実装（TDD Phase 3-5）

---

## 背景

Session 82でInspectionEngineの振る舞い記述とテストシナリオリストが完成。
24シナリオが定義済み。

---

## やること

### 1. TDD Phase 3: テストコード作成

優先順位に従ってテストコードを作成:

1. **D1-D3**: InspectionItem/Result構造体
2. **C1-C5**: 結果判定ロジック
3. **A1-A4**: 検査シーケンス実行
4. **G1-G5**: 各検査項目のUBX送信
5. **B1-B2**: 通信疎通
6. **E1-E2**: 状態連携
7. **F1-F3**: 異常系

### 2. TDD Phase 4: 実装（Red → Green）

テストを1つずつ通していく。

### 3. TDD Phase 5: リファクタリング

全テストがグリーンの状態で構造を改善。

---

## 完了条件

- [ ] 24テストシナリオのテストコードが作成されている
- [ ] 全テストがパスしている
- [ ] リファクタリングが完了している

---

## 参照資料

- [inspection-engine-behavior.md](../session82/inspection-engine-behavior.md) — 振る舞い記述＋テストシナリオ
- [device-manager-behavior.md](../session80/device-manager-behavior.md) — DeviceManager振る舞い記述
- [16-gnss-tool-architecture.md](../../docs/missions/m1-sensor-evaluation/gnss/16-gnss-tool-architecture.md) — アーキテクチャ設計

---

*計画作成: 2026-03-10 Session 82終了時*
