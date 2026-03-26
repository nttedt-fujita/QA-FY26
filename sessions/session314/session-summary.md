# Session 314 サマリー

**日時**: 2026-03-26
**目的**: PSIデータ分析・現状把握、Linear整備・全ミッション計画策定

---

## 実施内容

### 1. 社長フィードバック議事録の追記

Session 312のceo-feedback.mdに追記:
- 製販会議の頻度見直し（週1→月1の可能性）
- 産業機の購入承認フロー見直し（承認不要の方向）
- PSI資料は製販会議で使われていた
- システム設計に関係する情報の抽出セクションを追加

**成果物**: [sessions/session312/ceo-feedback.md](../session312/ceo-feedback.md)（更新）

### 2. PSI Excel構造分析（1次〜3次）

PSI Excel（64シート）と群馬通商Excel（26シート）を分析。

- **1次分析**: シート構成、行項目構造、データソース推定
- **2次分析（突合）**: Agriシート集計とPSI受注行の数値突合 → **粒剤12ヶ月完全一致**
- **3次分析**: Agriシートの深掘り（商流分布、機体種別推移、リードタイム）

**主な発見**:
- Agriシート集計 = PSI受注行 [確度A]
- kintoneがAgriの元データ [確度B]
- 供給側（入庫/出庫/在庫）のソースは不明 [確度D]
- 9月を境にAC102通常→可変施肥に100%移行 [確度A]

**成果物**:
- [psi-excel-analysis.md](psi-excel-analysis.md)
- [gunma-excel-analysis.md](gunma-excel-analysis.md)
- [psi-cross-reference-analysis.md](psi-cross-reference-analysis.md)
- [psi-3rd-analysis.md](psi-3rd-analysis.md)

### 3. 確度整理（A/B/C/D分類）

全分析資料に確度ラベルを付与。推測と事実を明確に分離。

**成果物**: [confidence-matrix.md](confidence-matrix.md)

### 4. 作業フロー仮説図（drawio）

掛川さんの週次PSI更新フローを仮説として図示（確度ラベル付き）。

**成果物**: [kakegawa-workflow-hypothesis.drawio](kakegawa-workflow-hypothesis.drawio)（3ページ）

### 5. 質問リスト統合・日程計画

全分析資料の質問を統合し、確認先・優先順位・会議vsチャットの判断を整理。

**成果物**:
- [hearing-questions.md](hearing-questions.md)
- [schedule-plan.md](schedule-plan.md)

### 6. Linear整備・全ミッション計画策定

- 「PSI自動化」プロジェクトを新規作成、M3/M4から分離
- M3とM4を分離
- 全6プロジェクトにプレフィックス・日付・マイルストーンを設定
- 全プロジェクトにupdatesを投稿
- linear-managementスキルにマイルストーン運用・プロジェクト命名規則・進捗監視ルールを追加

### 7. 新情報の記録

- GENにkintone CSVをインポートしている [A — 藤田さん確認]
- 藤田さんはkintoneに触れたことがない
- 5月にセンサー搭載実験機体の飛行予定がある

---

## 作成ファイル

| ファイル | 内容 | 配置先 |
|----------|------|--------|
| psi-excel-analysis.md | PSI Excel構造分析（確度付き） | sessions/ |
| gunma-excel-analysis.md | 群馬通商Excel構造分析（確度付き） | sessions/ |
| psi-cross-reference-analysis.md | 突合分析（確度付き） | sessions/ |
| psi-3rd-analysis.md | 3次分析（確度付き） | sessions/ |
| confidence-matrix.md | 確度整理マトリクス | sessions/ |
| kakegawa-workflow-hypothesis.drawio | 作業フロー仮説図（3ページ） | sessions/ |
| hearing-questions.md | 質問リスト統合 | sessions/ |
| schedule-plan.md | 日程計画 | sessions/ |
| psi-csv/ | PSI ExcelのCSV抽出（7シート） | sessions/ |
| gunma-csv/ | 群馬通商ExcelのCSV抽出（6シート） | sessions/ |

---

## Linear変更

| 変更 | 内容 |
|------|------|
| プロジェクト新規作成 | [M3/M4-PSI] PSI自動化、[M4] 工程不良DB |
| プロジェクト分離 | [M3/M4] → [M3] 受入検査DB + [M4] 工程不良DB |
| プロジェクト命名 | 全プロジェクトに[ミッション]プレフィックス付与 |
| 日付設定 | 全6プロジェクトにstartDate/targetDate設定 |
| マイルストーン | M1-B(2件)、M1-A(2件)、M2(2件)、M3(3件)、M4(2件) |
| Issue作成 | QA-15〜18（PSI関連4件） |
| Updates投稿 | 全5プロジェクト |

---

## 次セッションへの引き継ぎ

### 今日中にやること
1. 宮崎さんにkintone閲覧権限を依頼
2. 掛川さんに打ち合わせ日程調整の連絡（4/1水〜）

### 棚卸し期間（3/27-31）
- PSIプロトタイプ骨格作成（Go + SQLite、M3のコード流用）
- 和田さん・宮崎さん打ち合わせの準備

### 棚卸し明け（4/1〜）
1. 掛川さん打ち合わせ（drawio見せながら作業フロー確認）
2. GNSSツール仕上げ（4月中完成目標）
3. PSIプロトタイプ開発

### 技術スタック方針（未ADR化）
- PSIプロトタイプ: Go（M3流用）+ SQLite（M1方式）+ Next.js
- M3のhandler→repositoryパターンを流用

---

*作成: Session 314 (2026-03-26)*
