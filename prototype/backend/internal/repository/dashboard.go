package repository

import (
	"context"
	"fmt"
	"time"
)

// DashboardRepository はダッシュボード用のデータ取得を提供する
type DashboardRepository struct {
	db *DB
}

// NewDashboardRepository は新しいDashboardRepositoryを作成する
func NewDashboardRepository(db *DB) *DashboardRepository {
	return &DashboardRepository{db: db}
}

// MonthlyStats は月別の検査統計
type MonthlyStats struct {
	Month    string `json:"month"`     // YYYY-MM形式
	Total    int    `json:"total"`     // 総検査数
	PassQty  int    `json:"pass_qty"`  // 合格数
	FailQty  int    `json:"fail_qty"`  // 不合格数
	SkipQty  int    `json:"skip_qty"`  // 不問数
	PassRate float64 `json:"pass_rate"` // 合格率（%）
}

// PartDefectRate は部品別の不良率
type PartDefectRate struct {
	PartID    string  `json:"part_id"`
	PartName  string  `json:"part_name"`
	TotalQty  int     `json:"total_qty"`   // 検査数
	DefectQty int     `json:"defect_qty"`  // 不良数
	DefectRate float64 `json:"defect_rate"` // 不良率（%）
}

// SummaryStats は集計情報
type SummaryStats struct {
	TotalInspections int     `json:"total_inspections"` // 総検査数
	TotalLots        int     `json:"total_lots"`        // 総ロット数
	PassRate         float64 `json:"pass_rate"`         // 合格率
	AvgWorkTime      float64 `json:"avg_work_time"`     // 平均工数（分）
}

// GetMonthlySummary は指定期間の月別統計を取得する
func (r *DashboardRepository) GetMonthlySummary(ctx context.Context, months int) ([]MonthlyStats, error) {
	query := `
		SELECT
			TO_CHAR(inspection_date, 'YYYY-MM') AS month,
			COUNT(*) AS total,
			SUM(CASE WHEN result = '合格' THEN 1 ELSE 0 END) AS pass_qty,
			SUM(CASE WHEN result = '不合格' THEN 1 ELSE 0 END) AS fail_qty,
			SUM(CASE WHEN result = '不問' THEN 1 ELSE 0 END) AS skip_qty
		FROM inspection_records
		WHERE inspection_date >= CURRENT_DATE - INTERVAL '%d months'
		GROUP BY TO_CHAR(inspection_date, 'YYYY-MM')
		ORDER BY month DESC
	`

	rows, err := r.db.Pool().Query(ctx, fmt.Sprintf(query, months))
	if err != nil {
		return nil, fmt.Errorf("月別統計取得に失敗: %w", err)
	}
	defer rows.Close()

	var stats []MonthlyStats
	for rows.Next() {
		var s MonthlyStats
		err := rows.Scan(&s.Month, &s.Total, &s.PassQty, &s.FailQty, &s.SkipQty)
		if err != nil {
			return nil, fmt.Errorf("月別統計スキャンに失敗: %w", err)
		}
		// 合格率計算（合格 / (合格+不合格) * 100）
		if s.PassQty+s.FailQty > 0 {
			s.PassRate = float64(s.PassQty) / float64(s.PassQty+s.FailQty) * 100
		}
		stats = append(stats, s)
	}

	return stats, nil
}

// GetTopDefectParts は不良率トップNの部品を取得する
func (r *DashboardRepository) GetTopDefectParts(ctx context.Context, limit int, since time.Time) ([]PartDefectRate, error) {
	query := `
		SELECT
			p.part_id, p.name AS part_name,
			COUNT(*) AS total_qty,
			SUM(ir.defect_qty) AS defect_qty
		FROM inspection_records ir
		JOIN lots l ON ir.lot_id = l.lot_id
		JOIN parts p ON l.part_id = p.part_id
		WHERE ir.inspection_date >= $1
		GROUP BY p.part_id, p.name
		HAVING SUM(ir.defect_qty) > 0
		ORDER BY SUM(ir.defect_qty) DESC
		LIMIT $2
	`

	rows, err := r.db.Pool().Query(ctx, query, since, limit)
	if err != nil {
		return nil, fmt.Errorf("部品別不良率取得に失敗: %w", err)
	}
	defer rows.Close()

	var parts []PartDefectRate
	for rows.Next() {
		var p PartDefectRate
		err := rows.Scan(&p.PartID, &p.PartName, &p.TotalQty, &p.DefectQty)
		if err != nil {
			return nil, fmt.Errorf("部品別不良率スキャンに失敗: %w", err)
		}
		// 不良率計算
		if p.TotalQty > 0 {
			p.DefectRate = float64(p.DefectQty) / float64(p.TotalQty) * 100
		}
		parts = append(parts, p)
	}

	return parts, nil
}

// GetSummary は全体の集計情報を取得する
func (r *DashboardRepository) GetSummary(ctx context.Context) (*SummaryStats, error) {
	query := `
		SELECT
			COUNT(*) AS total_inspections,
			COUNT(DISTINCT lot_id) AS total_lots,
			COALESCE(AVG(work_time_min), 0) AS avg_work_time,
			SUM(CASE WHEN result = '合格' THEN 1 ELSE 0 END) AS pass_qty,
			SUM(CASE WHEN result = '不合格' THEN 1 ELSE 0 END) AS fail_qty
		FROM inspection_records
	`

	var stats SummaryStats
	var passQty, failQty int
	err := r.db.Pool().QueryRow(ctx, query).Scan(
		&stats.TotalInspections, &stats.TotalLots, &stats.AvgWorkTime, &passQty, &failQty,
	)
	if err != nil {
		return nil, fmt.Errorf("集計情報取得に失敗: %w", err)
	}

	// 合格率計算
	if passQty+failQty > 0 {
		stats.PassRate = float64(passQty) / float64(passQty+failQty) * 100
	}

	return &stats, nil
}

