# UBX MONメッセージ仕様

**作成日**: 2026-03-11
**参照PDF**: UBX-22008968 (u-blox F9 HPG 1.32 Interface Description) p.132-134

---

## MON-SPAN (0x0a 0x31) — RFスペクトラム

### 概要

RFスペクトラム解析用メッセージ。基本的なスペクトラムアナライザとして使用。

| 項目 | 値 |
|------|-----|
| Class | 0x0a |
| ID | 0x31 |
| Type | Periodic/polled |
| ペイロード長 | 4 + numRfBlocks × 272 バイト |

### 特徴

- **用途**: RFパスごとのスペクトラム表示
- **精度**: 比較分析用（絶対値の精度は高くない）
- **注意**: PGAゲインはスペクトラムデータに含まれない（別フィールド）

### ペイロード構造

**ヘッダ（4バイト）**:

| Offset | Type | Name | 説明 |
|--------|------|------|------|
| 0 | U1 | version | メッセージバージョン (0x00) |
| 1 | U1 | numRfBlocks | RFブロック数 |
| 2 | U1[2] | reserved0 | 予約 |

**繰り返しグループ（numRfBlocks回、各272バイト）**:

| Offset | Type | Name | Unit | 説明 |
|--------|------|------|------|------|
| 4 + n×272 | U1[256] | spectrum | dB | **スペクトラムデータ（256点）** |
| 260 + n×272 | U4 | span | Hz | 周波数スパン |
| 264 + n×272 | U4 | res | Hz | 周波数分解能 |
| 268 + n×272 | U4 | center | Hz | 中心周波数 |
| 272 + n×272 | U1 | pga | dB | PGAゲイン |
| 273 + n×272 | U1[3] | reserved1 | - | 予約 |

### 中心周波数計算式

```
f(i) = center + span × (i - 127) / 256
```

- i: ビンインデックス (0〜255)
- f(i): ビンiの中心周波数 (Hz)

### 128MHz/256点の場合

| 項目 | 値 |
|------|-----|
| span | 128,000,000 Hz |
| res | 500,000 Hz (500kHz) |

---

## MON-RF (0x0a 0x38) — RFステータス

### 概要

RF入力のステータス情報。ジャミング検出に使用。

| 項目 | 値 |
|------|-----|
| Class | 0x0a |
| ID | 0x38 |
| Type | Periodic/polled |

### 主要フィールド

| フィールド | 説明 |
|-----------|------|
| jammingState | ジャミング状態（0=unknown, 1=ok, 2=warning, 3=critical） |
| noisePerMS | ノイズレベル |
| agcCnt | AGCカウンタ |

---

## 参照

- UBX-22008968 p.132 (MON-RF)
- UBX-22008968 p.134 (MON-SPAN)
- [08-ubx-protocol-index.md](08-ubx-protocol-index.md)
