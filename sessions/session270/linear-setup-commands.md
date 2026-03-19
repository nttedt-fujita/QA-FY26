# Session 270でのLinear操作コマンド

**作成日**: 2026-03-19
**目的**: Linear初期設定と操作の具体的なコマンド記録

---

## 前提条件

### 1. Linear API Key設定

**場所**: `.env` ファイル（Git管理外）

```bash
# .env の内容確認
cat .env
```

**内容例**:
```
LINEAR_API_KEY=lin_api_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
```

**API Key確認コマンド**:
```bash
grep LINEAR_API_KEY .env | cut -d= -f2
```

---

## 実施した操作

### 1. Workspace情報の取得

**目的**: 接続確認、Team IDの取得

**コマンド**:
```bash
curl -s -X POST https://api.linear.app/graphql \
  -H "Authorization: $(grep LINEAR_API_KEY .env | cut -d= -f2)" \
  -H "Content-Type: application/json" \
  -d '{"query":"{ viewer { id name email } organization { id name } teams { nodes { id name } } }"}'
```

**結果**（Session 270）:
```json
{
  "data": {
    "viewer": {
      "id": "a6af0cc0-9489-43c6-b9a2-19e116bbc223",
      "name": "shinya.fujita@nttedt.co.jp",
      "email": "shinya.fujita@nttedt.co.jp"
    },
    "organization": {
      "id": "0bea64bb-7352-44bc-85ae-48553aadf70d",
      "name": "QA-FY26-FUJITA"
    },
    "teams": {
      "nodes": [
        {
          "id": "21970f95-f68f-4b1d-a698-5b67dbcdc0ae",
          "name": "QA-FY26-FUJITA"
        }
      ]
    }
  }
}
```

**重要な値**:
- **Team ID**: `21970f95-f68f-4b1d-a698-5b67dbcdc0ae`
- **Organization ID**: `0bea64bb-7352-44bc-85ae-48553aadf70d`

---

### 2. 既存Issue・Projectの確認

**目的**: 現在の状態を確認

**コマンド**:
```bash
curl -s -X POST https://api.linear.app/graphql \
  -H "Authorization: $(grep LINEAR_API_KEY .env | cut -d= -f2)" \
  -H "Content-Type: application/json" \
  -d '{"query":"{ issues { nodes { id identifier title state { name } project { id name } } } projects { nodes { id name state targetDate startDate } } }"}'
```

**結果**（Session 270開始時）:
- デフォルトissue: QA-1〜QA-4（オンボーディング用）
- Project: 0件

---

### 3. Project作成

**目的**: M3+M4検査プロセス改善のProjectを作成

**コマンド**:
```bash
curl -s -X POST https://api.linear.app/graphql \
  -H "Authorization: $(grep LINEAR_API_KEY .env | cut -d= -f2)" \
  -H "Content-Type: application/json" \
  -d '{
    "query": "mutation { projectCreate(input: { name: \"M3+M4: 検査プロセス改善\", description: \"受入検査・工程不良の現状可視化とプロセス改善\", teamIds: [\"21970f95-f68f-4b1d-a698-5b67dbcdc0ae\"], targetDate: \"2026-06-19\" }) { success project { id name } } }"
  }'
```

**結果**:
```json
{
  "data": {
    "projectCreate": {
      "success": true,
      "project": {
        "id": "1cff36fe-12d1-4cd8-89c6-b2bb66db991d",
        "name": "M3+M4: 検査プロセス改善"
      }
    }
  }
}
```

**作成されたProject ID**: `1cff36fe-12d1-4cd8-89c6-b2bb66db991d`

---

### 4. Issue作成

**目的**: 現在の進捗をIssueとして登録

#### 方法: JSONファイルを使用

GraphQLのエスケープ問題を回避するため、JSONファイルを使用。

**Issue 1: SIPOC作成と現場レビュー準備**

