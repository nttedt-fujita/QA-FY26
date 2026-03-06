# Session 24 計画

**予定日**: 2026-03-06以降
**目的**: 分析結果のレビュー・議論

---

## 背景

Session 19-23で受入検査データの分析基盤を構築した:
- CSV変換（Session 19）
- 月別分析スクリプト（Session 20）
- 名寄せ機能（Session 21）
- 品名名寄せ + データ異常レポート（Session 22）
- データクレンジング（Session 23）

分析スクリプトは完成したが、**分析結果の解釈・活用方法**がまだ議論されていない。

---

## やること

### 1. 分析結果のレビュー

- 月別サマリーの傾向確認
  - 工数の季節変動はあるか
  - 特定の月に工数が集中しているか
- パレート分析の妥当性
  - 上位20%の品名が何%の工数を占めているか
  - 重点管理すべき品名は何か
- 作業者別の傾向
  - 偏りはあるか
  - 負荷分散の余地はあるか

### 2. 4M分解の活用方法

- Man（作業者）: 作業者別工数の偏り
- Machine（設備）: 検査治具の使用状況
- Material（部品）: 品名別の工数傾向
- Method（検査内容）: 検査方法別の効率

### 3. 次のアクション検討

- 分析結果を誰に共有するか
- どのような改善提案ができるか

---

## 参考資料

- [tools/incoming_inspection/output/月別サマリー.csv](../../tools/incoming_inspection/output/月別サマリー.csv)
- [tools/incoming_inspection/output/detail/](../../tools/incoming_inspection/output/detail/)
- [Session 23 summary](../session23/session-summary.md)
