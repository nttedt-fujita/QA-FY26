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
