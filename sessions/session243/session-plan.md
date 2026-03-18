# Session 243 計画

**目的**: AI検査システム要件・要求の詳細整理

## 背景

- Session 242で宇枝部長からの問い合わせに対応するたたき台計画書を作成
- 最終的にパワポ資料として報告するための検討材料を整理中
- 今回は要件・要求の詳細を詰める

## やること

| # | 作業 | 読むべきファイル | 参照コマンド |
|---|------|-----------------|--------------|
| 1 | 員数確認の具体的な対象部品の確認 | ユーザーに質問 | - |
| 2 | 外観検査で検出したい不良の種類 | ユーザーに質問 | - |
| 3 | 現行検査工数の確認 | Excel or session235/csv/ | - |
| 4 | 既存AI調査資料の再確認 | ai-research/07, 08 | - |

## 確認すべき項目

### 1. 員数確認について

- どの部品が対象？（ネジ、コネクタ、基板など）
- 1ロットあたりの数量は？
- 現在の員数確認工数は？

### 2. 外観検査について

- 「軽微な不良」とは具体的に？（傷、欠け、変色など）
- 過去に発生した不良の種類は？
- 写真撮影の環境は？（照明、カメラなど）

### 3. 現行検査工数

- Session 235-237で分析した受入検査データを参照
- 月間検査工数: 約154時間（9,255分）
- 部品ごとの内訳は？

## 成果物

- 要件・要求の詳細整理ドキュメント
- たたき台計画書の更新（必要に応じて）

## 参照

- [ai-inspection-system-draft-plan.md](../session242/ai-inspection-system-draft-plan.md) — たたき台計画書
- [07_ai_integration_and_cost_analysis.md](../../docs/missions/m3-incoming-inspection-db/ai-research/07_ai_integration_and_cost_analysis.md) — AI連携設計・コスト分析
- [08_lean_improvement_proposals.md](../../docs/missions/m3-incoming-inspection-db/ai-research/08_lean_improvement_proposals.md) — 少人数で最大効果
- [session235/csv/](../session235/csv/) — 受入検査データCSV

---

*作成: Session 242 (2026-03-18)*
