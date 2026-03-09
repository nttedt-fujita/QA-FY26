package handler

import (
	"net/http"

	"github.com/nttedt/qa-inspection-api/internal/repository"
)

// MasterHandler はマスタデータAPIのハンドラー
type MasterHandler struct {
	partRepo           *repository.PartRepository
	inspectionItemRepo *repository.InspectionItemRepository
	workerRepo         *repository.WorkerRepository
}

// NewMasterHandler は新しいMasterHandlerを作成する
func NewMasterHandler(
	partRepo *repository.PartRepository,
	inspectionItemRepo *repository.InspectionItemRepository,
	workerRepo *repository.WorkerRepository,
) *MasterHandler {
	return &MasterHandler{
		partRepo:           partRepo,
		inspectionItemRepo: inspectionItemRepo,
		workerRepo:         workerRepo,
	}
}

// ListParts は部品一覧を取得する
// GET /api/v1/parts
func (h *MasterHandler) ListParts(w http.ResponseWriter, r *http.Request) {
	parts, err := h.partRepo.FindAll(r.Context())
	if err != nil {
		respondError(w, http.StatusInternalServerError, "部品一覧の取得に失敗しました")
		return
	}

	respondJSON(w, http.StatusOK, map[string]interface{}{
		"parts": parts,
	})
}

// ListInspectionItems は検査項目一覧を取得する
// GET /api/v1/inspection-items
func (h *MasterHandler) ListInspectionItems(w http.ResponseWriter, r *http.Request) {
	items, err := h.inspectionItemRepo.FindAll(r.Context())
	if err != nil {
		respondError(w, http.StatusInternalServerError, "検査項目一覧の取得に失敗しました")
		return
	}

	respondJSON(w, http.StatusOK, map[string]interface{}{
		"inspection_items": items,
	})
}

// ListWorkers は作業者一覧を取得する
// GET /api/v1/workers
func (h *MasterHandler) ListWorkers(w http.ResponseWriter, r *http.Request) {
	workers, err := h.workerRepo.FindAll(r.Context())
	if err != nil {
		respondError(w, http.StatusInternalServerError, "作業者一覧の取得に失敗しました")
		return
	}

	respondJSON(w, http.StatusOK, map[string]interface{}{
		"workers": workers,
	})
}
