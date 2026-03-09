# Backend操作コマンド
# Go API Server (Docker)

.PHONY: backend-up backend-down backend-logs backend-build backend-test backend-rebuild

# バックエンド起動
backend-up:
	docker compose up -d backend
	@echo "Backend起動完了: http://localhost:8080"

# バックエンド停止
backend-down:
	docker compose stop backend

# バックエンドログ
backend-logs:
	docker compose logs -f backend

# バックエンドビルド
backend-build:
	docker compose build backend

# バックエンド再ビルド＋起動
backend-rebuild:
	docker compose up -d --build backend

# バックエンドテスト（ローカル実行）
backend-test:
	cd backend && go test ./...

# バックエンドテスト（Docker経由）
backend-test-docker:
	docker compose -f docker-compose.test.yml up --build --abort-on-container-exit backend-test
	docker compose -f docker-compose.test.yml down -v

# APIヘルスチェック
backend-health:
	@curl -s http://localhost:8080/api/parts > /dev/null && echo "✓ API応答あり" || echo "✗ API応答なし"

# API一覧確認
backend-api-check:
	@echo "=== マスタAPI ==="
	@curl -s http://localhost:8080/api/parts | head -c 200 && echo "..."
	@echo ""
	@curl -s http://localhost:8080/api/inspection-items | head -c 200 && echo "..."
	@echo ""
	@curl -s http://localhost:8080/api/workers | head -c 200 && echo "..."
