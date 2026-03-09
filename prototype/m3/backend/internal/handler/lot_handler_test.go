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

// テストシナリオ（Session 36で承認済み）
// | # | シナリオ | 入力 | 期待結果 |
// |---|---------|------|---------|
// | 1 | ロット登録_正常 | `{part_id, quantity}` | 201 |
// | 2 | ロット一覧取得_正常 | なし | 200 |
// | 3 | ロット詳細取得_正常 | `lot_id` | 200 |
// | 4 | ロット詳細取得_存在しない | 不正なID | 404 |
// | 5 | ロット登録_必須項目欠落 | `{quantity}` のみ | 400 |

// TestLotCreate はロット登録APIの振る舞いをテストする
func TestLotCreate(t *testing.T) {
	// Arrange: テストDB接続
	db := setupTestDB(t)
	defer db.Close()

	repo := repository.NewLotRepository(db)
	h := handler.NewLotHandler(repo)

	tests := []struct {
		name          string
		requestBody   map[string]interface{}
		wantStatus    int
		shouldSucceed bool
	}{
		// 正常系
		{
			name: "ロット登録_正常_必須項目のみ",
			requestBody: map[string]interface{}{
				"part_id":  "PART-001",
				"quantity": 100,
			},
			wantStatus:    http.StatusCreated,
			shouldSucceed: true,
		},
		{
			name: "ロット登録_正常_全項目",
			requestBody: map[string]interface{}{
				"part_id":      "PART-001",
				"quantity":     50,
				"po_number":    "PO-2026-001",
				"arrival_date": "2026-03-06",
				"fw_version":   "1.0.0",
				"hw_version":   "2.0.0",
			},
			wantStatus:    http.StatusCreated,
			shouldSucceed: true,
		},
		// 異常系
		{
			name: "ロット登録_必須項目欠落_part_idなし",
			requestBody: map[string]interface{}{
				"quantity": 100,
			},
			wantStatus:    http.StatusBadRequest,
			shouldSucceed: false,
		},
		{
			name: "ロット登録_必須項目欠落_quantityなし",
			requestBody: map[string]interface{}{
				"part_id": "PART-001",
			},
			wantStatus:    http.StatusBadRequest,
			shouldSucceed: false,
		},
		{
			name:          "ロット登録_空のリクエスト",
			requestBody:   map[string]interface{}{},
			wantStatus:    http.StatusBadRequest,
			shouldSucceed: false,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// Arrange
			body, _ := json.Marshal(tt.requestBody)
			req := httptest.NewRequest(http.MethodPost, "/api/v1/lots", bytes.NewReader(body))
			req.Header.Set("Content-Type", "application/json")
			w := httptest.NewRecorder()

			// Act
			h.Create(w, req)

			// Assert
			if w.Code != tt.wantStatus {
				t.Errorf("status = %d, want %d", w.Code, tt.wantStatus)
			}

			if tt.shouldSucceed {
				// 成功時: lot_idが返ることを確認
				var resp map[string]interface{}
				if err := json.Unmarshal(w.Body.Bytes(), &resp); err != nil {
					t.Fatalf("レスポンスのパースに失敗: %v", err)
				}
				if _, ok := resp["lot_id"]; !ok {
					t.Error("レスポンスにlot_idが含まれていない")
				}
			}
		})
	}
}

// TestLotList はロット一覧取得APIの振る舞いをテストする
func TestLotList(t *testing.T) {
	// Arrange: テストDB接続
	db := setupTestDB(t)
	defer db.Close()

	repo := repository.NewLotRepository(db)
	h := handler.NewLotHandler(repo)

	tests := []struct {
		name          string
		setupData     func() // テストデータ投入用
		wantStatus    int
		wantMinCount  int // 最低限のロット数
		shouldSucceed bool
	}{
		// 正常系
		{
			name:          "ロット一覧取得_正常_データなし",
			setupData:     func() {}, // データ投入なし
			wantStatus:    http.StatusOK,
			wantMinCount:  0,
			shouldSucceed: true,
		},
		{
			name: "ロット一覧取得_正常_データあり",
			setupData: func() {
				// テストデータを投入
				createTestLot(t, repo, "PART-001", 100)
			},
			wantStatus:    http.StatusOK,
			wantMinCount:  1,
			shouldSucceed: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// Arrange: 各テストごとにDBをクリーン
			cleanupTestDB(t, db)
			if tt.setupData != nil {
				tt.setupData()
			}

			req := httptest.NewRequest(http.MethodGet, "/api/v1/lots", nil)
			w := httptest.NewRecorder()

			// Act
			h.List(w, req)

			// Assert
			if w.Code != tt.wantStatus {
				t.Errorf("status = %d, want %d", w.Code, tt.wantStatus)
			}

			if tt.shouldSucceed {
				var resp struct {
					Lots []interface{} `json:"lots"`
				}
				if err := json.Unmarshal(w.Body.Bytes(), &resp); err != nil {
					t.Fatalf("レスポンスのパースに失敗: %v", err)
				}
				if len(resp.Lots) < tt.wantMinCount {
					t.Errorf("lots count = %d, want >= %d", len(resp.Lots), tt.wantMinCount)
				}
			}
		})
	}
}

