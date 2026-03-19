# Session 273 計画

**目的**: トークン効率化の効果検証とLinear運用タスクの実施

**前提**: Session 272でトークン効率化の改善実施（-11,000 tokens推定）

---

## やること

| # | 作業 | 読むべきファイル | 参照コマンド |
|---|------|-----------------|--------------|
| 1 | トークン効率化の効果検証 | [session272/token-efficiency-improvement.md](../session272/token-efficiency-improvement.md) | - |
| 2 | Workflow State運用方針の議論・決定 | [session271/session-summary.md](../session271/session-summary.md) | - |
| 3 | メンバー招待の実施 | - | - |
| 4 | Timeline Viewの確認 | - | - |
| 5 | Issueの追加・完了 | [session267/context-and-background.md](../session267/context-and-background.md) | - |

---

## 詳細

### 1. トークン効率化の効果検証（最優先）

**目的**: Session 272で実施した改善の効果を確認

**確認項目**:
- [ ] セッション開始時のトークン消費量（目標: 40,000 tokens以下、20%以下）
- [ ] session-historyの読み込み時間
- [ ] 改善効果の実測

**推定効果**:
- 改善前: 57,327 tokens（28.7%）
- 改善後: 46,327 tokens（23.2%）
- 削減量: -11,000 tokens（-19.2%）

**記録**:
- `session273/token-efficiency-verification.md` に結果を記録

---

### 2. Workflow State運用方針の議論・決定

**背景**: Session 271でWorkflow Stateを取得したが、運用方針が未確定

**検討事項**:
- **Backlog**: 藤田さんがやる作業リスト全体で良いか？
- **Todo**: 次に着手するタスク（優先度高）で良いか？
- **In Progress**: 現在作業中（1件のみ推奨）で良いか？
- **Done**: 完了（自動アーカイブ待ち）で良いか？
- **Canceled / Duplicate**: 使い分けは？

**Session 271での提案**:
```markdown
| State | 用途 | 例 |
|-------|------|-----|
| **Backlog** | 今後やる予定のタスク | 「梱包変更作業調査」「フレーム調査」 |
| **Todo** | 次に着手するタスク（優先度高） | 「SIPOCレビュー準備」 |
| **In Progress** | 現在作業中 | 「プレート調査（ヒアリング中）」 |
| **Done** | 完了 | 「SIPOC作成」 |
```

**決定すべきこと**:
1. 上記の運用方針で良いか
2. 現在のIssue（QA-5, QA-6, QA-7）をどのStateに配置するか
3. 新規Issueを作成する際のStateは？

**成果物**: Workflow State運用方針のドキュメント（ADR or docs/tools/linear/）

---

### 3. メンバー招待の実施

**目的**: 宇枝さん、小笠原さん、石川さんをLinearに招待

**手順**:
1. Linear Web UI にアクセス（https://linear.app/）
2. Settings > Administration > Members に移動
3. 「Invite」ボタンをクリック
4. メールアドレスを入力（カンマ区切りで複数可）
   - 宇枝さん: `ueda@nttedt.co.jp`（確認必要）
   - 小笠原さん: `ogasawara@nttedt.co.jp`（確認必要）
   - 石川さん: `ishikawa@nttedt.co.jp`（確認必要）
5. 招待を送信

**注意**:
- Freeプランでは全員がAdminになる
- Guestは使用不可（Business/Enterpriseのみ）

---

### 4. Timeline Viewの確認

**目的**: ガントチャート表示の確認

**手順**:
1. Linear Web UI でProjectページに移動
2. 「Display」ドロップダウンをクリック
3. 「Timeline view」を選択
4. 表示を確認（Freeプランで利用可能か）
5. スクリーンショット取得

**確認項目**:
- Freeプランで利用可能か
- Project「M3+M4: 検査プロセス改善」がTimeline表示されるか
- 表示粒度（Week, Month, Quarter, Year）の切り替えが可能か

---

### 5. Issueの追加・完了

**完了するIssue**:
- QA-6（プレート調査）— Session 267で実態調査完了

**追加するIssue**:
- フレーム・モーター調査（暗黙知外部化）
- 田原さん・杉山さんヒアリング（SIPOC レビュー）
- Excel記録作業調査
- その他Phase 1の残タスク

**Issue完了の方法** (QA-6):
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

## 参照

- [session272/session-summary.md](../session272/session-summary.md) — 前セッションサマリー
- [session272/token-efficiency-improvement.md](../session272/token-efficiency-improvement.md) — トークン効率化の改善内容
- [session271/session-summary.md](../session271/session-summary.md) — Linear API調査
- [session271/linear-api-research.md](../session271/linear-api-research.md) — Linear API調査レポート
- [session270/linear-setup-commands.md](../session270/linear-setup-commands.md) — Linear操作コマンド集
- [session267/context-and-background.md](../session267/context-and-background.md) — 全体文脈整理

---

**期待成果**: 効果検証完了、Workflow State運用方針決定、メンバー招待完了、Timeline確認、Issue整理
