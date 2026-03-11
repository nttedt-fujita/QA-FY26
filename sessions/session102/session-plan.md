# Session 102 計画

**目的**: UBX通信タイミング問題の対策実装（案B: NMEA OFF/ON）

---

## 背景

Session 101で案A（B5 62同期）を実装したが、タイミングによっては依然として失敗する。
原因: 送ったpollと異なるUBXメッセージ（前回の応答残り）を受信してしまう。

案Bを実装して、検査中はNMEA出力を停止し、ノイズを根本的に排除する。

---

## やること

### 1. cfg_valset.rs 作成

**新規ファイル**: `backend/src/ubx/cfg_valset.rs`

**実装内容**:
- CFG-VALSET メッセージ生成関数
- NMEA OFF: `set_uart1_nmea_output(false)`
- NMEA ON: `set_uart1_nmea_output(true)`

**Key**: CFG-UART1OUTPROT-NMEA (0x10740002)

### 2. engine.rs で NMEA OFF/ON

**変更ファイル**: `backend/src/inspection/engine.rs`

**変更内容**:
1. `run` メソッドの冒頭でNMEAをOFF
2. `run` メソッドの最後でNMEAをON（成功/失敗に関わらず）

### 3. テスト追加

- cfg_valset.rs のユニットテスト
- NMEA OFF/ON のモックテスト

### 4. 実機テストで効果確認

```bash
cd prototype/m1-gnss
make dev-backend    # ターミナル1
make dev-frontend   # ターミナル2
# ブラウザで検査実行、5項目安定Pass確認
```

---

## 期待される結果

- 5項目すべて安定してPass
- タイミング依存の失敗がなくなる
- 検査終了後はNMEA出力が復帰

---

## 参照資料

- [Session 101 サマリー](../session101/session-summary.md)
- [タイミング問題対策計画](../session100/nmea-timing-fix-plan.md)
- [manager.rs](../../prototype/m1-gnss/backend/src/device/manager.rs)
- [engine.rs](../../prototype/m1-gnss/backend/src/inspection/engine.rs)

---

*計画作成: 2026-03-11 Session 101終了時*
