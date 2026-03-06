package repository

import (
	"context"
	"errors"
	"fmt"
	"time"

	"github.com/google/uuid"
	"github.com/jackc/pgx/v5"
)

// Lot はロット（入荷単位）を表す
type Lot struct {
	LotID       string     `json:"lot_id"`
	PartID      string     `json:"part_id"`
	PONumber    *string    `json:"po_number,omitempty"`
	ArrivalDate *time.Time `json:"arrival_date,omitempty"`
	Quantity    int        `json:"quantity"`
	FWVersion   *string    `json:"fw_version,omitempty"`
	HWVersion   *string    `json:"hw_version,omitempty"`
	CreatedAt   time.Time  `json:"created_at"`
	UpdatedAt   time.Time  `json:"updated_at"`
}

// ErrNotFound はレコードが見つからない場合のエラー
var ErrNotFound = errors.New("レコードが見つかりません")

// LotRepository はロットのCRUD操作を提供する
type LotRepository struct {
	db *DB
}

// NewLotRepository は新しいLotRepositoryを作成する
func NewLotRepository(db *DB) *LotRepository {
	return &LotRepository{db: db}
}

// Create は新しいロットを作成する
func (r *LotRepository) Create(ctx context.Context, lot *Lot) error {
	// lot_idを生成（UUIDベース）
	lot.LotID = "LOT-" + uuid.New().String()[:8]

	query := `
		INSERT INTO lots (lot_id, part_id, po_number, arrival_date, quantity, fw_version, hw_version)
		VALUES ($1, $2, $3, $4, $5, $6, $7)
		RETURNING created_at, updated_at
	`

	err := r.db.Pool().QueryRow(ctx, query,
		lot.LotID,
		lot.PartID,
		lot.PONumber,
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

// FindAll は全ロットを取得する
func (r *LotRepository) FindAll(ctx context.Context) ([]*Lot, error) {
	query := `
		SELECT lot_id, part_id, po_number, arrival_date, quantity, fw_version, hw_version, created_at, updated_at
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

// FindByID はIDでロットを取得する
func (r *LotRepository) FindByID(ctx context.Context, lotID string) (*Lot, error) {
	query := `
		SELECT lot_id, part_id, po_number, arrival_date, quantity, fw_version, hw_version, created_at, updated_at
		FROM lots
		WHERE lot_id = $1
	`

	lot := &Lot{}
	err := r.db.Pool().QueryRow(ctx, query, lotID).Scan(
		&lot.LotID,
		&lot.PartID,
		&lot.PONumber,
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
