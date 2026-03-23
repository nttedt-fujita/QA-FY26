# Session 291 サマリー

**日時**: 2026-03-23
**目的**: DX施策の現状把握 — 聞き取り内容整理、PSI分析、質問リスト作成

---

## 成果

### 資料のmd化
- 小笠原さん聞き取りPDF → [dx-hearing-summary.md](dx-hearing-summary.md)
- 宇枝さんスライド → [qr-lot-management-plan.md](qr-lot-management-plan.md)

### PSI分析
- PSI Excel構造調査（64シート、47週次スナップショット、製品32種）
- 最新PSI_Mar23をCSV化（データ + コメント282件）
- 「右側の手動入力」= コメント欄の顧客別内訳と判明

### 要求分析
- EARS整理 → ほぼ全てHowでWhyが不足 → [requirements-analysis.md](requirements-analysis.md)
- 追加ヒアリング質問リスト（人別） → [hearing-questions-all.csv](hearing-questions-all.csv)

### 聞き取り結果の反映
- 掛川さん: PSI集計 **毎週2h（年間104h）サービス残業**
- 小笠原さん: **製造委託先の実在庫が見えない**のが一番の課題
- 困ったエピソード・在庫差異は「無い」と回答（掛川さんが未然防止している可能性）
- GENメーカーへ小川さん経由でメール済、今後は直接やり取りOK
- 宮崎さんと明日打ち合わせ予定

### システム理解
- 鶴田システム = Excelツール（BOM+棚卸→在庫欠品予測を色分け表示）→ [tsuruta-system-flow.drawio](tsuruta-system-flow.drawio)
- 群馬通商入出庫管理リスト確認（26シート、S/N単位管理）
- kintone事前調査（ライトプランだとAPI不可、スタンダード以上が必要）

### Linear Issue作成
- QA-8: 社長説明スライド作成（3/25）
- QA-9: 宮崎さん打ち合わせ（kintone調査）
- QA-10: GENメーカー問い合わせ（API連携可否）
- QA-11: 棚卸参加（3月末）— 現場作業フロー理解
- QA-12: PSI集計自動化の設計

### その他
- Linear API Key設定（WSL環境に.env作成）
- 不要CSV未削除（hearing-questions-v2.csv、miyazaki-questions.csv）
- 群馬通商エクセルのCSV化未実施

---

## 次セッションでやること

1. **社長説明スライド作成**（3/25夕方期限、QA-8）
2. **宮崎さん打ち合わせ結果の反映**（QA-9）
3. 不要ファイル削除
4. 群馬通商エクセルCSV化
5. progress.md更新

---

## 重要な判断・発見

- DX施策はM3の延長として扱う
- 最優先は**PSI集計自動化**（掛川さんのサービス残業解消）
- 小笠原さんの一番の要求は「製造委託先の実在庫の可視化」
- 鶴田システムはExcelマクロであり、API連携は不可（ファイル読み書きで連携）
- kintoneのプラン（ライト/スタンダード）がPSI自動化の分岐点

---

*作成: Session 291 (2026-03-23)*
