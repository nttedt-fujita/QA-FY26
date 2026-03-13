# Session 173 サマリー

**日時**: 2026-03-13
**目的**: 生データ保存機能 Phase 1（BE）実装

---

## 実施内容

### 1. 設計判断
- オプションB vs C の工数比較を実施
- **オプションC（ハイブリッド）を採用**: KISS原則に基づき、シンプルなスナップショット方式を選択
- 工数差は約2-3セッション分（B: 3-4セッション、C: 1-2セッション）

### 2. BE実装（Phase 1完了）

| ファイル | 変更内容 |
|----------|----------|
| `repository/types.rs` | `OutdoorInspectionSnapshot`構造体追加 |
| `repository/sqlite.rs` | テーブル作成 + CRUD（insert/get） |
| `web/outdoor_inspection_api.rs` | POST拡張 + GET snapshots追加 |

### 3. 新規API

```
POST /api/outdoor-inspection-results
  body: { ..., snapshots: [{ timestamp_ms, data }] }  ← 拡張

GET /api/outdoor-inspection-results/{id}/snapshots   ← 新規
  response: { inspection_id, snapshots: [{ id, timestamp_ms, data }] }
```

---

## 確認した問題

- 既存テストコードで`serialport`クレートのAPI変更によるエラー（48個）
- 今回の変更とは無関係、別タスクとして対処予定
- ライブラリコードは正常にコンパイル

---

## 次セッション（Session 174）でやること

1. **Phase 2: FE実装**
   - useOutdoorInspection: スナップショット蓄積
   - saveResult: スナップショット送信

2. **または: テストコード修正**
   - serialportクレートAPI変更への対応

---

## 参照

- [raw-data-storage-plan.md](../session172/raw-data-storage-plan.md) - 設計計画書
