# Session 247 計画

**目的**: AI検査サービス選定 — 要求・要件・コストの整理

**前提**: Session 246でAWSサービスの出力仕様を調査し、「1万台規模を見据えたスケーラビリティ」が必要という新要件が判明

---

## やること

| # | 作業 | 読むべきファイル | 参照コマンド |
|---|------|-----------------|--------------|
| 1 | AI検査の要求整理（What） | ai-service-selection-notes.md, ai-inspection-requirements-draft.md | - |
| 2 | 要件整理（性能、コスト、運用） | 07_ai_integration_and_cost_analysis.md | - |
| 3 | 1万台規模のコスト試算 | - | - |
| 4 | サービス選定の判断 | 06_ai_visual_inspection_comparison.md | - |

---

## 詳細

### 1. AI検査の要求整理（What）

**確認すべきこと**:
- そもそも何を達成したいのか（要求）
- 工数削減？品質向上？属人化解消？
- どの部品・どの不良モードが対象か

### 2. 要件整理

**性能要件**:
- 推論速度（リアルタイム必要？バッチでOK？）
- 精度（許容できる誤検出率）
- 信頼度スコアの必要性

**コスト要件**:
- 月額上限
- 初期投資の許容範囲

**運用要件**:
- 学習データの準備可否
- 現場の撮影環境
- 人間レビューの要否

### 3. コスト試算（1万台規模）

**試算対象**:
- Bedrock（Claude）: トークン課金
- Rekognition Custom Labels: 画像枚数課金
- SageMaker: インスタンス時間課金

### 4. サービス選定

要求・要件・コストを踏まえて選択肢を絞る

---

## 参照

- [Session 246 サマリー](../session246/session-summary.md)
- [AI検査サービス選定中間メモ](../session246/ai-service-selection-notes.md)
- [AI外観検査サービス比較](../../docs/missions/m3-incoming-inspection-db/ai-research/06_ai_visual_inspection_comparison.md)
- [AI連携設計・コスト分析](../../docs/missions/m3-incoming-inspection-db/ai-research/07_ai_integration_and_cost_analysis.md)
- [AI検査要件整理（Session 243）](../session243/ai-inspection-requirements-draft.md)

---

*作成: Session 246 (2026-03-18)*
