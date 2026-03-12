# Session 130 計画

**目的**: M1-GNSS実装の続き — 屋外検査Phase 3（DB保存）

**前提**: Session 129でドキュメント整理ルール化完了、Phase 1-2（集計・UI）は完了済み

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | DBスキーマ設計の確認 | docs/missions/m1-sensor-evaluation/gnss/27-outdoor-inspection-implementation-plan.md（Phase 3セクション） |
| 2 | 既存リポジトリパターンの確認 | prototype/m1-gnss/backend/src/repository/sqlite.rs, repository/types.rs |
| 3 | 既存APIパターンの確認 | prototype/m1-gnss/backend/src/web/inspection_api.rs |
| 4 | FE型定義の確認 | prototype/m1-gnss/frontend/src/types/outdoor-inspection.ts |
| 5 | マイグレーション＆リポジトリ実装 | - |
| 6 | APIエンドポイント追加 | - |
| 7 | FE保存機能追加 | prototype/m1-gnss/frontend/src/hooks/useOutdoorInspection.ts |

---

## 詳細

### 1. DBスキーマ

27-outdoor-inspection-implementation-plan.md のPhase 3に定義あり:
- `outdoor_inspection_results` テーブル
- device_id, lot_id, 各指標値, 合否判定

### 2. 実装手順

1. `repository/sqlite.rs` にテーブル作成＆CRUD追加
2. `repository/types.rs` に `OutdoorInspectionResult` 構造体追加
3. `web/outdoor_inspection_api.rs` 新規作成
4. FE側に保存呼び出し追加

---

## 参照

- [Session 129 summary](../session129/session-summary.md)
- [27-outdoor-inspection-implementation-plan.md](../../docs/missions/m1-sensor-evaluation/gnss/27-outdoor-inspection-implementation-plan.md)
- [gnss/README.md チェックリスト](../../docs/missions/m1-sensor-evaluation/gnss/README.md)

---

*計画作成: 2026-03-12 Session 129終了時*
*更新: 2026-03-12 読むべきファイルを具体化*
