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

## Session 292 (2026-03-24)

**概要**: 社長説明スライド作成開始 — 業務フロー図作成、QRロット管理の課題整理

**重要な成果**:
- 社長説明スライドv2の骨格作成
- 業務フロー図（drawio）作成 — 3システム構成（GEN/kintone/PSI）を可視化
- 出庫フローの追加（GEN → 製造各社）

**作成ファイル**:

| ファイル | 内容 |
|----------|------|
| [session292/ceo-presentation-v2.md](../session292/ceo-presentation-v2.md) | 社長説明スライドv2（骨格） |
| [session292/current-system-flow.drawio](../session292/current-system-flow.drawio) | 業務フロー図（3システム構成） |
| [session292/session-summary.md](../session292/session-summary.md) | セッションサマリー |

**重要な判断**:
- QRコードロット管理は仕組みを詰めないと提案できない（分割・統合・員数変動が未設計）
- スケジュールは勝手に決められない（他ミッションとの兼ね合い）

**未完了**:
- drawioのスクリーンショット → スライド差し替え
- QRコードロット管理の仕組み整理
- 宮崎さん打ち合わせ結果の反映

**詳細**: [session292/session-summary.md](../session292/session-summary.md)

---

## Session 293 (2026-03-24)

**概要**: 業務フロー図の修正（小笠原さん・掛川さんフィードバック反映）

**重要な成果**:
- 業務フロー図v2作成 — フィードバックを反映した修正版
- GEN/kintone二重管理の問題を明示
- 組立委託→組立在庫追記のフローを追加
- 受入検査→kintoneの入庫カウントを追加

**フィードバック内容**:
- GEN→製造各社: 出庫 + 組立委託（部品を預けている）
- 組立完了時: GEN・kintone両方に「組立在庫」追記
- 組み上がり機体: 群馬通商で在庫管理
- 受入検査対象: バッテリー、チャージャー、散布装置も含む → kintoneに入庫カウント
- GEN vs kintone: 全在庫マスタ vs 営業の販売管理 → 二重管理

**作成ファイル**:

| ファイル | 内容 |
|----------|------|
| [session293/current-system-flow-v2.drawio](../session293/current-system-flow-v2.drawio) | 業務フロー図v2（フィードバック反映） |
| [session293/session-summary.md](../session293/session-summary.md) | セッションサマリー |

**未完了**:
- drawioのスクリーンショット → スライド差し替え
- QRコードロット管理の仕組み整理
- 宮崎さん打ち合わせ結果の反映

**詳細**: [session293/session-summary.md](../session293/session-summary.md)

---
