# Backend操作コマンド
# Rust + Actix-web

.PHONY: dev-backend test test-verbose build-backend clean-backend

# バックエンド起動（フォアグラウンド、デバッグログ付き）
dev-backend:
	cd backend && RUST_LOG=debug cargo run

# バックエンド起動（バックグラウンド）
dev-backend-bg:
	@echo "バックエンド起動中..."
	cd backend && RUST_LOG=debug cargo run > /tmp/m1-gnss-backend.log 2>&1 &
	@sleep 3
	@echo "起動完了: http://localhost:8080"
	@echo "ログ: tail -f /tmp/m1-gnss-backend.log"

# テスト実行
test:
	cd backend && cargo test

# テスト実行（出力あり）
test-verbose:
	cd backend && cargo test -- --nocapture

# リリースビルド
build-backend:
	cd backend && cargo build --release

# クリーン
clean-backend:
	cd backend && cargo clean
