# Phase 1: 設定構成の棚卸し

**目的**: 全ルール・スキル・CLAUDE.mdの責務を一覧化し、何がどこに書かれているかを把握する

**作成**: Session 284 (2026-03-23)

---

## 1. ルール一覧（17個）

| # | ファイル | 責務（1行要約） | カテゴリ |
|---|----------|----------------|---------|
| 01 | core-principles.md | 推測禁止・段階的検証・禁止ワードの定義 | コア原則 |
| 02 | coding-style.md | 日本語コメント・350行制限・既存パターン踏襲 | コーディング |
| 03 | work-cycle.md | 探索→計画→成果物→コミットの作業サイクル | ワークフロー |
| 04 | session-start.md | セッション開始時の必須処理（session-management/energy-management呼び出し） | セッション管理 |
| 05 | requirement-first.md | What(要求)とHow(要件)の区別・EARS・要求からの乖離チェック | 要件定義 |
| 06 | test-style.md | テーブルテスト形式・should_succeed・命名規則 | テスト |
| 07 | information-routing.md | 情報配置の判断フロー（rules/CLAUDE.md/SKILL.md/MEMORY.md/session-history） | 情報管理 |
| 08 | memory-management.md | MEMORY.mdの30行制限・書いてよい/いけない内容 | 情報管理 |
| 09 | tdd-skill-reference.md | TDD作業時にtdd-practiceスキルを読むことを強制 | テスト |
| 10 | adr-enforcement.md | 設計判断時のADR必須化・不変(Immutable)ルール | 設計判断 |
| 11 | hooks-review.md | セッション終了時のhooks振り返りを強制 | セッション管理 |
| 12 | makefile-structure.md | Makefileの分割構成・命名規則 | コーディング |
| 13 | spec-first-implementation.md | プロトコル実装前に仕様書を読むことを強制 | 実装 |
| 14 | document-management.md | 出典必須・番号自動採番・二重管理禁止・Curation over Creation | 情報管理 |
| 15 | pdf-handling.md | PDF直接読み込み禁止・Pythonで抽出・抽出状態テーブル管理 | 情報管理 |
| 16 | command-reference.md | Makeコマンド・API呼び出し前のリファレンス確認を強制 | 実装 |
| 17 | fact-vs-hypothesis.md | 推測と事実の分離・出典明記・仮説の明示 | コア原則 |

### カテゴリ別集計

| カテゴリ | 個数 | ルール番号 |
|----------|------|-----------|
| コア原則 | 2 | 01, 17 |
| コーディング | 2 | 02, 12 |
| ワークフロー | 1 | 03 |
| セッション管理 | 2 | 04, 11 |
| 要件定義 | 1 | 05 |
| テスト | 2 | 06, 09 |
| 情報管理 | 4 | 07, 08, 14, 15 |
| 設計判断 | 1 | 10 |
| 実装 | 2 | 13, 16 |

---

## 2. スキル一覧（22個）

| # | スキル名 | 責務（1行要約） | カテゴリ |
|---|----------|----------------|---------|
| 01 | adr-management | ADR作成・管理の手順とテンプレート | 設計判断 |
| 02 | claude-code-reference | Claude Code公式ドキュメント・モデル運用ルール | リファレンス |
| 03 | claude-code-setup | Claude Code設定階層・CLAUDE.md配置・Skills構造 | リファレンス |
| 04 | design-review | 新規ノード/クラス作成時の設計レビューチェックリスト | 設計判断 |
| 05 | dev-workflow | 4フェーズ開発（要件→基本設計→詳細設計→実装） | ワークフロー |
| 06 | diagram-design | Draw.io図作成時のデザイン原則（近接・整列・反復・対比） | 図表作成 |
| 07 | domain-modeling | 実装前のドメインモデリング手法 | 設計判断 |
| 08 | energy-management | ADHD向けエネルギー・集中管理（体調チェック） | セッション管理 |
| 09 | kougo-response | 向後さんからの質問・指摘への対応パターン | コミュニケーション |
| 10 | linear-management | Linearの無料プラン運用・アクティブissue監視 | プロジェクト管理 |
| 11 | marp-slides | Marpスライド作成のテンプレート・ガイド | 図表作成 |
| 12 | panda3d-source-verification | Panda3D内部実装の調査で推測を防ぐ検証手順 | リファレンス |
| 13 | qa-strategy | Subdomains分類に基づくQA投資配分・スコープ判断 | 品質管理 |
| 14 | requirements-definition | MECEとロジックツリーを使った要件定義フレームワーク | 要件定義 |
| 15 | session-management | セッション間の引き継ぎフレームワーク | セッション管理 |
| 16 | sipoc-facilitation | SIPOCダイアグラム作成のファシリテーションガイド | プロセス改善 |
| 17 | skill-authoring-guide | Agent Skills公式仕様に基づくスキル作成ガイド | リファレンス |
| 18 | tdd-practice | 古典派TDDの振る舞いテスト駆動開発 | テスト |
| 19 | team-communication | チーム開発でのコミュニケーション・合意形成 | コミュニケーション |
| 20 | token-monitoring | トークン使用量のモニタリング・分析 | セッション管理 |
| 21 | ui-color-design | UIカラー設計・アクセシビリティガイドライン | UI設計 |
| 22 | (**欠番確認中**) | - | - |

