#!/bin/bash
# 統合APIを100回呼び出してエラー発生回数を確認

SUCCESS=0
FAIL=0
ERRORS=()

echo "統合API 100回テスト開始..."

for i in $(seq 1 100); do
    result=$(curl -s http://localhost:8080/api/gnss-state | jq -r '.errors | length')

    if [ "$result" = "0" ]; then
        SUCCESS=$((SUCCESS + 1))
    else
        FAIL=$((FAIL + 1))
        error_detail=$(curl -s http://localhost:8080/api/gnss-state | jq -c '.errors')
        ERRORS+=("$i: $error_detail")
    fi

    # 進捗表示（10回ごと）
    if [ $((i % 10)) -eq 0 ]; then
        echo "  $i/100 完了 (成功: $SUCCESS, 失敗: $FAIL)"
    fi
done

echo ""
echo "=========================================="
echo "結果: 成功 $SUCCESS / 100, 失敗 $FAIL / 100"
echo "=========================================="

if [ $FAIL -gt 0 ]; then
    echo ""
    echo "エラー詳細:"
    for e in "${ERRORS[@]}"; do
        echo "  $e"
    done
fi
