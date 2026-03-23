# Session 291-300

## Session 291 (2026-03-23)

**概要**: DX施策の現状把握 — 聞き取り整理、PSI/鶴田システム/群馬通商の解析、要求分析

**重要な成果**:
- 聞き取りPDF・パワポをmd化（dx-hearing-summary.md、qr-lot-management-plan.md）
- PSI Excel解析 → CSV抽出（データ+コメント282件）
- 鶴田システム解析 → drawio作成（BOM+棚卸→在庫欠品予測の計算フロー）
- 群馬通商・在庫管理ツールのCSV化
- EARS要求分析 → ほぼ全てHowでWhyが不足と判明
- 人別質問リスト作成 → 掛川さん回答済（週2hサービス残業、年間104h）
- 小笠原さん回答: 製造委託先の実在庫が見えないのが一番の課題
- kintone事前調査（ライトプランだとAPI不可）
- Linear API Key設定（WSL環境）、Issue作成は次セッション

**作成ファイル**:

| ファイル | 内容 |
|----------|------|
| [session291/dx-hearing-summary.md](../session291/dx-hearing-summary.md) | 聞き取りまとめ（GEN/kintone/PSI） |
| [session291/qr-lot-management-plan.md](../session291/qr-lot-management-plan.md) | QRコードロット管理計画（宇枝さん資料） |
| [session291/requirements-analysis.md](../session291/requirements-analysis.md) | 要求分析（EARS整理） |
| [session291/hearing-questions-all.csv](../session291/hearing-questions-all.csv) | 人別質問リスト（回答付き） |
| [session291/tsuruta-system-flow.drawio](../session291/tsuruta-system-flow.drawio) | 鶴田システム処理フロー図 |
| [session291/PSI_Mar23_data.csv](../session291/PSI_Mar23_data.csv) | PSI最新データ |
| [session291/PSI_Mar23_comments.csv](../session291/PSI_Mar23_comments.csv) | PSIコメント（顧客別内訳282件） |
| [session291/gunma-tsusho-csv/](../session291/gunma-tsusho-csv/) | 群馬通商入出庫管理（26シート） |
| [session291/zaiko-tool-csv/](../session291/zaiko-tool-csv/) | 在庫管理ツール（35シート） |
| [session291/session-summary.md](../session291/session-summary.md) | セッションサマリー |

**重要な判断**:
- DX施策はM3の延長として扱う
- 最優先はPSI集計自動化（掛川さんのサービス残業解消）
- 小笠原さんの一番の要求は「製造委託先の実在庫の可視化」

**詳細**: [session291/session-summary.md](../session291/session-summary.md)

---
