# Session 332 計画

- 日付: TBD
- 前セッション: 331（QA-24 DX効果分析完了・Linear整理・ドキュメント正式配置）

## 目的

**QA-25: PSIプロトタイプ設計** — 作る方に入る

## 前提

- DX効果分析（QA-24）完了 → ROI 47〜98%、投資回収1.0〜2.1年
- Phase 1（受注+出荷の自動集計）のデータソース確度A
- プロト先行型で進める方針（先に作って見せながら議論）
- 正式資料: docs/missions/psi-automation/

## やること

| # | 作業 | 詳細 |
|---|------|------|
| 1 | **Phase 1の設計** | 受注+出荷の自動集計。kintone CSVの取込仕様確認 |
| 2 | **ダッシュボード画面のモックアップ** | 藤田がAIに修正依頼していく前提のたたき台 |
| 3 | **DB設計** | kintone CSVの項目 → PSI集計に必要なテーブル設計 |

## 参照資料

- docs/missions/psi-automation/README.md（全体インデックス）
- docs/missions/psi-automation/psi-automation-feasibility.md（実現可能性・データソース確度）
- docs/missions/psi-automation/psi-excel-analysis.md（PSI Excel構造）
- docs/missions/psi-automation/hearing-results-kakegawa.md（掛川さん作業フロー）

## 注意

- 技術スタック: Go + Next.js + SQLite（M3/M4と同じ、S313決定）
- まず動くものを作ること。完璧を目指さない
- 宇枝さんへの確認（PSI Excel廃止可否、プロト先行方針）は並行で進める
