# セッション履歴: Session 261〜270

## Session 261 (2026-03-19)

**概要**: 作業フロー可視化のベストプラクティス調査

**実施内容**:
1. SIPOC Diagram、Value Stream Mapping の調査
2. IQC（受入品質管理）2026年トレンドの調査
3. 隠れコスト可視化・リーン製造の8つのムダの調査
4. ベンダー管理・梱包標準化の調査
5. データ駆動型意思決定の落とし穴（測定バイアス等）の調査
6. 暗黙知の可視化・標準化の調査
7. 部分最適 vs 全体最適の調査

**作成ファイル**:

| ファイル | 内容 |
|----------|------|
| [best-practices-research.md](../session261/best-practices-research.md) | ベストプラクティス調査レポート（全8章、出典付き） |
| [session-summary.md](../session261/session-summary.md) | セッションサマリー |
| [session262/session-plan.md](../session262/session-plan.md) | 次セッションの計画 |

**主な発見**:
- SIPOC/VSM は部門横断的な理解を促進（現場と管理者のズレ解消）
- 隠れコストは従来の指標では見えない（8つのムダ: DOWNTIME）
- 測定バイアスに注意（「測定しやすいものだけ測定」する問題）
- 部分最適の罠（システム思考で全体最適を考える）
- 暗黙知の可視化が重要（「なんとなく」を標準化）
- ベンダー管理・梱包標準化は根本解決

**Session 260 の問題への適用**:
- 「なんとなく」で作業 → 暗黙知の可視化・標準化
- 隠れコスト（梱包変更作業） → 8つのムダ、VSM
- 測定しやすいものだけ測定 → 測定バイアスの是正
- 現場と管理者のズレ → SIPOC（部門横断的理解）
- 部分最適（員数確認だけAI化） → 全体最適の視点
- 梱包ルール標準化の必要性 → ベンダー管理

**次フェーズ提案**:
- Phase 1: 現状可視化（SIPOC作成、暗黙知外部化）
- Phase 2: ムダと隠れコストの特定（VSM作成、測定バイアス是正）
- Phase 3: 解決策の策定（未来マップ、ベンダー管理、上層部への説明資料）

**補足**:
- 社長のAI活用の背景: 福島県内での事業展開で補助金を得る目的
- AI活用が補助金申請の実績になる可能性
- AI活用の補助金観点は次セッション以降で検討

**次セッション**: [session262/session-plan.md](../session262/session-plan.md) — SIPOC作成準備・ヒアリング項目準備

---

## Session 262 (2026-03-19)

**概要**: SIPOC作成準備・ヒアリング項目準備・SIPOCスキル作成

**実施内容**:
1. SIPOC手法の深掘り調査（Step-by-Step、Common Mistakes、SIPOC vs VSM）
2. 既存QA知識との整合性確認（ISO 9001、IQC/IPQC/OQC、ロット管理、8D、リーン等）
3. SIPOCスキル作成（`~/.claude/skills/sipoc-facilitation/`）
4. テンプレート作成（汎用+受入検査用）
5. ヒアリング項目準備
6. Excelから事前抽出情報の作成（⚠️要修正: 具体的根拠不足）

**作成ファイル**:

| ファイル | 内容 |
|----------|------|
| [sipoc-methodology-deep-dive.md](../session262/sipoc-methodology-deep-dive.md) | SIPOC手法の深掘り調査レポート（7 sections） |
| `~/.claude/skills/sipoc-facilitation/SKILL.md` | SIPOCファシリテーションスキル |
| `~/.claude/skills/sipoc-facilitation/templates/sipoc-template.drawio` | 汎用SIPOCテンプレート |
| [sipoc-iqc-template.drawio](../session262/sipoc-iqc-template.drawio) | 受入検査用SIPOCテンプレート（Session 260の知見を反映） |
| [sipoc-workshop-hearing-items.md](../session262/sipoc-workshop-hearing-items.md) | SIPOCワークショップ用ヒアリング項目 |
| [pre-extracted-info-from-excel.md](../session262/pre-extracted-info-from-excel.md) | Excelから事前抽出した情報（⚠️要修正） |
| [session-summary.md](../session262/session-summary.md) | セッションサマリー |

**主な発見**:
- SIPOCの正しい作成順序: Process（P）から開始 → Suppliers/Inputs → Outputs → Customers
- SIPOC vs VSMの使い分け: SIPOC（高レベル俯瞰、Define）→ VSM（詳細、Measure）
- QA知識との高い整合性: ISO 9001、IQC/IPQC/OQC、ロット管理、8D、リーン等と統合可能
- 一般的な落とし穴: 詳細レベルの誤り、古いデータ、コラボレーション不足、過度な単純化

**残った課題**:
- pre-extracted-info-from-excel.md の修正（具体的な根拠（Excelの行番号、数値、日付）が不足）
- AI適用箇所の整理（現状は課題解決対象の確認・状況把握フェーズ）
- 分析ツール化の検討（Excelデータ分析、工数内訳可視化等）

**次セッション**: Session 263 — 田原さん・杉山さんとのSIPOC作成ワークショップ実施

---

## Session 263 (2026-03-19)

**概要**: SIPOCワークショップ準備完了（具体的根拠の追加、実施計画詳細化、印刷準備）

