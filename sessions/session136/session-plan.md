# Session 136 計画

**目的**: 残タスク消化（device_id紐付け または 屋外RTK確認）

**前提**: Session 135でGGA送信機能追加、NTRIP接続維持を確認

---

## やること（候補）

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | device_id紐付け実装 | prototype/m1-gnss/backend/src/web/outdoor_api.rs |
| 2 | 屋外でのRTK動作確認 | - |
| 3 | 自動保存に変更 | prototype/m1-gnss/frontend/src/app/outdoor-inspection/page.tsx |

---

## 残タスク一覧

1. device_id紐付け実装
2. 自動保存に変更（手動保存から）
3. u-center照合
4. GGA送信の正式実装（F9PのGGAを使用）

---

## 参照

- [Session 135 summary](../session135/session-summary.md)
- [Session 131 summary](../session131/session-summary.md)（残タスク整理）

---

*計画作成: 2026-03-12 Session 135終了時*
