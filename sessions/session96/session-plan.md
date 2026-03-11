# Session 96 計画

**目的**: 検査API実装

---

## やること

### 1. 検査API実装

| エンドポイント | 機能 |
|--------------|------|
| `POST /api/inspections` | 検査実行（ロットID + 装置パス指定） |
| `GET /api/inspections` | 検査履歴取得 |

### 2. InspectionServiceとの統合

- APIハンドラーからInspectionServiceを呼び出す
- converter.rsを使用して型変換

---

## 完了条件

- [ ] `POST /api/inspections` で検査が実行される
- [ ] 結果がDBに保存される
- [ ] `GET /api/inspections` で履歴が取得できる
- [ ] curl手動テストで動作確認

---

## 参照資料

- [UI設計書](../session92/ui-design-phase1.md) - API設計
- [InspectionService](../../prototype/m1-gnss/backend/src/service/inspection_service.rs)
- [実装ロードマップ](../session95/implementation-roadmap.md)

---

*計画作成: 2026-03-11 Session 95終了時*
