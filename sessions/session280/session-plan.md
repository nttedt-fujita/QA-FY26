# Session 280 計画

**目的**: progress.md を「ハブ形式」に書き換え + docs/index.md 削除

**前提**:
- Session 279 で方針策定・作業計画完了
- Session 278 で各ミッションの状況は確認済み

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | Session 278の中間ファイルを参照 | [session278/context-review-progress.md](../session278/context-review-progress.md) |
| 2 | Linear issueのリンクを収集 | Linear API |
| 3 | progress.md を書き換え | docs/progress.md |
| 4 | docs/index.md を削除 | - |

---

## Session 278で確認済みの情報（再調査不要）

| ミッション | 状況 | 再開時に読む |
|-----------|------|-------------|
| M1 | BBR優先順位問題解決済み、放置 | session229/session-summary.md |
| M2 | 放置中（FA率評価方法確認待ち） | 未特定 |
| M3 | ⏸️ストップ中 | docs/missions/m3-incoming-inspection-db/ai-research/ |
| M4 | 未着手（Excel未入手） | - |

---

## 詳細

### 1. Session 278の中間ファイルを参照

**やること**: 上記の表を使う（再調査しない）

### 2. Linear issueのリンクを収集

各ミッションに対応するLinear issueのURLを取得。

### 3. progress.md を書き換え

ハブ形式に書き換え:
```markdown
## M1: GNSS評価ツール

| 項目 | 内容 |
|------|------|
| 状態 | 開発中（Phase 3） |
| Linear | [リンク] |
| 最新セッション | [Session XXX](リンク) |
| 再開時に読む | [gnss/README.md](リンク) |
```

### 4. docs/index.md を削除

git rm で削除。

---

## 完了条件

- [ ] progress.md がハブ形式になっている
- [ ] 各ミッションに「再開時に読む」リンクがある
- [ ] docs/index.md が削除されている

---

## 参照

- [session279/docs-cleanup-plan.md](../session279/docs-cleanup-plan.md) — 全体作業計画
- [docs/progress-management-policy.md](../../docs/progress-management-policy.md) — 方針
