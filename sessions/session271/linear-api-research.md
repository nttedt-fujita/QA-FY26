# Session 271: Linear API調査レポート

**作成日**: 2026-03-19
**目的**: Linear API公式ドキュメント調査、アーカイブ・メンバー招待・Roadmap viewの仕様確認

---

## 調査結果サマリー

### 1. 認証方法の確認

**正しい認証ヘッダー**:
```bash
-H "Authorization: $(grep LINEAR_API_KEY .env | cut -d= -f2)"
```

**重要な発見**:
- 個人APIキーの場合、`Bearer`は**不要**
- `source .env && $LINEAR_API_KEY`方式は動作しない
- `.env`ファイルから直接grepで抽出する方法が正しい

**出典**: [Linear Developers - Getting started](https://linear.app/developers/graphql)

---

### 2. アーカイブ操作の仕様

#### 重要な発見: アーカイブは自動実行

**Linearのアーカイブは手動ではできない**:
- Issueを完了（closed status）にすると、設定されたアーカイブ期間後に自動アーカイブ
- チーム設定でアーカイブ期間を調整可能（Team Settings > Issue statuses & automations）
- 変更は24時間以内に反映

**アーカイブ条件**:
- Issueが完了（closed status）である
- 親issueが閉じられている
- すべてのサブissueが完了している
- 関連プロジェクトが完了している

**出典**: [Linear Docs - Delete and archive issues](https://linear.app/docs/delete-archive-issues)

#### Issue完了の方法

**`issueUpdate` Mutationを使用**:
```graphql
mutation {
  issueUpdate(
    id: "issue-id",
    input: { stateId: "completed-state-id" }
  ) {
    success
    issue { id state { name } }
  }
}
```

**Workflow States（QA-FY26-FUJITAチームの例）**:
| State ID | Name | Type | 説明 |
|----------|------|------|------|
| `9e1b52f8-91f8-48f6-a1ea-8d09af23dcaf` | Backlog | backlog | バックログ |
| `491cb226-953a-4f85-ab8c-2debbe177d17` | Todo | unstarted | 未着手 |
| `18c8a0ea-747d-4e33-bfaa-5eb18c2bdec1` | In Progress | started | 進行中 |
| `0a1243c8-726a-4e0c-838a-bf1066d26fea` | **Done** | **completed** | **完了** |
| `2610a692-a023-4301-b7ed-1fbca2e3b5bc` | Canceled | canceled | キャンセル |
| `e4cdaf51-001d-4833-988d-8d8d6860feab` | Duplicate | canceled | 重複 |

**Issue完了の実例**:
```bash
cat > /tmp/issue-complete.json << 'EOF'
{
  "query": "mutation ($id: String!, $stateId: String!) { issueUpdate(id: $id, input: { stateId: $stateId }) { success issue { id identifier state { name type } } } }",
  "variables": {
    "id": "issue-id-here",
    "stateId": "0a1243c8-726a-4e0c-838a-bf1066d26fea"
  }
}
EOF

curl -s -X POST https://api.linear.app/graphql \
  -H "Authorization: $(grep LINEAR_API_KEY .env | cut -d= -f2)" \
  -H "Content-Type: application/json" \
  -d @/tmp/issue-complete.json
```

#### ADR-017への影響

**修正が必要**:
- ❌ 旧: 「Issue完了時に即座にアーカイブ」
- ✅ 新: 「Issue完了後に自動アーカイブ（手動操作不要）」

---

### 3. メンバー招待の仕様

#### Freeプランの制約

**重要な発見**:
- **Freeプランでは全員がAdmin**になる（権限制限なし）
- **GuestアカウントはBusiness/Enterpriseプランのみ**
- Freeプランではゲスト招待は不可

**出典**:
- [Linear Docs - Invite members](https://linear.app/docs/invite-members)
- [Linear Docs - Members and roles](https://linear.app/docs/members-roles)
- [Master Guest User Management in Linear - SteelSync](https://www.steelsync.io/blog/master-guest-user-management-in-linear)

#### 招待方法

**UI操作が推奨**:
1. Settings > Administration > Members ページへ移動
2. 「Invite」ボタンをクリック
3. メールアドレスを入力（複数の場合はカンマ区切り）
4. チームを選択（オプション）
5. 招待を送信

**API経由の招待**:
- 公式ドキュメントに記載なし
- UI操作が推奨される

#### Session 271での対応

**招待対象**:
- 宇枝さん（上司）→ Admin
- 小笠原さん（上司）→ Admin
- 石川さん（チームメンバー）→ Admin

**招待手順**:
- Linear Web UI で手動招待
- ガイドドキュメントは作成しない（UI操作が明確なため）

---

### 4. Roadmap / Timeline Viewの仕様

#### Timeline Viewとは

**機能**:
- Projectの時系列表示
- 進捗、締切、依存関係を可視化
- ガントチャートのような使い方が可能

**表示粒度**:
- Week（週）
- Month（月）
- Quarter（四半期）
- Year（年）

**出典**: [Linear Docs - Timeline](https://linear.app/docs/timeline)

#### 使い方

**UI操作**:
1. 任意のProject viewに移動
2. 「Display」ドロップダウンをクリック
3. 「Timeline view」を選択

**Project Milestonesとの連携**:
- Projectにmilestoneを設定可能
- Timeline上でmilestoneの進捗を可視化

**出典**: [Linear Docs - Project milestones](https://linear.app/docs/project-milestones)

#### Freeプランでの利用可否

**調査結果**: 明示的な記載なし

**推測**（要検証）:
- Timeline viewはProject機能の一部
- Projectは基本機能なのでFreeプランでも利用可能と思われる
- 実際にLinear UIで確認が必要

---

## API操作の実例

### アクティブIssue数取得

```bash
curl -s -X POST https://api.linear.app/graphql \
  -H "Authorization: $(grep LINEAR_API_KEY .env | cut -d= -f2)" \
  -H "Content-Type: application/json" \
  -d '{"query":"{ issues { nodes { id identifier title state { name type } } } }"}'
```

**現在のアクティブIssue**（Session 271開始時）:
- QA-5: SIPOC作成と現場レビュー準備（Backlog）
- QA-6: プレート調査（暗黙知外部化）（Backlog）
- QA-7: 梱包変更作業・隠れコストの定量調査（Backlog）

**アクティブIssue数**: 3件（監視基準: ✅ 余裕あり）

### Workflow States取得

```bash
curl -s -X POST https://api.linear.app/graphql \
  -H "Authorization: $(grep LINEAR_API_KEY .env | cut -d= -f2)" \
  -H "Content-Type: application/json" \
  -d '{"query":"{ workflowStates { nodes { id name type team { id name } } } }"}'
```

---

## 次セッションでやること

### 1. ADR-017の修正

**修正箇所**:
- アーカイブ運用方針を「自動アーカイブ」に変更
- 「Issue完了時に即座にアーカイブ」を削除

**新ADRを作成するか、既存を修正するか**:
- ADRはImmutableが原則（ルール10参照）
- 今回は軽微な修正（誤解の訂正）なので、既存ADRを修正してもよい
- または、Superseded処理で新ADRを作成

### 2. linear-managementスキルの更新

**追加すべき内容**:
- 認証方法の正しい例（`grep`方式）
- Issue完了の方法（`issueUpdate` + `stateId`）
- アーカイブは自動実行であることを明記

### 3. メンバー招待の実施

**対象**:
- 宇枝さん、小笠原さん、石川さん

**方法**:
- Linear Web UI で手動招待
- Settings > Administration > Members > Invite

### 4. Timeline Viewの確認

**確認項目**:
- Freeプランで利用可能か
- 現在のProject（M3+M4: 検査プロセス改善）をTimeline表示できるか
- スクリーンショット取得

---

## 参照

### 公式ドキュメント

- [Linear Developers - Getting started](https://linear.app/developers/graphql)
- [Linear Docs - Delete and archive issues](https://linear.app/docs/delete-archive-issues)
- [Linear Docs - Invite members](https://linear.app/docs/invite-members)
- [Linear Docs - Members and roles](https://linear.app/docs/members-roles)
- [Linear Docs - Timeline](https://linear.app/docs/timeline)
- [Linear Docs - Project milestones](https://linear.app/docs/project-milestones)

### プロジェクト内資料

- [session270/linear-setup-commands.md](../session270/linear-setup-commands.md) — Session 270でのAPI操作記録
- [docs/adr/common/ADR-017-linear-free-plan-operation.md](../../docs/adr/common/ADR-017-linear-free-plan-operation.md) — Linear運用方針（修正必要）
- `~/.claude/skills/linear-management/SKILL.md` — Linear管理スキル（更新必要）

---

**調査完了**: 2026-03-19（Session 271）
