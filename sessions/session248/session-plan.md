# Session 248 計画

**目的**: AI検査・M3/M4統合システムのコスト試算と構成案作成

**前提**:
- Session 247で要求・方針を整理
- 1万台規模での検査工数をヒアリングで確認予定

---

## やること

| # | 作業 | 読むべきファイル | 参照コマンド |
|---|------|-----------------|--------------|
| 1 | 1万台規模の検査工数をヒアリング結果として取り込む | - | - |
| 2 | コスト再計算（1万台規模） | 07_ai_integration_and_cost_analysis.md, requirements-and-direction.md | - |
| 3 | M3/M4 + AI検査の統合システム構成案作成 | ai-service-selection-notes.md | - |
| 4 | 損益分岐点の算出 | - | - |

---

## 詳細

### 1. ヒアリング結果の取り込み

**確認すべきこと**:
- 1万台規模での月間検査工数
- 検査対象部品の種類・数

### 2. コスト再計算

**前提の更新**:
- 現状: 月300h（既存資料）
- 1万台: ヒアリング結果

**計算対象**:
- 構成A〜D それぞれのコスト
- 人件費との比較

### 3. 統合システム構成案

Session 246のAWSサービス比較を踏まえて:
- M3/M4の記録基盤
- AI検査（員数確認・外観検査）
- クラウド構成（最小〜フル）

### 4. 損益分岐点

各構成で:
- 初期投資の回収期間
- ランニングコストと人件費削減のバランス

---

## 参照

- [Session 247 サマリー](../session247/session-summary.md)
- [Session 247 要求整理](../session247/requirements-and-direction.md)
- [07_ai_integration_and_cost_analysis.md](../../docs/missions/m3-incoming-inspection-db/ai-research/07_ai_integration_and_cost_analysis.md)
- [Session 246 AI選定メモ](../session246/ai-service-selection-notes.md)

---

*作成: Session 247 (2026-03-18)*