**注**: skills/ディレクトリには23フォルダあるが、SKILL.mdがあるのは22個

### カテゴリ別集計

| カテゴリ | 個数 | スキル番号 |
|----------|------|-----------|
| セッション管理 | 3 | 08, 15, 20 |
| 設計判断 | 3 | 01, 04, 07 |
| リファレンス | 4 | 02, 03, 12, 17 |
| ワークフロー | 1 | 05 |
| 図表作成 | 2 | 06, 11 |
| コミュニケーション | 2 | 09, 19 |
| プロジェクト管理 | 1 | 10 |
| 品質管理 | 1 | 13 |
| 要件定義 | 1 | 14 |
| プロセス改善 | 1 | 16 |
| テスト | 1 | 18 |
| UI設計 | 1 | 21 |

---

## 3. CLAUDE.md 責務

### 3.1 グローバル CLAUDE.md (`~/.claude/CLAUDE.md`)

| セクション | 責務 |
|-----------|------|
| 基本方針 | 推測禁止・段階的検証・設計レビュースキル参照 |
| 禁止ワード | 使ってはいけない表現 |
| コーディングスタイル | 日本語コメント・350行制限・既存パターン踏襲 |
| ADR管理 | docs/adr/に記録すること |
| 作業サイクル | 探索→計画→成果物→コミット |
| スキル一覧 | 利用可能なスキルのインデックス |
| エージェント一覧 | 利用可能なエージェントのインデックス |
| コマンド一覧 | /plan, /tdd, /onboarding, /ticket |

**重複疑い**:
- 「基本方針」→ rule 01-core-principles.md と重複
- 「禁止ワード」→ rule 01-core-principles.md と重複
- 「コーディングスタイル」→ rule 02-coding-style.md と重複
- 「ADR管理」→ rule 10-adr-enforcement.md と重複
- 「作業サイクル」→ rule 03-work-cycle.md と重複

### 3.2 プロジェクト CLAUDE.md (`QA-FY26/CLAUDE.md`)

| セクション | 責務 |
|-----------|------|
| 概要 | プロジェクト説明・チームメンバー |
| セッションディレクトリ | sessions/の配置ルール |
| ドキュメント配置ルール | sessions/はドラフト、docs/に正式配置 |
| Makeコマンドルール | Makefile確認必須 |
| 現在の進捗 | docs/progress.mdへの参照 |
| 担当ミッション | 藤田さんの4ミッション |
| ファイル構成 | ディレクトリ説明 |
| ドキュメント構成 | docs/の構造 |
| 現状の業務環境 | Excel管理、Go+Next.js方針 |
| 環境変数・API Key | .env配置 |
| Linear運用 | Freeプラン運用・監視基準 |
| 品質管理設計ルール | qa-design-reviewスキル参照 |
| ADR一覧 | ミッション別ADRインデックス |
| 過去セッションの重要な決定 | 重要判断サマリ |

**重複疑い**:
- 「Makeコマンドルール」→ rule 16-command-reference.md と重複
- 「Linear運用」→ skill linear-management と重複（ただしインデックスとしては適切）

---

## 4. 責務の重複マトリクス（疑い）

| 責務 | ルール | スキル | グローバルCLAUDE.md | プロジェクトCLAUDE.md |
|------|--------|--------|-------------------|---------------------|
| 推測禁止 | 01, 17 | - | ✅ | - |
| 禁止ワード | 01 | - | ✅ | - |
| コーディングスタイル | 02 | - | ✅ | - |
| 作業サイクル | 03 | - | ✅ | - |
| セッション管理 | 04, 11 | 08, 15, 20 | - | - |
| 要件定義 | 05 | 14 | - | - |
| テスト | 06, 09 | 18 | - | - |
| 情報管理 | 07, 08, 14, 15 | - | - | ✅ (配置ルール) |
| ADR | 10 | 01 | ✅ | ✅ (一覧) |
| Makeコマンド | 12, 16 | - | - | ✅ |

---

## 5. 依存関係

### 5.1 ルール → スキル の参照

| ルール | 参照先スキル |
|--------|-------------|
| 04-session-start | session-management, energy-management |
| 09-tdd-skill-reference | tdd-practice |

### 5.2 スキル → スキル の参照

| スキル | 参照先スキル |
|--------|-------------|
| session-management | energy-management, linear-management |
| dev-workflow | adr-management, session-management |

### 5.3 スキル → ルール の参照

| スキル | 参照先ルール |
|--------|-------------|
| session-management | (06-test-style を暗黙的に想定) |

---

## 6. Phase 2で検証すべき問題仮説

1. **ルール01 vs ルール17**: 「推測禁止」と「推測と事実の分離」は統合可能か？
2. **ルール07 vs ルール14**: 「情報配置ルール」と「ドキュメント管理ルール」は統合可能か？
3. **グローバルCLAUDE.md vs ルール群**: CLAUDE.mdはインデックスに徹するべきか？
4. **セッション管理系**: ルール04, 11 とスキル08, 15, 20 の役割分担は適切か？

---

## 7. 次のアクション（Phase 2）

1. 上記の重複疑いを精査し、実際に重複しているかを確認
2. 重複している場合、SSOTの観点からどちらを正とするか決定
3. phase2-problems.md を作成

---

*作成: Session 284 (2026-03-23)*
