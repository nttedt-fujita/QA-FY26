# Session 116 計画

**目的**: Phase 1.5 残作業の実装

---

## 背景

Session 115でRTK関連ドキュメントを正式配置。
RTK実装は優先度が低いため、他のタスクを先に進める。

---

## やること候補

### 案A: MON-SPANパーサー実装

- ADR-008で「高」優先度
- 単体で実装可能（NTRIP不要）
- L2帯スペクトラム表示

### 案B: 屋内/屋外検査ページ分離

- Session 113で出たユーザー要望
- 現在の`/inspections`を分割
- ナビゲーション改善

### 案C: NAV-SIGを検査ロジックに組み込む

- Session 113の確認事項
- 現在はリアルタイムモニタリング専用
- 合否判定と連携

---

## 参照資料

- [outdoor-inspection-priority.md](../session111/outdoor-inspection-priority.md) — Phase 1.5優先度
- [m1-gnss-all-tasks.md](../session111/m1-gnss-all-tasks.md) — 全体タスク
- [ADR-008](../../docs/adr/m1/ADR-008-outdoor-inspection-criteria.md) — 屋外検査合格基準

---

*計画作成: 2026-03-11 Session 115終了時*
