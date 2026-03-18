# AI検査システムとM3連携設計（たたき台）

**作成日**: 2026-03-18（Session 245）
**ステータス**: たたき台（設計判断前）
**目的**: AI検査システムをM3（受入検査DB）とどう連携するかの選択肢整理

---

## 1. 前提

### AI検査の位置づけ

```
サプライヤ → IQC（受入検査） → 製造工程 → ...
              ↑
              AI検査（IQCの一部として実施）
```

- AI検査は**IQC（受入検査）の効率化**
- **ロット単位**で検査結果を記録
- 既存のM3スキーマとの連携が必要

### 現状のM3スキーマ（関連部分）

```
lots（ロット）
├─ lot_id (PK)
├─ part_id (FK)
├─ arrival_date
└─ quantity

inspection_records（検査記録）
├─ record_id (PK)
├─ lot_id (FK)
├─ item_id (FK) ← 検査項目
├─ result（合格/不合格/不問）
├─ defect_qty
└─ note
```

---

## 2. 連携アプローチの選択肢

### 案A: 既存テーブル拡張アプローチ

**概要**: inspection_recordsにAI関連カラムを追加

```sql
ALTER TABLE inspection_records ADD COLUMN
    is_ai_assisted BOOLEAN DEFAULT FALSE,  -- AI検査かどうか
    ai_model_version TEXT,                  -- 使用したAIモデルバージョン
    ai_confidence REAL,                     -- AI判定の信頼度（0.0-1.0）
    ai_raw_result TEXT;                     -- AI出力（JSON）
```

| メリット | デメリット |
|---------|-----------|
| 既存クエリがそのまま使える | AIカラムがNULLの既存データが混在 |
| スキーマ変更が最小限 | AI固有の複雑なデータ構造に対応しづらい |
| 人間検査とAI検査を統一的に扱える | 将来のAI機能拡張で肥大化する可能性 |

**適合**: MVP・プロトタイプ段階

---

### 案B: 新規テーブル追加アプローチ

**概要**: AI検査専用のテーブルを新設、inspection_recordsと1:1で紐づく

```sql
CREATE TABLE ai_inspection_results (
    ai_result_id TEXT PRIMARY KEY,
    record_id TEXT NOT NULL,           -- inspection_recordsへのFK
    model_name TEXT NOT NULL,          -- AIモデル名
    model_version TEXT NOT NULL,       -- バージョン
    confidence REAL NOT NULL,          -- 信頼度
    raw_output TEXT,                   -- JSON形式のAI出力
    inference_time_ms INTEGER,         -- 推論時間
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (record_id) REFERENCES inspection_records(record_id)
);

CREATE TABLE ai_detected_defects (
    detection_id TEXT PRIMARY KEY,
    ai_result_id TEXT NOT NULL,        -- ai_inspection_resultsへのFK
    defect_type TEXT NOT NULL,         -- 検出した不良種別
    bounding_box TEXT,                 -- 検出位置（JSON）
    confidence REAL NOT NULL,          -- 個別不良の信頼度
    image_url TEXT,                    -- 検出画像へのパス
    FOREIGN KEY (ai_result_id) REFERENCES ai_inspection_results(ai_result_id)
);
```

| メリット | デメリット |
|---------|-----------|
| AI固有のデータを正規化できる | テーブル数が増える |
| 複数の不良検出に対応しやすい | JOINが必要になる |
| 既存テーブルを変更しない | クエリが複雑になる |

**適合**: 本格運用・複数AI対応

---

### 案C: 別システム連携アプローチ

**概要**: AI検査システムは独立、M3にはサマリのみ記録

```
AI検査システム（独立DB）
├─ 画像データ
├─ 推論結果
├─ モデル管理
└─ 学習データ

    ↓ API連携

M3（受入検査DB）
├─ inspection_records
│   └─ ai_summary_url: AI結果へのリンク
└─ ai_inspection_summary（ビュー or 軽量テーブル）
```

| メリット | デメリット |
|---------|-----------|
| 責務が明確に分離 | システム間連携の複雑さ |
| AI側の自由度が高い | 2つのシステムを運用 |
| 将来的なスケールに対応 | データ整合性の管理が必要 |

**適合**: 大規模・長期運用

---

## 3. 推奨（プロトタイプ段階）

**案A（既存テーブル拡張）を推奨**

### 理由

1. **MVPとして十分**: まずAI検査が「動く」ことを証明
2. **移行コストが低い**: 既存のダッシュボード・クエリが使える
3. **後から拡張可能**: 本格運用時に案Bに移行できる

### 最小限の変更案

```sql
-- inspection_recordsに2カラム追加
ALTER TABLE inspection_records ADD COLUMN
    is_ai_assisted BOOLEAN DEFAULT FALSE,
    ai_confidence REAL;

-- inspection_itemsに「AI外観検査」を追加
INSERT INTO inspection_items (item_id, name, type)
VALUES ('ITEM-AI-001', 'AI外観検査', '全数');
```

### データの流れ

```
1. ロット登録（入荷時）
   └─ lots.lot_id = "LOT-2026-0318-001"

2. AI検査実施
   └─ inspection_records
       ├─ lot_id = "LOT-2026-0318-001"
       ├─ item_id = "ITEM-AI-001"
       ├─ result = "合格" or "不合格"
       ├─ is_ai_assisted = TRUE
       └─ ai_confidence = 0.95

3. 人間確認（不合格時）
   └─ defect_reports
       ├─ record_id = (上記のrecord_id)
       └─ detail = "シルク印刷カケ検出"
```

---

## 4. 検討事項（次セッション以降）

### 未決定事項

| 項目 | 選択肢 | 判断基準 |
|------|--------|---------|
| AI出力の保存形式 | JSON / 別テーブル | データ量・検索要件 |
| 画像データの保存先 | DB内 / ファイルシステム | サイズ・パフォーマンス |
| 信頼度しきい値 | 固定 / 部品ごと | 不良種別の特性 |
| 人間レビュー要否 | 必須 / 高信頼度時スキップ | 品質方針 |

### ヒアリングで確認すること

1. AI検査結果を後から参照したい粒度（ロット単位？個体単位？）
2. 画像データの保持期間
3. 既存の品質管理システムとの連携有無

---

## 5. 参照

- [Session 243: AI検査要件整理](ai-inspection-requirements-draft.md)
- [M3 schema.sql](../../prototype/m3/db/schema.sql)
- [to-be-model.drawio](../../docs/missions/m3-incoming-inspection-db/to-be/to-be-model.drawio)

---

*作成: Session 245 (2026-03-18)*
