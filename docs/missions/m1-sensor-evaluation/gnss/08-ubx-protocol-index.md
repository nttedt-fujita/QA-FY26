# UBXプロトコル仕様書 索引（Phase 2用）

**元PDF**: u-blox F9 HPG 1.32 Interface Description (UBX-22008968)
**文書番号**: UBX-22008968 R01 (02-May-2022)
**対象チップ**: ZED-F9P（マルチバンドGNSS + RTK、cm級精度）
**性格**: UBXプロトコル仕様書 — u-centerがF9Pチップと通信するときのメッセージ形式定義
**総ページ数**: 305ページ

---

## 評価に関連する主要メッセージ（Session 17で詳細確認済み）

| メッセージ | ページ | 取得できるデータ | 評価用途 |
|-----------|--------|----------------|---------|
| UBX-NAV-PVT | p.145 | fixType, **carrSoln**, numSV, hAcc, vAcc, pDOP | 最重要。RTK状態・精度・衛星数を一括取得 |
| UBX-NAV-HPPOSLLH | p.140 | hAcc/vAcc（0.1mm精度） | mm単位の高精度測位 |
| UBX-NAV-STATUS | p.155 | gpsFix, carrSoln, **ttff**（初期FIX時間ms） | 機体準備時間の自動計測 |
| UBX-NAV-SAT | p.150 | cno (dBHz), elev, qualityInd（衛星ごと） | 受信感度の自動記録 |
| UBX-NAV-SIG | p.152 | cno (dBHz)（L1/L2信号別） | L1/L2ごとの受信感度 |
| UBX-MON-RF | p.132 | **jammingState**, noisePerMS, agcCnt | ジャミング検出・ノイズ監視 |
| UBX-MON-SPAN | p.134 | 256点スペクトラムデータ | スペアナの自動取得 |

### RTK FIX状態の定義

`UBX-NAV-PVT.flags` の bits 7-6（`carrSoln`）:

| 値 | 意味 |
|----|------|
| 0 | RTKなし（通常測位） |
| 1 | RTK Float（不安定） |
| **2** | **RTK Fixed（cm級精度 = 合格）** |

---

## 確認済みページ範囲

- p.132-134（UBX-MON-RF, UBX-MON-SPAN）
- p.139-158（主要NAVメッセージ）

**未読範囲**: p.1-131, p.159-305（RTCM, SPARTN, Configuration等）

---

## 全目次（305ページ）

### 1. General information (p.15-21)

| セクション | ページ | 内容 |
|-----------|--------|------|
| 1.1 Document overview | 15 | 文書概要 |
| 1.2 Firmware and protocol versions | 15 | FW・プロトコルバージョン |
| 1.3 Receiver configuration | 17 | レシーバ設定 |
| 1.4 Message naming | 17 | メッセージ命名規則 |
| 1.5 GNSS, satellite, and signal identifiers | 18 | GNSS/衛星/信号の識別子 |
| 1.6 Message types | 21 | メッセージタイプ一覧 |

### 2. NMEA protocol (p.22-52)

| セクション | ページ | 内容 |
|-----------|--------|------|
| 2.1 NMEA frame structure | 22 | NMEAフレーム構造 |
| 2.2 NMEA protocol configuration | 22 | NMEA設定 |
| 2.3 NMEA-proprietary messages | 23 | 独自NMEAメッセージ |
| 2.4 NMEA multi-GNSS operation | 24 | マルチGNSS動作 |
| 2.5 NMEA data fields | 24 | データフィールド定義（Talker ID, 緯度経度形式, FIXフラグ等） |
| 2.6 NMEA messages overview | 27 | メッセージ一覧 |
| 2.7 Standard messages | 28 | 標準メッセージ（DTM, GBS, GGA, GLL, GNS, GSA, GST, GSV, RMC, VTG, ZDA等） |
| 2.8 Secondary output messages | 41 | セカンダリ出力 |
| 2.9 PUBX messages | 47 | u-blox独自メッセージ（CONFIG, POSITION, RATE, SVSTATUS, TIME） |

### 3. UBX protocol (p.53-198) ← Phase 2で最も重要

