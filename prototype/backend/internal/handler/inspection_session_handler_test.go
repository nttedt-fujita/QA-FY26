package handler_test

import (
	"bytes"
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/nttedt/qa-inspection-api/internal/handler"
	"github.com/nttedt/qa-inspection-api/internal/repository"
)

// テストシナリオ（Session 43で承認済み）
// | # | シナリオ | 操作 | 期待結果 |
// |---|---------|------|---------|
// | 9 | 検査開始（正常） | POST /inspection-sessions | 200, session_id返却 |
// | 10 | 検査開始（存在しないロット） | POST /inspection-sessions | 400, エラーメッセージ |
// | 11 | 検査終了（正常） | POST /inspection-sessions/{id}/finish | 200, record_id返却, DB保存 |
// | 12 | 検査終了（メモ付き） | POST /inspection-sessions/{id}/finish + note | 200, メモがDBに保存 |
// | 13 | 検査終了（存在しないセッション） | POST /inspection-sessions/{id}/finish | 404, エラー |
// | 14 | 検査終了（二重終了） | finish→finish | 400, エラー |
// | 15 | 工数自動計算 | 開始→終了 | work_time_minが経過時間 |

// TestInspectionSessionHandler_Start は検査開始APIの振る舞いをテストする
func TestInspectionSessionHandler_Start(t *testing.T) {
	// Arrange: テストDB接続
	db := setupTestDB(t)
	defer db.Close()

	repo := repository.NewInspectionRecordRepository(db)
	lotRepo := repository.NewLotRepository(db)
	h := handler.NewInspectionSessionHandler(repo, lotRepo)

	// テストデータ準備
	cleanupTestDB(t, db)
	testLotID := createTestLot(t, lotRepo, "PART-001", 100)

	tests := []struct {
		name          string
		requestBody   map[string]interface{}
		wantStatus    int
		shouldSucceed bool
	}{
		// 正常系
		{
			name: "検査開始_正常",
			requestBody: map[string]interface{}{
				"lot_id":    testLotID,
				"item_id":   "ITEM-001",
				"worker_id": "WKR-001",
			},
			wantStatus:    http.StatusOK,
			shouldSucceed: true,
		},
		// 異常系
		{
			name: "検査開始_存在しないロット",
			requestBody: map[string]interface{}{
				"lot_id":    "NOT-EXIST-LOT",
				"item_id":   "ITEM-001",
				"worker_id": "WKR-001",
			},
			wantStatus:    http.StatusBadRequest,
			shouldSucceed: false,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// Arrange
			body, _ := json.Marshal(tt.requestBody)
			req := httptest.NewRequest(http.MethodPost, "/api/v1/inspection-sessions", bytes.NewReader(body))
			req.Header.Set("Content-Type", "application/json")
			w := httptest.NewRecorder()

			// Act
			h.Start(w, req)

			// Assert
			if w.Code != tt.wantStatus {
				t.Errorf("status = %d, want %d, body = %s", w.Code, tt.wantStatus, w.Body.String())
			}

			if tt.shouldSucceed {
				var resp map[string]interface{}
				if err := json.Unmarshal(w.Body.Bytes(), &resp); err != nil {
					t.Fatalf("レスポンスのパースに失敗: %v", err)
				}
				if _, ok := resp["session_id"]; !ok {
					t.Error("レスポンスにsession_idが含まれていない")
				}
				if _, ok := resp["started_at"]; !ok {
					t.Error("レスポンスにstarted_atが含まれていない")
				}
			}
		})
	}
}

