# Session 153 計画

**目的**: FE側の状態表示改善（BE処理状況の可視化）

**前提**: Session 152で統合APIのタイムアウト問題は解決済み。FE側で処理状況が見えない問題が残っている。

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | リクエスト重複防止（前のリクエスト完了後に次を発行） | hooks/useGnssState.ts |
| 2 | 「取得中」表示の改善 | app/inspections/outdoor/page.tsx |
| 3 | 検査終了時の「終了処理中」表示 | app/inspections/outdoor/page.tsx |

---

## 詳細

### 1. リクエスト重複防止

現状: `setInterval` で1秒ごとにリクエスト発行 → APIが6秒かかるのでリクエストが溜まる

修正: 前のリクエスト完了後に次を発行する方式に変更

### 2. 「取得中」表示

`isLoading` を活用して、データ取得中であることをユーザーに表示

### 3. 検査終了時の表示

検査終了ボタン押下後、進行中のリクエストがあれば「終了処理中...」を表示

---

## 参照

- [Session 152 summary](../session152/session-summary.md)
- [useGnssState.ts](../../prototype/m1-gnss/frontend/src/hooks/useGnssState.ts)
