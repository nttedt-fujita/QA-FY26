# Session 174: 生データ保存機能 Phase 2（FE）+ NTRIP位置更新修正

**日時**: 2026-03-13

## 概要

Session 173で実装したBE側のスナップショット保存APIに対応するFE側の実装を完了。
また、屋外検査中にNTRIP GGA送信の位置が更新されない問題を修正。

## 成果物

| ファイル | 内容 |
|----------|------|
| [useOutdoorInspection.ts](../../prototype/m1-gnss/frontend/src/hooks/useOutdoorInspection.ts) | スナップショット蓄積・送信機能追加 |
| [outdoor/page.tsx](../../prototype/m1-gnss/frontend/src/app/inspections/outdoor/page.tsx) | addSnapshot()呼び出し追加 |
| [gnss_state_api.rs](../../prototype/m1-gnss/backend/src/web/gnss_state_api.rs) | NAV-PVT取得時にcurrent_position更新 |
| [session-summary.md](../session174/session-summary.md) | セッションサマリー |

## 実装内容

### FE: スナップショット機能

```typescript
// useOutdoorInspection.ts
- snapshots state追加
- addSnapshot(data: GnssStateResponse) 関数追加
- saveResult()でスナップショットも送信
```

### BE: NTRIP位置更新修正

**問題**: `gnss_state_api`（屋外検査で使用）が`current_position`を更新しておらず、NTRIP GGA送信の位置が古いままだった。

**修正**: NAV-PVT取得成功時に`current_position`を更新。

## 次回（Session 175）

1. 動作確認（実機テスト）
   - 屋外検査実行 → スナップショットがDBに保存されるか
   - `GET /api/outdoor-inspection-results/{id}/snapshots` で取得できるか
2. Phase 3: 再生機能（優先度低）

## 参照

- [raw-data-storage-plan.md](../session172/raw-data-storage-plan.md) - 設計計画書
- [session-173.md](session-173.md) - Phase 1（BE）実装