// TestLotGet はロット詳細取得APIの振る舞いをテストする
func TestLotGet(t *testing.T) {
	// Arrange: テストDB接続
	db := setupTestDB(t)
	defer db.Close()

	repo := repository.NewLotRepository(db)
	h := handler.NewLotHandler(repo)

	// テストデータ投入
	cleanupTestDB(t, db)
	testLotID := createTestLot(t, repo, "PART-001", 100)

	tests := []struct {
		name          string
		lotID         string
		wantStatus    int
		shouldSucceed bool
	}{
		// 正常系
		{
			name:          "ロット詳細取得_正常",
			lotID:         testLotID,
			wantStatus:    http.StatusOK,
			shouldSucceed: true,
		},
		// 異常系
		{
			name:          "ロット詳細取得_存在しないID",
			lotID:         "NOT-EXIST-LOT-ID",
			wantStatus:    http.StatusNotFound,
			shouldSucceed: false,
		},
		{
			name:          "ロット詳細取得_空のID",
			lotID:         "",
			wantStatus:    http.StatusBadRequest,
			shouldSucceed: false,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// Arrange
			req := httptest.NewRequest(http.MethodGet, "/api/v1/lots/"+tt.lotID, nil)
			// Go 1.22のパスパラメータ取得用にPathValueを設定
			req.SetPathValue("id", tt.lotID)
			w := httptest.NewRecorder()

			// Act
			h.Get(w, req)

			// Assert
			if w.Code != tt.wantStatus {
				t.Errorf("status = %d, want %d", w.Code, tt.wantStatus)
			}

			if tt.shouldSucceed {
				var resp map[string]interface{}
				if err := json.Unmarshal(w.Body.Bytes(), &resp); err != nil {
					t.Fatalf("レスポンスのパースに失敗: %v", err)
				}
				if resp["lot_id"] != tt.lotID {
					t.Errorf("lot_id = %v, want %v", resp["lot_id"], tt.lotID)
				}
			}
		})
	}
}

// ========================================
// テストヘルパー関数
// ========================================

// setupTestDB はテスト用のDB接続を作成する
func setupTestDB(t *testing.T) *repository.DB {
	t.Helper()
	// テスト環境のDB接続文字列
	// docker compose -f docker-compose.test.yml up -d で起動しておく
	dsn := "postgres://qa_user:qa_pass@localhost:5433/qa_inspection_test?sslmode=disable"
	db, err := repository.NewDB(context.Background(), dsn)
	if err != nil {
		t.Fatalf("DB接続に失敗: %v", err)
	}
	return db
}

// cleanupTestDB はテストデータをクリアする
func cleanupTestDB(t *testing.T, db *repository.DB) {
	t.Helper()
	// 外部キー制約の順序に注意してTRUNCATE
	_, err := db.Exec(context.Background(), `
		TRUNCATE TABLE waivers, defect_reports, inspection_records, lots CASCADE
	`)
	if err != nil {
		t.Fatalf("テストデータのクリーンアップに失敗: %v", err)
	}
}

// createTestLot はテスト用のロットを作成し、lot_idを返す
func createTestLot(t *testing.T, repo *repository.LotRepository, partID string, quantity int) string {
	t.Helper()
	lot := &repository.Lot{
		PartID:   partID,
		Quantity: quantity,
	}
	if err := repo.Create(context.Background(), lot); err != nil {
		t.Fatalf("テストロット作成に失敗: %v", err)
	}
	return lot.LotID
}
