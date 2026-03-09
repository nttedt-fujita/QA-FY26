package handler

import (
	"encoding/json"
	"net/http"
	"strconv"
	"time"

	"github.com/nttedt/qa-inspection-api/internal/repository"
)

// InspectionRecordHandler は検査記録のHTTPハンドラー
type InspectionRecordHandler struct {
	repo *repository.InspectionRecordRepository
}

// NewInspectionRecordHandler は新しいInspectionRecordHandlerを作成する
func NewInspectionRecordHandler(repo *repository.InspectionRecordRepository) *InspectionRecordHandler {
	return &InspectionRecordHandler{repo: repo}
}

// List は検査記録一覧を取得する
// GET /api/v1/inspection-records?date_from=2026-03-01&date_to=2026-03-31&lot_id=LOT-001&part_id=PART-001&result=合格&limit=50&offset=0
func (h *InspectionRecordHandler) List(w http.ResponseWriter, r *http.Request) {
	ctx := r.Context()
	query := r.URL.Query()

	filter := repository.ListFilter{
		Limit:  50,
		Offset: 0,
	}

	// 日付フィルター
	if dateFrom := query.Get("date_from"); dateFrom != "" {
		t, err := time.Parse("2006-01-02", dateFrom)
		if err == nil {
			filter.DateFrom = &t
		}
	}
	if dateTo := query.Get("date_to"); dateTo != "" {
		t, err := time.Parse("2006-01-02", dateTo)
		if err == nil {
			filter.DateTo = &t
		}
	}

	// その他のフィルター
	if lotID := query.Get("lot_id"); lotID != "" {
		filter.LotID = &lotID
	}
	if partID := query.Get("part_id"); partID != "" {
		filter.PartID = &partID
	}
	if result := query.Get("result"); result != "" {
		filter.Result = &result
	}

	// ページネーション
	if limit := query.Get("limit"); limit != "" {
		if l, err := strconv.Atoi(limit); err == nil && l > 0 {
			filter.Limit = l
		}
	}
	if offset := query.Get("offset"); offset != "" {
		if o, err := strconv.Atoi(offset); err == nil && o >= 0 {
			filter.Offset = o
		}
	}

	records, total, err := h.repo.ListWithDetails(ctx, filter)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	// レスポンス
	response := map[string]interface{}{
		"records": records,
		"total":   total,
		"limit":   filter.Limit,
		"offset":  filter.Offset,
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(response)
}
