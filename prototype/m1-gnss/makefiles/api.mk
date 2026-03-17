# API呼び出しコマンド（デバッグ用）

.PHONY: devices connect disconnect message-scan reset-config set-periodic-output set-periodic-output-flash mon-ver lots create-lot inspect inspections health

# ベースURL
API_URL := http://localhost:8080

# デバイスパス（デフォルト: /dev/ttyUSB0）
# 使い方: make connect DEVICE=/dev/ttyUSB1
DEVICE := /dev/ttyUSB0
DEVICE_ENCODED := $(subst /,%2F,$(DEVICE))

# ====================
# 装置API
# ====================

# 装置一覧
devices:
	@curl -s $(API_URL)/api/devices | jq .

# 装置接続（/dev/ttyUSB0）
connect:
	@curl -s -X POST "$(API_URL)/api/devices/$(DEVICE_ENCODED)/connect" | jq .

# 装置切断
disconnect:
	@curl -s -X POST "$(API_URL)/api/devices/$(DEVICE_ENCODED)/disconnect" | jq .

# メッセージスキャン（定期出力確認）
message-scan:
	@curl -s "$(API_URL)/api/devices/$(DEVICE_ENCODED)/message-scan" | jq .

# 設定リセット（BBR+Flash クリア）
reset-config:
	@curl -s -X POST "$(API_URL)/api/devices/$(DEVICE_ENCODED)/reset-config" | jq .

# 定期出力有効化（テスト用、RAM+BBR+Flashに保存）
set-periodic-output:
	@curl -s -X POST "$(API_URL)/api/devices/$(DEVICE_ENCODED)/set-periodic-output" | jq .

# 定期出力有効化（Flashのみに保存 - Flash搭載確認用）
# ACK → Flash搭載、NAK → Flash非搭載
set-periodic-output-flash:
	@curl -s -X POST "$(API_URL)/api/devices/$(DEVICE_ENCODED)/set-periodic-output?layer=flash" | jq .

# モジュール情報取得（Flash有無確認用）
mon-ver:
	@curl -s "$(API_URL)/api/devices/$(DEVICE_ENCODED)/mon-ver" | jq .

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
#   2. ターミナル1: make dev-backend（ログ表示）
#   3. ターミナル2: make rtk-debug（接続+ポーリング一括）
#
# 個別操作:
#   make rtk-connect    — デバイス接続 + NTRIP接続
#   make rtk-poll       — ポーリング5回
#   make rtk-disconnect — NTRIP切断
#   make rtk-status     — NTRIP状態確認

.PHONY: rtk-debug rtk-connect rtk-poll rtk-disconnect rtk-status

# 設定ファイル読み込み
-include ntrip.conf

# RTKデバッグ一括実行（接続 + ポーリング）
# ※事前に別ターミナルで make dev-backend を実行しておくこと
rtk-debug:
	@echo "=========================================="
	@echo "  RTKデバッグテスト"
	@echo "=========================================="
	@if [ -z "$(NTRIP_CASTER)" ]; then \
		echo "❌ ntrip.conf が見つかりません"; \
		echo "  cp ntrip.conf.example ntrip.conf"; \
		echo "  vim ntrip.conf"; \
		exit 1; \
	fi
	@echo ""
	@echo "[1/3] デバイス接続..."
	@curl -s -X POST "$(API_URL)/api/devices/$(DEVICE_ENCODED)/connect" | jq -r '.message // .error // "接続完了"'
	@echo ""
	@echo "[2/3] NTRIP接続..."
	@curl -s -X POST $(API_URL)/api/ntrip/connect \
		-H "Content-Type: application/json" \
		-d '{"caster_url":"$(NTRIP_CASTER)","port":$(NTRIP_PORT),"mountpoint":"$(NTRIP_MOUNT)","username":"$(NTRIP_USER)","password":"$(NTRIP_PASS)"}' | jq -r '.message // .error // "接続完了"'
	@echo ""
	@echo "[3/3] ポーリング開始..."
	@$(MAKE) -s rtk-poll

# RTK接続（デバイス接続 + NTRIP接続）
rtk-connect:
	@echo "=========================================="
	@echo "  RTK接続"
	@echo "=========================================="
	@if [ -z "$(NTRIP_CASTER)" ]; then \
		echo "❌ ntrip.conf が見つかりません"; \
		exit 1; \
	fi
	@echo "[1/2] デバイス接続..."
	@curl -s -X POST "$(API_URL)/api/devices/$(DEVICE_ENCODED)/connect" | jq .
	@echo ""
	@echo "[2/2] NTRIP接続..."
	@curl -s -X POST $(API_URL)/api/ntrip/connect \
		-H "Content-Type: application/json" \
		-d '{"caster_url":"$(NTRIP_CASTER)","port":$(NTRIP_PORT),"mountpoint":"$(NTRIP_MOUNT)","username":"$(NTRIP_USER)","password":"$(NTRIP_PASS)"}' | jq .
	@echo ""
	@echo "✅ RTK接続完了"
	@echo "  ポーリング: make rtk-poll"
	@echo "  切断: make rtk-disconnect"

# NTRIP切断
rtk-disconnect:
	@echo "NTRIP切断中..."
	@curl -s -X POST $(API_URL)/api/ntrip/disconnect | jq .

# NTRIP状態確認
rtk-status:
	@curl -s $(API_URL)/api/ntrip/status | jq .

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
# 統合API
# ====================

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
