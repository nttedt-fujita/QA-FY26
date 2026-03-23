# Session 283 計画

**目的**: linear-managementスキルとprogress.mdの連携を明示化

**前提**: Session 282でsession-managementスキル更新、progress.md整合性確認完了

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | linear-managementスキルに「progress.mdとの関係」セクション追加 | ~/.claude/skills/linear-management/SKILL.md |
| 2 | progress-management-policy.mdとの整合性確認 | docs/progress-management-policy.md |

---

## 詳細

### 1. linear-managementスキルへの追記

**現状の問題**:
- linear-managementを単独で読んだ時、progress.mdとの関係が分からない
- session-managementが両方を管理しているが、連携が暗黙的

**追記内容案**:
```markdown
## progress.mdとの関係

Linear issueの状態は `docs/progress.md` に反映される。

**更新タイミング**: セッション終了時（session-managementスキル セクション6）
**更新内容**: 「今週の活動」セクションのLinear issueリンク

詳細: `docs/progress-management-policy.md` 参照
```

---

## 参照

- [session282/session-summary.md](../session282/session-summary.md)
- [linear-management/SKILL.md](~/.claude/skills/linear-management/SKILL.md)
