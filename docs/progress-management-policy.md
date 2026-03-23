# 進捗管理方針

**作成**: Session 279 (2026-03-23)
**出典**: [sessions/session279/progress-management-research.md](../sessions/session279/progress-management-research.md)

---

## 目的

セッション開始時のコンテキスト確認コストを削減し、メンテナンスフリーな進捗管理を実現する。

---

## 原則

### 1. SSOT（Single Source of Truth）の明確化

| 情報の種類 | SSOT（正） | 役割 |
|-----------|-----------|------|
| **タスク状態** | Linear | 何をやるか、進捗状態 |
| **セッション記録** | session-history/ | 何をやったか（時系列） |
| **俯瞰ビュー** | progress.md | 各ミッションの現在地（ハブ） |

### 2. リンクで参照、複製しない

- 詳細は1箇所に書く
- 他からはリンクで参照
- 「to reuse means to link or reference, not copy and paste」

### 3. 更新は最小限に

- progress.md にはリンクと状態のみ
- 説明文は書かない（陳腐化の原因）

---

## progress.md の役割

### Before（現状）

```markdown
## M1-B: GNSS評価
**状況**: プロトタイプ実装中
**次のアクション**: 実機テスト継続、並列検査機能
```

→ 説明文が陳腐化、更新されない

### After（改善後）

```markdown
## M1: GNSS評価ツール

| 項目 | 内容 |
|------|------|
| 状態 | 開発中（Phase 3） |
| Linear | [M1-GNSS](リンク) |
| 最新セッション | [Session 229](sessions/session229/session-summary.md) |
| 再開時に読む | [gnss/README.md](missions/m1-sensor-evaluation/gnss/README.md) |
```

→ リンクのみ、更新は最小限

---

## 更新ルール

### セッション終了時（必須）

1. **session-history に追記**（従来通り）
2. **progress.md の該当ミッションを更新**
   - 最新セッションへのリンクを更新
   - 状態が変わった場合のみ状態を更新
   - 「再開時に読む」が変わった場合のみ更新

### 禁止

- progress.md に説明文を書くこと
- progress.md に詳細なファイル一覧を書くこと
- 更新を「後でまとめてやる」こと

---

## docs/index.md の扱い

**方針**: 廃止

- progress.md がハブとして機能するため不要
- 各ディレクトリのREADME.mdで十分
- 断捨離作業で削除する

---

## 関連ルール

- [14-document-management.md](../.claude/rules/14-document-management.md) — ドキュメント管理ルール
- [07-information-routing.md](../.claude/rules/07-information-routing.md) — 情報配置ルール
- [session-management スキル](../.claude/skills/session-management/SKILL.md) — セッション管理

---

## 出典

- [SSOT原則（Atlassian）](https://www.atlassian.com/work-management/knowledge-sharing/documentation/building-a-single-source-of-truth-ssot-for-your-team)
- [Living Documentation（CodeLucky）](https://codelucky.com/agile-documentation-living-approach/)
- [Docs as Code（Google）](https://google.github.io/styleguide/docguide/best_practices.html)