| セクション | ページ | 内容 |
|-----------|--------|------|
| 3.1-3.7 UBX protocol basics | 53-57 | フレーム構造、ペイロード定義、チェックサム、ポーリング |
| 3.8 UBX messages overview | 58 | 全メッセージ一覧表 |
| **3.9 UBX-ACK** | 62 | ACK/NAK応答 |
| **3.10 UBX-CFG** | 63 | **設定メッセージ群**（ANT, CFG, DAT, DGNSS, GEOFENCE, GNSS, INF, ITFM, MSG, NAV5, NAVX5, NMEA, PRT, RATE, RST, SBAS, TMODE3, TP5, USB, VALDEL, VALGET, VALSET） |
| 3.11 UBX-INF | 98 | 情報メッセージ（DEBUG, ERROR, NOTICE, WARNING） |
| 3.12 UBX-LOG | 100 | データロガー |
| 3.13 UBX-MGA | 106 | GNSS補助データ（BDS, GAL, GLO, GPS, QZSS） |
| **3.14 UBX-MON** | 126 | **モニタリング**（COMMS, GNSS, HW, IO, RF, SPAN, VER等） |
| **3.15 UBX-NAV** | 136 | **ナビゲーション**（PVT, HPPOSLLH, SAT, SIG, STATUS, SVIN等） |
| 3.16 UBX-NAV2 | 164 | セカンダリナビゲーション出力 |
| 3.17 UBX-RXM | 183 | レシーバマネージャ（RAWX, RTCM, SPARTN, MEASX等） |
| 3.18 UBX-SEC | 194 | セキュリティ（UNIQID） |
| 3.19 UBX-TIM | 194 | タイミング（TM2, TP, VRFY） |
| 3.20 UBX-UPD | 197 | バックアップ/リストア |

### 4. RTCM protocol (p.199-215)

| セクション | ページ | 内容 |
|-----------|--------|------|
| 4.1-4.3 RTCM overview | 199 | RTCMプロトコル概要・設定・メッセージ一覧 |
| 4.4 RTCM 3.3 messages | 200 | 全RTCMメッセージ定義（1001-1012: GPS/GLONASS RTK, 1074-1127: MSM4/5/7, 1230: GLOバイアス, 4072: u-blox独自） |

### 5. SPARTN protocol (p.216-222)

| セクション | ページ | 内容 |
|-----------|--------|------|
| 5.1-5.3 SPARTN overview | 216 | SPARTNプロトコル概要 |
| 5.4 SPARTN messages | 216 | OCB（軌道/クロック/バイアス）、HPAC（高精度大気補正）、GAD（地理エリア定義） |

### 6. Configuration interface (p.223-279)

| セクション | ページ | 内容 |
|-----------|--------|------|
| 6.1-6.7 Configuration basics | 223-227 | 設定DB、レイヤー、アクセス方法、トランザクション |
| 6.8 Configuration overview | 227 | 設定項目一覧 |
| **6.9 Configuration reference** | 228 | **全設定項目の詳細** |

#### 6.9の主要設定項目（Phase 2で関連するもの）

| 設定グループ | ページ | 内容 | Phase 2での関連性 |
|-------------|--------|------|-----------------|
| CFG-GNSS | 67 | GNSS選択設定 | 衛星コンステレーション設定確認 |
| CFG-HW | 229 | ハードウェア設定 | アンテナ設定 |
| CFG-ITFM | 232 | ジャミング・干渉モニタ設定 | ジャミング検出の閾値設定 |
| CFG-MSGOUT | 234 | メッセージ出力設定 | 自動取得するメッセージの選択 |
| CFG-NAVHPG | 256 | 高精度ナビゲーション設定 | RTK関連設定 |
| CFG-NAVSPG | 256 | 標準精度ナビゲーション設定 | FIXモード・動的モデル |
| CFG-QZSS | 261 | QZSS設定 | QZSS L2受信の設定確認 |
| CFG-RATE | 261 | 測定・ナビゲーションレート | 更新レート設定 |
| CFG-SIGNAL | 265 | 衛星信号設定 | L1/L2の有効無効、QZSS L2S等 |
| CFG-TMODE | 267 | タイムモード設定 | 基準局モード設定 |

### 付録 (p.280-305)

| セクション | ページ | 内容 |
|-----------|--------|------|
| Configuration defaults | 280 | デフォルト設定値一覧 |
| Related documents | 303 | 関連文書 |
| Revision history | 304 | 改訂履歴 |
