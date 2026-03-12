# Session 127 計画

**目的**: M1-GNSS ドキュメント整理の実行

**前提**: Session 126で作成した計画書に基づく

---

## やること

### 1. docs/gnss/ の番号振り直し（4ファイル）

| 現状 | 修正後 |
|------|--------|
| 23-outdoor-inspection-domain-model.md | **26**-outdoor-inspection-domain-model.md |
| 24-outdoor-inspection-implementation-plan.md | **27**-outdoor-inspection-implementation-plan.md |
| 25-tdd-review-result.md | **28**-tdd-review-result.md |
| architecture-nextjs-rust.md | **29**-architecture-nextjs-rust.md |

### 2. sessions/ の置き去りドキュメント整理（7件）

各ファイルの先頭を確認し、行き先を確定:

| Session | ファイル | 行き先（案） |
|---------|----------|-------------|
| 107 | ubx-spec-extract.md | docs/に移動（30番） |
| 107 | nav-sig-behavior-spec.md | docs/に移動（31番） |
| 111 | ttff-monrf-spec.md | docs/に移動（32番） |
| 111 | outdoor-inspection-priority.md | 削除 |
| 111 | m1-gnss-all-tasks.md | 削除 or 統合 |
| 111 | m1-gnss-milestone.md | 削除 |
| 112 | nav-sig-api-design.md | prototype/m1-gnss/docs/に移動 |

### 3. README.md の更新

追加する内容:
- 「○○するときは△△を見る」チェックリスト
- Phase情報の更新（Phase 3: ツール実装）
- 全ファイルの登録（番号順）

### 4. prototype/m1-gnss/CLAUDE.md の確認・更新

---

## 必要な読み込み

- docs/missions/m1-sensor-evaluation/gnss/README.md（編集対象）
- sessions/session107/*.md（先頭のみ）
- sessions/session111/*.md（先頭のみ）
- sessions/session112/*.md（先頭のみ）
- prototype/m1-gnss/CLAUDE.md

---

## 参照

- [Session 126 計画書](../session126/documentation-improvement-plan.md)

---

*計画作成: 2026-03-12 Session 126終了時*
