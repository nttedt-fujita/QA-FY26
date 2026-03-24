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

## Session 294 (2026-03-24)

**概要**: 在庫・ロット管理システム設計スキルの作成

**重要な成果**:
- `inventory-lot-design`スキル作成 — 管理粒度、分割・統合パターン、段階的導入の判断基準
- kintoneに既にシリアル番号等が管理されていると判明

**作成ファイル**:

| ファイル | 内容 |
|----------|------|
| [~/.claude/skills/inventory-lot-design/SKILL.md](~/.claude/skills/inventory-lot-design/SKILL.md) | 在庫・ロット管理設計スキル |
| [session294/session-summary.md](../session294/session-summary.md) | セッションサマリー |

**重要な判断**:
- ロット管理設計の前にkintone既存データを確認する必要がある
- 迷ったらロットから始める（シリアルへの移行は後から可能）

**未完了**:
- kintone既存データ構造の確認（宮崎さんに聞く）
- QRコードロット管理の仕組み整理
- スライドのスケジュール修正

**詳細**: [session294/session-summary.md](../session294/session-summary.md)

---

## Session 295 (2026-03-24)

**概要**: FY26在庫管理DXロードマップ作成 — 理想像・マイルストーン・AI連携ビジョン

**重要な成果**:
- 理想像（To-Be）の図作成 — 統合在庫管理システム（調達→組立→出荷の一貫管理）
- FY26ロードマップ作成 — Phase 1-3の段階的アプローチ
- AI連携ビジョンの整理 — 欠品リスクアラートがFY26着手候補
- ミッションバランスの整理 — M3/M4はDX施策に統合、M2は状況を見て判断

**作成ファイル**:

| ファイル | 内容 |
|----------|------|
| [session295/ideal-system-flow.drawio](../session295/ideal-system-flow.drawio) | 理想のシステム構成図 |
| [session295/fy26-roadmap.md](../session295/fy26-roadmap.md) | FY26在庫管理DXロードマップ |
| [session295/kintone-meeting-extract.md](../session295/kintone-meeting-extract.md) | kintone打ち合わせPDF抽出 |
| [session295/session-summary.md](../session295/session-summary.md) | セッションサマリー |

**重要な判断**:
- M3/M4はDX施策（在庫管理統合システム）に統合
- Phase 1（PSI自動化）はM1 GNSSと並行で4-6月
- AI連携は土台ができた上でPhase 3で検討
- M2は開発側との連携待ち、状況を見て判断

**FY26ロードマップ概要**:

| Phase | 期間 | 内容 | 受益者 |
|-------|------|------|--------|
| 1 | 4-6月 | PSI自動化 | 掛川さん |
| 2 | 7-12月 | 製造委託先可視化 | 小笠原さん |
| 3 | 1-3月 | 統合ダッシュボード + AI連携検討 | SCM全体 |

**詳細**: [session295/session-summary.md](../session295/session-summary.md)

---

## Session 296 (2026-03-24)

**概要**: FY26ロードマップ各フェーズの構成図作成

**重要な成果**:
- Phase 1構成図作成 — PSI自動化のBefore/After図
- Phase 2構成図作成 — 製造委託先可視化のBefore/After図
- Phase 3構成図作成 — 統合ダッシュボード + AI連携ビジョン

**作成ファイル**:

| ファイル | 内容 |
|----------|------|
| [session296/phase1-psi-automation.drawio](../session296/phase1-psi-automation.drawio) | Phase 1: PSI自動化（Before/After図） |
| [session296/phase2-manufacturer-visibility.drawio](../session296/phase2-manufacturer-visibility.drawio) | Phase 2: 製造委託先可視化（Before/After図） |
| [session296/phase3-integrated-dashboard.drawio](../session296/phase3-integrated-dashboard.drawio) | Phase 3: 統合ダッシュボード + AI連携ビジョン |
| [session296/session-summary.md](../session296/session-summary.md) | セッションサマリー |

**判明した情報**:
- kintone CSV出力は標準機能として搭載、ただしNTT東基盤での利用可否は要確認

**詳細**: [session296/session-summary.md](../session296/session-summary.md)

---

## Session 297 (2026-03-24)

**概要**: 宇枝さん・小笠原さん打ち合わせ結果の反映 — 受入検査フロー図・QRコード入出庫管理図の作成

**重要な成果**:
- 受入検査フロー図作成（現状 + QRコード導入後の理想）
- QRコード入出庫管理図作成（棚にQR貼付で入出庫自動記録）
- Session 31-296の背景整理（M3経緯、AI検査調査、SIPOC、DX施策統合）

**作成ファイル**:

| ファイル | 内容 |
|----------|------|
| [session297/iqc-flow-current-and-ideal.drawio](../session297/iqc-flow-current-and-ideal.drawio) | 受入検査フロー図（現状 + 理想） |
| [session297/qr-inventory-management.drawio](../session297/qr-inventory-management.drawio) | QRコード入出庫管理図（現状 + 理想） |

**次セッション**: kintone契約方式の整理（NTT東 vs NTTeDT独自）

**詳細**: [session297/session-summary.md](../session297/session-summary.md)

---

## Session 298 (2026-03-24)

**概要**: kintone契約方式の整理 / 4フェーズ構成への拡張

**重要な成果**:
- kintone契約方式の整理（独自契約推奨、和田さん・宮崎さんが対応中）
- 4フェーズ構成への拡張（Phase 2「入出庫の自動記録」を追加）
- fy26-roadmap.mdの更新

**作成ファイル**:

| ファイル | 内容 |
|----------|------|
| [session298/kintone-contract-summary.md](../session298/kintone-contract-summary.md) | kintone契約方式の整理 |
| [session298/phase-rename-proposal.md](../session298/phase-rename-proposal.md) | 4フェーズ構成の決定事項 |
| [session298/session-summary.md](../session298/session-summary.md) | セッションサマリー |

**4フェーズ構成（決定）**:

| Phase | 名称 | 内容 |
|-------|------|------|
| 1 | 集計業務の自動化 | PSI集計の手作業削減 |
| 2 | 入出庫の自動記録 | 受入検査・棚のQRコード化 |
| 3 | 委託先在庫の見える化 | 製造委託先の実在庫可視化 |
| 4 | SCM全体の一元管理 | 統合ダッシュボード + AI |

**重要な判断**:
- kintone独自契約が必要（理想像にはAPI連携が必須）
- 4フェーズ構成（Phase 2「入出庫の自動記録」を追加）

**次セッション**: 社長説明スライドの完成、図のスライド組み込み

**詳細**: [session298/session-summary.md](../session298/session-summary.md)

---

## Session 299 (2026-03-24)

**概要**: 写真から線図（テクニカルイラスト）への変換手法の調査

**重要な成果**:
- 写真→線図変換の技術調査（Web検索、エビデンス収集）
- 各手法の比較・評価レポート作成
- 推奨手法の選定（ChatGPT GPT-4o、TechLagoon、ColorBliss）

**作成ファイル**:

| ファイル | 内容 |
|----------|------|
| [session299/photo-to-line-drawing-research.md](../session299/photo-to-line-drawing-research.md) | 写真→線図変換の調査レポート |
| [session299/session-summary.md](../session299/session-summary.md) | セッションサマリー |

**調査結論**:
- ChatGPT GPT-4oが最も柔軟で高品質（プロンプトでスタイル指定可能）
- 無料で試すならTechLagoon（100%無料、カスタマイズ可能）
- ベクター形式が必要な場合は後処理でトレース

**詳細**: [session299/session-summary.md](../session299/session-summary.md)

---
