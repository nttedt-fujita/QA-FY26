# Session 154 計画

**目的**: FE側の状態表示改善

**前提**: Session 152-153でBE側は安定（統合API 100回テスト成功）

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | FE状態表示改善 | prototype/m1-gnss/frontend/src/app/inspections/outdoor/page.tsx |
| 2 | L1 C/N0が0になる原因調査 | docs/missions/m1-sensor-evaluation/gnss/ubx-nav-sig.md, frontend/src/lib/outdoor-inspection-calc.ts |

---

## 詳細

### 1. FE状態表示改善

Session 152で予定されていた作業:
- **リクエスト重複防止** - ボタン連打対策
- **「取得中」「終了処理中」表示** - BE処理中が見えない問題

### 2. L1 C/N0が0になる原因調査

屋外テストで全てのL1 C/N0が0dBHz:
- NAV-SIG仕様書を確認
- FE側の集計ロジック（`outdoor-inspection-calc.ts`）を確認
- L1信号の識別条件を確認

---

## 参照

- [Session 152 summary](../session-history/session-151-160.md)
- [Session 153 summary](../session153/session-summary.md)
