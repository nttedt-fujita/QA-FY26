package main

import (
	"context"
	"log"
	"net/http"
	"os"

	"github.com/nttedt/qa-inspection-api/internal/handler"
	"github.com/nttedt/qa-inspection-api/internal/repository"
)

// CORSミドルウェア
func corsMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		// CORS ヘッダー設定
		w.Header().Set("Access-Control-Allow-Origin", "*")
		w.Header().Set("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS")
		w.Header().Set("Access-Control-Allow-Headers", "Content-Type, Authorization")

		// プリフライトリクエストへの応答
		if r.Method == "OPTIONS" {
			w.WriteHeader(http.StatusOK)
			return
		}

		next.ServeHTTP(w, r)
	})
}

func main() {
	// DB接続
	dsn := os.Getenv("DATABASE_URL")
	if dsn == "" {
		dsn = "postgres://qa_user:qa_pass@localhost:5432/qa_inspection?sslmode=disable"
	}

	db, err := repository.NewDB(context.Background(), dsn)
	if err != nil {
		log.Fatalf("DB接続に失敗: %v", err)
	}
	defer db.Close()

	// リポジトリ・ハンドラー初期化
	lotRepo := repository.NewLotRepository(db)
	lotHandler := handler.NewLotHandler(lotRepo)

	partRepo := repository.NewPartRepository(db)
	inspectionItemRepo := repository.NewInspectionItemRepository(db)
	workerRepo := repository.NewWorkerRepository(db)
	masterHandler := handler.NewMasterHandler(partRepo, inspectionItemRepo, workerRepo)

	inspectionRecordRepo := repository.NewInspectionRecordRepository(db)
	inspectionSessionHandler := handler.NewInspectionSessionHandler(inspectionRecordRepo, lotRepo)

	mux := http.NewServeMux()

	// ヘルスチェック
	mux.HandleFunc("GET /health", func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		w.Write([]byte(`{"status":"ok"}`))
	})

	// ロットAPI
	mux.HandleFunc("POST /api/v1/lots", lotHandler.Create)
	mux.HandleFunc("GET /api/v1/lots", lotHandler.List)
	mux.HandleFunc("GET /api/v1/lots/{id}", lotHandler.Get)

	// マスタデータAPI
	mux.HandleFunc("GET /api/v1/parts", masterHandler.ListParts)
	mux.HandleFunc("GET /api/v1/inspection-items", masterHandler.ListInspectionItems)
	mux.HandleFunc("GET /api/v1/workers", masterHandler.ListWorkers)

	// 検査セッションAPI（カウンター方式）
	mux.HandleFunc("POST /api/v1/inspection-sessions", inspectionSessionHandler.Start)
	mux.HandleFunc("POST /api/v1/inspection-sessions/{id}/count", inspectionSessionHandler.AddCount)
	mux.HandleFunc("DELETE /api/v1/inspection-sessions/{id}/count", inspectionSessionHandler.Undo)
	mux.HandleFunc("POST /api/v1/inspection-sessions/{id}/finish", inspectionSessionHandler.Finish)

	port := os.Getenv("PORT")
	if port == "" {
		port = "8080"
	}

	log.Printf("Server starting on :%s", port)
	if err := http.ListenAndServe(":"+port, corsMiddleware(mux)); err != nil {
		log.Fatal(err)
	}
}
