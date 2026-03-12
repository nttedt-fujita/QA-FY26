#!/bin/bash
# 統合APIと同じ順序で単発APIを呼び、時間を計測

echo "=== 単発API連続呼び出しテスト ==="
echo ""

echo "1. NAV-SAT"
time curl -s http://localhost:8080/api/nav-sat | jq -c '.stats'
sleep 0.1

echo ""
echo "2. NAV-STATUS"
time curl -s http://localhost:8080/api/nav-status | jq -c '{gps_fix, msss}'
sleep 0.1

echo ""
echo "3. NAV-SIG"
time curl -s http://localhost:8080/api/nav-sig | jq -c '.stats'
sleep 0.1

echo ""
echo "4. MON-SPAN"
time curl -s http://localhost:8080/api/mon-span | jq -c '.blocks | length'

echo ""
echo "=== 完了 ==="
