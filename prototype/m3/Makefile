# M3 受入検査DB プロトタイプ
# makefiles/*.mk をインクルード

# makefilesディレクトリ内の全.mkファイルをインクルード
include makefiles/*.mk

# ヘルプ表示（デフォルトターゲット）
.PHONY: help
help:
	@echo "========================================"
	@echo "  M3 受入検査DB プロトタイプ"
	@echo "========================================"
	@echo ""
	@echo "【全体操作】"
	@echo "  make up        - 全サービス起動（DB + Backend）"
	@echo "  make down      - 全サービス停止"
	@echo "  make status    - 状態確認"
	@echo "  make logs      - ログ表示"
	@echo "  make clean     - 全クリーンアップ（データ削除）"
	@echo "  make rebuild   - イメージ再ビルド"
	@echo ""
	@echo "【DB操作】"
	@echo "  make db-up     - DB起動"
	@echo "  make db-down   - DB停止"
	@echo "  make db-status - DB状態確認"
	@echo "  make db-psql   - psqlで接続"
	@echo "  make db-reset  - DBリセット（全データ削除）"
	@echo "  make db-tables - テーブル一覧"
	@echo "  make db-counts - レコード数確認"
	@echo ""
	@echo "【Backend操作】"
	@echo "  make backend-up      - Backend起動"
	@echo "  make backend-down    - Backend停止"
	@echo "  make backend-logs    - Backendログ"
	@echo "  make backend-rebuild - 再ビルド＋起動"
	@echo "  make backend-test    - テスト実行"
	@echo "  make backend-health  - ヘルスチェック"
	@echo ""
	@echo "【Frontend操作】"
	@echo "  make frontend-dev     - 開発サーバー起動"
	@echo "  make frontend-build   - ビルド"
	@echo "  make frontend-install - 依存関係インストール"
	@echo ""
	@echo "【デモ】"
	@echo "  make demo-setup    - デモ環境セットアップ"
	@echo "  make demo-flow     - デモフロー説明表示"
	@echo "  make demo-api-test - APIテスト（curl）"
	@echo ""
