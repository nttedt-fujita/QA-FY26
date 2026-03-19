# Session 269 サマリー

---
作成: 2026-03-19
---

## 目的

Linear導入判断とセットアップ

## 実施内容

### 1. 方針確定（要求から離れないためのチェック）

**本来のミッション**:
- 宇枝さんへの説明資料作り（AI導入は厳しい、手を入れる部分が違う）
- Session 268でLinear調査に1日費やした → 今回で区切りをつける

**決定**:
- Linear Free導入（セットアップのみ、Issue登録は後回し）
- 次セッションから説明資料作り優先
- GNSS作業もあるが、まず説明資料を片付ける

### 2. Linear Free導入

**Workspace作成**:
- Workspace名: QA-FY26-FUJITA
- プラン: Free（250 active issues）
- 会社メールアドレスで既にアカウント作成済み → 新しいWorkspaceとして分離

**重要な発見**:
- 1つのアカウントで複数のWorkspaceに参加可能
- ソフトチームのWorkspaceと独立して運用できる
- Workspace単位で課金される

**Team確認**:
- Team ID: `QA`
- URL: https://linear.app/qa-fy26-fujita/settings/teams/QA

### 3. Claude Code連携セットアップ

**Linear MCP Server設定**:
```bash
claude mcp add --transport http linear-server https://mcp.linear.app/mcp
```
- 設定ファイル: ~/.claude.json に追加済み

**Personal API Key発行**:
- Settings → Security & access → API
- Key名: ClaudeCode
- API Key: `lin_api_***（.envに保存済み）`

**セキュア管理**:
- `.env` に実際のAPI Keyを保存（Git管理外）
- `.env.example` にフォーマット例を記録（Git管理下）
- `.gitignore` に `.env` を追加

### 4. GitHub連携とMCP Serverの違い

**Claude Code ⇔ Linear 連携（MCP Server）**:
- Claude CodeからLinearを操作（Issue作成・検索・更新）
- 自然言語で「Linearにissueを作って」等が可能

**Linear ⇔ GitHub 連携（別機能）**:
- LinearのIssueとGitHubのPR/Commitを連携
- オプション機能（M1-GNSS等でGitを使う場合に便利）

## 主な発見

1. **Linearの課金単位**: Workspace単位で独立して課金
2. **複数Workspace対応**: 1アカウントで複数Workspaceに参加可能
3. **Team ID確認方法**: URL（linear.app/.../settings/teams/XXX）で確認
4. **API Key発行場所**: Workspace設定ではなく、個人設定（Security & access）

## 残った課題

1. **Linear連携の動作確認**（次セッションで）
   - Claude Codeから「Linearにissueを作って」と頼んで、実際に作成されるか確認
   - OAuth認証フローが発生する可能性

2. **宇枝さん説明資料作成**（次セッションの主タスク）
   - AI導入が厳しい理由
   - 手を入れるべき部分（プロセス改善）
   - ヒアリング結果のまとめ（小笠原さんとの認識合わせ済み）

3. **GNSS作業への復帰**（説明資料完了後）

## 成果物

| ファイル | 内容 |
|----------|------|
| `.env` | Linear API Key（Git管理外） |
| `.env.example` | API Keyフォーマット例 |
| `.gitignore` | `.env`を除外設定 |

## 使用したコマンド

| 用途 | コマンド |
|------|----------|
| MCP Server追加 | `claude mcp add --transport http linear-server https://mcp.linear.app/mcp` |
| 環境変数設定 | `export LINEAR_API_KEY="..."` |

## 次セッションでやること

1. **Linear連携動作確認**
   - Claude Codeから実際にissueを作成してみる
   - OAuth認証が必要か確認

2. **宇枝さん説明資料作成開始**（優先）
   - AI導入が厳しい理由を整理
   - 手を入れるべき部分（プロセス改善、暗黙知外部化、隠れコスト特定）
   - ヒアリング結果のまとめ

3. **GNSS作業復帰の準備**
   - M1-GNSS Phase 3の状況確認

## 参照

- [session268/session-summary.md](../session268/session-summary.md) — Linear調査レポート
- [docs/tools/linear/pricing-and-features.md](../../docs/tools/linear/pricing-and-features.md) — Linear価格・機能調査
- [docs/tools/linear/hybrid-operation-rules.md](../../docs/tools/linear/hybrid-operation-rules.md) — ハイブリッド運用ルール

---

**期待成果**: Linear導入完了、次セッションから本来のミッション（説明資料作り）に集中できる状態
