package repository

import (
	"context"
	"fmt"
	"time"

	"github.com/google/uuid"
)

// InspectionRecord は検査記録を表す
type InspectionRecord struct {
	RecordID       string     `json:"record_id"`
	LotID          string     `json:"lot_id"`
	ItemID         string     `json:"item_id"`
	WorkerID       *string    `json:"worker_id,omitempty"`
	InspectionDate time.Time  `json:"inspection_date"`
	SampleQty      *int       `json:"sample_qty,omitempty"`
	Result         string     `json:"result"`
	DefectQty      int        `json:"defect_qty"`
	WorkTimeMin    *float64   `json:"work_time_min,omitempty"`
	Note           *string    `json:"note,omitempty"`
	CreatedAt      time.Time  `json:"created_at"`
	UpdatedAt      time.Time  `json:"updated_at"`
}

// InspectionRecordRepository は検査記録のCRUD操作を提供する
type InspectionRecordRepository struct {
	db *DB
}

// NewInspectionRecordRepository は新しいInspectionRecordRepositoryを作成する
func NewInspectionRecordRepository(db *DB) *InspectionRecordRepository {
	return &InspectionRecordRepository{db: db}
}

// Create は新しい検査記録を作成する
func (r *InspectionRecordRepository) Create(ctx context.Context, record *InspectionRecord) error {
	// record_idを生成
	record.RecordID = "REC-" + uuid.New().String()[:8]

	query := `
		INSERT INTO inspection_records
			(record_id, lot_id, item_id, worker_id, inspection_date, sample_qty, result, defect_qty, work_time_min, note)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
		RETURNING created_at, updated_at
	`

	err := r.db.Pool().QueryRow(ctx, query,
		record.RecordID,
		record.LotID,
		record.ItemID,
		record.WorkerID,
		record.InspectionDate,
		record.SampleQty,
		record.Result,
		record.DefectQty,
		record.WorkTimeMin,
		record.Note,
	).Scan(&record.CreatedAt, &record.UpdatedAt)

	if err != nil {
		return fmt.Errorf("検査記録作成に失敗: %w", err)
	}

	return nil
}

// InspectionRecordWithDetails は検査記録と関連データを含む
type InspectionRecordWithDetails struct {
	RecordID       string    `json:"record_id"`
	LotID          string    `json:"lot_id"`
	PartName       string    `json:"part_name"`
	ItemName       string    `json:"item_name"`
	WorkerName     *string   `json:"worker_name,omitempty"`
	InspectionDate time.Time `json:"inspection_date"`
	SampleQty      *int      `json:"sample_qty,omitempty"`
	Result         string    `json:"result"`
	DefectQty      int       `json:"defect_qty"`
	WorkTimeMin    *float64  `json:"work_time_min,omitempty"`
	Note           *string   `json:"note,omitempty"`
	CreatedAt      time.Time `json:"created_at"`
}

// ListFilter は検査記録一覧のフィルター条件
type ListFilter struct {
	DateFrom  *time.Time
	DateTo    *time.Time
	LotID     *string
	PartID    *string
	Result    *string
	Limit     int
	Offset    int
}

// ListWithDetails はフィルター条件に基づいて検査記録を取得する
func (r *InspectionRecordRepository) ListWithDetails(ctx context.Context, filter ListFilter) ([]InspectionRecordWithDetails, int, error) {
	// デフォルト値
	if filter.Limit == 0 {
		filter.Limit = 50
	}

	// WHERE句の構築
	whereClause := "WHERE 1=1"
	args := []interface{}{}
	argIndex := 1

	if filter.DateFrom != nil {
		whereClause += fmt.Sprintf(" AND ir.inspection_date >= $%d", argIndex)
		args = append(args, *filter.DateFrom)
		argIndex++
	}
	if filter.DateTo != nil {
		whereClause += fmt.Sprintf(" AND ir.inspection_date <= $%d", argIndex)
		args = append(args, *filter.DateTo)
		argIndex++
	}
	if filter.LotID != nil {
		whereClause += fmt.Sprintf(" AND ir.lot_id = $%d", argIndex)
		args = append(args, *filter.LotID)
		argIndex++
	}
	if filter.PartID != nil {
		whereClause += fmt.Sprintf(" AND l.part_id = $%d", argIndex)
		args = append(args, *filter.PartID)
		argIndex++
	}
	if filter.Result != nil {
		whereClause += fmt.Sprintf(" AND ir.result = $%d", argIndex)
		args = append(args, *filter.Result)
		argIndex++
	}

	// 総件数取得
	countQuery := fmt.Sprintf(`
		SELECT COUNT(*)
		FROM inspection_records ir
		JOIN lots l ON ir.lot_id = l.lot_id
		%s
	`, whereClause)
	var total int
	err := r.db.Pool().QueryRow(ctx, countQuery, args...).Scan(&total)
	if err != nil {
		return nil, 0, fmt.Errorf("検査記録件数取得に失敗: %w", err)
	}

	// データ取得
	query := fmt.Sprintf(`
		SELECT
			ir.record_id, ir.lot_id, p.name AS part_name, ii.name AS item_name,
			w.name AS worker_name, ir.inspection_date, ir.sample_qty,
			ir.result, ir.defect_qty, ir.work_time_min, ir.note, ir.created_at
		FROM inspection_records ir
		JOIN lots l ON ir.lot_id = l.lot_id
		JOIN parts p ON l.part_id = p.part_id
		JOIN inspection_items ii ON ir.item_id = ii.item_id
		LEFT JOIN workers w ON ir.worker_id = w.worker_id
		%s
		ORDER BY ir.inspection_date DESC, ir.created_at DESC
		LIMIT $%d OFFSET $%d
	`, whereClause, argIndex, argIndex+1)

	args = append(args, filter.Limit, filter.Offset)

	rows, err := r.db.Pool().Query(ctx, query, args...)
	if err != nil {
		return nil, 0, fmt.Errorf("検査記録一覧取得に失敗: %w", err)
	}
	defer rows.Close()

	var records []InspectionRecordWithDetails
	for rows.Next() {
		var rec InspectionRecordWithDetails
		err := rows.Scan(
			&rec.RecordID, &rec.LotID, &rec.PartName, &rec.ItemName,
			&rec.WorkerName, &rec.InspectionDate, &rec.SampleQty,
			&rec.Result, &rec.DefectQty, &rec.WorkTimeMin, &rec.Note, &rec.CreatedAt,
		)
		if err != nil {
			return nil, 0, fmt.Errorf("検査記録スキャンに失敗: %w", err)
		}
		records = append(records, rec)
	}

	return records, total, nil
}
