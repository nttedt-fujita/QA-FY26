# Session 209 計画

**目的**: 古い機のFWバージョン取得問題の正確な状況把握

**前提**: Session 208でリトライロジックを追加したが、根本原因が不明のまま

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | message-scan APIで定期出力の内容を確認 | `backend/src/web/message_scan_api.rs` |
| 2 | wait_for_ackの動作を確認（本当にACK-ACK受信しているか） | `backend/src/device/manager.rs` |
| 3 | 各デバイスで何が起きているか段階的に確認 | - |
| 4 | バッファ空待ちアプローチの検討（Session 168参照） | `sessions/session-history/session-161-170.md` |

---

## 確認手順

### 1. message-scan APIで定期出力確認

古い機を接続して、以下を実行：
```bash
curl http://localhost:8080/api/devices/%2Fdev%2FttyUSB0/message-scan
```

- 何のメッセージが出力されているか確認
- 定期出力が本当に止まっているか確認

### 2. wait_for_ackの問題確認

ログを見ると「ACK/NAKでない応答」が来ても`Ok(true)`を返している。
- これが「ACK-ACK受信」と誤報告する原因
- 修正が必要か検討

### 3. バッファ空待ちの検討

Session 168で実装した「バッファ空待ち」：
- gnss_state_api.rsで実装済み
- inspection/engine.rsにも同様のアプローチが必要か検討

---

## 参照

- [Session 202: message-scan API実装](../session-history/session-201-210.md)
- [Session 168: バッファ空待ち実装](../session-history/session-161-170.md)
- [Session 208 summary](../session208/session-summary.md)
