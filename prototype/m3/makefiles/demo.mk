# デモ・動作確認コマンド

.PHONY: demo demo-setup demo-flow

# デモ環境セットアップ
demo-setup: up
	@echo ""
	@echo "=== デモ環境セットアップ完了 ==="
	@echo ""
	@echo "次の手順でフロントエンドを起動してください:"
	@echo "  cd frontend && npm run dev"
	@echo ""
	@echo "ブラウザで http://localhost:3000 を開いてください"

# デモフロー説明
demo-flow:
	@echo "========================================"
	@echo "  M3 受入検査DB プロトタイプ デモフロー"
	@echo "========================================"
	@echo ""
	@echo "1. ロット登録画面 (http://localhost:3000)"
	@echo "   - 部品を選択"
	@echo "   - ロットID・数量を入力"
	@echo "   - 「登録して検査開始」をクリック"
	@echo ""
	@echo "2. 検査入力画面 (/inspection)"
	@echo "   - 作業者を選択"
	@echo "   - OK/NG/SKIPボタンで検査記録"
	@echo "   - 取り消しボタンで直前の入力を取り消し"
	@echo "   - 「検査完了」で終了"
	@echo ""
	@echo "3. 検査一覧画面 (/records)"
	@echo "   - フィルターで絞り込み"
	@echo "   - CSVエクスポート"
	@echo ""
	@echo "4. ダッシュボード画面 (/dashboard)"
	@echo "   - KPI表示（総検査数、不良率など）"
	@echo "   - 月別グラフ"
	@echo "   - 検査項目別・サプライヤー別分析"
	@echo ""

# APIテスト（curl）
demo-api-test:
	@echo "=== マスタAPI ==="
	@echo "GET /api/parts"
	@curl -s http://localhost:8080/api/parts | python3 -m json.tool 2>/dev/null || curl -s http://localhost:8080/api/parts
	@echo ""
	@echo "GET /api/inspection-items"
	@curl -s http://localhost:8080/api/inspection-items | python3 -m json.tool 2>/dev/null || curl -s http://localhost:8080/api/inspection-items
	@echo ""
	@echo "GET /api/workers"
	@curl -s http://localhost:8080/api/workers | python3 -m json.tool 2>/dev/null || curl -s http://localhost:8080/api/workers
