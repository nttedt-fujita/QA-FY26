# Session 268 サマリー

**日付**: 2026-03-19

**目的**: Linear調査・進捗管理の仕組み決定

---

## 実施内容

### 1. Linear調査（コスト、機能、運用）

**価格プラン調査**:
- 公式サイト（linear.app/pricing）から価格情報を取得
- Freeプラン: $0（250 active issues、メンバー無制限）
- Basicプラン: $10/user/月（無制限issues）
- Businessプラン: $16/user/月（高度な分析機能）
- Enterpriseプラン: カスタム価格

**重要な発見**:
- **250 issueの制限はアクティブissueのみ**
- アーカイブしたissueは無制限（出典: Linear公式ドキュメント）
- 自動アーカイブ機能あり

**Claude Code連携**:
- Linear MCP Server提供（OAuth 2.1認証）
- セットアップ: `claude mcp add --transport http linear-server https://mcp.linear.app/mcp`
- Issue、Project、Commentの検索・作成・更新が可能

### 2. Issue数見積もり

**QA-FY26プロジェクト全体**:
- 総Issue数: 165-230
- **アクティブ数（最大）: 71-101**

**内訳**:
| 項目 | 総Issue数 | アクティブ数（最大） |
|------|-----------|---------------------|
| 受入検査改善（Phase 1-3） | 55-60 | 41 |
| 他ミッション（M1-M4等） | 110-170 | 30-60 |

**Phase 1の詳細**（アクティブ14）:
- SIPOC作成: 2（組立検査、出荷検査）
- 暗黙知外部化: 5（部品調査3-5カテゴリ）
- 隠れコスト特定: 7（工数調査7項目）

**結論**: Freeプラン（250上限）では不足 → **Basicプラン必要**

### 3. コスト試算

| 人数 | 月額（年払い） | 年額 |
|------|--------------|------|
| 3名（藤田、田原、杉山） | $30 | **$360** |
| 5名（+いしかわ、こいたばし） | $50 | **$600** |

**判定**: 年$360-600は許容範囲内のコスト

### 4. 進捗管理の仕組み決定

**採用方針**: **Linear + Markdown ハイブリッド運用**

**役割分担**:
- **Linear**: 進捗管理・コラボレーション・可視化
- **Markdown**: 技術詳細・設計判断・履歴管理

**二重管理の禁止**:
- Linear: タスク概要 + Markdown資料へのリンク
- Markdown: 詳細な内容

---

## 主な発見

### 1. 250 issue制限の真実

**誤解**: 「Freeプランは総Issue数250まで」
**真実**: 「アクティブissueのみ250まで、アーカイブは無制限」

**出典**（原文抜粋）:
> "The free plan supports **up to 250 active issues with unlimited archived issues**, meaning archived issues don't count toward your limit."
>
> （出典: [Linear Docs - Billing and plans](https://linear.app/docs/billing-and-plans)）

### 2. アクティブissueの見積もり

**最悪ケース**: 71-101（全Phase・全ミッションが同時進行）
**現実的なケース**: 30-50（1つのPhase + 他ミッション）

**それでもBasicプラン推奨の理由**:
1. 安全マージン（予期しないタスク）
2. プロジェクト拡大
3. コストは許容範囲（年$360-600）

### 3. Claude Code連携の優位性

**Linear MCP Server**:
- 自然言語でLinear操作（例: "Create a Linear issue for プレート調査"）
- OAuth 2.1認証（セキュア）
- Issue、Project、Commentの検索・作成・更新

**他ツールとの比較**:
- ClickUp, Asana: Claude Code連携なし
- GitHub Projects: 無料だがコラボレーション弱い

---

## 成果物

### 正式ドキュメント（docs/tools/linear/）

| ファイル | 内容 |
|----------|------|
| [pricing-and-features.md](../../docs/tools/linear/pricing-and-features.md) | Linear価格・機能調査レポート（**出典・原文抜粋付き**） |
| [issue-estimate.md](../../docs/tools/linear/issue-estimate.md) | Issue数見積もり（Phase 1-3 + 他ミッション） |
| [hybrid-operation-rules.md](../../docs/tools/linear/hybrid-operation-rules.md) | Linear + Markdown ハイブリッド運用ルール |

### セッション資料（sessions/session268/）

| ファイル | 内容 |
|----------|------|
| [linear-research-report.md](linear-research-report.md) | Linear調査レポート（初期版、後に docs/ へ移動） |
| [issue-estimate.md](issue-estimate.md) | Issue数見積もり（初期版、後に docs/ へ移動） |
| [issue-estimate-explanation.md](issue-estimate-explanation.md) | Issue数見積もりの根拠と内訳（詳細解説） |
| [session-plan.md](session-plan.md) | セッション計画 |
| [session-summary.md](session-summary.md) | 本ファイル |

### 更新ファイル

| ファイル | 変更内容 |
|----------|----------|
| [docs/index.md](../../docs/index.md) | プロジェクト管理ツールセクション追加、tools/linear/ 索引追加 |

---

## 残った課題

### 未実施

- [ ] Linear MCP Serverのセットアップ（次セッションで実施）
- [ ] Linear Workspace作成
- [ ] 初期Issue作成（プレート調査等）
- [ ] phase1-progress.md 作成

### 検討事項

- [ ] コラボレーション方法の確立（田原さん・杉山さんとの共有）
- [ ] Linearの実運用開始タイミング
- [ ] GitHub Projectsとの併用検討（M1-GNSS等）

---

## 次セッションでやること

**優先度**:
1. **ユーザーから「色々言いたいことがある」** → ヒアリング・方針確認
2. Linear MCP Serverセットアップ
3. phase1-progress.md 作成（Markdown形式で進捗管理開始）
4. Linear Workspace作成（セットアップ後）

---

## 使用したツール・リソース

### Web検索

| クエリ | 目的 |
|--------|------|
| "Linear project management pricing 2026 free plan" | 価格プラン調査 |
| "Linear free plan 250 issues limit archived closed deleted 2026" | Issue制限の詳細確認 |
| "Linear CLI installation authentication commands 2026" | CLI調査 |
| "Linear API GraphQL webhook integration 2026" | API・Webhook調査 |

### WebFetch

| URL | 目的 |
|-----|------|
| https://linear.app/pricing | 公式価格表取得 |
| https://linear.app/docs/mcp | MCP Server詳細取得 |

---

## 参照

- [session267/session-summary.md](../session267/session-summary.md) — 前セッションサマリー
- [session267/context-and-background.md](../session267/context-and-background.md) — 全体文脈整理
- [session-plan.md](session-plan.md) — 本セッション計画
- [docs/tools/linear/pricing-and-features.md](../../docs/tools/linear/pricing-and-features.md) — Linear調査レポート（正式版）

---

**最終更新**: Session 268 (2026-03-19)
**次セッション**: Session 269 — ユーザーヒアリング + Linear導入判断
