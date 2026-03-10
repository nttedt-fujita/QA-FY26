# Session 75 計画

**目的**: ADR・ドキュメント整理（実装前の立ち止まり）

---

## 背景

Session 74でADR-005とSession 61の決定が乖離していることが発覚:
- ADR-005: 「静的HTML + JavaScript」
- Session 61: 「Next.js（React）」

この混乱を整理してから実装に進む。

---

## やること

### 1. ADR-005のメンテナンス

| 選択肢 | 内容 |
|--------|------|
| A. 更新 | Session 61の決定（Next.js）をADR-005に反映 |
| B. 廃止 | ADR-005を「Superseded」にし、Session 61をマスターとする |
| C. 分離 | ADR-005は初期方針として残し、別ADRで変更を記録 |

**判断基準**:
- ADRの目的は何か？
- 変更履歴を残すべきか？

### 2. ADR運用ルールの見直し

| 項目 | 現状 | 改善案 |
|------|------|--------|
| ADR更新タイミング | 不明確 | 決定変更時は必ずADR更新 |
| ADRとセッション履歴の関係 | 不明確 | ADR=承認済み決定、履歴=経緯 |
| Claudeの参照順序 | ADRのみ | セッション履歴→ADRの順 |

**更新対象ファイル**:
- `~/.claude/skills/adr-management/SKILL.md` — ADR運用スキル
- `~/.claude/rules/10-adr-enforcement.md` — ADR強制ルール
- プロジェクト `CLAUDE.md` — ADR一覧

### 3. CLAUDE.mdのADR一覧更新

- ADR-005の状態を明確化
- 「Session 61で変更あり」の注記を追加

### 4. Session 73/74のドキュメント正式配置

| ファイル | 配置先 |
|----------|--------|
| session73/gnss-tool-needs.md | ✅ 済（14-gnss-tool-needs.md） |
| session74/gnss-tool-requirements.md | `docs/missions/m1-sensor-evaluation/gnss/15-gnss-tool-requirements.md` |
| session74/gnss-tool-architecture.md | `docs/missions/m1-sensor-evaluation/gnss/16-gnss-tool-architecture.md` |
| session74/gnss-tool-implementation-plan.md | `docs/missions/m1-sensor-evaluation/gnss/17-gnss-tool-implementation-plan.md` |

### 5. hooks観察の整理

Session 74で記録した6件の観察を確認し、優先度をつける。

---

## 参照資料

- [ADR-005](../../docs/adr/ADR-005-gnss-tool-tech-stack.md) — 技術スタック
- [Session 61 サマリー](../session61/session-summary.md) — Next.js決定
- [Session 74 サマリー](../session74/session-summary.md) — 問題発覚

---

*計画作成: 2026-03-10 Session 74終了時*
