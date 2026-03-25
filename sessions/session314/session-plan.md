# Session 314 計画

**目的**: PSIプロトタイプの設計・着手

---

## 背景

- Session 313: PSIデータフロー整理、プロトタイプ構想を小笠原さん・掛川さんと合意
- kintone CSVが実質唯一のインプット（群馬通商Excelは初回のみ）
- 小さなDBシステムでCSVインポート→自動集計

---

## やること

| # | 作業 | 内容 |
|---|------|------|
| 1 | kintone CSVの構造確認 | 掛川さんにカラム構成を聞く or サンプルCSVを入手 |
| 2 | PSI Excelのシート構成確認 | どんなシートがあるか、出力として何が求められるか |
| 3 | DB構造の設計 | テーブル設計、インポート仕様 |
| 4 | 技術スタック決定 | Go? SQLite? |
| 5 | プロトタイプ着手 | CSV取込 → DB → 集計結果出力 |

---

## 未確認事項（Session 313からの持ち越し）

| # | 確認事項 | 確認先 |
|---|---------|--------|
| 1 | kintone CSVの具体的なカラム | 掛川さん |
| 2 | 群馬通商Excelの具体的なカラム | 掛川さん |
| 3 | 「受注案件一覧Agri」シートは誰が更新しているか | 掛川さん |
| 4 | 予測シートの更新頻度・タイミング | 長屋さん |
| 5 | PSI Excelの他のシート構成 | 掛川さん |
| 6 | 掛川さんの集計作業で一番時間がかかる部分 | 掛川さん |
| 7 | 出力としてPSIに求められるもの（誰が何を見るか） | 掛川さん・小笠原さん |

---

## 参照資料

| 資料 | 内容 |
|------|------|
| [sessions/session313/psi-dataflow-and-prototype-idea.md](../session313/psi-dataflow-and-prototype-idea.md) | PSIデータフロー・プロトタイプ構想 |
| [docs/technical-research/integration-decision-framework.md](../../docs/technical-research/integration-decision-framework.md) | 統合vs疎結合の判断基準 |
| [docs/technical-research/erp-overview.md](../../docs/technical-research/erp-overview.md) | ERP全体像 |
| [sessions/session305.5/gen-kintone-constraint-analysis.md](../session305.5/gen-kintone-constraint-analysis.md) | GEN/kintone制約分析 |

---

*作成: Session 313終了時 (2026-03-25)*
