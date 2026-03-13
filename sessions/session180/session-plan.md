# Session 180 計画

**目的**: Session 179の動作確認結果に基づく修正（必要な場合）

**前提**: Session 179でレスポンス駆動ポーリング実装完了。ユーザーがテスト中。

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | テスト結果の確認 | session179/session-summary.md |
| 2 | 問題があれば修正 | frontend/src/hooks/useGnssState.ts, frontend/src/app/inspections/outdoor/page.tsx |
| 3 | （余裕があれば）スナップショット可視化 | session172/raw-data-storage-plan.md |

---

## 詳細

### 1. テスト結果の確認

ユーザーのテスト結果を確認:
- レスポンス駆動ポーリングが正常に動作しているか
- 検査終了時に最後のサンプルが取得できているか
- NTRIP-RTCMとのMutex競合が発生していないか

### 2. 問題があれば修正

発生した問題に応じて修正:
- `delayAfterResponseMs`の調整
- completing状態の処理フロー修正

### 3. スナップショット可視化（Phase 3）

DBに保存済みのスナップショットをスカイプロット等で再表示する機能。
Session 172で計画済み、未実装。

---

## 参照

- [session179/session-summary.md](../session179/session-summary.md) - 前セッション
- [session172/raw-data-storage-plan.md](../session172/raw-data-storage-plan.md) - スナップショット計画
