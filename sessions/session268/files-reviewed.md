# 確認ファイル一覧（Session 268）

**作成**: 2026-03-19

---

## プロジェクト内ドキュメント

| ファイル | 内容 | 確認目的 |
|----------|------|----------|
| [session267/session-summary.md](../session267/session-summary.md) | 前セッションサマリー | 前回コンテキスト確認 |
| [session267/context-and-background.md](../session267/context-and-background.md) | 全体文脈整理（Session 241-267の経緯） | Phase構成確認 |
| [session268/session-plan.md](session-plan.md) | 本セッション計画 | 作業内容確認 |
| [docs/index.md](../../docs/index.md) | ドキュメントインデックス | 既存構成確認、索引追加 |

---

## Web検索で調査した情報

### 1. Linear価格プラン

| クエリ | 参照先 | 確認内容 |
|--------|--------|----------|
| "Linear project management pricing 2026 free plan" | [CompareTiers](https://comparetiers.com/tools/linear), [Top3.software](https://www.top3.software/r/linear-pricing) | 価格プラン概要 |

**主な発見**:
- Freeプラン: $0（250 active issues、メンバー無制限）
- Basicプラン: $10/user/月（無制限issues）

### 2. Issue数制限の詳細

| クエリ | 参照先 | 確認内容 |
|--------|--------|----------|
| "Linear free plan 250 issues limit archived closed deleted 2026" | [Billing and plans](https://linear.app/docs/billing-and-plans), [Delete and archive issues](https://linear.app/docs/delete-archive-issues) | Issue制限の詳細 |

**主な発見**（原文抜粋）:
> "The free plan supports **up to 250 active issues with unlimited archived issues**, meaning archived issues don't count toward your limit."

### 3. Linear CLI

| クエリ | 参照先 | 確認内容 |
|--------|--------|----------|
| "Linear CLI installation authentication commands 2026" | [GitHub - schpet/linear-cli](https://github.com/schpet/linear-cli), [GitHub - Finesssee/linear-cli](https://github.com/Finesssee/linear-cli) | CLI実装の調査 |

**主な発見**:
- 複数のLinear CLI実装が存在（Node.js, Rust, Python）
- 認証方法: OAuth または Personal API Key

### 4. Linear API・Webhook

| クエリ | 参照先 | 確認内容 |
|--------|--------|----------|
| "Linear API GraphQL webhook integration 2026" | [API and Webhooks](https://linear.app/docs/api-and-webhooks), [Webhooks](https://linear.app/developers/webhooks) | API・Webhook機能 |

**主な発見**:
- GraphQL API
- Webhook（データ変更時のHTTP通知）
- HMAC署名による検証

---

## WebFetchで取得した情報

### 1. Linear公式価格表

| URL | 取得内容 |
|-----|----------|
| https://linear.app/pricing | 価格プラン詳細（Free, Basic, Business, Enterprise） |

**原文抜粋**:
- Free Plan: $0, Members: Unlimited, Issues: 250 issues
- Basic Plan: $10 per user/month (billed yearly), Issues: Unlimited

### 2. Linear MCP Server

| URL | 取得内容 |
|-----|----------|
| https://linear.app/docs/mcp | MCP Server詳細（セットアップ方法、機能） |

**原文抜粋**:
> "The Linear MCP server is **a standardized interface enabling AI models and agents to access Linear data securely**."
>
> セットアップ: `claude mcp add --transport http linear-server https://mcp.linear.app/mcp`

---

## 調査の目的

**Linear調査**:
- QA-FY26プロジェクトでの進捗管理ツールとして採用可能か検証
- コスト、機能、運用方法を調査
- Claude Codeとの連携方法を確認

**結論**:
- Linear Basicプラン採用を推奨（年$360-600）
- Linear + Markdown ハイブリッド運用
- MCP Serverで強力な連携が可能

---

## 参照

- [session-summary.md](session-summary.md) — 本セッションサマリー
- [docs/tools/linear/pricing-and-features.md](../../docs/tools/linear/pricing-and-features.md) — Linear調査レポート（出典付き）

---

**最終更新**: Session 268 (2026-03-19)
