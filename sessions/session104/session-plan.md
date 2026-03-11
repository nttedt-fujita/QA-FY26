# Session 104 計画

**目的**: 実機テストでSession 103の効果を確認

---

## 背景

Session 103で以下を実装:
- CFG-VALSET後のACK確認（wait_for_ack）
- 詳細なデバッグログ

---

## やること

### 1. 実機テスト

```bash
cd prototype/m1-gnss/backend
RUST_LOG=debug cargo run
```

5項目全てPassするか確認:
1. 通信疎通（Connectivity）
2. FWバージョン
3. シリアル番号
4. 出力レート
5. ポート設定

### 2. ログ分析

エラーが発生した場合、ログから原因を特定:
- ACK-ACKは受信できているか
- drain_bufferで何が読み捨てられているか
- receive_ubxでUBX前に何のデータがあったか

### 3. 追加対策（必要に応じて）

効果がない場合:
- 案2: NMEA OFF後の待機時間延長（100ms〜200ms）
- 案3: 接続時にバッファ完全クリア（1秒間drain_buffer）

---

## 参照資料

- [Session 103 サマリー](../session103/session-summary.md)
- [engine.rs](../../prototype/m1-gnss/backend/src/inspection/engine.rs)
- [manager.rs](../../prototype/m1-gnss/backend/src/device/manager.rs)

---

*計画作成: 2026-03-11 Session 103終了時*
