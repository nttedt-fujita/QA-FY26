# Session 318 サマリー

**日時**: 2026-03-26
**目的**: S317資料の読み直し・整理 + 掛川さんミーティング準備 + PSIデータフロー突合分析

---

## 実施内容

### 1. S317資料の修正・整理

- `requirements-analysis.md` → `needs-analysis.md` にリネーム（要求=Need、要件=Requirementの使い分け）
- `needs-analysis.md` を責務ごとに3ファイルに分割:
  - `issues-analysis.md` — 課題分析（MECE+ロジックツリー）
  - `needs-hypotheses.md` — 要求仮説（EARS）
  - `priority-assessment.md` — 優先順位評価（マトリクス+ピラミッド）
- サプライヤ情報の誤り修正:「Excel品名にサプライヤ情報なし」→「サプライヤ列はあるが表記揺れ」（excel-review.mdで確認済み）
- `technical-constraints.md` の同様の誤り修正

### 2. 掛川さんミーティング準備

- PSI実データ画像から、PSI本体の情報量がこちらの理解より大きいことが判明（出荷週次内訳、生産数、拠点別在庫、発注関連）
- 「合ってますか？」形式の質問リスト作成（背景・根拠・確度付き）
- PSIデータフローのdrawio図作成

### 3. 突合分析（新規、6件実施）

| # | 突合 | 結果 | 確度 |
|---|------|------|------|
| 1 | kintone普及支援/受注管理 ↔ Agriシート | 2月以降一致（Kintone#=レコード番号、顧客名・受注日対応） | **A** |
| 2 | kintone発送製品(09) ↔ PSI出庫実績 | 粒剤のみ完全一致(44=44)。チャージャー差1、バッテリ差53、機体PSI未更新 | **B** |
| 3 | 群馬通商Excel「入庫保管中」↔ PSI群馬通商在庫 | BB102/粒剤/バッテリ3品目完全一致 | **A** |
| 4 | kintone機体マスタ(71)ロケーション ↔ PSI拠点別在庫 | BB102@群馬19、SL福島10完全一致 | **A** |
| 5 | kintoneオプションマスタ(72)ロケーション ↔ PSI在庫 | **一致せず**。マスタの更新タイミングが異なる | — |
| 6 | 群馬通商Excel月別出庫 ↔ PSI出庫実績 | **一致せず**。群馬通商の出庫=倉庫発送、PSI出庫=別概念 | — |

### 4. 判明した新事実

- kintone普及支援/受注管理は**2026年2月から運用開始**（2月以前の321件はkintone CSVに無い）
- PSI群馬通商在庫のソースは**群馬通商Excelの「入庫保管中」**で確定
- PSI出荷実績のソースはkintone発送製品(09)の可能性が高いが、粒剤以外は未確定
- 受入検査は品質保証（石川さん）の管轄のはずだが、実際はSCMの田原さんが実施（組織的経緯は不明）

---

## 成果物

| ファイル | 内容 |
|----------|------|
| [kakegawa-questions.md](kakegawa-questions.md) | 掛川さんへの確認事項（背景・根拠・突合結果付き） |
| [psi-dataflow-understanding.drawio](psi-dataflow-understanding.drawio) | PSIデータフロー図（確認済み/未確認を色分け） |
| [../session317/issues-analysis.md](../session317/issues-analysis.md) | 課題分析（S317分割後） |
| [../session317/needs-hypotheses.md](../session317/needs-hypotheses.md) | 要求仮説（S317分割後） |
| [../session317/priority-assessment.md](../session317/priority-assessment.md) | 優先順位評価（S317分割後） |

---

## 次セッションでやること

| # | やること | 理由 |
|---|---------|------|
| 1 | **掛川さんミーティングの結果反映** | 質問リストの★を埋める。drawio図を更新 |
| 2 | **GENアカウント取得後、GENの画面確認** | 生産計画・在庫・発注がGENにあるか確認 |
| 3 | **kintone全期間CSV再取得**（Windows PC） | 2月以前の受注データをkintoneから取れるか確認 |
| 4 | **PSI自動化の実現可能性評価** | 突合結果を踏まえ、kintone+群馬通商Excelでどこまで自動化できるか |

---

*作成: Session 318 (2026-03-26)*
