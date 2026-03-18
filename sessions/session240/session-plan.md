# Session 240 計画

**目的**: AI連携要件の確認（現プロトタイプとのギャップ分析）

## 背景

- Session 239でto-be/ドキュメント整理完了
- Session 238のm3-review-plan.mdに従い、AI連携要件を確認

## やること

| # | 作業 | 読むべきファイル | 参照コマンド |
|---|------|-----------------|--------------|
| 1 | 現アーキテクチャ確認 | prototype/m3/docs/ARCHITECTURE.md | - |
| 2 | AI連携要件確認 | session236/M3M4tools-AI-research/07_ai_integration_and_cost_analysis.md | - |
| 3 | Must/Shouldとの照合 | 上記2ファイル | - |
| 4 | ギャップ分析結果作成 | - | - |

## チェック項目（07より）

| 分類 | 項目 | 確認 |
|------|------|------|
| Must | 検査画像の保存機能 | ? |
| Must | 画像と検査メタデータの紐付け | ? |
| Must | 不良モードの統一分類コード | ? |
| Must | 良品/不良品のラベル付け | ? |
| Must | データエクスポート機能 | ? |
| Should | S3互換ストレージの採用 | ? |
| Should | AI判定結果の記録フィールド | ? |
| Should | フィードバックUIの設計 | ? |

## 成果物

- AI連携要件チェック結果
- 現プロトタイプのギャップ分析

## 参照

- [m3-review-plan.md](../session238/m3-review-plan.md) — 全体計画
- [Session 239 summary](../session239/session-summary.md)
