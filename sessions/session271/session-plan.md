# Session 271 計画

**目的**: Linear API公式ドキュメント調査とメンバー招待

**前提**: Session 270でLinear運用方針策定、スキル実装完了

---

## やること

| # | 作業 | 読むべきファイル | 参照コマンド |
|---|------|-----------------|--------------|
| 1 | Linear API公式ドキュメント調査 | - | - |
| 2 | メンバー招待手順の調査・実施 | [session270/session-summary.md](../session270/session-summary.md) | - |
| 3 | Issueの追加・アーカイブ | [session267/context-and-background.md](../session267/context-and-background.md) | - |
| 4 | ガントチャート表示確認 | - | - |

---

## 詳細

### 1. Linear API公式ドキュメント調査（最優先）

**目的**: 推測でAPI実装をしない、公式仕様に基づいた実装

**調査内容**:
- **GraphQL Schema**: Linear APIの全体構造
- **Issue作成**: 正式なフィールド、状態管理の仕様
- **Project作成**: Milestoneの設定方法
- **Roadmap view**: ガントチャート表示の設定
- **アーカイブ**: Archive操作の正式な仕様

**参照先**:
- [Linear API Documentation](https://developers.linear.app/docs)
- [Linear GraphQL API Reference](https://studio.apollographql.com/public/Linear-API/variant/current/home)

**成果物**: `sessions/session271/linear-api-research.md`

---

### 2. メンバー招待手順の調査・実施

**目的**: 宇枝さん、小笠原さん、石川さんをLinearに招待

**調査内容**:
- **Guest vs Member**: 権限の違い
- **招待手順**: UI操作またはAPI
- **推奨設定**: まずはGuestで招待（閲覧のみ）

**招待対象**:
- 宇枝さん（上司）
- 小笠原さん（上司）
- 石川さん（チームメンバー）

**成果物**: `docs/tools/linear/member-invitation-guide.md`

---

### 3. Issueの追加・アーカイブ

**目的**: 現在の進捗を全てLinearに反映

**追加するIssue**:
- フレーム・モーター調査（暗黙知外部化）
- 田原さん・杉山さんヒアリング
- Excel記録作業調査
- その他Phase 1の残タスク

**アーカイブ対象**:
- QA-6（プレート調査）— 完了済み
- デフォルトissue（QA-1〜QA-4）— 不要

**成果物**: Linearに反映（Issue作成・アーカイブ）

---

### 4. ガントチャート表示確認

**目的**: 他のメンバーが進捗・工程を確認できることを確認

**確認内容**:
- Roadmap viewの表示
- Project timeline
- Issue間の依存関係（必要に応じて）

**成果物**: スクリーンショット（確認用）

---

## 参照

- [session270/session-summary.md](../session270/session-summary.md) — 前セッションサマリー
- [docs/adr/common/ADR-017-linear-free-plan-operation.md](../../docs/adr/common/ADR-017-linear-free-plan-operation.md) — Linear運用方針
- `~/.claude/skills/linear-management/SKILL.md` — Linear管理スキル

---

## 制約

- **推測禁止**: Linear APIを推測で実装しない、必ず公式ドキュメントを確認
- **API調査優先**: Issue作成・アーカイブの前に、正式な仕様を確認

---

**期待成果**: Linear APIの正式な仕様に基づいた実装、メンバー招待完了、ガントチャート確認
