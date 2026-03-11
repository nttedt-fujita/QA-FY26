# 開発全体操作コマンド

.PHONY: dev stop-all build clean

# 両方起動（バックグラウンド）
dev: dev-backend-bg dev-frontend-bg
	@echo ""
	@echo "========================================"
	@echo "  起動完了"
	@echo "========================================"
	@echo "  Backend:  http://localhost:8080"
	@echo "  Frontend: http://localhost:3000"
	@echo ""

# 全プロセス停止
stop-all:
	@echo "プロセス停止中..."
	-pkill -f "target/debug/m1-gnss" 2>/dev/null || true
	-pkill -f "target/release/m1-gnss" 2>/dev/null || true
	-pkill -f "next dev" 2>/dev/null || true
	@echo "完了"

# 全体ビルド
build: build-backend build-frontend

# 全クリーン
clean: clean-backend clean-frontend
