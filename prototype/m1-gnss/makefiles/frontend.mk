# Frontend操作コマンド
# Next.js + TypeScript

.PHONY: dev-frontend dev-frontend-bg build-frontend clean-frontend install-frontend

# フロントエンド起動（フォアグラウンド）
dev-frontend:
	cd frontend && npm run dev

# フロントエンド起動（バックグラウンド）
dev-frontend-bg:
	@echo "フロントエンド起動中..."
	cd frontend && npm run dev > /tmp/m1-gnss-frontend.log 2>&1 &
	@sleep 2
	@echo "起動完了: http://localhost:3000"
	@echo "ログ: tail -f /tmp/m1-gnss-frontend.log"

# ビルド
build-frontend:
	cd frontend && npm run build

# クリーン
clean-frontend:
	rm -rf frontend/.next

# 依存関係インストール
install-frontend:
	cd frontend && npm install
