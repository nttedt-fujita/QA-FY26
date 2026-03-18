# Session 248 計画

**目的**: AI使用候補の効果検証・ヒアリング実施

**前提**:
- Session 247で要求・方針を整理
- そもそも何にAIを使うのが効果的か → 未検証
- ヒアリング確認項目を整理済み

---

## やること

| # | 作業 | 読むべきファイル | 参照コマンド |
|---|------|-----------------|--------------|
| 1 | ヒアリング実施（セクション7の確認項目を使用） | requirements-and-direction.md | - |
| 2 | AI使用候補の優先順位決定 | - | - |
| 3 | コスト再計算（1万台規模） | 07_ai_integration_and_cost_analysis.md | - |
| 4 | M3/M4 + AI検査の統合システム構成案作成 | ai-service-selection-notes.md | - |
| 5 | 損益分岐点の算出 | - | - |

---

## 詳細

### 1. ヒアリング実施

**確認項目**（requirements-and-direction.md セクション7）:
- A. AI使用候補の洗い出し
- B. 各候補の現状工数
- C. 1万台規模での変化
- D. AI化で削減できそうな工数の見積もり

### 2. AI使用候補の優先順位決定

ヒアリング結果を踏まえて:
- 最も効果が大きい候補を特定
- 技術的に実現可能な候補を絞り込み

### 3. コスト再計算

**前提の更新**:
- 現状: 月300h（既存資料）
- 1万台: ヒアリング結果

**計算対象**:
- 構成A〜D それぞれのコスト
- 人件費との比較

### 4. 統合システム構成案

Session 246のAWSサービス比較を踏まえて:
- M3/M4の記録基盤
- AI検査（員数確認・外観検査）
- クラウド構成（最小〜フル）

### 5. 損益分岐点

各構成で:
- 初期投資の回収期間
- ランニングコストと人件費削減のバランス

---

## 参照

- [Session 247 サマリー](../session247/session-summary.md)
- [Session 247 要求整理](../session247/requirements-and-direction.md)（ヒアリング確認項目: セクション7）
- [07_ai_integration_and_cost_analysis.md](../../docs/missions/m3-incoming-inspection-db/ai-research/07_ai_integration_and_cost_analysis.md)
- [Session 246 AI選定メモ](../session246/ai-service-selection-notes.md)

---

*作成: Session 247 (2026-03-18)*
