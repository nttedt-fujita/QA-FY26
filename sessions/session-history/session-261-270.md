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

## Session 266 (2026-03-19)

**概要**: 全体フロー確認用SIPOC作成・受入検査プロセス詳細化

**実施内容**:
1. 全体フロー確認用SIPOC作成（発注 → 出荷、10プロセス）
2. 受入検査プロセスの詳細化（5プロセス → 11プロセスに分解）
3. プロセス確認用チェックリスト作成（クローズドクエスチョン形式）

**作成ファイル**:

| ファイル | 内容 |
|----------|------|
| [overall-flow-sipoc-template.drawio](../session266/overall-flow-sipoc-template.drawio) | 全体フローSIPOC（発注 → 出荷、10プロセス） |
| [iqc-detailed-sipoc-template.drawio](../session266/iqc-detailed-sipoc-template.drawio) | 受入検査詳細SIPOC（11プロセス、梱包変更作業を明示化） |
| [process-checklist.md](../session266/process-checklist.md) | プロセス確認用チェックリスト（✅やってる/❌やってない/△部分的） |
| [session-summary.md](../session266/session-summary.md) | セッションサマリー |
| [session267/session-plan.md](../session267/session-plan.md) | 次セッション計画 |

**主な改善点**:

### 1. プロセス粒度の詳細化

**改善前（Session 262、粗い粒度）**:
1. 部品入荷
2. 検査準備（梱包確認）← 抽象的すぎる
3. 検査実施（外観・員数）
4. 記録作成（Excel入力）
5. 入庫/不合格処理

**改善後（Session 266、詳細化）**:
1. 部品入荷（納品受け取り）
2. 作業場所への移動
3. 検査段取り（場所・人員確保）
4. 開梱作業
5. **梱包変更作業**（Session 260の隠れコスト）
6. 員数確認
7. 外観検査
8. 測定（ノギス等）
9. 記録作成（Excel入力）
10. 入庫のための梱包作業
11. 入庫

### 2. クローズドクエスチョン形式のチェックリスト

**構成**:
- Part 1: 全体フロー（10プロセス）の確認
- Part 2: 受入検査詳細（11プロセス）の確認
  - 梱包変更作業について（最重要）
  - 員数確認、外観検査、測定、Excel記録
  - 抜けている作業の確認
- Part 3: 工数・時間に関する確認（相対的な比較）
- Part 4: 暗黙知の可視化

**効果**:
- 田原さん・杉山さんが「✅やってる」「❌やってない」「△部分的」で答えられる
- △や❌の場合は追加でヒアリング

**次セッション**: [session267/session-plan.md](../session267/session-plan.md) — 田原さん・杉山さんとのSIPOCワークショップ実施

---

## Session 267 (2026-03-19)

**概要**: 現場ヒアリング試行・全体文脈整理・進捗管理の仕組み検討

**実施内容**:
1. 田原さんに軽くヒアリング（忙しくて詳細は無理）
2. プレートの作業実態を発見（暗黙知と隠れコスト）
3. Session 241-267の文脈整理（AI外観検査 → AI員数確認 → 作業フロー可視化の経緯）
4. 要求（What）への立ち返り（コスト削減と情報の可視化）
5. 進捗管理の仕組み検討（Linear vs Markdown vs ハイブリッド）

**作成ファイル**:

| ファイル | 内容 |
|----------|------|
| [context-and-background.md](../session267/context-and-background.md) | 全体の文脈整理（Session 241-267の経緯、要求の変遷、Phase構成） |
| [session-summary.md](../session267/session-summary.md) | セッションサマリー |
| [session268/session-plan.md](../session268/session-plan.md) | 次セッション計画 |

**主な発見（プレートの作業実態）**:

### 暗黙知（やっている作業）
- シリアル番号突合作業（袋の表記と実物の印字を照合）
- 印字品質確認（色合い・視認性チェック）
- 小分け作業の実態（納品 → 入庫の間に小分け）

### 隠れコスト（無駄な作業）
- **小分け再作業**: 出庫先で基準が違い、再度小分けし直し（一貫性がない）
- **サイレントチェンジ対応**: 印字色が勝手に変わり、都度確認が必要（サプライヤー管理の弱さ）

**全体の文脈整理**:

### 要求（What）の変遷
1. 当初（Session 242-247）: 不良品の市場流出を防ぐ、AI検査で工数削減
2. 方向転換1（Session 256）: AI外観検査 → AI員数確認
3. 方向転換2（Session 260）: AI化 → 作業フロー全体の可視化
4. 現在（Session 260-267）: コスト削減と情報の可視化（Phase 1-3構成）

### Phase構成（Session 261で策定）
- Phase 1: 現状可視化（SIPOC作成 ✅、暗黙知外部化 🟡、隠れコスト特定 🟡）
- Phase 2: ムダと隠れコストの特定（VSM作成、8つのムダ分析）
- Phase 3: 解決策の策定（未来マップ、ベンダー管理、上層部資料）

