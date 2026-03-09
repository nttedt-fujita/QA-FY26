# Docker全体操作コマンド

.PHONY: up down status logs clean

# 全サービス起動（DB + Backend）
up:
	docker compose up -d
	@echo ""
	@echo "✓ サービス起動完了"
	@echo "  - DB: localhost:5432"
	@echo "  - Backend: http://localhost:8080"
	@echo "  - Frontend: cd frontend && npm run dev"

# 全サービス停止
down:
	docker compose down

# 全サービス状態確認
status:
	@echo "=== Docker Compose 状態 ==="
	@docker compose ps
	@echo ""
	@echo "=== ヘルスチェック ==="
	@docker compose exec db pg_isready -U qa_user -d qa_inspection 2>/dev/null && echo "✓ DB: 接続可能" || echo "✗ DB: 接続不可"
	@curl -s http://localhost:8080/api/parts > /dev/null && echo "✓ API: 応答あり" || echo "✗ API: 応答なし"

# 全サービスログ
logs:
	docker compose logs -f

# 全クリーンアップ（ボリューム含む）
clean:
	@echo "⚠️  全データを削除します（DBデータ含む）"
	@read -p "続行しますか？ [y/N] " confirm && [ "$$confirm" = "y" ] || exit 1
	docker compose down -v --remove-orphans
	@echo "クリーンアップ完了"

# イメージ再ビルド
rebuild:
	docker compose build --no-cache
	docker compose up -d