**JSONファイル作成** (`/tmp/linear-issue-1.json`):
```json
{
  "query": "mutation CreateIssue($input: IssueCreateInput!) { issueCreate(input: $input) { success issue { id identifier state { name } } } }",
  "variables": {
    "input": {
      "title": "SIPOC作成と現場レビュー準備",
      "description": "## 作業内容\n- [x] 全体フローSIPOC作成（発注→出荷、10プロセス）\n- [x] 受入検査詳細SIPOC作成（11プロセス、梱包変更作業を明示化）\n- [ ] 田原さん・杉山さんとのレビューセッション\n- [ ] フィードバック反映\n\n## 参考資料\n- session266/overall-flow-sipoc-template.drawio\n- session266/iqc-detailed-sipoc-template.drawio",
      "teamId": "21970f95-f68f-4b1d-a698-5b67dbcdc0ae",
      "projectId": "1cff36fe-12d1-4cd8-89c6-b2bb66db991d"
    }
  }
}
```

**コマンド**:
```bash
curl -s -X POST https://api.linear.app/graphql \
  -H "Authorization: $(grep LINEAR_API_KEY .env | cut -d= -f2)" \
  -H "Content-Type: application/json" \
  -d @/tmp/linear-issue-1.json
```

**結果**:
```json
{
  "data": {
    "issueCreate": {
      "success": true,
      "issue": {
        "id": "0557799e-8929-4a03-adeb-2442aaa8eb52",
        "identifier": "QA-5",
        "state": {"name": "Backlog"}
      }
    }
  }
}
```

**Issue 2: プレート調査（暗黙知外部化）**

**JSONファイル** (`/tmp/linear-issue-2.json`):
```json
{
  "query": "mutation CreateIssue($input: IssueCreateInput!) { issueCreate(input: $input) { success issue { id identifier state { name } } } }",
  "variables": {
    "input": {
      "title": "プレート調査（暗黙知外部化）",
      "description": "## 作業内容\n- [x] シリアル番号突合作業の実態調査\n- [x] 印字品質確認の基準ヒアリング\n- [x] 小分け作業の実態把握\n- [x] 隠れコスト（小分け再作業、サイレントチェンジ対応）の発見\n\n## 成果\n- session267で田原さんから実態をヒアリング完了\n- 暗黙知と隠れコストを可視化",
      "teamId": "21970f95-f68f-4b1d-a698-5b67dbcdc0ae",
      "projectId": "1cff36fe-12d1-4cd8-89c6-b2bb66db991d"
    }
  }
}
```

**コマンド**:
```bash
curl -s -X POST https://api.linear.app/graphql \
  -H "Authorization: $(grep LINEAR_API_KEY .env | cut -d= -f2)" \
  -H "Content-Type: application/json" \
  -d @/tmp/linear-issue-2.json
```

**結果**: QA-6作成

**Issue 3: 梱包変更作業・隠れコストの定量調査**

**JSONファイル** (`/tmp/linear-issue-3.json`):
```json
{
  "query": "mutation CreateIssue($input: IssueCreateInput!) { issueCreate(input: $input) { success issue { id identifier state { name } } } }",
  "variables": {
    "input": {
      "title": "梱包変更作業・隠れコストの定量調査",
      "description": "## 作業内容\n- [ ] 梱包変更作業の発生頻度を調査（Excelデータ分析）\n- [ ] 工数見積もり（1件あたりの時間）\n- [ ] 年間コストの試算\n- [ ] 小分け再作業の発生頻度調査\n- [ ] サイレントチェンジ対応の工数調査\n\n## 背景\n- Session 260で発見した隠れコスト\n- AI員数確認よりも大きな問題の可能性",
      "teamId": "21970f95-f68f-4b1d-a698-5b67dbcdc0ae",
      "projectId": "1cff36fe-12d1-4cd8-89c6-b2bb66db991d"
    }
  }
}
```

**コマンド**:
```bash
curl -s -X POST https://api.linear.app/graphql \
  -H "Authorization: $(grep LINEAR_API_KEY .env | cut -d= -f2)" \
  -H "Content-Type: application/json" \
  -d @/tmp/linear-issue-3.json
```

