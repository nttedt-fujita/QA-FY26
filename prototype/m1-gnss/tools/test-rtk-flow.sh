#!/bin/bash
# RTK機能の一貫テスト
# NTRIP接続中にgnss-stateを繰り返し呼び出し、ログを保存

set -e

API_URL="http://localhost:8080"
LOG_DIR="/tmp/rtk-test-$(date +%Y%m%d-%H%M%S)"
mkdir -p "$LOG_DIR"

echo "=========================================="
echo "  RTK機能テスト"
echo "  ログ出力: $LOG_DIR"
echo "=========================================="
echo ""

# 環境変数チェック
if [ -z "$NTRIP_CASTER" ]; then
    echo "❌ 環境変数が設定されていません:"
    echo "  export NTRIP_CASTER=ntrip.example.jp"
    echo "  export NTRIP_PORT=2101"
    echo "  export NTRIP_MOUNT=MOUNTPOINT"
    echo "  export NTRIP_USER=username"
    echo "  export NTRIP_PASS=password"
    exit 1
fi

# テスト設定
DURATION=${DURATION:-30}  # 検査時間（秒）
INTERVAL=${INTERVAL:-1}   # ポーリング間隔（秒）

echo "[設定]"
echo "  検査時間: ${DURATION}秒"
echo "  ポーリング間隔: ${INTERVAL}秒"
echo "  NTRIPキャスター: $NTRIP_CASTER:$NTRIP_PORT/$NTRIP_MOUNT"
echo ""

# Step 1: デバイス接続確認
echo "[1/5] デバイス接続確認..."
device_resp=$(curl -s "$API_URL/api/devices")
echo "$device_resp" | tee "$LOG_DIR/01-devices.json" | jq .
connected=$(echo "$device_resp" | jq -r '.devices[] | select(.is_connected == true) | .path' | head -1)
if [ -z "$connected" ]; then
    echo "  ⚠️ 接続済みデバイスなし。接続します..."
    connect_resp=$(curl -s -X POST "$API_URL/api/devices/%2Fdev%2FttyUSB0/connect")
    echo "$connect_resp" | tee "$LOG_DIR/01-connect.json" | jq .
    sleep 1
else
    echo "  ✅ デバイス接続済み: $connected"
fi
echo ""

# Step 2: NTRIP接続前のgnss-state取得（ベースライン）
echo "[2/5] NTRIP接続前のgnss-state取得（ベースライン）..."
baseline_resp=$(curl -s "$API_URL/api/gnss-state")
echo "$baseline_resp" | tee "$LOG_DIR/02-baseline-gnss-state.json" | jq .
baseline_errors=$(echo "$baseline_resp" | jq -r '.errors | length')
if [ "$baseline_errors" -gt 0 ]; then
    echo "  ⚠️ ベースラインでエラーあり（続行）"
else
    echo "  ✅ ベースラインOK"
fi
echo ""

# Step 3: NTRIP接続
echo "[3/5] NTRIP接続..."
ntrip_connect_resp=$(curl -s -X POST "$API_URL/api/ntrip/connect" \
    -H "Content-Type: application/json" \
    -d "{\"caster_url\":\"$NTRIP_CASTER\",\"port\":$NTRIP_PORT,\"mountpoint\":\"$NTRIP_MOUNT\",\"username\":\"$NTRIP_USER\",\"password\":\"$NTRIP_PASS\"}")
echo "$ntrip_connect_resp" | tee "$LOG_DIR/03-ntrip-connect.json" | jq .
sleep 2

ntrip_status=$(curl -s "$API_URL/api/ntrip/status")
echo "$ntrip_status" | tee "$LOG_DIR/03-ntrip-status.json" | jq .
state=$(echo "$ntrip_status" | jq -r '.state')
if [ "$state" != "Connected" ]; then
    echo "  ❌ NTRIP接続失敗（状態: $state）"
    exit 1
fi
echo "  ✅ NTRIP接続成功"
echo ""

# Step 4: gnss-state繰り返し呼び出し
echo "[4/5] gnss-state繰り返し呼び出し（${DURATION}秒間）..."
echo "  ログ: $LOG_DIR/04-polling-*.json"
echo ""

success_count=0
error_count=0
total_count=$((DURATION / INTERVAL))

for i in $(seq 1 $total_count); do
    start_ms=$(date +%s%3N)
    resp=$(curl -s "$API_URL/api/gnss-state")
    end_ms=$(date +%s%3N)
    elapsed=$((end_ms - start_ms))

    echo "$resp" > "$LOG_DIR/04-polling-$(printf '%03d' $i).json"

    errors=$(echo "$resp" | jq -r '.errors | length')
    if [ "$errors" -gt 0 ]; then
        error_count=$((error_count + 1))
        error_list=$(echo "$resp" | jq -r '.errors | join(", ")')
        printf "\r[%3d/%d] ❌ ${elapsed}ms エラー: %s\n" "$i" "$total_count" "$error_list"
    else
        success_count=$((success_count + 1))
        # RTK状態を表示
        carr_soln=$(echo "$resp" | jq -r '.nav_pvt.carr_soln // "N/A"')
        case "$carr_soln" in
            2) rtk_status="Fixed" ;;
            1) rtk_status="Float" ;;
            0) rtk_status="No RTK" ;;
            *) rtk_status="$carr_soln" ;;
        esac
        printf "\r[%3d/%d] ✅ ${elapsed}ms RTK: %s" "$i" "$total_count" "$rtk_status"
    fi

    sleep "$INTERVAL"
done
echo ""
echo ""

# Step 5: NTRIP切断
echo "[5/5] NTRIP切断..."
ntrip_disconnect_resp=$(curl -s -X POST "$API_URL/api/ntrip/disconnect")
echo "$ntrip_disconnect_resp" | tee "$LOG_DIR/05-ntrip-disconnect.json" | jq .
echo ""

# 結果サマリー
echo "=========================================="
echo "  テスト完了"
echo "=========================================="
echo "  成功: $success_count / $total_count"
echo "  失敗: $error_count / $total_count"
echo "  ログ: $LOG_DIR"
echo ""

if [ "$error_count" -gt 0 ]; then
    echo "❌ エラーあり。ログを確認してください:"
    echo "  ls $LOG_DIR/04-polling-*.json"
    echo "  cat $LOG_DIR/04-polling-*.json | jq 'select(.errors | length > 0)'"
    exit 1
else
    echo "✅ 全てのポーリングが成功しました"
fi
