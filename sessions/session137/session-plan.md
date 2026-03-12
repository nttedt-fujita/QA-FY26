# Session 137 計画

**目的**: device_id紐付け実装

**前提**: Session 136でドメインモデル・DB構造を確認済み

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | DBクリーンアップ | - |
| 2 | BE: 屋外検査保存APIに serial_number 追加 | prototype/m1-gnss/backend/src/web/outdoor_inspection_api.rs |
| 3 | BE: serial_number → device_id 解決 | prototype/m1-gnss/backend/src/repository/sqlite.rs |
| 4 | FE: 保存時に serial_number を送信 | prototype/m1-gnss/frontend/src/app/inspections/outdoor/page.tsx |

---

## 詳細

### 1. DBクリーンアップ

```sql
-- 異常データ削除
DELETE FROM indoor_inspections WHERE device_id = 1;
DELETE FROM devices WHERE id = 1;

-- 屋外検査結果も削除（テストデータ）
DELETE FROM outdoor_inspection_results;
```

### 2. BE: 屋外検査保存APIの変更

`SaveOutdoorResultRequest` に `serial_number` を追加：
- `serial_number: Option<String>` を追加
- `serial_number` があれば `device_id` を解決
- 見つからなければエラー（「先に屋内検査を実行してください」）

### 3. FE: 保存時の変更

`connectedDevice.serial_number` を保存APIに渡す。

---

## 残タスク（Session 136時点）

1. **device_id紐付け実装** ← 今回
2. 自動保存に変更
3. u-center照合
4. GGA送信の正式実装（F9PのGGA使用）

---

## 参照

- [Session 136 summary](../session136/session-summary.md)
- [26-outdoor-inspection-domain-model.md](../../docs/missions/m1-sensor-evaluation/gnss/26-outdoor-inspection-domain-model.md)

---

*計画作成: 2026-03-12 Session 136終了時*
