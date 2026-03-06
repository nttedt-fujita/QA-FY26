# Session 25 計画

**予定日**: 2026-03-06以降
**目的**: 矢印記号修正、報告資料作成

---

## 背景

Session 24で以下が判明:
- 矢印記号（↓↑）15件が工数として集計されていない
- 記録が適切にできていない、確認体制も未整備
- 小笠原さんへの報告が必要

---

## やること

### 1. 矢印記号の集計修正（優先度高）

- `tools/incoming_inspection/monthly_analysis.py` を修正
- 「↓」「↑」を前後の行の工数として処理
- テストコードも追加

### 2. 小笠原さん報告資料作成

- 分析結果サマリーをパワポにまとめる
- 含めるべき内容:
  - Excelから分かったこと（月別傾向、工数上位品目）
  - データの限界（杉山さんの記録のみ）
  - データ品質問題（記録不備、確認体制未整備）

### 3. （余裕があれば）ドメインモデリング継続

- As-Isモデルの概念図作成（Draw.io）
- ユビキタス言語集のドラフト

---

## 参考資料

- [Session 24 summary](../session24/session-summary.md)
- [docs/missions/m3-incoming-inspection-db/excel-analysis-summary.md](../../docs/missions/m3-incoming-inspection-db/excel-analysis-summary.md)
- [docs/missions/m3-incoming-inspection-db/domain-modeling-approach.md](../../docs/missions/m3-incoming-inspection-db/domain-modeling-approach.md)
