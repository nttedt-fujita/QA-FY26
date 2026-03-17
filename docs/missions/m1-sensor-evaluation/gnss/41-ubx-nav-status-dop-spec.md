# UBXメッセージ仕様書（Session 64用）

**元PDF**: u-blox F9 HPG 1.32 Interface Description (UBX-22008968)
**抽出日**: 2026-03-10

---

## 1. UBX-NAV-STATUS (0x01 0x03)

**用途**: レシーバーナビゲーション状態
**タイプ**: Periodic/Polled
**ペイロード長**: 16バイト

### ペイロード構造

| Offset | 型 | 名前 | スケール | 単位 | 説明 |
|--------|-----|------|---------|------|------|
| 0 | U4 | iTOW | - | ms | GPSの週内時刻 |
| 4 | U1 | gpsFix | - | - | FIXタイプ（下記参照） |
| 5 | X1 | flags | - | - | ナビゲーションステータスフラグ |
| 6 | X1 | fixStat | - | - | FIXステータス情報 |
| 7 | X1 | flags2 | - | - | 追加フラグ |
| 8 | U4 | ttff | - | ms | **Time to First Fix** |
| 12 | U4 | msss | - | ms | 起動/リセットからの経過時間 |

### gpsFix値

| 値 | 意味 |
|----|------|
| 0x00 | No fix |
| 0x01 | Dead reckoning only |
| 0x02 | 2D-fix |
| 0x03 | 3D-fix |
| 0x04 | GPS + dead reckoning combined |
| 0x05 | Time only fix |

### flags (byte 5)

| ビット | 名前 | 説明 |
|--------|------|------|
| 0 | gpsFixOk | 1 = 位置・速度が有効（DOP・ACCマスク内） |
| 1 | diffSoln | 1 = 差分補正適用済み |
| 2 | wknSet | 1 = 週番号有効 |
| 3 | towSet | 1 = TOW有効 |

### fixStat (byte 6)

| ビット | 名前 | 説明 |
|--------|------|------|
| 0 | diffCorr | 1 = 差分補正データ利用可能 |
| 1 | carrSolnValid | 1 = carrSoln有効 |
| 7-6 | mapMatching | マップマッチング状態 (00=none, 01=有効だが未使用, 10=使用中, 11=使用中+DR有効) |

### flags2 (byte 7)

| ビット | 名前 | 説明 |
|--------|------|------|
| 1-0 | psmState | パワーセーブモード状態 (0=ACQUISITION, 1=TRACKING, 2=POWER OPTIMIZED, 3=INACTIVE) |
| 4-3 | spoofDetState | スプーフィング検出状態 (0=不明, 1=スプーフィングなし, 2=スプーフィング検出, 3=複数検出) |
| **7-6** | **carrSoln** | **RTK状態 (0=なし, 1=Float, 2=Fixed)** |

---

## 2. UBX-NAV-DOP (0x01 0x04)

**用途**: 精度劣化係数（DOP）
**タイプ**: Periodic/Polled
**ペイロード長**: 18バイト

**重要**: 全てのDOP値はスケール0.01（例: 156 → 1.56）

### ペイロード構造

| Offset | 型 | 名前 | スケール | 単位 | 説明 |
|--------|-----|------|---------|------|------|
| 0 | U4 | iTOW | - | ms | GPSの週内時刻 |
| 4 | U2 | gDOP | 0.01 | - | Geometric DOP |
| 6 | U2 | pDOP | 0.01 | - | Position DOP |
| 8 | U2 | tDOP | 0.01 | - | Time DOP |
| 10 | U2 | vDOP | 0.01 | - | Vertical DOP |
| 12 | U2 | hDOP | 0.01 | - | Horizontal DOP |
| 14 | U2 | nDOP | 0.01 | - | Northing DOP |
| 16 | U2 | eDOP | 0.01 | - | Easting DOP |

---

## 3. UBX-MON-RF (0x0a 0x38)

**用途**: RF情報（ジャミング状態など）
**タイプ**: Periodic/Polled
**ペイロード長**: 4 + nBlocks × 24 バイト

### ヘッダー部

| Offset | 型 | 名前 | スケール | 単位 | 説明 |
|--------|-----|------|---------|------|------|
| 0 | U1 | version | - | - | メッセージバージョン (0x00) |
| 1 | U1 | nBlocks | - | - | RFブロック数 |
| 2 | U1[2] | reserved0 | - | - | 予約 |

### 繰り返しブロック（nBlocks回）

| Offset | 型 | 名前 | スケール | 単位 | 説明 |
|--------|-----|------|---------|------|------|
| 4 + n×24 | U1 | blockId | - | - | RFブロックID (0=L1, 1=L2/L5) |
| 5 + n×24 | X1 | flags | - | - | フラグ |
| 6 + n×24 | U1 | antStatus | - | - | アンテナ状態 |
| 7 + n×24 | U1 | antPower | - | - | アンテナ電源状態 |
| 8 + n×24 | U4 | postStatus | - | - | POSTステータス |
| 12 + n×24 | U1[4] | reserved1 | - | - | 予約 |
| 16 + n×24 | U2 | noisePerMS | - | - | ノイズレベル |
| 18 + n×24 | U2 | agcCnt | - | - | AGCモニタ (0-8191) |
| 20 + n×24 | U1 | cwSuppression | - | - | CW干渉抑制レベル (0=なし, 255=強) |
| 21 + n×24 | I1 | ofsI | - | - | I成分のインバランス |
| 22 + n×24 | U1 | magI | - | - | I成分の大きさ |
| 23 + n×24 | I1 | ofsQ | - | - | Q成分のインバランス |
| 24 + n×24 | U1 | magQ | - | - | Q成分の大きさ |
| 25 + n×24 | U1[3] | reserved2 | - | - | 予約 |

### flags (byte 5) のjammingState

| ビット | 名前 | 説明 |
|--------|------|------|
| **1-0** | **jammingState** | 0=不明/無効, 1=OK, **2=警告（干渉あるがFIX可）**, **3=危険（干渉ありFIX不可）** |

### antStatus値

| 値 | 意味 |
|----|------|
| 0x00 | INIT |
| 0x01 | DONTKNOW |
| 0x02 | OK |
| 0x03 | SHORT |
| 0x04 | OPEN |

### antPower値

| 値 | 意味 |
|----|------|
| 0x00 | OFF |
| 0x01 | ON |
| 0x02 | DONTKNOW |

---

## パーサー実装優先度

1. **NAV-STATUS** — TTFF取得、carrSoln（RTK状態）
2. **NAV-DOP** — 精度劣化係数（pDOP, hDOP, vDOP）
3. **MON-RF** — ジャミング状態監視

---

## UBXフレーム構造（復習）

```
[0xB5, 0x62, Class, ID, LenL, LenH, Payload..., CK_A, CK_B]
```

- Sync: 0xB5 0x62
- Class: 1バイト
- ID: 1バイト
- Length: 2バイト（リトルエンディアン）
- Payload: Lengthバイト
- Checksum: 2バイト（8-bit Fletcher checksum、ClassからPayload末尾まで）
