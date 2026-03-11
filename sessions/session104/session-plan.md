# Session 104 計画

**目的**: NMEA ON後のACK待ち追加（残りの1%エラー対策）

---

## 背景

Session 103で100回以上テスト:
- 99%以上Pass
- 1回だけFwVersionエラー

**原因**: NMEA ON送信後にACK-ACKを待っていない
- 前回検査のNMEA ONのACK-ACKが次の検査で遅れて届く
- Connectivityの応答として処理され、以降1つずつズレる

---

## やること

### 1. NMEA ON後のACK待ち追加

engine.rs の修正:
```rust
// 現在
if let Err(e) = manager.send_ubx(&nmea_on_msg) {
    warn!("NMEA ON送信エラー: {}", e);
} else {
    debug!("NMEA ON 送信完了");
}

// 修正後
if let Err(e) = manager.send_ubx(&nmea_on_msg) {
    warn!("NMEA ON送信エラー: {}", e);
} else {
    match manager.wait_for_ack(0x06, 0x8A, std::time::Duration::from_millis(500)) {
        Ok(true) => info!("[NMEA制御] ACK-ACK受信、NMEA ON適用完了"),
        Ok(false) => warn!("[NMEA制御] ACK-NAK受信"),
        Err(e) => warn!("[NMEA制御] ACK待機エラー: {}", e),
    }
}
```

### 2. 実機テスト

連続100回以上テストして0%エラーを確認

---

## 参照資料

- [Session 103 サマリー](../session103/session-summary.md)
- [engine.rs](../../prototype/m1-gnss/backend/src/inspection/engine.rs)

---

*計画作成: 2026-03-11 Session 103終了時*
