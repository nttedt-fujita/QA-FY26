# M1-GNSS 評価ツール
# makefiles/*.mk をインクルード

include makefiles/*.mk

# ヘルプ表示（デフォルトターゲット）
.PHONY: help
help:
	@echo "========================================"
	@echo "  M1-GNSS 評価ツール"
	@echo "========================================"
	@echo ""
	@echo "【開発サーバー】"
	@echo "  make dev-backend    バックエンド起動（デバッグログ付き）"
	@echo "  make dev-frontend   フロントエンド起動"
	@echo "  make dev            両方起動（バックグラウンド）"
	@echo "  make stop-all       全プロセス停止"
	@echo ""
	@echo "【テスト・ビルド】"
	@echo "  make test           バックエンドテスト"
	@echo "  make test-verbose   テスト（出力あり）"
	@echo "  make build          リリースビルド"
	@echo "  make clean          ビルド成果物削除"
	@echo ""
	@echo "【API呼び出し（デバッグ用）】"
	@echo "  make devices        装置一覧"
	@echo "  make connect        装置接続（/dev/ttyUSB0）"
	@echo "  make disconnect     装置切断"
	@echo "  make lots           ロット一覧"
	@echo "  make create-lot     テストロット作成"
	@echo "  make inspect LOT_ID=1  検査実行"
	@echo "  make inspections    検査履歴"
	@echo ""
	@echo "【Docker（今後対応）】"
	@echo "  make docker-build   イメージビルド"
	@echo "  make docker-run     コンテナ起動（USB対応）"
	@echo ""
