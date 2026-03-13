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

# ====================
# RTKデバッグテスト
# ====================
# 使い方:
#   1. ntrip.conf を作成（cp ntrip.conf.example ntrip.conf）
#   2. ターミナル1: make rtk-log
#   3. ターミナル2: make rtk-start → make rtk-poll → make rtk-stop

.PHONY: rtk-log rtk-start rtk-poll rtk-stop

# 設定ファイル読み込み
-include ntrip.conf

# ログファイル
RTK_LOG := /tmp/m1-gnss-rtk-debug.log

# バックエンドログ監視（別ターミナルで実行）
rtk-log:
	@echo "ログ監視開始: $(RTK_LOG)"
	@echo "Ctrl+C で終了"
	@touch $(RTK_LOG)
	tail -f $(RTK_LOG) | grep --line-buffered -E '\[NTRIP|\[GNSS-STATE'

# RTKテスト開始（バックエンド起動 + デバイス接続 + NTRIP接続）
rtk-start:
	@echo "=========================================="
	@echo "  RTKテスト開始"
	@echo "=========================================="
	@if [ -z "$(NTRIP_CASTER)" ]; then \
		echo "❌ ntrip.conf が見つかりません"; \
		echo "  cp ntrip.conf.example ntrip.conf"; \
		echo "  vim ntrip.conf"; \
		exit 1; \
	fi
	@echo "[1/3] バックエンド起動..."
	@pkill -f "target/debug/m1-gnss" 2>/dev/null || true
	@cd backend && RUST_LOG=debug cargo run > $(RTK_LOG) 2>&1 &
	@sleep 3
	@curl -s $(API_URL)/health > /dev/null && echo "  ✅ バックエンド起動完了" || (echo "  ❌ 起動失敗"; exit 1)
	@echo ""
	@echo "[2/3] デバイス接続..."
	@curl -s -X POST "$(API_URL)/api/devices/%2Fdev%2FttyUSB0/connect" | jq .
	@echo ""
	@echo "[3/3] NTRIP接続..."
	@curl -s -X POST $(API_URL)/api/ntrip/connect \
		-H "Content-Type: application/json" \
		-d '{"caster_url":"$(NTRIP_CASTER)","port":$(NTRIP_PORT),"mountpoint":"$(NTRIP_MOUNT)","username":"$(NTRIP_USER)","password":"$(NTRIP_PASS)"}' | jq .
	@echo ""
	@echo "✅ RTKテスト準備完了"
	@echo "  ログ監視: make rtk-log（別ターミナル）"
	@echo "  ポーリング: make rtk-poll"
	@echo "  終了: make rtk-stop"

# gnss-stateポーリング（5回）
rtk-poll:
	@echo "=========================================="
	@echo "  gnss-state 5回ポーリング"
	@echo "=========================================="
	@for i in 1 2 3 4 5; do \
		echo ""; \
		echo "[$$i/5] gnss-state呼び出し..."; \
		start=$$(date +%s%3N); \
		result=$$(curl -s $(API_URL)/api/gnss-state); \
		end=$$(date +%s%3N); \
		elapsed=$$((end - start)); \
		errors=$$(echo "$$result" | jq -r '.errors | length'); \
		carr=$$(echo "$$result" | jq -r '.nav_pvt.carr_soln // "N/A"'); \
		if [ "$$errors" -gt 0 ]; then \
			echo "  ⚠️ $${elapsed}ms エラー:"; \
			echo "$$result" | jq '.errors'; \
		else \
			case "$$carr" in \
				2) rtk="Fixed" ;; \
				1) rtk="Float" ;; \
				0) rtk="None" ;; \
				*) rtk="$$carr" ;; \
			esac; \
			echo "  ✅ $${elapsed}ms RTK: $$rtk"; \
		fi; \
		sleep 1; \
	done
	@echo ""
	@echo "✅ ポーリング完了"

# RTKテスト終了（NTRIP切断 + バックエンド停止）
rtk-stop:
	@echo "=========================================="
	@echo "  RTKテスト終了"
	@echo "=========================================="
	@echo "[1/2] NTRIP切断..."
	@curl -s -X POST $(API_URL)/api/ntrip/disconnect | jq . || true
	@echo ""
	@echo "[2/2] バックエンド停止..."
	@pkill -f "target/debug/m1-gnss" 2>/dev/null && echo "  ✅ 停止完了" || echo "  (既に停止済み)"
	@echo ""
	@echo "✅ RTKテスト終了"
	@echo "  ログ: $(RTK_LOG)"

