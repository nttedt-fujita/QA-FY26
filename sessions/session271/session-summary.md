# Session 271 サマリー

**日付**: 2026-03-19
**目的**: Linear API公式ドキュメント調査とメンバー招待

---

## 実施内容

### 1. Linear API認証の確認

**問題**: Session開始時、前回（Session 270）で動作していたLinear APIが認証エラーになった

**原因**: 認証ヘッダーの形式が異なっていた
- ❌ 今回試した方法: `source .env && ... -H "Authorization: $LINEAR_API_KEY"`
- ✅ 正しい方法: `-H "Authorization: $(grep LINEAR_API_KEY .env | cut -d= -f2)"`

**解決**: [session270/linear-setup-commands.md](../session270/linear-setup-commands.md)を参照し、前回と同じ方法を使用

**教訓**: 前回の作業記録（コマンドの実例）が重要

### 2. アーカイブ操作の仕様確認

**重要な発見**: **Linearのアーカイブは自動実行（手動操作不可）**

**仕様**:
- Issueを完了状態（Done）にすると、設定されたアーカイブ期間後に自動アーカイブ
- 手動でのアーカイブ操作は不可
- チーム設定（Team Settings > Issue statuses & automations）でアーカイブ期間を調整可能

**Issue完了の方法**:
1. Workflow Stateを取得して完了状態のstate IDを確認
2. `issueUpdate` Mutationで`stateId`を完了状態（Done）に変更

**QA-FY26-FUJITAチームのWorkflow States**:
| State ID | Name | Type |
|----------|------|------|
| `9e1b52f8-91f8-48f6-a1ea-8d09af23dcaf` | Backlog | backlog |
| `491cb226-953a-4f85-ab8c-2debbe177d17` | Todo | unstarted |
| `18c8a0ea-747d-4e33-bfaa-5eb18c2bdec1` | In Progress | started |
| `0a1243c8-726a-4e0c-838a-bf1066d26fea` | **Done** | **completed** |
| `2610a692-a023-4301-b7ed-1fbca2e3b5bc` | Canceled | canceled |
| `e4cdaf51-001d-4833-988d-8d8d6860feab` | Duplicate | canceled |

