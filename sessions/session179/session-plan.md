# Session 179 計画

**目的**: FE表示タイミング調整（BE処理完了を待ってから結果表示）

**前提**: Session 178で動作確認済み。POST後に1件分（約5秒）の処理が残る問題あり。

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | 現状の結果表示タイミングを確認 | frontend/src/app/inspections/outdoor/page.tsx |
| 2 | BE処理完了を待つ方式を検討 | - |
| 3 | 実装 | - |
| 4 | （余裕があれば）スナップショット可視化 | session172/raw-data-storage-plan.md |

---

## 詳細

### 1. 問題

現状: FEは30秒タイマー終了後、すぐに結果を表示
問題: BEに1件分（約5秒）のリクエストが残っている場合、その結果がFEに反映されない

### 2. 解決方針（案）

- **案A**: POSTを最後のgnss-state完了後に実行
- **案B**: POST後にBE処理完了を待つ（ポーリング or WebSocket）
- **案C**: 現状維持（5秒程度のずれは許容）

### 3. スナップショット可視化（Phase 3）

DBに保存済みのスナップショットをスカイプロット等で再表示する機能。
Session 172で計画済み、未実装。

---

## 参照

- [session178/session-summary.md](../session178/session-summary.md) - 前セッション
- [session172/raw-data-storage-plan.md](../session172/raw-data-storage-plan.md) - スナップショット計画
