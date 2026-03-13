# NTRIP/RTK仕様調査結果

---
作成: Session 156 (2026-03-13)
更新: Session 157 (2026-03-13)
状態: 完了
出典:
  - docs/missions/m1-sensor-evaluation/gnss/20-ntrip-rtk-implementation.md
  - docs/missions/m1-sensor-evaluation/gnss/21-ntrip-protocol-spec.md
  - docs/missions/m1-sensor-evaluation/gnss/22-rtk-configuration.md
  - docs/missions/m1-sensor-evaluation/gnss/32-cfg-msgout-periodic-output.md
  - sessions/session156/ublox-mon-comms.md（PDF抽出）
---

## 確認項目と回答

### 1. ZED-F9Pは全二重通信か

**判断: 並行動作可能（確定）**

**間接的証拠**:
- UBX-MON-COMMSでtxPending/rxPendingが独立フィールドとして存在（ublox-mon-comms.md p.126-127）
- 送信バッファと受信バッファが別々に管理されている
- CFG-UARTxINPROT/CFG-UARTxOUTPROTが独立した設定項目（Session 157 追加調査）
- 入力プロトコル（RTCM3X）と出力プロトコル（UBX, NMEA）を独立して設定可能

**追加調査結果（Session 157）**:
- u-blox Interface Description p.270-274を確認
- 「full duplex」という直接的記述は見つからなかったが、設計上全二重を前提としている
- CFG-UART1INPROT-RTCM3X（入力有効化）とCFG-UART1OUTPROT-UBX（出力有効化）が独立設定

**根拠**:
- RTKモード時、RTCM入力を受けながら同時にUBX-NAV-PVT、NAV-SAT、NAV-SIG、NAV-STATUS等を出力（22-rtk-configuration.md p.19-20）
- NTRIPキャスターからの low-bandwidth streaming: 50-500 Bytes/sec（21-ntrip-protocol-spec.md）
- ZED-F9Pは「RTCMデータを受信すると自動的にRTK floatモードに移行」（20-ntrip-rtk-implementation.md）

**ZED-F9P側の動作**:
- RTCM入力 → 内部でRTK演算に使用
- UBX出力 → 定期出力設定に従って出力
- 入力と出力は独立して動作

### 2. RTCMデータの流量

**判断: 50-500 Bytes/sec（低帯域）**

**根拠**:
- NTRIPキャスター仕様: "low-bandwidth streaming data (from 50 up to 500 Bytes/sec per stream)"（21-ntrip-protocol-spec.md）
- Source-Tableの`<bitrate>`フィールド: 500～5000 bps

**参考**: USB通信は12 Mbps、シリアル通信は115200 bps（デフォルト）なので、RTCM流量は通信帯域に対して十分に小さい。

### 3. DeviceManagerのロック

**判断: 要コード確認（Session 157で実施）**

**確認すべきこと**:
- `device/manager.rs` のread/write実装
- Mutexの粒度（ポート全体 or read/write別）
- `ntrip_api.rs` のRTCM転送ループがどのようにロックを取るか

### 4. 定期出力の設定

**判断: NTRIP接続前後で設定変更は不要**

**根拠**:
- CFG-MSGOUTで設定した定期出力は、RTCM入力の有無に関係なく動作（32-cfg-msgout-periodic-output.md）
- ZED-F9Pは「RTCMデータを受信すると自動的に」RTK処理を開始（追加設定不要）
- 定期出力とRTCM入力は独立した機能

---

## 判断の分岐（更新）

### 確認結果A: 並行動作可能（仕様上はこちら）

- ZED-F9P側: RTCM入力と定期出力は並行動作できる
- **残課題**: DeviceManagerのロックがボトルネックにならないか確認（Session 157）

### 確認結果B: シーケンシャル必須

- DeviceManagerのロックがread/writeを相互ブロックする場合のみ該当
- その場合: RTCM転送中はUBXポーリング/定期出力を停止する設計変更が必要

---

## 次のアクション

1. **Session 157**: DeviceManagerのコード確認
   - `device/manager.rs` のread/write実装
   - `ntrip_api.rs` のRTCM転送ループ
   - ロック競合の可能性を評価

2. **判断後**: 必要に応じてADR-013を作成

---

## 参照した仕様書

| # | ファイル | 内容 |
|---|----------|------|
| 1 | [20-ntrip-rtk-implementation.md](../../docs/missions/m1-sensor-evaluation/gnss/20-ntrip-rtk-implementation.md) | NTRIP/RTK実装方針 |
| 2 | [21-ntrip-protocol-spec.md](../../docs/missions/m1-sensor-evaluation/gnss/21-ntrip-protocol-spec.md) | NTRIPプロトコル仕様 |
| 3 | [22-rtk-configuration.md](../../docs/missions/m1-sensor-evaluation/gnss/22-rtk-configuration.md) | RTK設定（Integration Manual抽出） |
| 4 | [32-cfg-msgout-periodic-output.md](../../docs/missions/m1-sensor-evaluation/gnss/32-cfg-msgout-periodic-output.md) | 定期出力設定 |