**実施内容**:
1. pre-extracted-info-from-excel.md の修正（具体的な根拠（行番号、数値、日付）を追加）
2. SIPOCワークショップ実施計画の詳細化（Phase 0-6のタイムライン、確認ポイント、質問例）
3. ワークショップ資料の印刷準備チェックリスト作成
4. 次セッション計画の立案（ドメインモデルと課題の整合性確認）

**作成ファイル**:

| ファイル | 内容 |
|----------|------|
| [pre-extracted-info-from-excel.md](../session262/pre-extracted-info-from-excel.md)（修正） | Excelから抽出した情報（具体的な行番号・品番・数値を追加） |
| [sipoc-workshop-execution-plan.md](../session263/sipoc-workshop-execution-plan.md) | SIPOCワークショップ実施計画（Phase 0-6の詳細タイムライン） |
| [print-preparation-checklist.md](../session263/print-preparation-checklist.md) | 印刷資料・デジタル資料・会場準備のチェックリスト |
| [session264/session-plan.md](../session264/session-plan.md) | 次セッション計画（ドメインモデルと課題の整合性確認） |

**主な発見**:
- 具体的根拠の重要性: Excel行番号・品番・数値を明示することでワークショップで確認しやすくなる
- ワークショップの成功要因: 詳細なタイムライン、確認ポイント、質問例を事前に用意
- 「完璧な図を描く」ことが目的ではない: 分からないことを洗い出すことが重要
- ドメインモデル確認の必要性: 開発中に新しい課題が判明したため、定期的な見直しが必要

**次セッション**: [session264/session-plan.md](../session264/session-plan.md) — ドメインモデルと課題の整合性確認

---

## Session 264 (2026-03-19)

**概要**: ドメインモデルと課題の整合性確認

**実施内容**:
1. 現在のM3ドメインモデル確認（schema.sql、Goコード）
2. Excelの課題とドメインモデルの対応確認（9つの課題カテゴリを検証）
3. ドメインモデル修正の必要性判断（優先度A/B/Cに分類）
4. 修正提案の作成（サプライヤーロット番号フィールドの追加）

**作成ファイル**:

| ファイル | 内容 |
|----------|------|
| [domain-model-review.md](../session264/domain-model-review.md) | ドメインモデルと課題の対応表 |
| [domain-model-revision-proposal.md](../session264/domain-model-revision-proposal.md) | 優先度A修正の実施計画（サプライヤーロット番号追加） |
| [files-reviewed.md](../session264/files-reviewed.md) | 確認ファイル一覧 |
| [session-summary.md](../session264/session-summary.md) | セッションサマリー |
| [session265/session-plan.md](../session265/session-plan.md) | 次セッション計画 |

**主な発見**:
- ドメインモデルは基本的に課題に対応している（表記揺れ、混在問題等は正規化で解決済み）
- 優先度Aの必須修正は1件のみ: **サプライヤーロット番号の欠落** → `lots.supplier_lot_number` フィールドを追加
- 優先度Bはワークショップで確認: 複数人作業の記録、検査記録の粒度、defect_qty のDEFAULT値
- 優先度Cは将来的に検討: 測定結果の構造化、改善提案の記録

**ドメインモデル構成**（確認結果）:
```
マスタデータ: suppliers, parts, inspection_items, workers
トランザクションデータ: lots, inspection_records, defect_reports, waivers
```

**次セッション**: [session265/session-plan.md](../session265/session-plan.md) — サプライヤーロット番号フィールドの追加（優先度A修正）

---

## Session 265 (2026-03-19)

**概要**: 田原さんヒアリング準備・プロセス確認項目の整理

**実施内容**:
1. Session 264の計画見直し（サプライヤーロット番号フィールド追加の判断を見直し）
2. AI導入方針の整理（員数確認AI化 → 梱包変更作業でのコスト削減）
3. 田原さんヒアリング項目の作成（プロセスの正確性確認に焦点）
4. プロセス粒度の問題発見（クローズドクエスチョンができない）
5. 全体フローの重要性（発注 → 出荷までの把握が必要）

**作成ファイル**:

| ファイル | 内容 |
|----------|------|
| [tahara-hearing-items.md](../session265/tahara-hearing-items.md) | 田原さんヒアリング項目（プロセス確認、使用資料整理） |
| [session-summary.md](../session265/session-summary.md) | セッションサマリー |
| [session266/session-plan.md](../session266/session-plan.md) | 次セッション計画 |

**主な発見**:
- サプライヤーロット番号の実態: ロット管理自体がされていない、中国製部品はサプライヤーロットなし → 追加は保留
- 工数可視化の問題: 田原さんに聞いても正確な数字が出せない、この「分からない」状態自体が問題
- AI導入方針の転換: 員数確認AI化 → 梱包変更作業でのコスト削減の方が効果大
- プロセスの粒度が粗すぎる: 「検査準備（梱包確認）」では抽象的すぎて、クローズドクエスチョンができない
- 全体フローの把握が必要: 発注 → 組立 → 出荷検査 → 倉庫までの全体フロー可視化

**次セッション**: [session266/session-plan.md](../session266/session-plan.md) — 全体フロー確認用SIPOC作成・受入検査プロセス詳細化

---
