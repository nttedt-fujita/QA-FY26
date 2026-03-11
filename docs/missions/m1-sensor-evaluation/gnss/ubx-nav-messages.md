# UBX NAVメッセージ仕様

**作成日**: 2026-03-11
**参照PDF**: UBX-22008968 (u-blox F9 HPG 1.32 Interface Description) p.150-154

---

## NAV-SAT (0x01 0x35) — 衛星情報

### 概要

衛星単位の情報。スカイプロット表示に適している。

| 項目 | 値 |
|------|-----|
| Class | 0x01 |
| ID | 0x35 |
| Type | Periodic/polled |
| ペイロード長 | 8 + numSvs × 12 バイト |

### ペイロード構造

**ヘッダ（8バイト）**:

| Offset | Type | Name | 説明 |
|--------|------|------|------|
| 0 | U4 | iTOW | GPS Time of Week (ms) |
| 4 | U1 | version | メッセージバージョン (0x01) |
| 5 | U1 | numSvs | 衛星数 |
| 6 | U1[2] | reserved0 | 予約 |

**繰り返しグループ（numSvs回、各12バイト）**:

| Offset | Type | Name | Scale | Unit | 説明 |
|--------|------|------|-------|------|------|
| 8 + n×12 | U1 | gnssId | - | - | GNSS識別子 |
| 9 + n×12 | U1 | svId | - | - | 衛星番号 |
| 10 + n×12 | U1 | cno | - | dBHz | C/N0（衛星代表値） |
| 11 + n×12 | I1 | elev | - | deg | **仰角** (+/-90) |
| 12 + n×12 | I2 | azim | - | deg | **方位角** (0-360) |
| 14 + n×12 | I2 | prRes | 0.1 | m | 疑似距離残差 |
| 16 + n×12 | X4 | flags | - | - | フラグ |

### flagsフィールド

| Bit | Name | 説明 |
|-----|------|------|
| 0-2 | qualityInd | 品質指標 (0=no signal, 4=code locked, 5-7=carrier locked) |
| 3 | svUsed | ナビゲーションに使用中 |
| 4-5 | health | 健全性 (0=unknown, 1=healthy, 2=unhealthy) |
| 6 | diffCorr | 差分補正データあり |
| 8-10 | orbitSource | 軌道ソース (1=ephemeris, 2=almanac) |

---

## NAV-SIG (0x01 0x43) — 信号情報

### 概要

信号単位の情報。**L1/L2別のC/N0比較**に適している。

| 項目 | 値 |
|------|-----|
| Class | 0x01 |
| ID | 0x43 |
| Type | Periodic/polled |
| ペイロード長 | 8 + numSigs × 16 バイト |
| 最大信号数 | 120（F9プラットフォーム） |

### ペイロード構造

**ヘッダ（8バイト）**:

| Offset | Type | Name | 説明 |
|--------|------|------|------|
| 0 | U4 | iTOW | GPS Time of Week (ms) |
| 4 | U1 | version | メッセージバージョン (0x00) |
| 5 | U1 | numSigs | 信号数 |
| 6 | U1[2] | reserved0 | 予約 |

**繰り返しグループ（numSigs回、各16バイト）**:

| Offset | Type | Name | Scale | Unit | 説明 |
|--------|------|------|-------|------|------|
| 8 + n×16 | U1 | gnssId | - | - | GNSS識別子 |
| 9 + n×16 | U1 | svId | - | - | 衛星番号 |
| 10 + n×16 | U1 | sigId | - | - | **信号識別子（L1/L2/L5区別）** |
| 11 + n×16 | U1 | freqId | - | - | GLONASS周波数スロット+7 |
| 12 + n×16 | I2 | prRes | 0.1 | m | 疑似距離残差 |
| 14 + n×16 | U1 | cno | - | dBHz | **C/N0（信号ごと）** |
| 15 + n×16 | U1 | qualityInd | - | - | 品質指標 (0-7) |
| 16 + n×16 | U1 | corrSource | - | - | 補正ソース |
| 17 + n×16 | U1 | ionoModel | - | - | 電離層モデル |
| 18 + n×16 | X2 | sigFlags | - | - | フラグ |
| 20 + n×16 | U1[4] | reserved1 | - | - | 予約 |

### qualityInd値

| 値 | 意味 |
|----|------|
| 0 | no signal |
| 1 | searching signal |
| 2 | signal acquired |
| 3 | signal detected but unusable |
| 4 | code locked and time synchronized |
| 5-7 | code and carrier locked and time synchronized |

### corrSource値

| 値 | 意味 |
|----|------|
| 0 | no corrections |
| 1 | SBAS corrections |
| 2 | BeiDou corrections |
| 3 | RTCM2 corrections |
| 4 | RTCM3 OSR corrections |
| 5 | RTCM3 SSR corrections |
| 6 | QZSS SLAS corrections |
| 7 | SPARTN corrections |
| 8 | CLAS corrections |

### ionoModel値

| 値 | 意味 |
|----|------|
| 0 | no model |
| 1 | Klobuchar model transmitted by GPS |
| 2 | SBAS model |
| 3 | Klobuchar model transmitted by BeiDou |
| 8 | Iono delay derived from dual frequency observations |

### sigFlagsフィールド

| Bit | Name | 説明 |
|-----|------|------|
| 0-1 | health | 健全性 (0=unknown, 1=healthy, 2=unhealthy) |
| 2 | prSmoothed | 疑似距離がスムージング済み |
| 3 | prUsed | 疑似距離がナビゲーションに使用中 |
| 4 | crUsed | キャリアレンジが使用中 |
| 5 | doUsed | ドップラーが使用中 |
| 6 | prCorrUsed | 疑似距離補正が使用中 |
| 7 | crCorrUsed | キャリアレンジ補正が使用中 |
| 8 | doCorrUsed | ドップラー補正が使用中 |

---

## NAV-SAT vs NAV-SIG 比較

| 項目 | NAV-SAT | NAV-SIG |
|------|---------|---------|
| 粒度 | 衛星単位 | **信号単位（L1/L2/L5別）** |
| C/N0 | 衛星の代表値 | **信号ごとの値** |
| 仰角/方位角 | **あり** | なし |
| エントリ数 | 衛星数 | 信号数（衛星×周波数） |
| 用途 | スカイプロット | **L1/L2詳細比較、受信感度評価** |

**結論**: 屋外検査では両方を組み合わせて使用
- 受信感度評価: NAV-SIG（L1/L2別C/N0）
- スカイプロット: NAV-SAT（仰角・方位角）

---

## 参照

- UBX-22008968 p.150-151 (NAV-SAT)
- UBX-22008968 p.152-154 (NAV-SIG)
- [ubx-signal-identifiers.md](ubx-signal-identifiers.md) — sigId定義
