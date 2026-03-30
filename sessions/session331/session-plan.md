# Session 331 計画

- 日付: TBD
- 前セッション: 330（QA-27 DX事例調査完了）

## 目的

**QA-24: DX効果分析** — 自社の投資対効果を試算する

## 前提

Session 328/330で基盤データが揃っている:
- CoQ PAF法の知識（docs/qa-knowledge/coq-knowledge.md）
- PSI管理指標（docs/qa-knowledge/scm-knowledge.md）
- 業界DX事例の定量データ（sessions/session330/case-studies.md）
- 自社現状データ（sessions/session327/gunma-excel-reanalysis.md 等）

## やること

| # | 作業 | 詳細 |
|---|------|------|
| 1 | **自社現状のコスト試算** | 手動転記・仕損品追跡・在庫タイムラグのCoQ分類・時間試算 |
| 2 | **DX後のBefore/After比較** | 業界事例の改善率を自社規模に当てはめる |
| 3 | **投資対効果（ROI）試算** | 開発コスト vs 削減効果（回収期間） |
| 4 | **優先順位マトリクス作成** | 効果大×実現容易性のマトリクス |

## 参照資料

- sessions/session330/case-studies.md（定量データ）
- docs/qa-knowledge/coq-knowledge.md（CoQ PAF法）
- docs/qa-knowledge/scm-knowledge.md（PSI管理指標）
- sessions/session327/gunma-excel-reanalysis.md（群馬通商Excel分析）
- sessions/session324b/kintone-shipment-analysis.md（出庫データ分析）

## 成果物

- sessions/session331/dx-effect-analysis.md（DX効果試算シート）
- sessions/session331/priority-matrix.md（優先順位マトリクス）

## 注意

- 自社規模（バッテリ仕損24%、在庫週次タイムラグ等）の実数値を使うこと
- ROI計算式: (年間便益 - 年間運用コスト) / 初期投資 × 100
- 推測値はその旨を明記し、範囲（最小〜最大）で示す
