# Session 129 計画

**目的**: ドキュメント整理の続き — グローバルルールへの反映

**前提**: Session 128でM1ドキュメント整理・検証完了

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | session-management スキルの計画テンプレート更新 | ~/.claude/skills/session-management/SKILL.md |
| 2 | 「読むべきファイル」カラム必須化 | docs/documentation-improvement-plan.md (セクション5) |
| 3 | `~/.claude/rules/14-document-management.md` 新規作成 | docs/documentation-improvement-plan.md (セクション4.7) |

---

## 詳細

### 1. session-management スキルの計画テンプレート更新

Session 127で決定した方針:
- セッション計画に「読むべきファイル」を明記することで、トークン節約
- 計画作成時にチェックリストを参照して記入
- セッション開始時は計画を読むだけで済むようにする

### 2. 新ルールの作成

`~/.claude/rules/14-document-management.md` に以下を含める:
- 出典必須ルール
- 番号の自動採番ルール
- セッション終了時の即時処理（正式配置 or 削除）

---

## 参照

- [Session 128 summary](../session128/session-summary.md)
- [documentation-improvement-plan.md](../../docs/documentation-improvement-plan.md)

---

*計画作成: 2026-03-12 Session 128終了時*
