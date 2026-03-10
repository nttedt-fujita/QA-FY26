# CFG-RATE / CFG-PRT 仕様書

**元PDF**: u-blox F9 HPG 1.32 Interface Description (UBX-22008968)
**抽出日**: 2026-03-10 Session 78

---

## 1. UBX-CFG-RATE (0x06 0x08)

**用途**: ナビゲーション/測定レート設定
**タイプ**: Get/set
**ペイロード長**: 6バイト
**備考**: Protocol versions > 23.01では非推奨（UBX-CFG-VALSET等を使用）

### ペイロード構造

| Offset | 型 | 名前 | スケール | 単位 | 説明 |
|--------|-----|------|---------|------|------|
| 0 | U2 | measRate | - | ms | 測定間隔（例: 100ms→10Hz, 1000ms→1Hz）最小25ms |
| 2 | U2 | navRate | - | cycles | 測定に対するナビゲーション比率（最大127）|
| 4 | U2 | timeRef | - | - | 時刻基準（下記参照） |

### timeRef値

| 値 | 意味 |
|----|------|
| 0 | UTC time |
| 1 | GPS time |
| 2 | GLONASS time |
| 3 | BeiDou time |
| 4 | Galileo time |
| 5 | NavIC time (protocol >= 29.00) |

### 重要な制約

- measRateは25ms以上（protocol < 24.00では50ms以上）
- navRateは最大127
- 更新レートは消費電力に直接影響
- パワーセーブモード時は設定値と異なる場合あり

---

## 2. UBX-CFG-PRT (0x06 0x00)

**用途**: ポート設定
**タイプ**: Get/set (Poll/Set)
**備考**: Protocol versions > 23.01では非推奨

### 2.1 Poll Request

**ペイロード長**: 1バイト

| Offset | 型 | 名前 | 説明 |
|--------|-----|------|------|
| 0 | U1 | portID | ポート識別子 |

### 2.2 UART Port Configuration

**ペイロード長**: 20バイト

| Offset | 型 | 名前 | 説明 |
|--------|-----|------|------|
| 0 | U1 | portID | ポート識別子（UART: 1, 2） |
| 1 | U1 | reserved0 | 予約 |
| 2 | X2 | txReady | TX ready PIN設定 |
| 4 | X4 | mode | UARTモード（下記参照） |
| 8 | U4 | baudRate | ボーレート (bits/s) |
| 12 | X2 | inProtoMask | 入力プロトコルマスク |
| 14 | X2 | outProtoMask | 出力プロトコルマスク |
| 16 | X2 | flags | フラグ |
| 18 | U1[2] | reserved1 | 予約 |

### 2.3 USB Port Configuration

**ペイロード長**: 20バイト
**portID**: 3

| Offset | 型 | 名前 | 説明 |
|--------|-----|------|------|
| 0 | U1 | portID | 3 (USB) |
| 1 | U1 | reserved0 | 予約 |
| 2 | X2 | txReady | TX ready PIN設定 |
| 4 | U1[8] | reserved1 | 予約 |
| 12 | X2 | inProtoMask | 入力プロトコルマスク |
| 14 | X2 | outProtoMask | 出力プロトコルマスク |
| 16 | U1[2] | reserved2 | 予約 |
| 18 | U1[2] | reserved3 | 予約 |

### portID値

| 値 | 意味 |
|----|------|
| 0 | I2C (DDC) |
| 1 | UART1 |
| 2 | UART2 |
| 3 | USB |
| 4 | SPI |

### mode（UARTのみ）

| ビット | 名前 | 説明 |
|--------|------|------|
| 7-6 | charLen | 文字長 (11=8bit) |
| 11-9 | parity | パリティ (000=Even, 001=Odd, 10X=なし) |
| 13-12 | nStopBits | ストップビット (00=1, 01=1.5, 10=2, 11=0.5) |

### inProtoMask / outProtoMask

| ビット | 名前 | 説明 |
|--------|------|------|
| 0 | ubx | UBXプロトコル |
| 1 | nmea | NMEAプロトコル |
| 2 | rtcm (in only) | RTCM2プロトコル |
| 5 | rtcm3 | RTCM3プロトコル |

---

## GNSS評価ツールでの用途

### CFG-RATE
- **受入検査での確認**: 出力レートが期待値（例: 1Hz）に設定されているか
- **フィールド**: measRate, navRate, timeRef

### CFG-PRT
- **受入検査での確認**: USBポートの設定が正しいか
- **フィールド**: portID(=3), inProtoMask, outProtoMask
- **USBポートでの期待値**: UBX有効、NMEA有効

---

## パーサー実装の考慮事項

### CFG-RATE
- シンプルな固定長（6バイト）
- スケール変換不要

### CFG-PRT
- portIDによってペイロード構造が異なる
- GNSS評価ツールでは**USBポート（portID=3）のみ**をサポートすれば十分
- UART/SPI/I2Cの詳細なmode解析は不要

---

*作成: 2026-03-10 Session 78*
