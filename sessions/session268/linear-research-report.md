# Linear調査レポート

**作成**: Session 268 (2026-03-19)

**目的**: QA-FY26プロジェクトでの進捗管理ツールとしてLinearを検討するための調査

---

## 1. コスト（価格プラン）

公式サイトからの情報（出典: [Linear Pricing](https://linear.app/pricing)）

| プラン | 価格 | メンバー | Issue数 | チーム数 | ファイルアップロード | 主な機能 |
|--------|------|----------|---------|---------|---------------------|---------|
| **Free** | $0 | 無制限 | 250 | 2 | 10MB | Slack & GitHub連携、AI agents |
| **Basic** | $10/user/月（年払い） | 無制限 | 無制限 | 5 | 無制限 | Freeの全機能 + 管理者ロール |
| **Business** | $16/user/月（年払い） | 無制限 | 無制限 | 無制限 | 無制限 | プライベートチーム、ゲスト、Triage Intelligence、Linear Insights、Linear Asks、Zendesk/Intercom連携 |
| **Enterprise** | カスタム（年払いのみ） | 無制限 | 無制限 | 無制限 | 無制限 | SAML/SCIM、PO請求、詳細管理権限、エンタープライズセキュリティ、マイグレーション支援、優先サポート、アカウント管理 |

### QA-FY26プロジェクトでの想定コスト

**前提条件**:
- 利用者: 藤田さん + 田原さん + 杉山さん（＋α: いしかわさん、こいたばしさん等） = 3-5名
- Issue数: Phase 1で30-50程度を想定（SIPOC作成、暗黙知外部化、部品調査等）

**選択肢**:

| プラン | 月額コスト（年払い） | 年間コスト | 制約 | 判定 |
|--------|---------------------|-----------|------|------|
| Free | $0 | $0 | **Issue数上限250** | ✅ 短期的にはOK（Phase 1完了まで） |
| Basic | $30-50（3-5名） | $360-600 | Issue無制限 | 🟡 必要になったら検討 |
| Business | $48-80（3-5名） | $576-960 | 高度な分析機能 | ❌ 当面は不要 |

**結論**: **Freeプランで開始**。Issue数が250に近づいたらBasicプランへ移行を検討。

---

## 2. 機能

### 2.1 コア機能

**Issue管理**:
- Issue作成・更新・削除
- ステータス管理（Todo, In Progress, Done等）
- 優先度設定（Urgent, High, Medium, Low, No priority）
- 担当者割り当て
- ラベル・タグ付け
- コメント機能

**プロジェクト管理**:
- Cycles（スプリント）管理
- Roadmap（ロードマップ）機能
- Project（プロジェクト）管理
- Epic（エピック）管理

**コラボレーション**:
- コメント・メンション
- 通知機能
- Slack連携
- GitHub連携

### 2.2 進捗可視化

**ダッシュボード**:
- Issue一覧（フィルター、ソート）
- 進捗率表示
- カスタムビュー（ボード、リスト、テーブル）

**分析機能**（Business以上）:
- Linear Insights: データ分析・レポート
- Triage Intelligence: 優先度自動提案

### 2.3 API連携

**GraphQL API**:
- 公式APIはGraphQLで実装（出典: [Linear Developers](https://linear.app/developers)）
- Issue、Comment、Project等の操作が可能
- OAuth 2.0 または Personal API Key で認証

**Webhook**:
- データ変更時のHTTP通知（出典: [Linear Webhooks](https://linear.app/developers/webhooks)）
- Issue作成・更新・削除、Comment追加等のイベント対応
- HMAC署名による検証

### 2.4 エクスポート機能

**標準機能**:
- CSVエクスポート（Issue一覧）
- Markdown変換（Issue本文）

**API経由**:
- GraphQL APIでデータ取得
- カスタムエクスポート処理を実装可能

---

## 3. Claude Codeとの連携

### 3.1 Linear MCP Server（推奨）

**MCP（Model Context Protocol）サーバー**（出典: [Linear MCP Server](https://linear.app/docs/mcp)）:
- Linearが公式提供するClaude Code連携機能
- OAuth 2.1認証（セキュア）
- Issue、Project、Commentの検索・作成・更新が可能

**セットアップ手順**:
```bash
# 1. MCP Serverを追加
claude mcp add --transport http linear-server https://mcp.linear.app/mcp

# 2. Claude Codeセッション内で認証
/mcp
# → OAuth認証フローが開始
```

**利点**:
- Claude CodeからLinear操作を自然言語で実行
- 例: "Create a Linear issue for プレート調査"
- セッション終了時に自動でIssue更新

**制約**:
- OAuth認証が必要（個人アカウントでログイン）
- チーム全体での認証管理が必要

### 3.2 Linear CLI（代替案）

複数のLinear CLI実装が存在（出典: [GitHub - schpet/linear-cli](https://github.com/schpet/linear-cli), [GitHub - Finesssee/linear-cli](https://github.com/Finesssee/linear-cli)）:

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

**利点**:
- Bash経由でLinear操作が可能
- スクリプト自動化が容易

**欠点**:
- MCP Serverより操作が複雑
- Claude Codeとの統合が弱い

### 3.3 推奨アプローチ

**QA-FY26プロジェクトでの推奨**:
1. **Linear MCP Server**を使用（Claude Codeとの親和性が高い）
2. 必要に応じて**Linear CLI**も併用（スクリプト自動化用）

---

## 4. 運用方法

### 4.1 Workspace/Project/Epicの構成

**推奨構成**:
```
Workspace: QA-FY26
├── Project: Phase 1（現状可視化）
│   ├── Epic: SIPOC作成
│   │   ├── Issue: 全体フローSIPOC作成
│   │   ├── Issue: 受入検査詳細SIPOC作成
│   │   └── ...
│   ├── Epic: 暗黙知外部化
│   │   ├── Issue: プレート調査
│   │   ├── Issue: フレーム調査
│   │   └── ...
│   └── Epic: 隠れコスト特定
│       ├── Issue: 梱包変更作業の工数調査
│       └── ...
├── Project: Phase 2（ムダと隠れコスト分析）
└── Project: Phase 3（解決策策定）
```

### 4.2 Issue作成・更新のワークフロー

**セッション開始時**:
1. 今セッションで取り組むIssueを確認
2. Issueのステータスを`In Progress`に変更

**セッション中**:
1. 進捗をコメントで記録
2. 新しい発見・課題があれば新規Issue作成

**セッション終了時**:
1. 完了したIssueを`Done`に変更
2. 未完了のIssueにコメントで状況を記録
3. 次セッションの計画をIssueに反映

### 4.3 他のツールとの連携

**GitHub連携**:
- 技術プロトタイプ（M1-GNSS等）のPR作成時にLinear IssueをリンクGitHub連携によりコミット時にIssueを自動更新

**Slack連携**:
- チーム通知（Issue作成・更新時）
- コメント通知

**Markdown連携（ハイブリッド運用）**:
- 詳細な技術資料: sessions/session-XXX/ に配置
- 進捗管理: Linear
- 重要な決定: CLAUDE.md または docs/adr/

---

## 5. 選択肢の比較

Session 267で検討した3つの選択肢を再評価:

| 項目 | A. Linearのみ | B. Markdownのみ | C. Linear + Markdown ハイブリッド |
|------|--------------|----------------|----------------------------------|
| **コスト** | $0（Freeプラン） | $0 | $0 |
| **コラボレーション** | ✅ 強い（コメント、通知、Slack連携） | ❌ 弱い（Git操作のみ） | ✅ 両方の利点 |
| **進捗可視化** | ✅ 強い（ダッシュボード、フィルター） | 🟡 普通（表形式） | ✅ Linearで可視化 |
| **技術資料管理** | 🟡 可能（Issue本文・コメント） | ✅ 強い（Markdown） | ✅ 両方の利点 |
| **履歴管理** | 🟡 Linear内の履歴 | ✅ Git履歴 | ✅ 両方 |
| **Claude Code連携** | ✅ MCP Server | ✅ 標準ツール | ✅ 両方 |
| **運用コスト** | 🟡 Learning curve | ✅ 既存フロー | 🟡 少し複雑 |
| **長期保守** | 🟡 Linearに依存 | ✅ 自己管理 | ✅ 冗長性あり |

**推奨**: **C. Linear + Markdown ハイブリッド**

**理由**:
- **Linear**: 進捗管理・コラボレーション・可視化
- **Markdown**: 技術詳細・設計判断・履歴管理
- **相互補完**: Linearの弱点（長期保守、技術詳細）をMarkdownで補う

---

## 6. 次ステップ

### 6.1 即時実施

- [x] Linear調査完了
- [ ] Linear MCP Serverのセットアップ
- [ ] QA-FY26 Workspace作成
- [ ] Phase 1 Project作成
- [ ] 初期Issue作成（プレート調査等）

### 6.2 今後の検討

- Phase 1完了時にIssue数を確認（250に近づいていればBasicプランへ）
- コラボレーションパターンの確立（田原さん・杉山さんとの共有方法）
- 週次/月次の進捗レビュー方法

---

## 参照

**公式ドキュメント**:
- [Linear Pricing](https://linear.app/pricing) — 価格プラン
- [Linear MCP Server](https://linear.app/docs/mcp) — Claude Code連携
- [Linear Developers](https://linear.app/developers) — API・Webhook
- [Linear Webhooks](https://linear.app/developers/webhooks) — Webhook詳細

**Linear CLI実装**:
- [GitHub - schpet/linear-cli](https://github.com/schpet/linear-cli) — Node.js実装
- [GitHub - Finesssee/linear-cli](https://github.com/Finesssee/linear-cli) — Rust実装
- [GitHub - AdiKsOnDev/linear-cli](https://github.com/AdiKsOnDev/linear-cli) — Python実装

**価格比較サイト**:
- [Linear Pricing & Plans 2026](https://www.top3.software/r/linear-pricing)
- [Linear Pricing 2026](https://www.g2.com/products/linear/pricing)
- [Linear Pricing 2026: Free + $8-$16/User Plans](https://costbench.com/software/developer-tools/linear/)

**その他**:
- [Linear integration | docs](https://docs.trunk.io/flaky-tests/webhooks/linear-integration) — CI連携
- [Linear Webhooks: Complete Guide with Payload Examples [2025]](https://inventivehq.com/blog/linear-webhooks-guide) — Webhook実装ガイド

---

**結論**: QA-FY26プロジェクトでは**Freeプランで開始**し、**Linear + Markdown ハイブリッド運用**を推奨。Claude CodeとはMCP Serverで連携。
