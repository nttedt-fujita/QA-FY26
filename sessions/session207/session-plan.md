# Session 207 計画

**目的**: 複数台一括検査機能の実装（Phase 1）

**前提**: Session 206で実装計画策定済み

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | BE: BatchInspection型定義 | `session206/multi-device-inspection-plan.md` |
| 2 | BE: batch_inspectionハンドラー実装 | `backend/src/web/inspection_api.rs` |
| 3 | BE: ルート追加 + 動作確認（curl） | - |
| 4 | FE: api.ts に runBatchInspection追加 | `frontend/src/lib/api.ts` |
| 5 | FE: 検査画面の複数デバイス表示 | `frontend/src/app/inspections/indoor/page.tsx` |
| 6 | FE: 一括検査ボタン + 結果表示 | - |
| 7 | FE: 点滅ボタン連携 | `frontend/src/components/DeviceCard.tsx` |
| 8 | 実機テスト（3台） | - |

---

## 実装方針（Session 206で策定済み）

### バックエンド

```
POST /api/inspections/batch
  ├─ multi_manager.get_all_connected_devices() で全デバイス取得
  ├─ パス名でソート（ttyUSB0 < ttyUSB1 < ttyUSB2）
  ├─ 各デバイスに対して順次検査実行
  └─ BatchInspectionResponse を返却
```

### フロントエンド

- 複数デバイス表示（`connectedDevice` → `connectedDevices[]`）
- 「全台検査開始」ボタン
- 結果一覧 + 点滅ボタン

---

## 参照

- [Session 206 summary](../session206/session-summary.md)
- [実装計画](../session206/multi-device-inspection-plan.md)
- [要件定義](../session205/multi-device-inspection-requirements.md)
