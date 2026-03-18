# AI連携要件ギャップ分析

**作成日**: 2026-03-18（Session 240）
**目的**: 現プロトタイプとAI連携要件（07_ai_integration_and_cost_analysis.md）の照合

---

## 要件チェック結果

### Must（必須）

| # | 要件 | プロトタイプ状態 | ギャップ |
|---|------|-----------------|---------|
| 1 | 検査画像の保存機能 | ❌ 未実装 | **画像保存機能なし** — スキーマに画像関連フィールドなし |
| 2 | 画像と検査メタデータの紐付け | ❌ 未実装 | 検査IDをキーとした画像紐付け未設計 |
| 3 | 不良モードの統一分類コード | ⚠️ 部分的 | `defect_reports.defect_type` はTEXT型（自由入力）— 統一コード体系なし |
| 4 | 良品/不良品のラベル付け | ✅ 対応済み | `inspection_records.result` で合格/不合格/不問を記録 |
| 5 | データエクスポート機能 | ❌ 未実装 | CSV/JSONエクスポートAPIなし |

### Should（強く推奨）

| # | 要件 | プロトタイプ状態 | ギャップ |
|---|------|-----------------|---------|
| 1 | 撮像条件の標準化ガイドライン | ❌ 未策定 | 運用レベルの話（コード外） |
| 2 | S3互換ストレージの採用 | ❌ 未設計 | 画像保存自体が未実装 |
| 3 | AI判定結果の記録フィールド | ❌ 未実装 | スキーマに該当フィールドなし |
| 4 | フィードバックUIの設計 | ❌ 未実装 | UI設計段階で考慮されていない |

### Could（あれば望ましい）

| # | 要件 | プロトタイプ状態 |
|---|------|-----------------|
| 1 | AI推論API呼び出し機能 | ❌ 未実装（将来実装） |
| 2 | 検査結果ダッシュボード | ✅ 対応済み | `/api/v1/dashboard/*` 実装済み |
| 3 | サプライヤースコアカード自動生成 | ⚠️ 部分的 | サプライヤー別統計はあるがスコアカード形式ではない |

---

## ギャップサマリー

### 主要な未対応項目（Must）

1. **画像保存機能** — AI連携の基盤として最重要
2. **不良モード統一コード** — 分類体系の設計が必要
3. **エクスポート機能** — 学習データ抽出に必須

### 設計時の考慮不足

プロトタイプ設計時（Session 33-47）はAI連携を主眼としておらず、「検査記録の電子化」に焦点を当てていた。07_ai_integration_and_cost_analysis.md は Session 185（2026-03-07）で作成されたが、既にプロトタイプ完成後だった。

---

## 対応方針案（M3再開時）

### Phase 3で追加すべき設計

#### 1. 画像保存（Must #1, #2）

```sql
-- inspection_images テーブル追加
CREATE TABLE inspection_images (
    image_id TEXT PRIMARY KEY,
    record_id TEXT NOT NULL,           -- 検査記録（FK）
    storage_key TEXT NOT NULL,         -- S3キー or ファイルパス
    file_name TEXT,                    -- 元ファイル名
    content_type TEXT,                 -- MIME type
    file_size INTEGER,                 -- バイト数
    capture_device TEXT,               -- 撮影デバイス
    capture_conditions JSONB,          -- 撮影条件（照明、角度等）
    label TEXT,                        -- 良品/不良品/未ラベル
    ai_prediction TEXT,                -- AI判定結果（将来用）
    ai_confidence REAL,                -- AI確信度（将来用）
    human_feedback TEXT,               -- 人間のフィードバック（将来用）
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (record_id) REFERENCES inspection_records(record_id)
);
```

#### 2. 不良モード統一コード（Must #3）

```sql
-- defect_modes マスタテーブル追加
CREATE TABLE defect_modes (
    mode_id TEXT PRIMARY KEY,          -- コード（例: DIM-001, APP-001）
    category TEXT NOT NULL,            -- カテゴリ（寸法/外観/機能/材料）
    name TEXT NOT NULL,                -- 名称
    description TEXT,                  -- 説明
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- defect_reports.defect_type を FK に変更
-- ALTER TABLE defect_reports ADD COLUMN mode_id TEXT REFERENCES defect_modes(mode_id);
```

#### 3. エクスポートAPI（Must #5）

```
GET /api/v1/export/inspection-records?format=csv&from=2026-01-01&to=2026-03-31
GET /api/v1/export/images?manifest=sagemaker  -- SageMaker manifest形式
```

---

## 結論

| 項目 | 状態 |
|------|------|
| Must 5項目 | **1/5 対応済み**（良品/不良品ラベル付けのみ） |
| Should 4項目 | **0/4 対応済み** |
| Could 3項目 | **1.5/3 対応済み**（ダッシュボードは実装済み） |

**判断**: M3再開時は、上記ギャップを埋める設計変更が必要。ただし、現在M3は⏸️ストップ中（M4優先、Session 52決定）のため、即時対応は不要。

---

## 参照

- [07_ai_integration_and_cost_analysis.md](07_ai_integration_and_cost_analysis.md)
- [prototype/m3/docs/ARCHITECTURE.md](../../../../prototype/m3/docs/ARCHITECTURE.md)
- [prototype/m3/db/schema.sql](../../../../prototype/m3/db/schema.sql)
