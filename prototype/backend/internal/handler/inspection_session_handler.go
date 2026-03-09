package handler

import (
	"context"
	"encoding/json"
	"net/http"
	"sync"
	"time"

	"github.com/nttedt/qa-inspection-api/internal/repository"
	"github.com/nttedt/qa-inspection-api/internal/session"
)

// InspectionSessionHandler は検査セッションAPIのハンドラー
type InspectionSessionHandler struct {
	repo    *repository.InspectionRecordRepository
	lotRepo *repository.LotRepository
	// インメモリでセッションを管理（プロトタイプ）
	sessions map[string]*session.InspectionSession
	mu       sync.RWMutex
}

// NewInspectionSessionHandler は新しいInspectionSessionHandlerを作成する
func NewInspectionSessionHandler(repo *repository.InspectionRecordRepository, lotRepo *repository.LotRepository) *InspectionSessionHandler {
	return &InspectionSessionHandler{
		repo:     repo,
		lotRepo:  lotRepo,
		sessions: make(map[string]*session.InspectionSession),
	}
}

// StartRequest は検査開始リクエスト
type StartRequest struct {
	LotID    string `json:"lot_id"`
	ItemID   string `json:"item_id"`
	WorkerID string `json:"worker_id"`
}

// Start は検査を開始する
// POST /api/v1/inspection-sessions
func (h *InspectionSessionHandler) Start(w http.ResponseWriter, r *http.Request) {
	var req StartRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		respondJSON(w, http.StatusBadRequest, map[string]string{"error": "リクエストが不正です"})
		return
	}

	// ロットの存在確認
	_, err := h.lotRepo.FindByID(context.Background(), req.LotID)
	if err != nil {
		if err == repository.ErrNotFound {
			respondJSON(w, http.StatusBadRequest, map[string]string{"error": "指定されたロットが存在しません"})
			return
		}
		respondJSON(w, http.StatusInternalServerError, map[string]string{"error": "ロット確認に失敗しました"})
		return
	}

	// セッション作成
	s := session.NewInspectionSession(req.LotID, req.ItemID, req.WorkerID)

	// インメモリに保存
	h.mu.Lock()
	h.sessions[s.ID] = s
	h.mu.Unlock()

	respondJSON(w, http.StatusOK, map[string]interface{}{
		"session_id": s.ID,
		"started_at": s.StartedAt.Format(time.RFC3339),
	})
}

// CountRequest はカウント追加リクエスト
type CountRequest struct {
	Result string `json:"result"` // pass, fail, waiver
}

// AddCount はカウントを追加する
// POST /api/v1/inspection-sessions/{id}/count
func (h *InspectionSessionHandler) AddCount(w http.ResponseWriter, r *http.Request) {
	sessionID := r.PathValue("id")

	h.mu.RLock()
	s, ok := h.sessions[sessionID]
	h.mu.RUnlock()

	if !ok {
		respondJSON(w, http.StatusNotFound, map[string]string{"error": "セッションが見つかりません"})
		return
	}

	var req CountRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		respondJSON(w, http.StatusBadRequest, map[string]string{"error": "リクエストが不正です"})
		return
	}

	if err := s.AddCount(req.Result); err != nil {
		respondJSON(w, http.StatusBadRequest, map[string]string{"error": err.Error()})
		return
	}

	counts := s.GetCounts()
	respondJSON(w, http.StatusOK, map[string]interface{}{
		"ok_count":   counts.Pass,
		"ng_count":   counts.Fail,
		"skip_count": counts.Waiver,
	})
}

// Undo は直前のカウントを取り消す
// DELETE /api/v1/inspection-sessions/{id}/count
func (h *InspectionSessionHandler) Undo(w http.ResponseWriter, r *http.Request) {
	sessionID := r.PathValue("id")

	h.mu.RLock()
	s, ok := h.sessions[sessionID]
	h.mu.RUnlock()

	if !ok {
		respondJSON(w, http.StatusNotFound, map[string]string{"error": "セッションが見つかりません"})
		return
	}

	if err := s.Undo(); err != nil {
		respondJSON(w, http.StatusBadRequest, map[string]string{"error": err.Error()})
		return
	}

	counts := s.GetCounts()
	respondJSON(w, http.StatusOK, map[string]interface{}{
		"ok_count":   counts.Pass,
		"ng_count":   counts.Fail,
		"skip_count": counts.Waiver,
	})
}

// FinishRequest は検査終了リクエスト
type FinishRequest struct {
	Note string `json:"note,omitempty"`
}

// Finish は検査を終了し、DBに保存する
// POST /api/v1/inspection-sessions/{id}/finish
func (h *InspectionSessionHandler) Finish(w http.ResponseWriter, r *http.Request) {
	sessionID := r.PathValue("id")

	h.mu.Lock()
	s, ok := h.sessions[sessionID]
	if ok {
		// セッションを削除（二重終了防止）
		delete(h.sessions, sessionID)
	}
	h.mu.Unlock()

	if !ok {
		respondJSON(w, http.StatusNotFound, map[string]string{"error": "セッションが見つかりません"})
		return
	}

	var req FinishRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		// 空のボディも許容
		req = FinishRequest{}
	}

	// セッション終了
	result, err := s.Finish(req.Note)
	if err != nil {
		respondJSON(w, http.StatusInternalServerError, map[string]string{"error": err.Error()})
		return
	}

	// 結果を判定（不合格が1つでもあれば不合格、そうでなければ合格）
	inspResult := "合格"
	if result.FailCount > 0 {
		inspResult = "不合格"
	}

	// DBに保存
	workerID := result.WorkerID
	note := result.Note
	record := &repository.InspectionRecord{
		LotID:          result.LotID,
		ItemID:         result.ItemID,
		WorkerID:       &workerID,
		InspectionDate: time.Now(),
		Result:         inspResult,
		DefectQty:      result.FailCount,
		WorkTimeMin:    &result.WorkTimeMin,
		Note:           &note,
	}

	if err := h.repo.Create(context.Background(), record); err != nil {
		respondJSON(w, http.StatusInternalServerError, map[string]string{"error": "検査記録の保存に失敗しました"})
		return
	}

	respondJSON(w, http.StatusOK, map[string]interface{}{
		"message":              "検査完了",
		"inspection_record_id": record.RecordID,
		"man_hours":            result.WorkTimeMin,
		"result":               inspResult,
		"ok_count":             result.PassCount,
		"ng_count":             result.FailCount,
		"skip_count":           result.WaiverCount,
	})
}
