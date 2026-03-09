package handler

import (
	"encoding/json"
	"net/http"
	"strconv"
	"time"

	"github.com/nttedt/qa-inspection-api/internal/repository"
)

// DashboardHandler はダッシュボードのHTTPハンドラー
type DashboardHandler struct {
	repo *repository.DashboardRepository
}

// NewDashboardHandler は新しいDashboardHandlerを作成する
func NewDashboardHandler(repo *repository.DashboardRepository) *DashboardHandler {
	return &DashboardHandler{repo: repo}
}

// GetSummary は集計情報を取得する
// GET /api/v1/dashboard/summary
func (h *DashboardHandler) GetSummary(w http.ResponseWriter, r *http.Request) {
	ctx := r.Context()

	stats, err := h.repo.GetSummary(ctx)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(stats)
}

// GetMonthlyStats は月別統計を取得する
// GET /api/v1/dashboard/monthly?months=6
func (h *DashboardHandler) GetMonthlyStats(w http.ResponseWriter, r *http.Request) {
	ctx := r.Context()

	months := 6 // デフォルト6ヶ月
	if m := r.URL.Query().Get("months"); m != "" {
		if v, err := strconv.Atoi(m); err == nil && v > 0 {
			months = v
		}
	}

	stats, err := h.repo.GetMonthlySummary(ctx, months)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]interface{}{
		"monthly_stats": stats,
	})
}

// GetTopDefects は不良率トップの部品を取得する
// GET /api/v1/dashboard/top-defects?limit=5&months=3
func (h *DashboardHandler) GetTopDefects(w http.ResponseWriter, r *http.Request) {
	ctx := r.Context()
	query := r.URL.Query()

	limit := 5 // デフォルト5件
	if l := query.Get("limit"); l != "" {
		if v, err := strconv.Atoi(l); err == nil && v > 0 {
			limit = v
		}
	}

	months := 3 // デフォルト3ヶ月
	if m := query.Get("months"); m != "" {
		if v, err := strconv.Atoi(m); err == nil && v > 0 {
			months = v
		}
	}

	since := time.Now().AddDate(0, -months, 0)
	parts, err := h.repo.GetTopDefectParts(ctx, limit, since)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]interface{}{
		"top_defects": parts,
	})
}

// GetItemStats は検査項目別統計を取得する
// GET /api/v1/dashboard/items
func (h *DashboardHandler) GetItemStats(w http.ResponseWriter, r *http.Request) {
	ctx := r.Context()

	stats, err := h.repo.GetItemStats(ctx)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]interface{}{
		"item_stats": stats,
	})
}

// GetRecentRecords は直近の検査記録を取得する
// GET /api/v1/dashboard/recent?limit=5
func (h *DashboardHandler) GetRecentRecords(w http.ResponseWriter, r *http.Request) {
	ctx := r.Context()

	limit := 5 // デフォルト5件
	if l := r.URL.Query().Get("limit"); l != "" {
		if v, err := strconv.Atoi(l); err == nil && v > 0 {
			limit = v
		}
	}

	records, err := h.repo.GetRecentRecords(ctx, limit)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]interface{}{
		"recent_records": records,
	})
}

// GetSupplierDefects はサプライヤー別不良率を取得する
// GET /api/v1/dashboard/suppliers
func (h *DashboardHandler) GetSupplierDefects(w http.ResponseWriter, r *http.Request) {
	ctx := r.Context()

	rates, err := h.repo.GetSupplierDefectRates(ctx)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]interface{}{
		"supplier_defects": rates,
	})
}
