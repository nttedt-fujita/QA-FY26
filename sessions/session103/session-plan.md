# Session 103 計画

**目的**: UBX通信タイミング問題の根本解決

---

## 背景

Session 101-102で案A（B5 62同期）と案B（NMEA OFF/ON）を実装したが、エラー率はあまり改善していない。

- 通信疎通（Connectivity）やFWバージョンでエラー発生
- 特に最初の1-2項目でエラーになりやすい傾向

---

## 現状の問題仮説

1. **CFG-VALSETが効く前にpollが送信されている**
   - NMEA OFF送信後の50ms待機では不十分
   - CFG-VALSETのACK-ACKを待つべきかも

2. **drain_buffer後にNMEAが届いている**
   - drain_buffer完了後、poll送信前の50ms待機中にNMEAが来る
   - NMEAがOFFになっていないと解決しない

3. **接続直後のバッファに古いデータがある**
   - 接続時点で既にバッファにデータが溜まっている
   - 最初の検査でそれを読んでしまう

---

## やること

### 案1: CFG-VALSET後のACK確認

```
1. NMEA OFF送信
2. ACK-ACK (B5 62 05 01) を受信するまで待つ
3. 検査開始
```

### 案2: NMEA OFF後の待機時間延長

```
1. NMEA OFF送信
2. 100ms〜200ms待機（現在50ms）
3. 検査開始
```

### 案3: 接続時にバッファ完全クリア

```
1. 接続直後に1秒間drain_buffer
2. その後NMEA OFF
3. 検査開始
```

---

## 調査方法

1. ログでNMEA OFF送信タイミングと最初のpoll送信タイミングを確認
2. ACK-ACKが返ってきているか確認
3. 最初のpoll応答で何が来ているか確認

---

## 参照資料

- [Session 102 サマリー](../session102/session-summary.md)
- [タイミング問題対策計画](../session100/nmea-timing-fix-plan.md)
- [engine.rs](../../prototype/m1-gnss/backend/src/inspection/engine.rs)

---

*計画作成: 2026-03-11 Session 102終了時*