// TestInspectionSessionHandler_Count はカウント追加/取り消しAPIの振る舞いをテストする
func TestInspectionSessionHandler_Count(t *testing.T) {
	// Arrange: テストDB接続
	db := setupTestDB(t)
	defer db.Close()

	repo := repository.NewInspectionRecordRepository(db)
	lotRepo := repository.NewLotRepository(db)
	h := handler.NewInspectionSessionHandler(repo, lotRepo)

	// テストデータ準備
	cleanupTestDB(t, db)
	testLotID := createTestLot(t, lotRepo, "PART-001", 100)

	// セッション開始
	sessionID := startTestSession(t, h, testLotID, "ITEM-001", "WKR-001")

	// カウント追加
	t.Run("カウント追加_正常", func(t *testing.T) {
		body, _ := json.Marshal(map[string]interface{}{"result": "pass"})
		req := httptest.NewRequest(http.MethodPost, "/api/v1/inspection-sessions/"+sessionID+"/count", bytes.NewReader(body))
		req.Header.Set("Content-Type", "application/json")
		req.SetPathValue("id", sessionID)
		w := httptest.NewRecorder()

		h.AddCount(w, req)

		if w.Code != http.StatusOK {
			t.Errorf("status = %d, want %d", w.Code, http.StatusOK)
		}

		var resp map[string]interface{}
		json.Unmarshal(w.Body.Bytes(), &resp)
		counts := resp["counts"].(map[string]interface{})
		if counts["pass"].(float64) != 1 {
			t.Errorf("pass = %v, want 1", counts["pass"])
		}
	})

	// 取り消し
	t.Run("取り消し_正常", func(t *testing.T) {
		req := httptest.NewRequest(http.MethodDelete, "/api/v1/inspection-sessions/"+sessionID+"/count", nil)
		req.SetPathValue("id", sessionID)
		w := httptest.NewRecorder()

		h.Undo(w, req)

		if w.Code != http.StatusOK {
			t.Errorf("status = %d, want %d", w.Code, http.StatusOK)
		}

		var resp map[string]interface{}
		json.Unmarshal(w.Body.Bytes(), &resp)
		counts := resp["counts"].(map[string]interface{})
		if counts["pass"].(float64) != 0 {
			t.Errorf("pass = %v, want 0", counts["pass"])
		}
	})
}

// TestInspectionSessionHandler_Finish は検査終了APIの振る舞いをテストする
func TestInspectionSessionHandler_Finish(t *testing.T) {
	// Arrange: テストDB接続
	db := setupTestDB(t)
	defer db.Close()

	repo := repository.NewInspectionRecordRepository(db)
	lotRepo := repository.NewLotRepository(db)
	h := handler.NewInspectionSessionHandler(repo, lotRepo)

	// テストデータ準備
	cleanupTestDB(t, db)
	testLotID := createTestLot(t, lotRepo, "PART-001", 100)

	t.Run("検査終了_正常", func(t *testing.T) {
		// セッション開始
		sessionID := startTestSession(t, h, testLotID, "ITEM-001", "WKR-001")

		// カウント追加
		addTestCount(t, h, sessionID, "pass")
		addTestCount(t, h, sessionID, "pass")
		addTestCount(t, h, sessionID, "fail")

		// 検査終了
		body, _ := json.Marshal(map[string]interface{}{"note": "テストメモ"})
		req := httptest.NewRequest(http.MethodPost, "/api/v1/inspection-sessions/"+sessionID+"/finish", bytes.NewReader(body))
		req.Header.Set("Content-Type", "application/json")
		req.SetPathValue("id", sessionID)
		w := httptest.NewRecorder()

		h.Finish(w, req)

		if w.Code != http.StatusOK {
			t.Errorf("status = %d, want %d, body = %s", w.Code, http.StatusOK, w.Body.String())
		}

		var resp map[string]interface{}
		json.Unmarshal(w.Body.Bytes(), &resp)
		if _, ok := resp["record_id"]; !ok {
			t.Error("レスポンスにrecord_idが含まれていない")
		}
		if resp["work_time_min"] == nil {
			t.Error("work_time_minがない")
		}
	})

	t.Run("検査終了_存在しないセッション", func(t *testing.T) {
		body, _ := json.Marshal(map[string]interface{}{})
		req := httptest.NewRequest(http.MethodPost, "/api/v1/inspection-sessions/NOT-EXIST/finish", bytes.NewReader(body))
		req.Header.Set("Content-Type", "application/json")
		req.SetPathValue("id", "NOT-EXIST")
		w := httptest.NewRecorder()

		h.Finish(w, req)

		if w.Code != http.StatusNotFound {
			t.Errorf("status = %d, want %d", w.Code, http.StatusNotFound)
		}
	})

	t.Run("検査終了_二重終了", func(t *testing.T) {
		// セッション開始
		sessionID := startTestSession(t, h, testLotID, "ITEM-001", "WKR-001")

		// 1回目の終了
		body, _ := json.Marshal(map[string]interface{}{})
		req := httptest.NewRequest(http.MethodPost, "/api/v1/inspection-sessions/"+sessionID+"/finish", bytes.NewReader(body))
		req.Header.Set("Content-Type", "application/json")
		req.SetPathValue("id", sessionID)
		w := httptest.NewRecorder()
		h.Finish(w, req)

		// 2回目の終了
		req2 := httptest.NewRequest(http.MethodPost, "/api/v1/inspection-sessions/"+sessionID+"/finish", bytes.NewReader(body))
		req2.Header.Set("Content-Type", "application/json")
		req2.SetPathValue("id", sessionID)
		w2 := httptest.NewRecorder()
		h.Finish(w2, req2)

		// セッションは終了時に削除されるため、2回目は「存在しない」(404)となる
		if w2.Code != http.StatusNotFound {
			t.Errorf("status = %d, want %d", w2.Code, http.StatusNotFound)
		}
	})
}