// ItemStats は検査項目別の統計
type ItemStats struct {
	ItemID   string  `json:"item_id"`
	ItemName string  `json:"item_name"`
	Total    int     `json:"total"`
	PassQty  int     `json:"pass_qty"`
	FailQty  int     `json:"fail_qty"`
	PassRate float64 `json:"pass_rate"`
}

// GetItemStats は検査項目別の統計を取得する
func (r *DashboardRepository) GetItemStats(ctx context.Context) ([]ItemStats, error) {
	query := `
		SELECT
			ii.item_id, ii.name AS item_name,
			COUNT(*) AS total,
			SUM(CASE WHEN ir.result = '合格' THEN 1 ELSE 0 END) AS pass_qty,
			SUM(CASE WHEN ir.result = '不合格' THEN 1 ELSE 0 END) AS fail_qty
		FROM inspection_records ir
		JOIN inspection_items ii ON ir.item_id = ii.item_id
		GROUP BY ii.item_id, ii.name
		ORDER BY COUNT(*) DESC
	`

	rows, err := r.db.Pool().Query(ctx, query)
	if err != nil {
		return nil, fmt.Errorf("検査項目別統計取得に失敗: %w", err)
	}
	defer rows.Close()

	var stats []ItemStats
	for rows.Next() {
		var s ItemStats
		err := rows.Scan(&s.ItemID, &s.ItemName, &s.Total, &s.PassQty, &s.FailQty)
		if err != nil {
			return nil, fmt.Errorf("検査項目別統計スキャンに失敗: %w", err)
		}
		if s.PassQty+s.FailQty > 0 {
			s.PassRate = float64(s.PassQty) / float64(s.PassQty+s.FailQty) * 100
		}
		stats = append(stats, s)
	}

	return stats, nil
}

// RecentRecord は直近の検査記録
type RecentRecord struct {
	RecordID       string    `json:"record_id"`
	LotID          string    `json:"lot_id"`
	PartName       string    `json:"part_name"`
	ItemName       string    `json:"item_name"`
	Result         string    `json:"result"`
	InspectionDate time.Time `json:"inspection_date"`
}

// GetRecentRecords は直近の検査記録を取得する
func (r *DashboardRepository) GetRecentRecords(ctx context.Context, limit int) ([]RecentRecord, error) {
	query := `
		SELECT
			ir.record_id, ir.lot_id, p.name AS part_name, ii.name AS item_name,
			ir.result, ir.inspection_date
		FROM inspection_records ir
		JOIN lots l ON ir.lot_id = l.lot_id
		JOIN parts p ON l.part_id = p.part_id
		JOIN inspection_items ii ON ir.item_id = ii.item_id
		ORDER BY ir.created_at DESC
		LIMIT $1
	`

	rows, err := r.db.Pool().Query(ctx, query, limit)
	if err != nil {
		return nil, fmt.Errorf("直近検査記録取得に失敗: %w", err)
	}
	defer rows.Close()

	var records []RecentRecord
	for rows.Next() {
		var rec RecentRecord
		err := rows.Scan(&rec.RecordID, &rec.LotID, &rec.PartName, &rec.ItemName, &rec.Result, &rec.InspectionDate)
		if err != nil {
			return nil, fmt.Errorf("直近検査記録スキャンに失敗: %w", err)
		}
		records = append(records, rec)
	}

	return records, nil
}

// SupplierDefectRate はサプライヤー別の不良率
type SupplierDefectRate struct {
	SupplierID   string  `json:"supplier_id"`
	SupplierName string  `json:"supplier_name"`
	TotalQty     int     `json:"total_qty"`
	DefectQty    int     `json:"defect_qty"`
	DefectRate   float64 `json:"defect_rate"`
}

// GetSupplierDefectRates はサプライヤー別の不良率を取得する
func (r *DashboardRepository) GetSupplierDefectRates(ctx context.Context) ([]SupplierDefectRate, error) {
	query := `
		SELECT
			s.supplier_id, s.name AS supplier_name,
			COUNT(*) AS total_qty,
			SUM(ir.defect_qty) AS defect_qty
		FROM inspection_records ir
		JOIN lots l ON ir.lot_id = l.lot_id
		JOIN parts p ON l.part_id = p.part_id
		JOIN suppliers s ON p.supplier_id = s.supplier_id
		GROUP BY s.supplier_id, s.name
		ORDER BY SUM(ir.defect_qty) DESC
	`

	rows, err := r.db.Pool().Query(ctx, query)
	if err != nil {
		return nil, fmt.Errorf("サプライヤー別不良率取得に失敗: %w", err)
	}
	defer rows.Close()

	var rates []SupplierDefectRate
	for rows.Next() {
		var r SupplierDefectRate
		err := rows.Scan(&r.SupplierID, &r.SupplierName, &r.TotalQty, &r.DefectQty)
		if err != nil {
			return nil, fmt.Errorf("サプライヤー別不良率スキャンに失敗: %w", err)
		}
		if r.TotalQty > 0 {
			r.DefectRate = float64(r.DefectQty) / float64(r.TotalQty) * 100
		}
		rates = append(rates, r)
	}

	return rates, nil
}
