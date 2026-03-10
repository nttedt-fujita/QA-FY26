# CFG-RATE / CFG-PRT 振る舞い記述（TDD Phase 1）

**作成日**: 2026-03-10 Session 78
**ステータス**: Phase 1 完了、Phase 2 以降は次セッションで実施

---

## CFG-RATE (0x06 0x08)

**この機能を使う人（GNSS評価ツール）は:**
- F9Pからの応答バイト列を入力し、測定レート設定を取得することを期待する

### 振る舞い（正常系）

| No | 振る舞い |
|----|----------|
| R1 | 有効なCFG-RATEメッセージをパースすると、measRate/navRate/timeRefが取得できる |
| R2 | measRate=1000ms（1Hz）が正しく読み取れる |
| R3 | measRate=25ms（最小値、40Hz）が正しく読み取れる |
| R4 | navRate=1が正しく読み取れる |
| R5 | navRate=127（最大値）が正しく読み取れる |
| R6 | timeRef=0〜5の各値が正しく読み取れる（UTC/GPS/GLONASS/BeiDou/Galileo/NavIC） |

### 振る舞い（異常系）

| No | 振る舞い |
|----|----------|
| R7 | ヘッダーが0xB5 0x62でない場合、InvalidHeaderエラーを返す |
| R8 | Class/IDが0x06 0x08でない場合、MessageMismatchエラーを返す |
| R9 | チェックサムが不正な場合、ChecksumErrorを返す |
| R10 | ペイロード長が6バイト未満の場合、InsufficientLengthエラーを返す |

---

## CFG-PRT (0x06 0x00) — USBポートのみ

**この機能を使う人（GNSS評価ツール）は:**
- F9PからのUSBポート設定応答を入力し、プロトコル設定を取得することを期待する

### 振る舞い（正常系）

| No | 振る舞い |
|----|----------|
| P1 | 有効なCFG-PRT（USB, portID=3）メッセージをパースすると、portID/inProtoMask/outProtoMaskが取得できる |
| P2 | inProtoMaskのUBXビット(bit0)が正しく読み取れる |
| P3 | inProtoMaskのNMEAビット(bit1)が正しく読み取れる |
| P4 | inProtoMaskのRTCM3ビット(bit5)が正しく読み取れる |
| P5 | outProtoMaskのUBXビット(bit0)が正しく読み取れる |
| P6 | outProtoMaskのNMEAビット(bit1)が正しく読み取れる |
| P7 | outProtoMaskのRTCM3ビット(bit5)が正しく読み取れる |

### 振る舞い（異常系）

| No | 振る舞い |
|----|----------|
| P8 | ヘッダーが0xB5 0x62でない場合、InvalidHeaderエラーを返す |
| P9 | Class/IDが0x06 0x00でない場合、MessageMismatchエラーを返す |
| P10 | チェックサムが不正な場合、ChecksumErrorを返す |
| P11 | ペイロード長が20バイト未満の場合、InsufficientLengthエラーを返す |

### スコープ外（実装しない）

- UART(portID=1,2)のmode解析
- SPI(portID=4)のmode解析
- I2C(portID=0)のmode解析

---

## 次セッションでやること

- TDD Phase 2: テストシナリオリスト作成
- TDD Phase 3: テストコード作成
- TDD Phase 4: 実装（Red → Green）
- TDD Phase 5: リファクタリング

---

## 参照資料

- [cfg-rate-prt-spec.md](cfg-rate-prt-spec.md) — 仕様書（PDFから抽出）
- [cfg-rate-prt-raw.md](cfg-rate-prt-raw.md) — PDFから抽出した生データ