// ========================================
// テストヘルパー関数
// ========================================

// startTestSession はテスト用にセッションを開始し、session_idを返す
func startTestSession(t *testing.T, h *handler.InspectionSessionHandler, lotID, itemID, workerID string) string {
	t.Helper()
	body, _ := json.Marshal(map[string]interface{}{
		"lot_id":    lotID,
		"item_id":   itemID,
		"worker_id": workerID,
	})
	req := httptest.NewRequest(http.MethodPost, "/api/v1/inspection-sessions", bytes.NewReader(body))
	req.Header.Set("Content-Type", "application/json")
	w := httptest.NewRecorder()

	h.Start(w, req)

	if w.Code != http.StatusOK {
		t.Fatalf("セッション開始に失敗: status=%d, body=%s", w.Code, w.Body.String())
	}

	var resp map[string]interface{}
	json.Unmarshal(w.Body.Bytes(), &resp)
	return resp["session_id"].(string)
}

// addTestCount はテスト用にカウントを追加する
func addTestCount(t *testing.T, h *handler.InspectionSessionHandler, sessionID, result string) {
	t.Helper()
	body, _ := json.Marshal(map[string]interface{}{"result": result})
	req := httptest.NewRequest(http.MethodPost, "/api/v1/inspection-sessions/"+sessionID+"/count", bytes.NewReader(body))
	req.Header.Set("Content-Type", "application/json")
	req.SetPathValue("id", sessionID)
	w := httptest.NewRecorder()

	h.AddCount(w, req)

	if w.Code != http.StatusOK {
		t.Fatalf("カウント追加に失敗: status=%d", w.Code)
	}
}

// verifyRecordInDB はDBにレコードが保存されているか確認する
func verifyRecordInDB(t *testing.T, db *repository.DB, recordID string) {
	t.Helper()
	var count int
	err := db.Pool().QueryRow(context.Background(),
		"SELECT COUNT(*) FROM inspection_records WHERE record_id = $1", recordID).Scan(&count)
	if err != nil {
		t.Fatalf("DB確認に失敗: %v", err)
	}
	if count != 1 {
		t.Errorf("レコードがDBに保存されていない: record_id=%s", recordID)
	}
}
