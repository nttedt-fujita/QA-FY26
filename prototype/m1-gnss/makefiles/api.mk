# API呼び出しコマンド（デバッグ用）

.PHONY: devices connect disconnect lots create-lot inspect inspections health

# ベースURL
API_URL := http://localhost:8080

# ====================
# 装置API
# ====================

# 装置一覧
devices:
	@curl -s $(API_URL)/api/devices | jq .

# 装置接続（/dev/ttyUSB0）
connect:
	@curl -s -X POST "$(API_URL)/api/devices/%2Fdev%2FttyUSB0/connect" | jq .

# 装置切断
disconnect:
	@curl -s -X POST "$(API_URL)/api/devices/%2Fdev%2FttyUSB0/disconnect" | jq .

# ====================
# ロットAPI
# ====================

# ロット一覧
lots:
	@curl -s $(API_URL)/api/lots | jq .

# ロット作成（テスト用）
create-lot:
	@curl -s -X POST $(API_URL)/api/lots \
		-H "Content-Type: application/json" \
		-d '{"lot_number": "TEST-001", "output_rate": 1, "uart1_enabled": true, "uart2_enabled": false, "note": "テスト用"}' | jq .

# ====================
# 検査API
# ====================

# 検査実行（引数: LOT_ID）
# 使い方: make inspect LOT_ID=1
inspect:
ifndef LOT_ID
	@echo "使い方: make inspect LOT_ID=1"
else
	@curl -s -X POST "$(API_URL)/api/inspections" \
		-H "Content-Type: application/json" \
		-d '{"lot_id": $(LOT_ID)}' | jq .
endif

# 検査履歴
inspections:
	@curl -s $(API_URL)/api/inspections | jq .

# ====================
# ヘルスチェック
# ====================

health:
	@curl -s $(API_URL)/health | jq .

# ====================
# 連続テスト
# ====================

# 連続テスト（デフォルト100回）
# 使い方: make stress-test [COUNT=100] [LOT_ID=1]
LOG_DIR := /tmp/gnss-stress-test
COUNT := 100
LOT_ID := 1

.PHONY: stress-test stress-clean

stress-test:
	@mkdir -p $(LOG_DIR)
	@echo "=========================================="
	@echo "  連続テスト開始: $(COUNT)回"
	@echo "  LOT_ID: $(LOT_ID)"
	@echo "  ログ出力: $(LOG_DIR)/"
	@echo "=========================================="
	@pass=0; fail=0; \
	for i in $$(seq 1 $(COUNT)); do \
		result=$$(curl -s -X POST "$(API_URL)/api/inspections" \
			-H "Content-Type: application/json" \
			-d '{"lot_id": $(LOT_ID)}'); \
		verdict=$$(echo "$$result" | jq -r '.overall_result // "Error"'); \
		if [ "$$verdict" = "Pass" ]; then \
			pass=$$((pass + 1)); \
			printf "\r[%3d/%d] Pass: %d, Fail: %d" $$i $(COUNT) $$pass $$fail; \
		else \
			fail=$$((fail + 1)); \
			printf "\n[%3d/%d] *** FAIL/ERROR ***\n" $$i $(COUNT); \
			echo "$$result" | tee "$(LOG_DIR)/error-$$i.json" | jq .; \
		fi; \
		sleep 0.3; \
	done; \
	echo ""; \
	echo "=========================================="; \
	echo "  結果: Pass=$$pass, Fail=$$fail / $(COUNT)"; \
	echo "  エラーログ: $(LOG_DIR)/error-*.json"; \
	echo "=========================================="

# テストログ削除
stress-clean:
	@rm -rf $(LOG_DIR)
	@echo "ログ削除完了: $(LOG_DIR)"
