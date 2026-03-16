# 複数台検査機能 実装計画

作成: Session 206 (2026-03-16)

---

## 1. 現状と目標

### 現状

| 項目 | 状態 |
|------|------|
| MultiDeviceManager | ✅ 複数台管理可能 |
| LED点滅API | ✅ デバイスパス指定で動作 |
| 屋内検査API | ⚠️ `get_first_device_manager()` で1台目のみ |
| 屋内検査画面 | ⚠️ 1台表示、一括/個別検査UIなし |

### 目標（Session 205要件）

| 項目 | 要件 |
|------|------|
| 検査方式 | 一括・個別の両方対応 |
| DB登録タイミング | 検査完了時に自動登録 |
| DB登録順 | USBポート順（ttyUSB0 → 1 → 2） |
| 検査後フロー | 結果表示 → 点滅で識別 → 取り外し |

---

## 2. 実装範囲

### Phase 1: 一括検査対応（今回実装）

1. **バックエンド**: 複数デバイス一括検査API
2. **フロントエンド**: 検査画面の複数台対応

### Phase 2: 個別検査対応（次回以降）

1. デバイスカードに「この1台を検査」ボタン追加
2. 個別検査API（デバイスパス指定）

---

## 3. バックエンド変更

### 3.1 新規API: POST /api/inspections/batch

```rust
/// 一括検査リクエスト
#[derive(Debug, Deserialize)]
pub struct BatchInspectionRequest {
    pub lot_id: Option<i64>,
}

/// 一括検査レスポンス
#[derive(Debug, Serialize)]
pub struct BatchInspectionResponse {
    pub results: Vec<DeviceInspectionResult>,
    pub summary: BatchSummary,
}

#[derive(Debug, Serialize)]
pub struct DeviceInspectionResult {
    pub path: String,               // ttyUSB0, ttyUSB1, ...
    pub serial_number: String,
    pub overall_result: String,     // Pass / Fail / Error
    pub inspection_id: i64,
    pub items: Vec<ItemResultResponse>,
}

#[derive(Debug, Serialize)]
pub struct BatchSummary {
    pub total: usize,
    pub passed: usize,
    pub failed: usize,
}
```

### 3.2 実装方針

```
POST /api/inspections/batch
  ├─ multi_manager.get_all_connected_devices() で全デバイス取得
  ├─ パス名でソート（ttyUSB0 < ttyUSB1 < ttyUSB2）
  ├─ 各デバイスに対して順次検査実行
  │   ├─ InspectionService.run_and_save()
  │   └─ 結果を収集
  ├─ 集計（合格/不合格数）
  └─ BatchInspectionResponse を返却
```

### 3.3 変更ファイル

| ファイル | 変更内容 |
|----------|----------|
| `backend/src/web/inspection_api.rs` | BatchInspection型追加、batch_inspectionハンドラー追加 |
| `backend/src/web/mod.rs` | ルート追加 |
| `backend/src/device/multi_manager.rs` | get_all_connected_paths() 追加（必要なら） |

---

## 4. フロントエンド変更

### 4.1 検査画面の変更

現在の `inspections/indoor/page.tsx` を改修:

1. **複数デバイス表示**: `connectedDevice` → `connectedDevices[]`
2. **一括検査ボタン**: 「全台検査開始」
3. **結果表示**: 複数デバイスの結果を一覧表示
4. **点滅連携**: 結果カードに点滅ボタン

### 4.2 画面レイアウト（要件より）

```
┌─────────────────────────────────────────┐
│ 検査結果 (3台中 2台合格)                 │
├─────────────────────────────────────────┤
│ ✅ ttyUSB0 / 9543F2097F : 合格           │
│    [💡点滅]                              │
├─────────────────────────────────────────┤
│ ❌ ttyUSB1 / A5400AEB1F : NG             │
│    理由: L2受信率不足                    │
│    [💡点滅]                              │
├─────────────────────────────────────────┤
│ ✅ ttyUSB2 / A44052ED9D : 合格           │
│    [💡点滅]                              │
└─────────────────────────────────────────┘
```

### 4.3 変更ファイル

| ファイル | 変更内容 |
|----------|----------|
| `frontend/src/lib/api.ts` | runBatchInspection() 追加 |
| `frontend/src/app/inspections/indoor/page.tsx` | 複数デバイス対応 |
| `frontend/src/components/BatchInspectionResult.tsx` | 新規：一括結果表示 |

---

## 5. 実装順序

| # | 作業 | 見積もり |
|---|------|----------|
| 1 | BE: BatchInspection型定義 | 小 |
| 2 | BE: batch_inspectionハンドラー実装 | 中 |
| 3 | BE: ルート追加 + 動作確認（curl） | 小 |
| 4 | FE: api.ts に runBatchInspection追加 | 小 |
| 5 | FE: 検査画面の複数デバイス表示 | 中 |
| 6 | FE: 一括検査ボタン + 結果表示 | 中 |
| 7 | FE: 点滅ボタン連携 | 小 |
| 8 | 実機テスト（3台） | - |

---

## 6. スコープ外（将来課題）

### 6.1 同一シリアルの重複レコード問題

**課題**: 同じシリアル番号の装置を検査すると、毎回新しいレコードが作成される

**現状**: `devices`テーブルにシリアル番号で検索し、なければINSERT、あればUPDATE
（ただし`indoor_inspections`は毎回新規INSERT）

**検討事項**:
- 最新の検査結果のみ保持するか
- 履歴として全検査を残すか
- 「最新」フラグを持たせるか

→ 別セッションで検討

### 6.2 個別検査対応

Phase 2として次回以降に実装

---

## 7. 参照

- [Session 205: 要件定義](../session205/multi-device-inspection-requirements.md)
- [ADR-014: 複数装置同時対応](../../docs/adr/m1/ADR-014-multi-device-manager.md)
