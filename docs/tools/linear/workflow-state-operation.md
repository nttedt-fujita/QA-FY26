# Linearの Workflow State 運用方針

**作成**: Session 273 (2026-03-19)
**対象**: QA-FY26-FUJITA Workspace

---

## 概要

LinearのWorkflow State（Issue状態）の運用方針を定義する。アクティブIssue数を抑制し、Freeプラン（250 active issues）の制限内で効率的に運用する。

**関連**: [ADR-017: Linearの無料プラン運用方針](../../adr/common/ADR-017-linear-free-plan-operation.md)

---

## Workflow States 定義

QA-FY26-FUJITAチームのWorkflow States（Session 271で取得）:

| State | Type | State ID |
|-------|------|----------|
| Backlog | backlog | `9e1b52f8-91f8-48f6-a1ea-8d09af23dcaf` |
| Todo | unstarted | `491cb226-953a-4f85-ab8c-2debbe177d17` |
| In Progress | started | `18c8a0ea-747d-4e33-bfaa-5eb18c2bdec1` |
| Done | completed | `0a1243c8-726a-4e0c-838a-bf1066d26fea` |
| Canceled | canceled | `2610a692-a023-4301-b7ed-1fbca2e3b5bc` |
| Duplicate | canceled | `e4cdaf51-001d-4833-988d-8d8d6860feab` |

---

## 各Stateの使い分け

### 通常フロー

| State | 用途 | 移動タイミング | 例 |
|-------|------|--------------|-----|
| **Backlog** | 今後やる予定のタスク全体 | Issue作成時のデフォルト | 「梱包変更作業調査」「フレーム調査」 |
| **Todo** | 次に着手するタスク（優先度高） | 着手前に移動 | 「次セッションで着手予定のタスク」 |
| **In Progress** | 現在作業中 | 作業開始時に移動 | 「現在ヒアリング中」 |
| **Done** | 完了（自動アーカイブ待ち） | **作業完了時すぐに移動** | 「調査完了」 |

### 例外フロー

| State | 用途 | 移動タイミング | 例 |
|-------|------|--------------|-----|
| **Canceled** | 中止・不要になった | 方針変更時 | 「方針変更で不要になったタスク」 |
| **Duplicate** | 重複Issue | 重複発見時 | 「既存のQA-3と同じ内容」 |

---

## 運用の重要原則

### 1. Issue完了の徹底（最重要）

**ルール**: **完了したらすぐにDone状態に変更**

**理由**:
- Done状態 → 自動アーカイブ → アクティブ数から除外
- 完了したのに放置 → アクティブ数を圧迫 → Freeプラン上限到達

**実施タイミング**:
- セッション終了時にチェック
- 完了したIssueをすぐにDone状態に変更

### 2. In Progressは1件推奨

**理由**: 並行作業を抑制し、集中して完了させる

**複数になる場合**:
- ヒアリング待ち等、自分で進められない状態のみ許容

### 3. Issue粒度は中程度（作業単位）

**階層構造**:
```
Phase > Epic > Issue（作業単位）> チェックリスト（サブタスク）
```

**良い例**:
- ✅ Issue: 「SIPOC作成と現場レビュー」（作業単位）
- ✅ チェックリスト: 「全体フローSIPOC作成」「受入検査詳細SIPOC作成」

**悪い例**:
- ❌ Issue: 「SIPOCの表を作る」（細かすぎ）
- ❌ Issue: 「プレートの寸法を測る」（細かすぎ）

**詳細**: [ADR-017: Issue粒度](../../adr/common/ADR-017-linear-free-plan-operation.md#2-issue粒度)

---

## 新規Issue作成時のルール

### デフォルトState

**Backlog** をデフォルトにする

**理由**:
- 作成時点ではまだ優先度が決まっていない
- Todoへの移動: 着手前に判断

### Issue説明文にチェックリストを記載

**テンプレート**:
```markdown
## 作業内容
- [ ] サブタスク1
- [ ] サブタスク2
- [ ] サブタスク3

## 参考資料
- [関連ドキュメント](path/to/doc.md)
```

**効果**:
- 細かいタスクをIssueにせずチェックリストで管理
- アクティブIssue数を抑制

---

## セッション終了時のチェックリスト

**IMPORTANT**: セッション終了時に必ず確認すること

- [ ] **完了したIssueをDone状態に変更したか**
- [ ] 中止・重複のIssueをCanceled/Duplicateに変更したか
- [ ] In ProgressのIssueは1件以内か（複数ある場合は理由を明確に）
- [ ] 新規Issueを作成した場合、適切なStateに配置したか

---

## アクティブIssue数の監視

**セッション開始時に自動確認**（`linear-management` スキル）:

| アクティブissue数 | 状態 | 対応 |
|-----------------|------|------|
| 0-100件 | ✅ 余裕 | 継続 |
| 100-150件 | 🟡 注意 | 監視強化、Issue完了徹底 |
| 150-200件 | 🟠 警告 | **小笠原さん・宇枝さんに相談** |
| 200-250件 | 🔴 限界 | Basicプラン移行（$10/月） |

**詳細**: [ADR-017: 監視の仕組み](../../adr/common/ADR-017-linear-free-plan-operation.md#4-監視の仕組み)

---

## Issue完了（Done状態に変更）の方法

### API経由

```bash
cat > /tmp/issue-complete.json << 'EOF'
{
  "query": "mutation ($id: String!, $stateId: String!) { issueUpdate(id: $id, input: { stateId: $stateId }) { success issue { id identifier state { name type } } } }",
  "variables": {
    "id": "ISSUE_UUID",
    "stateId": "0a1243c8-726a-4e0c-838a-bf1066d26fea"
  }
}
EOF

curl -s -X POST https://api.linear.app/graphql \
  -H "Authorization: $(grep LINEAR_API_KEY .env | cut -d= -f2)" \
  -H "Content-Type: application/json" \
  -d @/tmp/issue-complete.json
```

**Done状態のState ID**: `0a1243c8-726a-4e0c-838a-bf1066d26fea`

### Web UI経由

1. Linear Web UIでIssueを開く
2. Stateドロップダウンをクリック
3. 「Done」を選択

---

## 自動アーカイブの仕組み

**Linearのアーカイブは自動実行**:
- Issueを完了状態（Done）にすると、設定されたアーカイブ期間後に自動アーカイブ
- 手動でのアーカイブ操作は不可（Linear仕様）
- アーカイブしたissueは無制限（検索・参照可能）

**出典**: [Linear Docs - Delete and archive issues](https://linear.app/docs/delete-archive-issues)

---

## 参照

### プロジェクト内ドキュメント

- [ADR-017: Linearの無料プラン運用方針](../../adr/common/ADR-017-linear-free-plan-operation.md)
- [issue-estimate.md](issue-estimate.md) — Issue数見積もり
- [hybrid-operation-rules.md](hybrid-operation-rules.md) — Linear + Markdown ハイブリッド運用
- `~/.claude/skills/linear-management/SKILL.md` — Linear管理スキル

### 外部ドキュメント

- [Linear Docs - Delete and archive issues](https://linear.app/docs/delete-archive-issues)
- [Linear Docs - Workflow states](https://linear.app/docs/workflow-states)

---

**作成**: Session 273 (2026-03-19)
**更新**: -
