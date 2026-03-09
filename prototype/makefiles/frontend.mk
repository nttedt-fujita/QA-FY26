# Frontend操作コマンド
# Next.js (ローカル実行)

.PHONY: frontend-dev frontend-build frontend-install frontend-lint

# フロントエンド開発サーバー起動
frontend-dev:
	cd frontend && npm run dev

# フロントエンドビルド
frontend-build:
	cd frontend && npm run build

# 依存関係インストール
frontend-install:
	cd frontend && npm install

# Lint実行
frontend-lint:
	cd frontend && npm run lint

# フロントエンド状態確認
frontend-status:
	@echo "=== Node.jsバージョン ==="
	@node --version
	@echo ""
	@echo "=== package.json scripts ==="
	@cd frontend && cat package.json | grep -A 10 '"scripts"'
