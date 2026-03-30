# PSI自動化プロジェクト

**目的**: 掛川さんが毎週日曜に2時間かけているPSI Excel手動集計（年104h）を自動化する

**Linearプロジェクト**: [M3/M4-PSI] PSI自動化

---

## ドキュメント一覧

### 調査・分析

| # | ドキュメント | 作成セッション | 内容 |
|---|------------|--------------|------|
| 1 | [investigation-timeline.md](investigation-timeline.md) | S326 | 調査経緯まとめ（S313〜S325の時系列） |
| 2 | [psi-excel-analysis.md](psi-excel-analysis.md) | S314 | PSI Excel構造分析（64シート構成） |
| 3 | [gunma-excel-reanalysis.md](gunma-excel-reanalysis.md) | S327 | 群馬通商Excel分析（フラグ・レコード番号の実態確認） |
| 4 | [kintone-system-analysis.md](kintone-system-analysis.md) | S316 | kintone全体像（導入背景・部署別管理範囲） |
| 5 | [csv-analysis-summary.md](csv-analysis-summary.md) | S316 | kintone CSV 1次分析（7アプリ・リレーション） |
| 6 | [domain-model-as-is.md](domain-model-as-is.md) | S317 | As-Isドメインモデル（確度付き） |
| 7 | [hearing-results-kakegawa.md](hearing-results-kakegawa.md) | S320 | 掛川さんヒアリング結果（作業フロー全体像） |
| 8 | [psi-automation-feasibility.md](psi-automation-feasibility.md) | S320 | 実現可能性評価（データソース確定状況） |

### DX効果分析

| # | ドキュメント | 作成セッション | 内容 |
|---|------------|--------------|------|
| 9 | [case-studies.md](case-studies.md) | S330 | DX事例調査（定量データ付き） |
| 10 | [dx-effect-analysis.md](dx-effect-analysis.md) | S331 | DX効果分析・ROI試算 |
| 11 | [priority-matrix.md](priority-matrix.md) | S331 | フェーズ別優先順位・進め方 |

### 図

| # | ドキュメント | 作成セッション | 内容 |
|---|------------|--------------|------|
| 12 | [diagrams/psi-workflow-consolidated.drawio](diagrams/psi-workflow-consolidated.drawio) | S326 | PSI集計作業フロー（集約版） |
| 13 | [diagrams/domain-model-as-is.drawio](diagrams/domain-model-as-is.drawio) | S317 | As-Isドメインモデル図 |

---

## 制約条件

| 制約 | 内容 | 確認セッション |
|------|------|--------------|
| GEN | CSVインポートのみ。API連携不可 | S331 |
| kintone | NTT東日本基盤の制約あり。当面CSV手動DL | S298, S317 |
| kintone改善 | 宮崎さんに依頼して対処可能 | S331 |

## フェーズ構成

| Phase | 対象 | データソース確度 | 状態 |
|-------|------|---------------|------|
| Phase 1 | 受注+出荷の自動集計 | A | 設計前 |
| Phase 2 | 在庫の追加 | A/B+ | 設計前 |
| Phase 3 | 生産・発注・予測の追加 | B〜D | 未確認事項あり |
