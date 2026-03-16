# Session 206 サマリー

日時: 2026-03-16

---

## 実施内容

1. **現在の屋内検査実装を確認**
   - バックエンド: `get_first_device_manager()` で1台目のみ検査
   - フロントエンド: 1台表示、一括/個別検査UIなし

2. **複数台対応の実装計画策定**
   - [multi-device-inspection-plan.md](./multi-device-inspection-plan.md) 作成

---

## 策定した計画

### Phase 1（次セッション〜）: 一括検査対応

| # | 作業 |
|---|------|
| 1 | BE: BatchInspection型定義 |
| 2 | BE: batch_inspectionハンドラー実装 |
| 3 | BE: ルート追加 + 動作確認（curl） |
| 4 | FE: api.ts に runBatchInspection追加 |
| 5 | FE: 検査画面の複数デバイス表示 |
| 6 | FE: 一括検査ボタン + 結果表示 |
| 7 | FE: 点滅ボタン連携 |
| 8 | 実機テスト（3台） |

### Phase 2（将来）: 個別検査対応

- デバイスカードに「この1台を検査」ボタン追加

---

## スコープ外メモ

### 同一シリアルの重複レコード問題

**課題**: 同じシリアル番号の装置を検査すると、毎回新しい`indoor_inspections`レコードが作成される

**検討事項**:
- 最新の検査結果のみ保持 vs 履歴として全検査を残す
- 「最新」フラグを持たせるか

→ 別セッションで検討

---

## 次セッション向け

1. [multi-device-inspection-plan.md](./multi-device-inspection-plan.md) を読む
2. Phase 1の実装を開始（BE → FE の順）

---

## 参照

- [Session 205: 要件定義](../session205/multi-device-inspection-requirements.md)
- [ADR-014: 複数装置同時対応](../../docs/adr/m1/ADR-014-multi-device-manager.md)
