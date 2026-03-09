# DB操作コマンド
# PostgreSQL (Docker)

.PHONY: db-up db-down db-status db-logs db-psql db-reset

# DB起動
db-up:
	docker compose up -d db
	@echo "DB起動完了。ヘルスチェック待ち..."
	@docker compose exec db pg_isready -U qa_user -d qa_inspection || true

# DB停止
db-down:
	docker compose stop db

# DB状態確認
db-status:
	@echo "=== コンテナ状態 ==="
	@docker compose ps db
	@echo ""
	@echo "=== DB接続確認 ==="
	@docker compose exec db pg_isready -U qa_user -d qa_inspection 2>/dev/null && echo "✓ 接続可能" || echo "✗ 接続不可"

# DBログ表示
db-logs:
	docker compose logs -f db

# psqlで接続
db-psql:
	docker compose exec db psql -U qa_user -d qa_inspection

# DBリセット（データ削除 + 再作成）
db-reset:
	@echo "⚠️  DBをリセットします（全データ削除）"
	@read -p "続行しますか？ [y/N] " confirm && [ "$$confirm" = "y" ] || exit 1
	docker compose down -v db
	docker compose up -d db
	@echo "DBリセット完了"

# テーブル一覧
db-tables:
	docker compose exec db psql -U qa_user -d qa_inspection -c "\dt"

# レコード数確認
db-counts:
	@echo "=== テーブル別レコード数 ==="
	@docker compose exec db psql -U qa_user -d qa_inspection -c "\
		SELECT 'suppliers' as table_name, COUNT(*) FROM suppliers \
		UNION ALL SELECT 'parts', COUNT(*) FROM parts \
		UNION ALL SELECT 'inspection_items', COUNT(*) FROM inspection_items \
		UNION ALL SELECT 'workers', COUNT(*) FROM workers \
		UNION ALL SELECT 'lots', COUNT(*) FROM lots \
		UNION ALL SELECT 'inspection_records', COUNT(*) FROM inspection_records \
		ORDER BY table_name;"
