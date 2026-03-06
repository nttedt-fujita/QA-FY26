# Session 20 計画

**目的**: 受入検査工数の月別分析（パレート + 4M）の実装

---

## 背景（Session 19の結論）

- Excel→CSV変換済み、整合性テスト62件パス
- 分析方針を策定済み: [session19/analysis-plan.md](../session19/analysis-plan.md)
- データ: 574件、530.5時間、222品名

---

## やること

### 1. 月別分析スクリプトの作成

analysis-plan.mdに従い、以下のCSVを出力するスクリプトを作成:

- **サマリーCSV**: 月ごとの概要（件数・工数・パレート指標・カテゴリ割合）
- **月別×部品CSV**: パレート分析用
- **月別×作業者CSV**: Man分析用
- **月別×カテゴリCSV**: Material分析用
- **月別×検査内容CSV**: Method分析用

### 2. 整合性テスト（TDD）

分析結果のテストをTDDで作成

### 3. （余裕があれば）表記揺れの名寄せ

品名・作業者名の表記揺れを解消し、再集計

---

## 未完了の他タスク（M1-B GNSS関連）

- 合格基準のエビデンス収集（Web調査）
- 末永さんヒアリング準備

---

## 参考資料

- [Session 19 分析方針](../session19/analysis-plan.md)
- [品名別工数集計CSV](../session19/csv-output/summary/品名別_検査工数集計.csv)
- [Session 6 Excelレビュー](../session6/excel-review.md)
