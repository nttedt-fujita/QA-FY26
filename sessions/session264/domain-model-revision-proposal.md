# M3 ドメインモデル修正提案

**作成日**: 2026-03-19 (Session 264)
**目的**: 優先度Aの必須修正（サプライヤーロット番号の追加）の実施計画
**背景**: [domain-model-review.md](domain-model-review.md)（ドメインモデルと課題の対応レビュー）

---

## 修正内容（優先度A: 必須）

### 1. サプライヤーロット番号フィールドの追加

**問題点**:
- 現在の `lots.lot_id` はシステム内部のID（例: `LOT-12345678`）
- **サプライヤーが発行したロット番号**（トレーサビリティに必要）を記録するフィールドがない
- Excel課題「ロット番号の欠落」に対応するため

**修正内容**:
```sql
ALTER TABLE lots ADD COLUMN supplier_lot_number TEXT;
```

**フィールド仕様**:
| フィールド名 | 型 | NULL許容 | デフォルト | 説明 |
|------------|-----|---------|----------|------|
| `supplier_lot_number` | TEXT | ✅ Yes | NULL | サプライヤーが発行したロット番号 |

**NULL許容の理由**:
- サプライヤーがロット番号を発行していない場合がある
- 既存データのマイグレーションでNULLを許容する必要がある

---

## 実装計画

### Phase 1: スキーマ修正

**ファイル**: `prototype/m3/db/schema.sql`

**修正箇所**（行57-68）:
```sql
-- ロット（入荷単位）
CREATE TABLE lots (
    lot_id TEXT PRIMARY KEY,
    part_id TEXT NOT NULL,             -- 部品（FK）
    po_number TEXT,                    -- 発注番号（トレーサビリティ用）
    supplier_lot_number TEXT,          -- サプライヤーロット番号（追加）
    arrival_date DATE,                 -- 入荷日
    quantity INTEGER NOT NULL,         -- 入荷数量
    fw_version TEXT,                   -- FWバージョン
    hw_version TEXT,                   -- HWバージョン
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (part_id) REFERENCES parts(part_id)
);
```

**マイグレーションスクリプト**:

新規ファイル: `prototype/m3/db/migrations/001_add_supplier_lot_number.sql`

```sql
-- Migration: サプライヤーロット番号の追加
-- Date: 2026-03-19
-- Session: 264

ALTER TABLE lots ADD COLUMN supplier_lot_number TEXT;

-- コメント: 既存データはNULLのまま（後で手動入力またはインポート時に設定）
```

---

### Phase 2: Goコードの修正

#### 2.1 Lot構造体の修正

**ファイル**: `prototype/m3/backend/internal/repository/lot.go`

**修正箇所**（行13-24）:
```go
// Lot はロット（入荷単位）を表す
type Lot struct {
	LotID             string     `json:"lot_id"`
	PartID            string     `json:"part_id"`
	PONumber          *string    `json:"po_number,omitempty"`
	SupplierLotNumber *string    `json:"supplier_lot_number,omitempty"` // 追加
	ArrivalDate       *time.Time `json:"arrival_date,omitempty"`
	Quantity          int        `json:"quantity"`
	FWVersion         *string    `json:"fw_version,omitempty"`
	HWVersion         *string    `json:"hw_version,omitempty"`
	CreatedAt         time.Time  `json:"created_at"`
	UpdatedAt         time.Time  `json:"updated_at"`
}
```

#### 2.2 Create関数の修正

**ファイル**: `prototype/m3/backend/internal/repository/lot.go`

**修正箇所**（行40-65）:
```go
// Create は新しいロットを作成する
func (r *LotRepository) Create(ctx context.Context, lot *Lot) error {
	// lot_idを生成（UUIDベース）
	lot.LotID = "LOT-" + uuid.New().String()[:8]

	query := `
		INSERT INTO lots (lot_id, part_id, po_number, supplier_lot_number, arrival_date, quantity, fw_version, hw_version)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
		RETURNING created_at, updated_at
	`

	err := r.db.Pool().QueryRow(ctx, query,
		lot.LotID,
		lot.PartID,
		lot.PONumber,
		lot.SupplierLotNumber, // 追加
		lot.ArrivalDate,
		lot.Quantity,
		lot.FWVersion,
		lot.HWVersion,
	).Scan(&lot.CreatedAt, &lot.UpdatedAt)

	if err != nil {
		return fmt.Errorf("ロット作成に失敗: %w", err)
	}

	return nil
}
```

#### 2.3 FindAll関数の修正

**修正箇所**（行68-106）:
```go
// FindAll は全ロットを取得する
func (r *LotRepository) FindAll(ctx context.Context) ([]*Lot, error) {
	query := `
		SELECT lot_id, part_id, po_number, supplier_lot_number, arrival_date, quantity, fw_version, hw_version, created_at, updated_at
		FROM lots
		ORDER BY created_at DESC
	`

	rows, err := r.db.Pool().Query(ctx, query)
	if err != nil {
		return nil, fmt.Errorf("ロット一覧取得に失敗: %w", err)
	}
	defer rows.Close()

	var lots []*Lot
	for rows.Next() {
		lot := &Lot{}
		err := rows.Scan(
			&lot.LotID,
			&lot.PartID,
			&lot.PONumber,
			&lot.SupplierLotNumber, // 追加
			&lot.ArrivalDate,
			&lot.Quantity,
			&lot.FWVersion,
			&lot.HWVersion,
			&lot.CreatedAt,
			&lot.UpdatedAt,
		)
		if err != nil {
			return nil, fmt.Errorf("ロット行のスキャンに失敗: %w", err)
		}
		lots = append(lots, lot)
	}

	if err := rows.Err(); err != nil {
		return nil, fmt.Errorf("ロット一覧の読み取りに失敗: %w", err)
	}

	return lots, nil
}
```

