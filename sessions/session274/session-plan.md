# Session 274 計画

**目的**: Linear運用タスクの完了（Issue整理と新規Issue追加）

**前提**: Session 273でWorkflow State運用方針を確立、Timeline viewの使い方を確認

---

## やること

| # | 作業 | 読むべきファイル | 参照コマンド |
|---|------|-----------------|--------------|
| 1 | 既存Issueの状態確認・整理 | [session273/session-summary.md](../session273/session-summary.md) | - |
| 2 | QA-6（プレート調査）をDone状態に変更 | [session267/context-and-background.md](../session267/context-and-background.md) | - |
| 3 | 新規Issueの追加（Phase 1の残タスク） | [session267/context-and-background.md](../session267/context-and-background.md) | - |
| 4 | Timeline viewで進捗確認 | [docs/tools/linear/workflow-state-operation.md](../../docs/tools/linear/workflow-state-operation.md) | - |

---

## 詳細

### 1. 既存Issueの状態確認・整理

**現在のIssue**:
- QA-5: ?（状態未確認）
- QA-6: プレート調査（Session 267で完了済み）→ Done状態に変更
- QA-7: 梱包変更等実装・隠れコストの定量調査（In Progress）

**確認事項**:
- QA-5, QA-7の現在の状態
- 適切なWorkflow Stateに配置されているか

---

### 2. QA-6をDone状態に変更

**Issue情報**:
- ID: `e14aa063-27b2-44fb-9132-25d1b29c4f2c`
- Done状態のState ID: `0a1243c8-726a-4e0c-838a-bf1066d26fea`

**API操作**:
```bash
cat > /tmp/issue-complete-qa6.json << 'EOF'
{
  "query": "mutation ($id: String!, $stateId: String!) { issueUpdate(id: $id, input: { stateId: $stateId }) { success issue { id identifier state { name type } } } }",
  "variables": {
    "id": "e14aa063-27b2-44fb-9132-25d1b29c4f2c",
    "stateId": "0a1243c8-726a-4e0c-838a-bf1066d26fea"
  }
}
EOF

curl -s -X POST https://api.linear.app/graphql \
  -H "Authorization: $(grep LINEAR_API_KEY .env | cut -d= -f2)" \
  -H "Content-Type: application/json" \
  -d @/tmp/issue-complete-qa6.json
```

---

### 3. 新規Issueの追加

**追加するIssue**（Phase 1の残タスク）:
1. **フレーム・モーター調査**（暗黙知外部化）
2. **田原さん・杉山さんヒアリング**（SIPOC レビュー）
3. **Excel記録作業調査**
4. その他、必要に応じて追加

**Issue作成時のルール**:
- デフォルトStateは「Backlog」
- 説明文にチェックリストを記載
- ProjectにM3+M4を紐付け

**参照**: [docs/tools/linear/workflow-state-operation.md](../../docs/tools/linear/workflow-state-operation.md)

---

### 4. Timeline viewで進捗確認

**手順**:
1. Workspace > Views でカスタムビュー（進捗確認View等）を開く
2. 右上の「Timeline」ボタンをクリック
3. M3+M4プロジェクトの進捗を確認

**確認事項**:
- Issueが適切に配置されているか
- Timeline上でプロジェクトの進捗が可視化されているか

---

## 参照

- [session273/session-summary.md](../session273/session-summary.md) — 前セッションサマリー
- [docs/tools/linear/workflow-state-operation.md](../../docs/tools/linear/workflow-state-operation.md) — Workflow State運用方針
- [session267/context-and-background.md](../session267/context-and-background.md) — Phase 1全体文脈
- [session270/linear-setup-commands.md](../session270/linear-setup-commands.md) — Linear操作コマンド集

---

**期待成果**: 既存Issue整理完了、新規Issue追加完了、Timeline viewで進捗可視化
