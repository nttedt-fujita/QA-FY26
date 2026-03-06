package main

import (
	"context"
	"log"
	"net/http"
	"os"

	"github.com/nttedt/qa-inspection-api/internal/handler"
	"github.com/nttedt/qa-inspection-api/internal/repository"
)

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

	port := os.Getenv("PORT")
	if port == "" {
		port = "8080"
	}

	log.Printf("Server starting on :%s", port)
	if err := http.ListenAndServe(":"+port, mux); err != nil {
		log.Fatal(err)
	}
}