#### 2.4 FindByID関数の修正

**修正箇所**（行108-138）:
```go
// FindByID はIDでロットを取得する
func (r *LotRepository) FindByID(ctx context.Context, lotID string) (*Lot, error) {
	query := `
		SELECT lot_id, part_id, po_number, supplier_lot_number, arrival_date, quantity, fw_version, hw_version, created_at, updated_at
		FROM lots
		WHERE lot_id = $1
	`

	lot := &Lot{}
	err := r.db.Pool().QueryRow(ctx, query, lotID).Scan(
		&lot.LotID,
		&lot.PartID,
		&lot.PONumber,
		&lot.SupplierLotNumber, // 追加
		&lot.ArrivalDate,
		&lot.Quantity,
		&lot.FWVersion,
		&lot.HWVersion,
		&lot.CreatedAt,
		&lot.UpdatedAt,
	)

	if err != nil {
		if errors.Is(err, pgx.ErrNoRows) {
			return nil, ErrNotFound
		}
		return nil, fmt.Errorf("ロット取得に失敗: %w", err)
	}

	return lot, nil
}
```

---

### Phase 3: テストの修正

**ファイル**: `prototype/m3/backend/internal/handler/lot_handler_test.go`

**修正が必要なテストケース**:
- ロット作成のテストで `supplier_lot_number` を含めたテストケースを追加
- JSON レスポンスに `supplier_lot_number` が含まれることを確認

**例**:
```go
func TestCreateLot_WithSupplierLotNumber(t *testing.T) {
	// 省略（テスト実装は別途）
}
```

---

### Phase 4: フロントエンドの修正

**影響範囲**:
- ロット作成フォームに「サプライヤーロット番号」入力欄を追加
- ロット一覧・詳細画面に「サプライヤーロット番号」を表示

**ファイル**（推測）:
- `prototype/m3/frontend/src/app/lots/new/page.tsx`（作成フォーム）
- `prototype/m3/frontend/src/app/lots/page.tsx`（一覧）
- `prototype/m3/frontend/src/types/lot.ts`（型定義）

**型定義の修正例**:
```typescript
export type Lot = {
  lot_id: string;
  part_id: string;
  po_number?: string;
  supplier_lot_number?: string; // 追加
  arrival_date?: string;
  quantity: number;
  fw_version?: string;
  hw_version?: string;
  created_at: string;
  updated_at: string;
};
```

---

## 実施順序

1. **スキーマ修正** → マイグレーション実行
2. **Goコード修正** → バックエンドビルド確認
3. **テスト修正** → テスト実行・合格確認
4. **フロントエンド修正** → 動作確認

---

## テスト計画

### 単体テスト

- [ ] Lot構造体のJSON marshaling/unmarshaling
- [ ] LotRepository.Create でsupplier_lot_numberが正しく保存される
- [ ] LotRepository.FindAll でsupplier_lot_numberが正しく取得される
- [ ] LotRepository.FindByID でsupplier_lot_numberが正しく取得される

### 統合テスト

- [ ] POST /api/lots で supplier_lot_number を含むリクエストが成功する
- [ ] GET /api/lots で supplier_lot_number が返される
- [ ] GET /api/lots/:id で supplier_lot_number が返される

### E2Eテスト（手動）

- [ ] ロット作成フォームで「サプライヤーロット番号」を入力できる
- [ ] ロット一覧で「サプライヤーロット番号」が表示される
- [ ] NULLの場合、空欄またはハイフンで表示される

---

## ADR作成の要否

**判断**: ADR作成は**不要**

**理由**:
- この修正は「欠落フィールドの追加」であり、設計判断ではない
- Excel課題への対応として明らか
- トレーサビリティの基本要件

**代替**: この修正提案ドキュメントを記録として残す

---

## 残課題（優先度B: ワークショップ後に判断）

以下はSIPOCワークショップで確認後に判断:

| 課題 | 確認内容 |
|------|---------|
| 複数人作業の記録 | 「西村・西浦」のペアは何を意味するか？複数人記録が必要か？ |
| 検査方法詳細の記録 | 検査記録の粒度は？（1ロット1検査項目1レコード? or まとめて1レコード?） |
| defect_qty のDEFAULT値 | 0 と NULL の運用ルールは？ |

---

## 参照

- [domain-model-review.md](domain-model-review.md) — ドメインモデルと課題の対応レビュー
- [sessions/session6/excel-review.md](../session6/excel-review.md) — Excel課題の詳細
- [prototype/m3/db/schema.sql](../../prototype/m3/db/schema.sql) — 現在のスキーマ

---

*作成: Session 264 (2026-03-19)*