### 時間軸
- 3ヶ月: Phase 1完了目標
- 1年後くらい: コスト削減と情報の可視化を達成

**進捗管理の仕組み検討**:
- Linear vs Markdown進捗管理の比較
- ハイブリッド案の提案（Linear: タスク管理・コラボレーション、Markdown: 技術詳細・履歴）
- Linearのコスト調査が必要（次セッション）

**次セッション**: [session268/session-plan.md](../session268/session-plan.md) — Linear調査・進捗管理の仕組み決定

---

## Session 268 (2026-03-19)

**概要**: Linear調査・進捗管理の仕組み決定

**実施内容**:
1. Linear価格・機能調査（公式サイト、MCP Server、CLI、API）
2. Issue数見積もり（QA-FY26プロジェクト全体）
3. コスト試算（Basicプラン $10/user/月、年$360-600）
4. 進捗管理の仕組み決定（Linear + Markdown ハイブリッド運用）

**作成ファイル**:

| ファイル | 内容 |
|----------|------|
| [docs/tools/linear/pricing-and-features.md](../../docs/tools/linear/pricing-and-features.md) | Linear価格・機能調査レポート（**出典・原文抜粋付き**） |
| [docs/tools/linear/issue-estimate.md](../../docs/tools/linear/issue-estimate.md) | Issue数見積もり（Phase 1-3 + 他ミッション） |
| [docs/tools/linear/hybrid-operation-rules.md](../../docs/tools/linear/hybrid-operation-rules.md) | Linear + Markdown ハイブリッド運用ルール |
| [session268/issue-estimate-explanation.md](../session268/issue-estimate-explanation.md) | Issue数見積もりの根拠と内訳（詳細解説） |
| [session268/session-summary.md](../session268/session-summary.md) | セッションサマリー |
| [session269/session-plan.md](../session269/session-plan.md) | 次セッション計画 |

**主な発見**:
- **250 issue制限の真実**: アクティブissueのみ250まで、アーカイブは無制限
- **アクティブissue見積もり**: 71-101（最悪ケース）、30-50（現実的）
- **Basicプラン必要**: Freeプラン（250上限）では不足、年$360-600は許容範囲
- **Claude Code連携**: Linear MCP Serverで強力な連携が可能

**調査結論**:
- **推奨**: Linear Basicプラン採用（$10/user/月、年$360-600）
- **運用方針**: Linear + Markdown ハイブリッド運用
  - Linear: 進捗管理・コラボレーション・可視化
  - Markdown: 技術詳細・設計判断・履歴管理

**出典付き調査の徹底**:
- 全ての調査レポートに出典URLと原文抜粋を併記
- 推測と事実を明確に分離
- docs/ に正式配置（然るべきところへの設置）

**次セッション**: [session269/session-plan.md](../session269/session-plan.md) — ユーザーヒアリング + Linear導入判断

---

## Session 269 (2026-03-19)

**概要**: Linear Free導入完了・次セッション方針確定（説明資料作り優先）

**実施内容**:
1. 要求（What）の確認と方針決定（本来のミッション: 宇枝さん説明資料作り）
2. Linear Free Workspace作成（QA-FY26-FUJITA）
3. Linear MCP Server設定・Personal API Key発行
4. セキュア管理（.env / .env.example / .gitignore）
5. 次セッション計画作成（説明資料作り優先）

**作成ファイル**:

| ファイル | 内容 |
|----------|------|
| `.env` | Linear API Key（Git管理外） |
| `.env.example` | API Keyフォーマット例（Git管理下） |
| `.gitignore` | `.env`を除外設定 |
| [session269/session-summary.md](../session269/session-summary.md) | セッションサマリー |
| [session270/session-plan.md](../session270/session-plan.md) | 次セッション計画（説明資料作り優先） |

**主な発見**:
- **Linearの課金単位**: Workspace単位で独立して課金
- **複数Workspace対応**: 1アカウントで複数Workspaceに参加可能（ソフトチームとQAを分離）
- **Team ID確認方法**: URL（linear.app/.../settings/teams/XXX）で確認
- **API Key発行場所**: Workspace設定ではなく、個人設定（Security & access）
- **要求の再確認**: Session 268でLinear調査に1日費やした → 本来のミッション（説明資料作り）に戻る

**Linear導入の状態**:
- ✅ Workspace作成済み（QA-FY26-FUJITA、Team ID: QA）
- ✅ MCP Server設定済み（~/.claude.json）
- ✅ API Key発行・保存済み（.env）
- ⏳ 動作確認（次セッションで）

**方針転換（要求への立ち返り）**:
- Session 268でLinear調査に時間を費やした → 今回で区切り
- **本来のミッション**: 宇枝さんへの説明資料作り（AI導入は厳しい、手を入れる部分が違う）
- **次セッション優先順位**: 説明資料作り > GNSS作業

**次セッション**: [session270/session-plan.md](../session270/session-plan.md) — 宇枝さん説明資料作成開始（AI導入の現実を伝える）

---