# ====================
# NTRIP API（個別操作）
# ====================

# NTRIP設定（環境変数で指定）
# export NTRIP_CASTER=ntrip.example.jp
# export NTRIP_PORT=2101
# export NTRIP_MOUNT=MOUNTPOINT
# export NTRIP_USER=username
# export NTRIP_PASS=password

.PHONY: ntrip-connect ntrip-status ntrip-disconnect gnss-state ntrip-test

# NTRIP接続
ntrip-connect:
ifndef NTRIP_CASTER
	@echo "環境変数を設定してください:"
	@echo "  export NTRIP_CASTER=ntrip.example.jp"
	@echo "  export NTRIP_PORT=2101"
	@echo "  export NTRIP_MOUNT=MOUNTPOINT"
	@echo "  export NTRIP_USER=username"
	@echo "  export NTRIP_PASS=password"
else
	@echo "NTRIP接続開始: $(NTRIP_CASTER):$(NTRIP_PORT)/$(NTRIP_MOUNT)"
	@curl -s -X POST $(API_URL)/api/ntrip/connect \
		-H "Content-Type: application/json" \
		-d '{"caster_url":"$(NTRIP_CASTER)","port":$(NTRIP_PORT),"mountpoint":"$(NTRIP_MOUNT)","username":"$(NTRIP_USER)","password":"$(NTRIP_PASS)"}' | jq .
endif

# NTRIP状態確認
ntrip-status:
	@curl -s $(API_URL)/api/ntrip/status | jq .

# NTRIP切断
ntrip-disconnect:
	@curl -s -X POST $(API_URL)/api/ntrip/disconnect | jq .

# ====================
# 統合API
# ====================

# GNSS状態取得（統合API）
gnss-state:
	@curl -s $(API_URL)/api/gnss-state | jq .

# ====================
# RTK統合テスト
# ====================

# RTK機能の一貫テスト（NTRIP接続 → gnss-state繰り返し → 切断）
# 使い方: make rtk-test DURATION=30 INTERVAL=1
.PHONY: rtk-test
rtk-test:
	@DURATION=$(DURATION) INTERVAL=$(INTERVAL) ./tools/test-rtk-flow.sh

DURATION := 30
INTERVAL := 1

# ====================
# NTRIP + UBXポーリング テスト
# ====================

# NTRIP接続中にUBXポーリングをテスト
# 手順: ntrip-connect → ntrip-test → ntrip-disconnect
ntrip-test:
	@echo "=========================================="
	@echo "  NTRIP + UBXポーリング テスト"
	@echo "=========================================="
	@echo ""
	@echo "[1/4] NTRIP状態確認..."
	@ntrip_state=$$(curl -s $(API_URL)/api/ntrip/status | jq -r '.state'); \
	if [ "$$ntrip_state" != "Connected" ]; then \
		echo "  ❌ NTRIP未接続（状態: $$ntrip_state）"; \
		echo "  先に make ntrip-connect を実行してください"; \
		exit 1; \
	fi; \
	echo "  ✅ NTRIP接続済み"
	@echo ""
	@echo "[2/4] 統合API呼び出し（1回目）..."
	@start_time=$$(date +%s%3N); \
	result1=$$(curl -s $(API_URL)/api/gnss-state); \
	end_time=$$(date +%s%3N); \
	elapsed=$$((end_time - start_time)); \
	errors=$$(echo "$$result1" | jq -r '.errors | length'); \
	echo "  応答時間: $${elapsed}ms"; \
	if [ "$$errors" -gt 0 ]; then \
		echo "  ⚠️ エラーあり:"; \
		echo "$$result1" | jq '.errors'; \
	else \
		echo "  ✅ 成功（エラーなし）"; \
	fi
	@echo ""
	@echo "[3/4] 2秒待機..."
	@sleep 2
	@echo ""
	@echo "[4/4] 統合API呼び出し（2回目）..."
	@start_time=$$(date +%s%3N); \
	result2=$$(curl -s $(API_URL)/api/gnss-state); \
	end_time=$$(date +%s%3N); \
	elapsed=$$((end_time - start_time)); \
	errors=$$(echo "$$result2" | jq -r '.errors | length'); \
	echo "  応答時間: $${elapsed}ms"; \
	if [ "$$errors" -gt 0 ]; then \
		echo "  ⚠️ エラーあり:"; \
		echo "$$result2" | jq '.errors'; \
	else \
		echo "  ✅ 成功（エラーなし）"; \
	fi
	@echo ""
	@echo "=========================================="
	@echo "  テスト完了"
	@echo "=========================================="