**出典**: [Linear Docs - Delete and archive issues](https://linear.app/docs/delete-archive-issues)

### 3. メンバー招待の仕様確認

**重要な発見**:
- **Freeプランでは全員がAdmin**になる（権限制限なし）
- **GuestアカウントはBusiness/Enterpriseプランのみ**（Freeでは不可）

**招待方法**:
- UI操作が推奨（Settings > Administration > Members > Invite）
- API経由の招待は公式ドキュメントに記載なし

**出典**:
- [Linear Docs - Invite members](https://linear.app/docs/invite-members)
- [Linear Docs - Members and roles](https://linear.app/docs/members-roles)
- [Master Guest User Management in Linear - SteelSync](https://www.steelsync.io/blog/master-guest-user-management-in-linear)

### 4. Roadmap / Timeline View の確認

**機能**:
- Projectの時系列表示
- 進捗、締切、依存関係を可視化
- ガントチャートのような使い方が可能

**表示粒度**: Week, Month, Quarter, Year

**使い方**: Project view > Display dropdown > Timeline view

**Freeプランでの利用可否**: 明示的な記載なし（基本機能と思われるが要検証）

**出典**: [Linear Docs - Timeline](https://linear.app/docs/timeline)

### 5. linear-managementスキルの更新

**追加内容**:
- 認証方法の正しい例（`grep`方式）
- アーカイブは自動実行であることを明記
- Issue完了の方法（`issueUpdate` + `stateId`）

**更新箇所**: `~/.claude/skills/linear-management/SKILL.md`

### 6. ADR-017の修正

**修正理由**: Linearの仕様を誤解していた（手動アーカイブができると思っていた）

**修正内容**:
- Statusに修正履歴を追加
- 「アーカイブ運用」セクションを修正（自動アーカイブ）
- Consequencesの「悪い点」を修正

**修正方針**: 既存ADRを修正（誤解の訂正レベル、方針は変わっていない）

**ファイル**: [docs/adr/common/ADR-017-linear-free-plan-operation.md](../../docs/adr/common/ADR-017-linear-free-plan-operation.md)

---

## 主な発見

### 1. Linearのアーカイブは自動実行

**影響**:
- ADR-017の「Issue完了時に即座にアーカイブ」という記載が誤り
- 正しくは「Issue完了すると自動アーカイブ」
- 手動アーカイブ操作は不要（むしろできない）

### 2. API認証の正しい方法

**正しい方法**:
```bash
-H "Authorization: $(grep LINEAR_API_KEY .env | cut -d= -f2)"
```

**間違った方法**:
```bash
source .env && ... -H "Authorization: $LINEAR_API_KEY"
```

### 3. Freeプランの制約

- 全員がAdmin（権限制限なし）
- GuestアカウントはBusiness/Enterpriseのみ
- メンバー招待はUI操作推奨

---

## 成果物

### 作成ファイル

| ファイル | 内容 |
|----------|------|
| [session271/linear-api-research.md](linear-api-research.md) | Linear API調査レポート（認証、アーカイブ、メンバー招待、Timeline view） |
| [session271/session-summary.md](session-summary.md) | セッションサマリー |

### 更新ファイル

| ファイル | 変更内容 |
|----------|----------|
| `~/.claude/skills/linear-management/SKILL.md` | 認証方法、アーカイブ運用、Issue完了方法を追加 |
| [docs/adr/common/ADR-017-linear-free-plan-operation.md](../../docs/adr/common/ADR-017-linear-free-plan-operation.md) | アーカイブ運用の記載を修正（自動実行） |

---

## Linear監視結果

**アクティブissue数**: 3件（QA-5, QA-6, QA-7）
**状態**: ✅ 余裕あり（0-100件）
**対応**: Freeプラン継続、問題なし

---

## 残った課題

### 1. メンバー招待の実施

**対象**:
- 宇枝さん（上司）→ Admin
- 小笠原さん（上司）→ Admin
- 石川さん（チームメンバー）→ Admin

**方法**: Linear Web UI で手動招待（Settings > Administration > Members > Invite）

### 2. Timeline Viewの確認

**確認項目**:
- Freeプランで利用可能か
- 現在のProject（M3+M4: 検査プロセス改善）をTimeline表示できるか
- スクリーンショット取得

### 3. Issueの追加・完了

**追加するIssue**:
- フレーム・モーター調査（暗黙知外部化）
- 田原さん・杉山さんヒアリング
- Excel記録作業調査
- その他Phase 1の残タスク

**完了するIssue**:
- QA-6（プレート調査）— Session 267で完了済み

---

## 次セッションでやること

### 優先度: 高（Linear関連）

1. **メンバー招待の実施**
   - Linear Web UI で宇枝さん、小笠原さん、石川さんを招待

2. **Timeline Viewの確認**
   - Freeプランで利用可能か確認
   - 現在のProjectをTimeline表示してスクリーンショット

3. **Issueの追加・完了**
   - 完了済みのQA-6をDone状態に変更
   - 新規Issueを追加（Phase 1の残タスク）

### 優先度: 中（GNSS作業）

4. **GNSS作業の継続**
   - M1-GNSS関連の作業（前セッションから持ち越し）

### 優先度: 低（説明資料）

5. **宇枝さん説明資料作成**
   - AI導入の現実を伝える資料
   - 検査プロセス改善の進捗報告

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
- [session270/session-summary.md](../session270/session-summary.md) — 前セッションサマリー
- [docs/adr/common/ADR-017-linear-free-plan-operation.md](../../docs/adr/common/ADR-017-linear-free-plan-operation.md) — Linear運用方針（修正済み）
- `~/.claude/skills/linear-management/SKILL.md` — Linear管理スキル（更新済み）

---

**次セッション**: session272（Linear操作続き or GNSS作業）
