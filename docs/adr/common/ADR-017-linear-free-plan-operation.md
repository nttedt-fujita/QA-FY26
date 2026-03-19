# ADR-017: Linearの無料プラン運用方針

## Status
Accepted（2026-03-19、Session 270）

## Context

### 背景

QA-FY26プロジェクトの進捗管理にLinearを導入（Session 269）。以下の要件を満たす運用方針が必要：

1. **進捗の可視化**: 他のメンバー（田原さん、杉山さん）や上司（小笠原さん、宇枝さん）が進捗・工程を確認できる
2. **予算制約**: 初期段階では追加コストを避けたい、有料化する場合は上司に相談
3. **Linear制限**: Freeプラン（250 active issues）で運用可能か検討が必要

### Linear料金プラン

**出典**: [Linear Docs - Billing and plans](https://linear.app/docs/billing-and-plans)

| プラン | 料金 | Active issue制限 | Archived issue |
|--------|------|-----------------|----------------|
| **Free** | 無料 | 250件まで | 無制限 |
| **Basic** | $10/user/月 | 無制限 | 無制限 |

**重要**: アーカイブしたissueは制限にカウントされない。

### Issue数見積もり（Session 268）

**出典**: [docs/tools/linear/issue-estimate.md](../../tools/linear/issue-estimate.md)

| 範囲 | 総Issue数 | アクティブ数（最大） |
|------|-----------|---------------------|
| M3+M4 Phase 1（現状可視化） | 28-33 | 14 |
| M3+M4 Phase 2-3 | 27 | 27 |
| M1-M4全体 | 165-230 | 71-101 |

**見積もりの前提**:
- 全ミッション同時進行の場合: アクティブ71-101件
- 現実的な運用（順次進行）: アクティブ30-50件

### ユーザーの要望（Session 270）

- 「お金かかりそうになったら小笠原さん・宇枝さんに相談」
- 他の人が見て「進捗・工程の流れが分かる粒度」
- アクティブissue数を常に監視する仕組み

---

## Decision

### 1. 料金プラン

**Freeプランで運用開始**:
- アクティブissue数を常に監視
- 150件到達時に上司に相談
- 200件超えでBasicプラン（$10/月）移行検討

**判断基準**:

| アクティブissue数 | 状態 | 対応 |
|-----------------|------|------|
| **0-100件** | ✅ 余裕あり | Freeプラン継続 |
| **100-150件** | 🟡 注意 | 監視強化、アーカイブ徹底 |
| **150-200件** | 🟠 警告 | **小笠原さん・宇枝さんに事前相談** |
| **200-250件** | 🔴 限界 | Basicプラン移行必須（$10/月） |

### 2. Issue粒度

**中程度（作業単位）**:
- Phase > Epic > Issue（作業単位）> チェックリスト（サブタスク）
- 例: 「SIPOC作成と現場レビュー」「プレート調査」「梱包変更作業の工数調査」

**理由**:
- ✅ 進捗・工程が可視化される
- ✅ アクティブissue数を抑制（Phase 1で5-10件程度）
- ✅ Freeプランの枠に余裕

**Issue説明文にチェックリストを記載**:
```markdown
## 作業内容
- [x] 全体フローSIPOC作成
- [x] 受入検査詳細SIPOC作成
- [ ] 田原さん・杉山さんレビュー
- [ ] フィードバック反映
```

### 3. アーカイブ運用

**Issue完了時に即座にアーカイブ**:
- アクティブissue数を常に最小限に保つ
- アーカイブしたissueは無制限（検索・参照可能）

### 4. 監視の仕組み

**セッション開始時に自動確認**:
- `linear-management` スキルで実装
- `session-management` スキルから呼び出し
- アクティブissue数を取得し、基準に基づいて警告

**実装**:
- `~/.claude/skills/linear-management/SKILL.md`（Session 270で作成）
- Linear GraphQL APIでアクティブissue数を取得
- 150件到達時に上司への相談を提案

---

## Consequences

### 良い点

1. **コスト節約**: Freeプランで当面運用可能
2. **柔軟な移行**: 必要になったら上司に相談してBasicプラン移行
3. **制約の明確化**: 150件で相談、200件で移行という基準が明確
4. **自動監視**: セッション開始時に自動確認、上限到達前に警告

### 悪い点

1. **アーカイブの徹底が必要**: Issue完了時に毎回アーカイブ
2. **粒度の制約**: 細かすぎるissueは作成できない（チェックリストで代用）
3. **上限リスク**: 250件到達時に新規issue作成不可（ただし事前警告あり）

### リスク

| リスク | 対策 |
|--------|------|
| アクティブissueが予想以上に増える | 150件で相談開始、アーカイブ徹底 |
| アーカイブ忘れで上限到達 | セッション開始時に自動監視・警告 |
| 複数ミッション同時進行で枠不足 | 優先度を決めて順次進行、または相談 |

---

## 参照

### プロジェクト内ドキュメント

- [docs/tools/linear/issue-estimate.md](../../tools/linear/issue-estimate.md) — Issue数見積もり（Session 268）
- [docs/tools/linear/hybrid-operation-rules.md](../../tools/linear/hybrid-operation-rules.md) — Linear + Markdown ハイブリッド運用
- `~/.claude/skills/linear-management/SKILL.md` — Linear管理スキル（Session 270で作成）
- `~/.claude/skills/session-management/SKILL.md` — セッション管理スキル（Linear呼び出しを追加）

### 外部ドキュメント

- [Linear Docs - Billing and plans](https://linear.app/docs/billing-and-plans)
- [Delete and archive issues](https://linear.app/docs/delete-archive-issues)

---

**作成**: Session 270 (2026-03-19)
**決定者**: 藤田さん
**次回レビュー**: アクティブissue 100件到達時、または3ヶ月後
