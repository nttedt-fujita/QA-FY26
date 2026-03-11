# 統合DB設計

**作成日**: 2026-03-11 Session 86
**ステータス**: ドラフト

---

## 1. 既存設計との差分

| テーブル | Session 63 | Session 86 |
|----------|-----------|------------|
| **lots** | - | ✅ 新規追加 |
| **devices** | ✓ | 拡張（lot_id追加） |
| **indoor_inspections** | - | ✅ 新規追加 |
| **inspection_item_results** | - | ✅ 新規追加 |
| **outdoor_measurements** | measurement_sessions | 名前変更 |
| 計測データテーブル | ✓ | 維持 |

---

## 2. 統合スキーマ

```sql
-- ============================================================
-- ロット（入荷単位）
-- ============================================================
CREATE TABLE IF NOT EXISTS lots (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    lot_number TEXT NOT NULL UNIQUE,       -- ロット番号

    -- 期待値（屋内検査用）
    expected_rate_ms INTEGER,              -- 出力レート期待値 (ms)
    expected_port_in_proto TEXT,           -- ポート入力プロトコル期待値
    expected_port_out_proto TEXT,          -- ポート出力プロトコル期待値

    memo TEXT,                             -- メモ
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- ============================================================
-- 装置（個別のGNSSモジュール）
-- ============================================================
CREATE TABLE IF NOT EXISTS devices (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    lot_id INTEGER,                        -- 所属ロット（FK）← 追加
    serial_number TEXT NOT NULL UNIQUE,    -- シリアル番号（SEC-UNIQID）
    model_number TEXT,                     -- 型番
    memo TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),

    FOREIGN KEY (lot_id) REFERENCES lots(id)
);

-- ============================================================
-- 屋内検査（1回の検査作業）
-- ============================================================
CREATE TABLE IF NOT EXISTS indoor_inspections (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    device_id INTEGER NOT NULL,            -- 対象装置（FK）
    inspected_at TEXT NOT NULL,            -- 検査日時

    -- 総合判定
    overall_result TEXT,                   -- Pass/Fail/Partial

    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),

    FOREIGN KEY (device_id) REFERENCES devices(id)
);

-- ============================================================
-- 検査項目結果（各項目の結果）
-- ============================================================
CREATE TABLE IF NOT EXISTS inspection_item_results (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    inspection_id INTEGER NOT NULL,        -- 屋内検査（FK）

    item_name TEXT NOT NULL,               -- 項目名（communication/serial/fw/rate/port）
    verdict TEXT NOT NULL,                 -- 判定（Pass/Fail/Error/Recorded）
    actual_value TEXT,                     -- 実測値
    expected_value TEXT,                   -- 期待値

    created_at TEXT NOT NULL DEFAULT (datetime('now')),

    FOREIGN KEY (inspection_id) REFERENCES indoor_inspections(id)
);

CREATE INDEX IF NOT EXISTS idx_inspection_results_inspection
    ON inspection_item_results(inspection_id);

-- ============================================================
-- 屋外計測（1回の計測作業）← 旧 measurement_sessions
-- ============================================================
CREATE TABLE IF NOT EXISTS outdoor_measurements (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    device_id INTEGER NOT NULL,            -- 対象装置（FK）

    -- 基本情報
    started_at TEXT NOT NULL,
    ended_at TEXT,
    evaluator TEXT,

    -- 計測条件
    location_lat REAL,
    location_lon REAL,
    location_description TEXT,
    environment TEXT,                      -- 屋外/屋内
    weather TEXT,
    antenna_config TEXT,
    base_station_info TEXT,

    -- 評価結果
    judgment TEXT,                         -- 合格/不合格/保留
    comment TEXT,

    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),

    FOREIGN KEY (device_id) REFERENCES devices(id)
);

-- ============================================================
-- 計測データテーブル（既存を維持）
-- ============================================================
-- nav_pvt_records
-- nav_status_records
-- nav_dop_records
-- satellites
-- signals
-- mon_span_records
-- mon_rf_records
--
-- ※ session_id → measurement_id に変更（FKはoutdoor_measurementsを参照）
```

---

## 3. ER図

```
lots（ロット）
  │
  └── devices（装置） 1:N
        │
        ├── indoor_inspections（屋内検査） 1:N
        │     │
        │     └── inspection_item_results（検査項目結果） 1:N
        │
        └── outdoor_measurements（屋外計測） 1:N
              │
              ├── nav_pvt_records 1:N
              ├── nav_status_records 1:N
              ├── nav_dop_records 1:N
              ├── satellites 1:N
              ├── signals 1:N
              ├── mon_span_records 1:N
              └── mon_rf_records 1:N
```

---

## 4. 移行作業

### 変更が必要なファイル

| ファイル | 変更内容 |
|----------|---------|
| `db/schema.sql` | 新スキーマに置き換え |
| `src/repository/types.rs` | 新しいエンティティ追加 |
| `src/repository/sqlite.rs` | 新しいテーブル対応 |
| `src/inspection/engine.rs` | 結果保存先の変更 |

### 既存コードへの影響

| コンポーネント | 影響 |
|---------------|------|
| UBXパーサー | **影響なし** |
| DeviceManager | **影響なし** |
| InspectionEngine | **要修正**（結果保存先） |
| Repository | **要修正**（新テーブル追加） |

---

## 5. 未反映の議論内容（Session 87で要検討）

**IMPORTANT**: 以下の議論内容がDB設計に十分反映されていない。

| 議論内容 | 現状 | 要検討 |
|----------|------|--------|
| FWバージョンの多数派・はずれ値比較 | inspection_item_resultsに保存 | ロット単位で集計する設計が不明確 |
| FW期待値はロットにない（意図的） | lotsにexpected_fw_versionなし | コメントで意図を説明すべき |
| 装置のFWバージョンを直接見れる | devicesにfw_versionカラムなし | 追加するか、クエリで対応するか |
| 検査項目5つ | item_nameに入る | 具体的なitem_name値の定義が必要 |

### 検討オプション

**FWバージョンの扱い**:
1. **devicesにfw_versionカラム追加** — 最新FWを直接参照できる
2. **クエリで対応** — inspection_item_resultsからitem_name='fw'を集計

**item_nameの定義**:
```
communication — 通信疎通
serial — シリアル番号
fw — FWバージョン
rate — 出力レート
port — ポート設定
```

---

## 6. 質問

1. この設計で進めてよいか？
2. Session 63のスキーマは破棄して新しいものに置き換える？
3. 既存の計測データテーブル（nav_pvt等）の`session_id`を`measurement_id`にリネームする？
4. **FWバージョンはdevicesに持たせるか、クエリで対応するか？**

---

*作成: 2026-03-11 Session 86*
*更新: 2026-03-11 Session 86終了時 — 未反映の議論内容を追記*
