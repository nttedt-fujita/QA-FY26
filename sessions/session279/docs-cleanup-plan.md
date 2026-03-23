# docs/断捨離 作業計画

**作成**: Session 279 (2026-03-23)
**方針**: [docs/progress-management-policy.md](../../docs/progress-management-policy.md)
**更新**: Session 281 (2026-03-23) — 完了状態を反映

---

## 概要

docs/ディレクトリの整理とprogress.mdの再構築を、複数セッションに分けて実施する。

**開始時の状態**:
- Markdownファイル: 141個
- docs/index.md: Session 47で停止（廃止予定）
- docs/progress.md: Session 234で停止
- missions/m1-sensor-evaluation/gnss/: 46ファイル

---

## 実績

| 計画Session | 作業内容 | 実施Session | 状態 |
|------------|---------|------------|------|
| 280 | progress.md再構築 | 280 | ✅完了 |
| 281 | technical-research/整理 | 280（前倒し） | ✅完了（14→8ファイル） |
| 282 | M3/M4整理 | 281（前倒し） | ✅完了（削除なし、リンク修正） |
| 283 | M1/M2整理 | 281（前倒し） | ✅完了（削除なし、リンク修正） |
| 284 | スキル更新 | 282 | ✅完了 |

**結果**: 計画5セッション → 実績3セッションで完了見込み

---

## マイルストーン（完了状態）

### Session 280: progress.md 再構築 ✅

**実施日**: 2026-03-23

**完了条件**:
- [x] progress.md がハブ形式になっている
- [x] 各ミッションに「再開時に読む」リンクがある
- [x] docs/index.md が削除されている

**追加実施**: technical-research/整理（Session 281計画を前倒し）
- 削除: 6ファイル（kintone関連、旧技術比較）
- 移動: AWS関連をサブディレクトリに整理
- 結果: 14→8ファイル

---

### Session 281: M3/M4 + M1/M2 整理 ✅

**実施日**: 2026-03-23

**M3（受入検査DB）**:
- ファイル数: 22（6サブディレクトリ）
- 判断: 全ファイル再開時に必要、削除対象なし
- 修正: README.mdの壊れリンク修正

**M4（工程不良DB）**:
- ファイル数: 2
- 判断: 変更なし

**M1（センサー評価）**:
- ファイル数: 68（gnss/51 + lidar/14 + ルート3）
- 判断: 全ファイル整理済み（連番管理）、削除対象なし
- 修正: gnss/README.mdの壊れリンク削除、パス修正

**M2（点群データ検証）**:
- ファイル数: 4
- 判断: 削除対象なし
- 修正: パス修正

---

### Session 282: スキル更新 ✅

**実施日**: 2026-03-23

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | session-management スキルに更新ルール追加 | ~/.claude/skills/session-management/SKILL.md |
| 2 | 全体の整合性確認 | docs/progress.md, 各README.md |
| 3 | 必要ならスキル化（progress-management） | - |

**完了条件**:
- [x] session-management スキルが更新されている
- [x] 進捗管理の運用が確立されている

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
