# UBXメッセージ仕様メモ

**作成日**: 2026-03-11
**セッション**: 106
**参照PDF**: UBX-22008968 (u-blox F9 HPG 1.32 Interface Description)

---

## MON-SPAN (0x0a 0x31) — p.134

### 概要

RFスペクトラム解析用メッセージ。基本的なスペクトラムアナライザとして使用。

### 特徴

- **Type**: Periodic/polled
- **用途**: RFパスごとのスペクトラム表示
- **精度**: 比較分析用（絶対値の精度は高くない）
- **注意**: PGAゲインはスペクトラムデータに含まれない（別フィールド）

### ペイロード構造

| Offset | Type | Name | Scale | Unit | Description |
|--------|------|------|-------|------|-------------|
| 0 | U1 | version | - | - | メッセージバージョン (0x00) |
| 1 | U1 | numRfBlocks | - | - | RFブロック数 |
| 2 | U1[2] | reserved0 | - | - | 予約 |

**繰り返しグループ (numRfBlocks回)**:

| Offset | Type | Name | Scale | Unit | Description |
|--------|------|------|-------|------|-------------|
| 4 + n×272 | U1[256] | spectrum | - | dB | スペクトラムデータ（256点） |
| 260 + n×272 | U4 | span | - | Hz | 周波数スパン |
| 264 + n×272 | U4 | res | - | Hz | 周波数分解能 |
| 268 + n×272 | U4 | center | - | Hz | 中心周波数 |
| 272 + n×272 | U1 | pga | - | dB | PGAゲイン |
| 273 + n×272 | U1[3] | reserved1 | - | - | 予約 |

**ペイロード長**: 4 + numRfBlocks × 272 バイト

### 中心周波数計算式

```
f(i) = center + span × (i - 127) / 256
```

- i: ビンインデックス (0〜255)
- f(i): ビンiの中心周波数 (Hz)

### 128MHz/256点の場合の分解能

- span = 128,000,000 Hz
- res = span / 256 = 500,000 Hz (500kHz)

---

## NAV-SAT (0x01 0x35) — p.150-151

### 概要

衛星単位の情報。スカイプロット表示に適している。

### 特徴

- **Type**: Periodic/polled
- **粒度**: 衛星単位（L1/L2/L5の統合値）
- **用途**: スカイプロット、衛星捕捉状況の確認

### ペイロード構造（ヘッダ）

| Offset | Type | Name | Scale | Unit | Description |
|--------|------|------|-------|------|-------------|
| 0 | U4 | iTOW | - | ms | GPSタイム |
| 4 | U1 | version | - | - | バージョン (0x01) |
| 5 | U1 | numSvs | - | - | 衛星数 |
| 6 | U1[2] | reserved0 | - | - | 予約 |

**繰り返しグループ (numSvs回、各12バイト)**:

| Offset | Type | Name | Scale | Unit | Description |
|--------|------|------|-------|------|-------------|
| 8 + n×12 | U1 | gnssId | - | - | GNSS識別子 |
| 9 + n×12 | U1 | svId | - | - | 衛星番号 |
| 10 + n×12 | U1 | cno | - | dBHz | C/N0（信号強度） |
| 11 + n×12 | I1 | elev | - | deg | 仰角 (+/-90) |
| 12 + n×12 | I2 | azim | - | deg | 方位角 (0-360) |
| 14 + n×12 | I2 | prRes | 0.1 | m | 疑似距離残差 |
| 16 + n×12 | X4 | flags | - | - | フラグ（下記） |

### flagsフィールド

| Bit | Name | Description |
|-----|------|-------------|
| 0-2 | qualityInd | 品質指標 (0=no signal, 4=code locked, 5-7=carrier locked) |
| 3 | svUsed | ナビゲーションに使用中 |
| 4-5 | health | 健全性 (0=unknown, 1=healthy, 2=unhealthy) |
| 6 | diffCorr | 差分補正データあり |
| 8-10 | orbitSource | 軌道ソース (1=ephemeris, 2=almanac) |

---

## NAV-SIG (0x01 0x43) — p.152-154

### 概要

信号単位の情報。**L1/L2別のC/N0比較**に適している。

### 特徴

- **Type**: Periodic/polled
- **粒度**: 信号単位（L1/L2/L5が別々のエントリ）
- **用途**: L1/L2詳細比較、受信感度評価
- **最大信号数**: 120（F9プラットフォーム）

### ペイロード構造（ヘッダ）

| Offset | Type | Name | Scale | Unit | Description |
|--------|------|------|-------|------|-------------|
| 0 | U4 | iTOW | - | ms | GPSタイム |
| 4 | U1 | version | - | - | バージョン (0x00) |
| 5 | U1 | numSigs | - | - | 信号数 |
| 6 | U1[2] | reserved0 | - | - | 予約 |

**繰り返しグループ (numSigs回、各16バイト)**:

| Offset | Type | Name | Scale | Unit | Description |
|--------|------|------|-------|------|-------------|
| 8 + n×16 | U1 | gnssId | - | - | GNSS識別子 |
| 9 + n×16 | U1 | svId | - | - | 衛星番号 |
| 10 + n×16 | U1 | sigId | - | - | **信号識別子（L1/L2/L5区別）** |
| 11 + n×16 | U1 | freqId | - | - | GLONASS周波数スロット+7 |
| 12 + n×16 | I2 | prRes | 0.1 | m | 疑似距離残差 |
| 14 + n×16 | U1 | cno | - | dBHz | C/N0（信号強度） |
| 15 + n×16 | U1 | qualityInd | - | - | 品質指標 (0-7) |
| 16 + n×16 | U1 | corrSource | - | - | 補正ソース |
| 17 + n×16 | U1 | ionoModel | - | - | 電離層モデル |
| 18 + n×16 | X2 | sigFlags | - | - | フラグ |

### sigId（信号識別子）の値

参照: [08-ubx-protocol-index.md](../../docs/missions/m1-sensor-evaluation/gnss/08-ubx-protocol-index.md)

---

## NAV-SAT vs NAV-SIG 比較

| 項目 | NAV-SAT | NAV-SIG |
|------|---------|---------|
| 粒度 | 衛星単位 | 信号単位（L1/L2/L5別） |
| C/N0 | 衛星の代表値 | **信号ごとの値** |
| 用途 | スカイプロット | **L1/L2詳細比較** |
| エントリ数 | 衛星数 | 信号数（衛星×周波数） |
| 推奨 | 概要把握 | **受信感度評価（屋外検査）** |

**結論**: 屋外検査では**NAV-SIG**を使用（L1/L2別の受信感度比較が必要なため）

---

## 参照

- UBX-22008968 p.134 (MON-SPAN)
- UBX-22008968 p.150-151 (NAV-SAT)
- UBX-22008968 p.152-154 (NAV-SIG)
- [07-cross-sheet-findings.md](../../docs/missions/m1-sensor-evaluation/gnss/07-cross-sheet-findings.md)
