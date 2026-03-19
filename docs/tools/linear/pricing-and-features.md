# Linear — 価格・機能調査レポート

---
作成: Session 268 (2026-03-19)
出典: Linear公式サイト、公式ドキュメント
---

## 目的

QA-FY26プロジェクトでの進捗管理ツールとしてLinearを検討するための調査。

---

## 1. 価格プラン

### 1.1 公式価格表

**出典**: [Linear Pricing](https://linear.app/pricing)

**原文抜粋**（WebFetch結果から）:
> **Free Plan**
> - Price: $0
> - Members: Unlimited
> - Issues: 250 issues
> - Teams: 2 teams
> - File Upload: 10MB
> - Key Features: Slack & GitHub access, AI agents
>
> **Basic Plan**
> - Price: $10 per user/month (billed yearly)
> - Members: Unlimited
> - Issues: Unlimited
> - Teams: 5 teams
> - File Upload: Unlimited
> - Key Features: All Free features plus admin roles
>
> **Business Plan**
> - Price: $16 per user/month (billed yearly)
> - Members: Unlimited
> - Issues: Unlimited
> - Teams: Unlimited
> - File Upload: Unlimited
> - Key Features: Private teams and guests, Triage Intelligence, Linear Insights, Linear Asks, Zendesk and Intercom integrations
>
> **Enterprise Plan**
> - Price: Custom (annual billing only)
> - Key Features: All Business features plus SAML/SCIM, invoice/PO billing, granular admin controls, enterprise-grade security, migration support, priority support, account management

### 1.2 価格表（整理版）

| プラン | 価格 | メンバー | Issue数 | チーム数 | ファイルアップロード | 主な機能 |
|--------|------|----------|---------|---------|---------------------|---------|
| **Free** | $0 | 無制限 | **250** | 2 | 10MB | Slack & GitHub連携、AI agents |
| **Basic** | **$10/user/月**（年払い） | 無制限 | **無制限** | 5 | 無制限 | Freeの全機能 + 管理者ロール |
| **Business** | $16/user/月（年払い） | 無制限 | 無制限 | 無制限 | 無制限 | プライベートチーム、ゲスト、Triage Intelligence、Linear Insights、Linear Asks、Zendesk/Intercom連携 |
| **Enterprise** | カスタム（年払いのみ） | 無制限 | 無制限 | 無制限 | 無制限 | SAML/SCIM、PO請求、詳細管理権限、エンタープライズセキュリティ、マイグレーション支援、優先サポート、アカウント管理 |

---

## 2. Issue数制限の詳細

### 2.1 「250 issues」の意味

**出典**: [Linear Docs - Billing and plans](https://linear.app/docs/billing-and-plans), [Delete and archive issues](https://linear.app/docs/delete-archive-issues)

**原文抜粋**（Web検索結果から）:
> "The free plan supports **up to 250 active issues with unlimited archived issues**, meaning archived issues don't count toward your limit."
>
> "If you have over 250 issues, you will no longer be able to create new issues."

**重要なポイント**:
- **制限対象**: アクティブなissueのみ（Open, In Progress, Blocked等）
- **制限対象外**: アーカイブしたissue（Archived）— **無制限**
- **超過時の挙動**: 新規issue作成が不可になる

### 2.2 自動アーカイブ機能

**出典**: [Auto-archive cycles and projects & deleting issues – Changelog](https://linear.app/changelog/2021-04-15-auto-archive-cycles-and-projects-and-deleting-issues)

**原文抜粋**:
> "Linear can **automatically close issues that haven't been updated in a set period** and can also **automatically archive closed issues** to un-clutter users' views after a while."

**機能**:
1. 一定期間更新されないissueを自動クローズ
2. クローズされたissueを自動アーカイブ

**運用への影響**:
- 完了したissueを随時アーカイブすれば、Freeプランでも長期運用可能
- ただし、**アクティブなissueが250を超える場合は不可**

---

## 3. QA-FY26プロジェクトでのコスト試算

### 3.1 Issue数見積もり

**詳細**: [session268/issue-estimate.md](../../../sessions/session268/issue-estimate.md)

**結論**:
- Phase 1-3（受入検査改善）: 総55-60 issue、アクティブ最大41
- 他ミッション（M1-M4等）: 総110-170 issue、アクティブ30-60
- **プロジェクト全体**: 総165-230 issue、**アクティブ71-101**

### 3.2 Freeプランの可否

| 判断 | 根拠 |
|------|------|
| ❌ **Freeプランでは不足** | アクティブissueが71-101に達し、250の上限に対して余裕がない |
| ⚠️ **運用上のリスク** | 新規issue作成不可になる可能性が高い |
| ✅ **Basicプランが必要** | アクティブissueを無制限に扱える |

### 3.3 Basicプランのコスト

**前提条件**:
- 利用者: 藤田さん + 田原さん + 杉山さん（＋α: いしかわさん、こいたばしさん等） = 3-5名

| 人数 | 月額（年払い） | 年額 |
|------|--------------|------|
| 3名 | **$30** | **$360** |
| 5名 | **$50** | **$600** |

**判定**: 年$360-600は許容範囲内のコスト

---

## 4. Claude Codeとの連携

### 4.1 Linear MCP Server（推奨）

**MCP（Model Context Protocol）サーバー**

**出典**: [Linear MCP Server](https://linear.app/docs/mcp)

**原文抜粋**（WebFetch結果から）:
> "The Linear MCP server is **a standardized interface enabling AI models and agents to access Linear data securely**. As described, it 'provides a standardized interface that allows any compatible AI model or agent to access your Linear data in a simple and secure way.'"
>
> "The server enables **finding, creating, and updating objects in Linear like issues, projects, and comments** with additional functionality planned."

**セットアップ手順**（原文）:
> "**For Claude Code:**
> Users execute `claude mcp add --transport http linear-server https://mcp.linear.app/mcp`, then run `/mcp` in a session to authenticate."

**手順**:
```bash
# 1. MCP Serverを追加
claude mcp add --transport http linear-server https://mcp.linear.app/mcp

# 2. Claude Codeセッション内で認証
/mcp
# → OAuth認証フローが開始
```

**機能**:
- Issue、Project、Commentの検索・作成・更新
- 自然言語でLinear操作（例: "Create a Linear issue for プレート調査"）
- OAuth 2.1認証（セキュア）

### 4.2 Linear CLI（代替案）

複数のLinear CLI実装が存在します。

**出典**:
- [GitHub - schpet/linear-cli](https://github.com/schpet/linear-cli)
- [GitHub - Finesssee/linear-cli](https://github.com/Finesssee/linear-cli)
- [GitHub - AdiKsOnDev/linear-cli](https://github.com/AdiKsOnDev/linear-cli)

**schpet/linear-cli（Node.js）**:
```bash
# インストール
npm install -g @schpet/linear-cli

# 認証
linear auth login
# または環境変数
export LINEAR_API_KEY="lin_api_..."

# Issue作成
linear issue create --title "タイトル" --description "詳細"
```

**Finesssee/linear-cli（Rust）**:
```bash
# インストール（Cargo）
cargo install linear-cli

# 認証
linear-cli auth login
# またはAPIキー設定
linear-cli config set-key lin_api_xxxxxxxxxxxxx

# Issue作成
linear-cli issue create "タイトル"
```

**推奨**: **Linear MCP Server**を使用（Claude Codeとの親和性が高い）

---

## 5. API・Webhook

### 5.1 GraphQL API

**出典**: [Linear Developers](https://linear.app/developers), [Getting started – Linear Developers](https://linear.app/developers/graphql)

**原文抜粋**（Web検索結果から）:
> "Linear's public API is built using GraphQL"

**機能**:
- Issue、Comment、Project、Cycle等の操作
- OAuth 2.0 または Personal API Key で認証
- 公式SDKあり（Node.js, Python等）

### 5.2 Webhook

**出典**: [Webhooks – Linear Developers](https://linear.app/developers/webhooks)

**原文抜粋**（Web検索結果から）:
> "Linear provides webhooks which allow you to **receive HTTP push notifications whenever data is created, updated or removed**, which allows you to build integrations that respond to changes in real time, such as triggering CI builds, updating external systems, or sending messages based on issue activity."
>
> "Linear supports 'data change webhooks' for various models and events, including **issues, issue attachments, issue comments, issue labels, comment reactions, projects, project updates, cycles, issue SLA, and OAuthApp revoked**."

**セキュリティ**:
- HMAC署名による検証
- IP address validation

**作成方法**:
1. Linear UI経由
2. GraphQL API経由（プログラマティック）

---

## 6. エクスポート機能

### 6.1 標準機能

- CSVエクスポート（Issue一覧）
- Markdown変換（Issue本文）

### 6.2 API経由

- GraphQL APIでデータ取得
- カスタムエクスポート処理を実装可能

---

## 7. 他ツールとの比較

### 7.1 競合ツール

| ツール | Freeプラン | 有料プラン | Claude Code連携 | 備考 |
|--------|-----------|-----------|-----------------|------|
| **Linear** | 250 active issues | $10/user/月（無制限） | ✅ MCP Server | 本調査対象 |
| **ClickUp** | 100MB, 無制限タスク | $7/user/月 | ❌ なし | 価格は安いが連携弱い |
| **Asana** | 無制限タスク（15名まで） | $10.99/user/月 | ❌ なし | Freeプランは魅力的 |
| **GitHub Projects** | 無制限（Gitリポジトリ必要） | 無料 | 🟡 GitHub連携 | 技術プロトタイプ向き |

**出典**:
- [7 Best Linear Alternatives (2026): Cheaper, More Customizable Options](https://www.ideaplan.io/alternatives/linear)
- ClickUp公式サイト
- Asana公式サイト
- GitHub Projects公式サイト

### 7.2 GitHub Projectsの検討

**メリット**:
- ✅ 完全無料、無制限issue
- ✅ Git連携が強い（技術プロトタイプ向き）
- ✅ Claude CodeからGitHub CLIで操作可能

**デメリット**:
- ❌ コラボレーション機能が弱い（田原さん・杉山さん向けではない）
- ❌ 進捗可視化がLinearより劣る
- ❌ 非技術者には学習コストが高い

**結論**: M1-GNSS等の技術プロトタイプには適しているが、QA-FY26全体の進捗管理には不向き

---

## 8. 推奨事項

### 8.1 採用プラン

**推奨**: **Linear Basicプラン**（$10/user/月、年払い）

**理由**:
1. **Issue数の見積もり**: アクティブ71-101に達し、Freeプラン（250上限）では余裕がない
2. **Claude Code連携**: MCP Serverによる強力な連携
3. **進捗可視化**: ダッシュボード、フィルター、カスタムビューが充実
4. **コスト**: 年$360-600は許容範囲内
5. **コラボレーション**: Slack連携、コメント、通知が強力

### 8.2 運用方法

**ハイブリッド運用**（Linear + Markdown）:
- **Linear**: 進捗管理・コラボレーション・可視化
- **Markdown**: 技術詳細・設計判断・履歴管理

**Workspace/Project構成**:
```
Workspace: QA-FY26
├── Project: Phase 1（現状可視化）
│   ├── Epic: SIPOC作成
│   ├── Epic: 暗黙知外部化
│   └── Epic: 隠れコスト特定
├── Project: Phase 2（ムダと隠れコスト分析）
└── Project: Phase 3（解決策策定）
```

**セッション終了時のワークフロー**:
1. 完了したIssueを`Done`に変更
2. `Done`になったIssueをアーカイブ（アクティブ数を削減）
3. 次セッションの計画をIssueに反映
4. session-summary.mdにも記録（冗長性確保）

---

## 9. 次ステップ

### 9.1 即時実施

- [ ] Linear Basicプラン申し込み（年払い）
- [ ] Linear MCP Serverのセットアップ
- [ ] QA-FY26 Workspace作成
- [ ] Phase 1 Project作成
- [ ] 初期Issue作成（プレート調査等）

### 9.2 今後の検討

- コラボレーションパターンの確立（田原さん・杉山さんとの共有方法）
- 週次/月次の進捗レビュー方法
- GitHub Projectsとの併用検討（M1-GNSS等の技術プロトタイプ）

---

## 参照

### 公式ドキュメント

- [Linear Pricing](https://linear.app/pricing) — 価格プラン
- [Linear MCP Server](https://linear.app/docs/mcp) — Claude Code連携
- [Linear Developers](https://linear.app/developers) — API・Webhook
- [Linear Webhooks](https://linear.app/developers/webhooks) — Webhook詳細
- [Billing and plans – Linear Docs](https://linear.app/docs/billing-and-plans) — Issue制限の詳細
- [Delete and archive issues](https://linear.app/docs/delete-archive-issues) — アーカイブ機能

### Linear CLI実装

- [GitHub - schpet/linear-cli](https://github.com/schpet/linear-cli) — Node.js実装
- [GitHub - Finesssee/linear-cli](https://github.com/Finesssee/linear-cli) — Rust実装
- [GitHub - AdiKsOnDev/linear-cli](https://github.com/AdiKsOnDev/linear-cli) — Python実装

### 比較・レビュー

- [Linear Review: Features, Pricing, Pros & Cons 2026](https://work-management.org/software-development/linear-review/)
- [7 Best Linear Alternatives (2026): Cheaper, More Customizable Options](https://www.ideaplan.io/alternatives/linear)
- [Linear Vs. Jira Comparison: Which Tool is Best for 2026 | ClickUp](https://clickup.com/blog/linear-vs-jira/)

### Webhook実装ガイド

- [Linear Webhooks: Complete Guide with Payload Examples [2025]](https://inventivehq.com/blog/linear-webhooks-guide)
- [Linear integration | docs](https://docs.trunk.io/flaky-tests/webhooks/linear-integration)

---

**最終更新**: Session 268 (2026-03-19)
**次回レビュー**: Linear導入後、Phase 1完了時（Issue数の実績確認）
