package repository

import (
	"context"
	"fmt"
	"time"
)

// ========================================
// 部品（Parts）
// ========================================

// Part は部品を表す
type Part struct {
	PartID          string    `json:"part_id"`
	Name            string    `json:"name"`
	Category        string    `json:"category"`
	SupplierID      *string   `json:"supplier_id,omitempty"`
	SpecDocURL      *string   `json:"spec_doc_url,omitempty"`
	LatestFWVersion *string   `json:"latest_fw_version,omitempty"`
	CreatedAt       time.Time `json:"created_at"`
	UpdatedAt       time.Time `json:"updated_at"`
}

// PartRepository は部品のCRUD操作を提供する
type PartRepository struct {
	db *DB
}

// NewPartRepository は新しいPartRepositoryを作成する
func NewPartRepository(db *DB) *PartRepository {
	return &PartRepository{db: db}
}

// FindAll は全部品を取得する
func (r *PartRepository) FindAll(ctx context.Context) ([]*Part, error) {
	query := `
		SELECT part_id, name, category, supplier_id, spec_doc_url, latest_fw_version, created_at, updated_at
		FROM parts
		ORDER BY category, name
	`

	rows, err := r.db.Pool().Query(ctx, query)
	if err != nil {
		return nil, fmt.Errorf("部品一覧取得に失敗: %w", err)
	}
	defer rows.Close()

	var parts []*Part
	for rows.Next() {
		part := &Part{}
		err := rows.Scan(
			&part.PartID,
			&part.Name,
			&part.Category,
			&part.SupplierID,
			&part.SpecDocURL,
			&part.LatestFWVersion,
			&part.CreatedAt,
			&part.UpdatedAt,
		)
		if err != nil {
			return nil, fmt.Errorf("部品行のスキャンに失敗: %w", err)
		}
		parts = append(parts, part)
	}

	if err := rows.Err(); err != nil {
		return nil, fmt.Errorf("部品一覧の読み取りに失敗: %w", err)
	}

	return parts, nil
}

// ========================================
// 検査項目（InspectionItems）
// ========================================

// InspectionItem は検査項目を表す
type InspectionItem struct {
	ItemID      string    `json:"item_id"`
	Name        string    `json:"name"`
	Type        string    `json:"type"`
	StdTimeMin  *float64  `json:"std_time_min,omitempty"`
	Description *string   `json:"description,omitempty"`
	CreatedAt   time.Time `json:"created_at"`
	UpdatedAt   time.Time `json:"updated_at"`
}

// InspectionItemRepository は検査項目のCRUD操作を提供する
type InspectionItemRepository struct {
	db *DB
}

// NewInspectionItemRepository は新しいInspectionItemRepositoryを作成する
func NewInspectionItemRepository(db *DB) *InspectionItemRepository {
	return &InspectionItemRepository{db: db}
}

// FindAll は全検査項目を取得する
func (r *InspectionItemRepository) FindAll(ctx context.Context) ([]*InspectionItem, error) {
	query := `
		SELECT item_id, name, type, std_time_min, description, created_at, updated_at
		FROM inspection_items
		ORDER BY item_id
	`

	rows, err := r.db.Pool().Query(ctx, query)
	if err != nil {
		return nil, fmt.Errorf("検査項目一覧取得に失敗: %w", err)
	}
	defer rows.Close()

	var items []*InspectionItem
	for rows.Next() {
		item := &InspectionItem{}
		err := rows.Scan(
			&item.ItemID,
			&item.Name,
			&item.Type,
			&item.StdTimeMin,
			&item.Description,
			&item.CreatedAt,
			&item.UpdatedAt,
		)
		if err != nil {
			return nil, fmt.Errorf("検査項目行のスキャンに失敗: %w", err)
		}
		items = append(items, item)
	}

	if err := rows.Err(); err != nil {
		return nil, fmt.Errorf("検査項目一覧の読み取りに失敗: %w", err)
	}

	return items, nil
}

// ========================================
// 作業者（Workers）
// ========================================

// Worker は作業者を表す
type Worker struct {
	WorkerID  string    `json:"worker_id"`
	Name      string    `json:"name"`
	Role      *string   `json:"role,omitempty"`
	CreatedAt time.Time `json:"created_at"`
	UpdatedAt time.Time `json:"updated_at"`
}

// WorkerRepository は作業者のCRUD操作を提供する
type WorkerRepository struct {
	db *DB
}

// NewWorkerRepository は新しいWorkerRepositoryを作成する
func NewWorkerRepository(db *DB) *WorkerRepository {
	return &WorkerRepository{db: db}
}

// FindAll は全作業者を取得する
func (r *WorkerRepository) FindAll(ctx context.Context) ([]*Worker, error) {
	query := `
		SELECT worker_id, name, role, created_at, updated_at
		FROM workers
		ORDER BY worker_id
	`

	rows, err := r.db.Pool().Query(ctx, query)
	if err != nil {
		return nil, fmt.Errorf("作業者一覧取得に失敗: %w", err)
	}
	defer rows.Close()

	var workers []*Worker
	for rows.Next() {
		worker := &Worker{}
		err := rows.Scan(
			&worker.WorkerID,
			&worker.Name,
			&worker.Role,
			&worker.CreatedAt,
			&worker.UpdatedAt,
		)
		if err != nil {
			return nil, fmt.Errorf("作業者行のスキャンに失敗: %w", err)
		}
		workers = append(workers, worker)
	}

	if err := rows.Err(); err != nil {
		return nil, fmt.Errorf("作業者一覧の読み取りに失敗: %w", err)
	}

	return workers, nil
}
