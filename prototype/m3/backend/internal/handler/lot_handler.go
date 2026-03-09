package handler

import (
	"encoding/json"
	"errors"
	"net/http"
	"time"

	"github.com/nttedt/qa-inspection-api/internal/repository"
)

// LotHandler はロットAPIのハンドラー
type LotHandler struct {
	repo *repository.LotRepository
}

// NewLotHandler は新しいLotHandlerを作成する
func NewLotHandler(repo *repository.LotRepository) *LotHandler {
	return &LotHandler{repo: repo}
}

// CreateLotRequest はロット登録リクエスト
type CreateLotRequest struct {
	PartID      string  `json:"part_id"`
	Quantity    int     `json:"quantity"`
	PONumber    *string `json:"po_number,omitempty"`
	ArrivalDate *string `json:"arrival_date,omitempty"` // "2006-01-02" 形式
	FWVersion   *string `json:"fw_version,omitempty"`
	HWVersion   *string `json:"hw_version,omitempty"`
}

// Create はロットを登録する
// POST /api/v1/lots
func (h *LotHandler) Create(w http.ResponseWriter, r *http.Request) {
	var req CreateLotRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		respondError(w, http.StatusBadRequest, "リクエストの解析に失敗しました")
		return
	}

	// バリデーション
	if req.PartID == "" {
		respondError(w, http.StatusBadRequest, "part_idは必須です")
		return
	}
	if req.Quantity <= 0 {
		respondError(w, http.StatusBadRequest, "quantityは1以上の整数が必須です")
		return
	}

	// 日付の変換
	var arrivalDate *time.Time
	if req.ArrivalDate != nil {
		t, err := time.Parse("2006-01-02", *req.ArrivalDate)
		if err != nil {
			respondError(w, http.StatusBadRequest, "arrival_dateの形式が不正です（YYYY-MM-DD）")
			return
		}
		arrivalDate = &t
	}

	lot := &repository.Lot{
		PartID:      req.PartID,
		Quantity:    req.Quantity,
		PONumber:    req.PONumber,
		ArrivalDate: arrivalDate,
		FWVersion:   req.FWVersion,
		HWVersion:   req.HWVersion,
	}

	if err := h.repo.Create(r.Context(), lot); err != nil {
		respondError(w, http.StatusInternalServerError, "ロットの登録に失敗しました")
		return
	}

	respondJSON(w, http.StatusCreated, map[string]interface{}{
		"lot_id":  lot.LotID,
		"message": "ロットを登録しました",
	})
}

// List はロット一覧を取得する
// GET /api/v1/lots
func (h *LotHandler) List(w http.ResponseWriter, r *http.Request) {
	lots, err := h.repo.FindAll(r.Context())
	if err != nil {
		respondError(w, http.StatusInternalServerError, "ロット一覧の取得に失敗しました")
		return
	}

	respondJSON(w, http.StatusOK, map[string]interface{}{
		"lots": lots,
	})
}

// Get はロット詳細を取得する
// GET /api/v1/lots/{id}
func (h *LotHandler) Get(w http.ResponseWriter, r *http.Request) {
	lotID := r.PathValue("id")
	if lotID == "" {
		respondError(w, http.StatusBadRequest, "ロットIDが指定されていません")
		return
	}

	lot, err := h.repo.FindByID(r.Context(), lotID)
	if err != nil {
		if errors.Is(err, repository.ErrNotFound) {
			respondError(w, http.StatusNotFound, "ロットが見つかりません")
			return
		}
		respondError(w, http.StatusInternalServerError, "ロットの取得に失敗しました")
		return
	}

	respondJSON(w, http.StatusOK, lot)
}

// ========================================
// ヘルパー関数
// ========================================

func respondJSON(w http.ResponseWriter, status int, data interface{}) {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(status)
	json.NewEncoder(w).Encode(data)
}

func respondError(w http.ResponseWriter, status int, message string) {
	respondJSON(w, status, map[string]string{
		"error": message,
	})
}