**結果**: QA-7作成

---

## Session 270終了時の状態

### 作成されたリソース

| 種類 | ID | 識別子 | 名前 |
|------|-----|--------|------|
| Project | 1cff36fe-12d1-4cd8-89c6-b2bb66db991d | - | M3+M4: 検査プロセス改善 |
| Issue | 0557799e-8929-4a03-adeb-2442aaa8eb52 | QA-5 | SIPOC作成と現場レビュー準備 |
| Issue | e14aa063-27b2-44fb-9132-25d1b29c4f2c | QA-6 | プレート調査（暗黙知外部化） |
| Issue | 5c12dfa5-89da-4935-b8da-bca357c07533 | QA-7 | 梱包変更作業・隠れコストの定量調査 |

### アクティブissue数

- デフォルトissue: 4件（QA-1〜QA-4）
- 新規作成: 3件（QA-5〜QA-7）
- **合計**: 7件

---

## トラブルシューティング

### 1. API Key認証エラー

**エラーメッセージ例**:
```json
{"errors":[{"message":"Unauthorized","extensions":{"code":"UNAUTHENTICATED"}}]}
```

**確認手順**:
```bash
# 1. .envファイルの存在確認
ls -la .env

# 2. API Keyの内容確認
cat .env

# 3. API Keyの形式確認（lin_api_で始まる）
grep LINEAR_API_KEY .env

# 4. 環境変数への読み込み確認
echo "$(grep LINEAR_API_KEY .env | cut -d= -f2)"
```

**修正方法**:
- Linear UIでAPI Keyを再発行
- `.env`ファイルを更新

### 2. GraphQLエスケープエラー

**問題**: curlコマンド内で直接JSONを書くとエスケープ地獄

**解決策**: JSONファイルを使用
```bash
# JSONファイルに保存
cat > /tmp/request.json << 'EOF'
{
  "query": "...",
  "variables": {...}
}
EOF

# ファイルを参照
curl ... -d @/tmp/request.json
```

### 3. Team ID不明

**取得コマンド**:
```bash
curl -s -X POST https://api.linear.app/graphql \
  -H "Authorization: $(grep LINEAR_API_KEY .env | cut -d= -f2)" \
  -H "Content-Type: application/json" \
  -d '{"query":"{ teams { nodes { id name } } }"}' | grep -o '"id":"[^"]*"'
```

---

## 次セッションで確認すべきこと

1. **API Key有効性**
   ```bash
   curl -s -X POST https://api.linear.app/graphql \
     -H "Authorization: $(grep LINEAR_API_KEY .env | cut -d= -f2)" \
     -H "Content-Type: application/json" \
     -d '{"query":"{ viewer { name } }"}'
   ```

2. **作成したProjectの確認**
   ```bash
   curl -s -X POST https://api.linear.app/graphql \
     -H "Authorization: $(grep LINEAR_API_KEY .env | cut -d= -f2)" \
     -H "Content-Type: application/json" \
     -d '{"query":"{ projects { nodes { id name } } }"}'
   ```

3. **作成したIssueの確認**
   ```bash
   curl -s -X POST https://api.linear.app/graphql \
     -H "Authorization: $(grep LINEAR_API_KEY .env | cut -d= -f2)" \
     -H "Content-Type: application/json" \
     -d '{"query":"{ issues(filter: { project: { id: { eq: \"1cff36fe-12d1-4cd8-89c6-b2bb66db991d\" } } }) { nodes { identifier title } } }"}'
   ```

---

## 参照

- [Linear API Documentation](https://developers.linear.app/docs)
- [Linear GraphQL API Reference](https://studio.apollographql.com/public/Linear-API/variant/current/home)
- [session270/session-summary.md](session-summary.md)
- [docs/adr/common/ADR-017-linear-free-plan-operation.md](../../docs/adr/common/ADR-017-linear-free-plan-operation.md)

---

**作成**: Session 270 (2026-03-19)
**更新**: Session 271（トラブルシューティング追加予定）
