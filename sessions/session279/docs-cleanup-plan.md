# docs/断捨離 作業計画

**作成**: Session 279 (2026-03-23)
**方針**: [docs/progress-management-policy.md](../../docs/progress-management-policy.md)

---

## 概要

docs/ディレクトリの整理とprogress.mdの再構築を、複数セッションに分けて実施する。

**現状**:
- Markdownファイル: 141個
- docs/index.md: Session 47で停止（廃止予定）
- docs/progress.md: Session 234で停止
- missions/m1-sensor-evaluation/gnss/: 46ファイル

---

## マイルストーン

### Session 280: progress.md 再構築

**目標**: progress.mdを「ハブ形式」に書き換える

**前提**: Session 278 で各ミッションの状況は確認済み（[context-review-progress.md](../session278/context-review-progress.md)）

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | Session 278の中間ファイルを参照 | [session278/context-review-progress.md](../session278/context-review-progress.md) |
| 2 | Linear issueのリンクを収集 | Linear API |
| 3 | progress.md を書き換え | docs/progress.md |
| 4 | docs/index.md を削除 | - |

**Session 278で確認済みの情報**:
- M1: 再開時に読む → session229/session-summary.md
- M2: 放置中（FA率評価方法確認待ち）
- M3: 再開時に読む → docs/missions/m3-incoming-inspection-db/ai-research/
- M4: 未着手（Excel未入手）

**完了条件**:
- [ ] progress.md がハブ形式になっている
- [ ] 各ミッションに「再開時に読む」リンクがある
- [ ] docs/index.md が削除されている

---

### Session 281: docs/technical-research/ の整理

**目標**: 古い技術調査ファイルを断捨離

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | technical-research/ の各ファイルを確認 | docs/technical-research/*.md |
| 2 | 不要なファイルを削除 | - |
| 3 | 残すファイルをmissions/に移動（必要なら） | - |

**完了条件**:
- [ ] technical-research/ のファイル数が半減
- [ ] 残すファイルの理由が明確

---

### Session 282: docs/missions/ の整理（M3/M4）

**目標**: M3/M4のドキュメントを整理

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | M3のai-research/を確認 | docs/missions/m3-incoming-inspection-db/ai-research/ |
| 2 | 不要なファイルを削除 | - |
| 3 | README.mdを更新 | 各README.md |

**完了条件**:
- [ ] M3/M4のREADME.mdが最新状態
- [ ] 不要ファイルが削除されている

---

### Session 283: docs/missions/ の整理（M1/M2）

**目標**: M1/M2のドキュメントを整理

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | gnss/の46ファイルを確認 | docs/missions/m1-sensor-evaluation/gnss/ |
| 2 | 統合できるファイルを統合 | - |
| 3 | README.mdを更新 | 各README.md |

**完了条件**:
- [ ] gnss/のファイル構成が明確
- [ ] README.mdが最新状態

---

### Session 284: session-management スキル更新 + 最終確認

**目標**: 運用ルールを確定

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | session-management スキルに更新ルール追加 | ~/.claude/skills/session-management/SKILL.md |
| 2 | 全体の整合性確認 | docs/progress.md, 各README.md |
| 3 | 必要ならスキル化（progress-management） | - |

**完了条件**:
- [ ] session-management スキルが更新されている
- [ ] 進捗管理の運用が確立されている

---

## 注意事項

- 各セッションで無理に全て終わらせようとしない
- 判断に迷ったら「残す」より「削除」を優先（session-historyに記録あり）
- 削除前にファイル名と概要をsession-summaryに記録

---

## 参考

| ファイル | 内容 |
|----------|------|
| [progress-management-policy.md](../../docs/progress-management-policy.md) | 方針 |
| [progress-management-research.md](progress-management-research.md) | 調査結果 |
