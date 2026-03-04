# ミッション別アプローチ 調査レポート

**作成日**: 2026-03-04
**作成者**: Claude (Session 1)
**対象**: 藤田さん担当の4つのミッション

---

## ミッション一覧と推奨スケジュール

| # | ミッション | スケジュール感 | 難易度 |
|---|-----------|--------------|--------|
| M1 | AirGrow搭載センサーの定量的評価手法の策定（Lidar/GNSS） | Step1-3（スライド2のスケジュール通り） | 高 |
| M2 | 点群データ検証方法の策定 | M1と並行可能 | 中〜高 |
| M3 | 受入検査データベース化 | 独立して進行可能 | 中 |
| M4 | 工程不良データベース化 | M3と並行・共通設計可能 | 中 |

---

## M1: AirGrow搭載センサーの定量的評価手法の策定

### 1-A. Lidarセンサー評価

#### 性能評価の主要指標

| 評価指標 | 定義 | UAV用Lidarの典型値 |
|---------|------|-------------------|
| **測距精度 (Range Accuracy)** | 真値に対する測距値の系統的偏差（バイアス） | 1〜3 cm @ 100m |
| **測距精密度 (Range Precision)** | 同一ターゲットに対する繰り返し測定のばらつき (1σ) | 0.5〜2 cm |
| **角度精度 (Angular Accuracy)** | スキャンミラー角度の系統的誤差 | 0.01°〜0.05° |
| **レンジノイズ (Range Noise)** | 短時間測距値の確率的変動 | 表面反射率・距離に依存 |
| **最大検出距離** | 所定の反射率ターゲットを検出可能な最大距離 | 100〜450m (反射率80%) |
| **点群密度** | 単位面積当たりの計測点数 | 50〜400 pts/m² (高度50m) |
| **マルチエコー分離能** | 複数の反射を分離できる最小距離差 | 0.5〜1.5m |

#### 評価試験方法

**(a) 室内リファレンスターゲット試験**

> **ASTM E2938-15 "Standard Test Method for Evaluating the Relative-Range Measurement Performance of 3D Imaging Systems in the Medium Range" (要旨):**
> "This test method covers procedures for evaluating the relative-range measurement performance of 3D imaging systems over a range of 2 m to 150 m. The test method uses sphere targets of known diameter placed at known positions to evaluate systematic and random range errors."

試験手順:
1. 校正済みの拡散反射板（反射率10%, 50%, 80%の3種以上）を既知距離に配置
2. 各距離・各反射率で最低100回の測距を実施
3. 平均誤差（バイアス）と標準偏差（精密度）を算出
4. 距離依存性・反射率依存性をプロットしてスペック適合を判定

**(b) 屋外フィールド試験**

> **出典**: [Microdrones — LiDAR Point Cloud Quality Control](https://www.microdrones.com/en/content/lidar-point-cloud-quality-control-automating-accuracy-and-precision-testing/)
> "LiDAR accuracy is commonly evaluated using ground control points (GCPs) with known coordinates, with surveyors comparing LiDAR measurements to independently surveyed points, typically requiring a minimum of 20-30 checkpoints"

1. GCPを測量級トータルステーション or RTK-GNSSで設置（基準精度 < 1cm）
2. 既知座標のGCPを含む領域をUAV-Lidarでスキャン
3. GCP座標と点群座標の差分から評価:
   - 絶対精度: RMSE_z（鉛直）, RMSE_xy（水平）
   - 相対精度: 近接GCP間の距離誤差

**(c) 環境条件試験**

> **出典**: [PMC — An Automotive LiDAR Performance Test Method in Dynamic Driving Conditions](https://pmc.ncbi.nlm.nih.gov/articles/PMC10147061/)
> "Performance evaluations examine measurement distances ranging from 10 to 30 m, varying rainfall intensities (0-40 mm/h), and different observation angles"

外乱要因として以下を評価:
- 日照条件（直射日光 vs 曇天 vs 夜間）
- 降雨条件（0〜40mm/h）
- 観測角度（0°〜60°）
- 温度条件（-10°C〜50°C）

#### 関連規格

| 規格 | 内容 | 適用 |
|------|------|------|
| **ASTM E2938-15** | 中距離3Dイメージングの測距性能評価 | 室内精度試験 |
| **ASTM E3125-17** | 点間距離精度評価 | 精度試験 |
| **ISO 17123-9:2018** | TLSのフィールド評価手順 | 屋外精度試験（UAV-Lidarに準用可能） |
| **IEC 60825-1:2014** | レーザー製品の安全クラス分類 | レーザー安全 |
| **VDI/VDE 2634 Part 3** | 3D光学計測システムの受入検査 | 受入評価 |
| **UL4700** | 自律走行車両向けLidar評価（Detection Probability等） | 参考 |

> **ISO 17123-9:2018 (概要):**
> "This document specifies field procedures to be adopted when determining and evaluating the precision (repeatability) of terrestrial laser scanners and their ancillary equipment when used in building and civil engineering surveys."
> → UAV搭載Lidarにおいても、高度・飛行パラメータを考慮した上で準用できる。

---

### 1-B. GNSS評価

#### 精度指標

> **出典**: [Juniper Systems — GNSS/GPS Accuracy Explained](https://junipersys.com/support/article/6614)
> "DRMS is a single number that expresses 2D accuracy. If the product page for a GPS receiver states a horizontal accuracy of 1 m 2DRMS, this means that during testing, 95–98% of data points collected with the receiver were found to be within a 1-meter radius of a true point."

> **出典**: [GNSS SDR — Accuracy](https://gnss-sdr.org/design-forces/accuracy/)
> "CEP (Circular Error Probable) represents the radius of a circle centered on the true position, within which there is a 50% probability that the measured position will fall."

| 指標 | 定義 | 確率 | 計算式 |
|------|------|------|--------|
| **CEP** | 水平測位誤差の50%が収まる円の半径 | 50% | CEP ≈ 0.589 × (σ_N + σ_E) |
| **DRMS** | 水平誤差の二乗平均平方根 | ~65% | DRMS = √(σ_N² + σ_E²) |
| **2DRMS** | DRMSの2倍 | ~95% | 2DRMS = 2 × DRMS |
| **R95** | 水平誤差の95%が収まる円の半径 | 95% | 分布から数値的に算出 |
| **SEP** | 三次元測位誤差の50%が収まる球の半径 | 50% | SEP ≈ 0.513 × (σ_N + σ_E + σ_U) |

#### RTK-GNSS評価項目

| 試験項目 | 方法 | 合格基準例 |
|---------|------|-----------|
| **Fix率** | 既知基線で一定時間観測し、Fix解の割合 | > 95% (開放環境) |
| **TTFF** | RTK Fix解を最初に得るまでの時間 | < 30秒（再初期化時） |
| **Fix精度 (水平)** | Fix解の2DRMS | < 2cm + 1ppm × 基線長 |
| **Fix精度 (垂直)** | Fix解の垂直方向2σ | < 3cm + 1ppm × 基線長 |
| **基線長依存性** | 基線長を変えた場合の精度劣化率 | ppm値で評価 |
| **Wrong Fix率** | Fix解の誤解率 | < 0.1% |

#### 基準局との比較試験

1. 国土地理院の電子基準点（GEONET）を利用、または自社基準局を設置
2. 基線解析による評価（PPK: Post-Processing Kinematic）
3. 解析ツール: **RTKLIB**（オープンソース）— ratio値 > 3.0 で Fix解と判定
4. UAV搭載時: RMSE_xy < 2〜5cm, RMSE_z < 3〜8cm が一般的目標値

> **RTKLIB (T. Takasu, 2013) より:**
> "The ratio test is used to validate the integer ambiguity resolution. If the ratio of the second-best to the best residual exceeds the threshold (default: 3.0), the solution is accepted as a fixed solution."

---

### M1 推奨段取り

```
Step 1 (1Q): 評価手法の調査・計画策定
  ├── ASTM E2938/ISO 17123-9に基づく評価項目の選定
  ├── CEP/2DRMS指標の定義、RTK評価項目の選定
  ├── 評価環境（室内/屋外）の設計
  └── 【成果物】評価計画書

Step 2 (2Q-3Q): 評価環境構築・評価実施
  ├── リファレンスターゲット/GCPの設置・校正
  ├── Lidar精度試験（室内 → 屋外フィールド）
  ├── GNSS精度試験（静的 → 動的 → RTK）
  └── 【成果物】評価データ・分析結果

Step 3 (3Q-FY27 1Q): 標準化・文書化
  ├── 評価手順書(SOP)の作成
  ├── 合格判定基準の設定
  └── 【成果物】評価手順書、判定基準書
```

---

## M2: 点群データ検証方法の策定

### 品質指標

| 品質指標 | 定義 | 目標値例 |
|---------|------|---------|
| **点密度** | 単位面積当たりの点数 (pts/m²) | ≥ 8 pts/m² (USGS QL1) |
| **点密度の均一性** | 点密度の変動係数 (CV) | CV < 30% |
| **ノイズ率** | 全点数に対する外れ値の割合 | < 0.5% |
| **絶対精度** | GCPに対する系統的誤差 (RMSE) | RMSE_z < 10cm |
| **相対精度** | ストリップ間の整合性 | < 7cm |
| **網羅性** | 計測対象エリアのカバー率 | データボイド < 4 × NPS² |
| **分類精度** | 点群分類の正答率 | 地表面分類 > 95% |

### ASPRS精度基準

> **ASPRS Positional Accuracy Standards for Digital Geospatial Data (Edition 2, 2023) より:**
> "Vertical accuracy shall be tested by comparing the elevations of the dataset with elevations of higher-accuracy check points (at least three times more accurate than the dataset)."
> "RMSE_z = √[ Σ(z_data,i - z_check,i)² / n ]"
> "Non-vegetated vertical accuracy (NVA) at 95% confidence level = 1.9600 × RMSE_z"
> "A minimum of 30 checkpoints are required by ASPRS for 'Tested to Meet ...' accuracy reporting."

| 精度クラス | RMSE_z (cm) | NVA 95% (cm) |
|-----------|-------------|-------------|
| 1 cm | 1.0 | 2.0 |
| 5 cm | 5.0 | 9.8 |
| 10 cm | 10.0 | 19.6 |
| 20 cm | 20.0 | 39.2 |

### 点群品質評価の7つの方法

> **出典**: [Map Library — 7 Ways to Evaluate Point Cloud Accuracy and Quality](https://www.maplibrary.org/10350/how-to-evaluate-point-cloud-accuracy-and-quality/)

| # | 方法 | 説明 |
|---|------|------|
| 1 | **統計分析による幾何学的精度測定** | RMSE・点間距離分析で参照座標との偏差を定量化 |
| 2 | **点群密度とカバレッジ評価** | グリッドセル内の点密度統計で均一性を検証 |
| 3 | **ノイズレベルと外れ値検出** | 統計的フィルタリングで局所表面フィッティングとの変動を測定 |
| 4 | **表面再構成品質の検証** | ハウスドルフ距離・曲率分析 |
| 5 | **登録・位置合わせ精度分析** | 複数スキャンの重複領域で変換行列の精度を評価 |
| 6 | **自動品質制御ワークフロー** | 閾値ベースの自動検証プロセス導入 |
| 7 | **専門ソフトウェア活用** | CloudCompare, Autodesk ReCap Pro等で標準化レポート生成 |

### 検証ツール

| ツール | 用途 | ライセンス |
|-------|------|-----------|
| **CloudCompare** | 点群比較・解析（M3C2, ICP） | GPL v2 (OSS) |
| **PDAL** | 点群処理パイプライン | BSD (OSS) |
| **LAStools** | LASファイル処理・品質チェック | 一部商用 |
| **PCL** | 点群処理（フィルタ、特徴量等） | BSD (OSS) |
| **RTKLIB** | GNSS後処理解析 | BSD-2 (OSS) |

### M2 推奨段取り

```
Step 1: 品質指標の定義と検証手順の策定
  ├── ASPRS/USGS基準に基づく精度クラスの選定
  ├── 点密度・ノイズ率・網羅性の基準値設定
  └── 検証ツール（CloudCompare/PDAL）の選定・習得

Step 2: 検証パイプラインの構築
  ├── GCP設置手順の標準化
  ├── 自動検証スクリプトの開発（Python + PDAL）
  └── レポート自動生成の仕組み構築

Step 3: 試行運用と改善
  ├── 実フライトデータでの試行
  ├── 基準値の妥当性確認
  └── 検証手順書のファイナライズ
```

---

## M3: 受入検査データベース化

### データベースの構成

```
受入検査DB
├── マスタデータ
│   ├── 部品マスタ (parts_master)
│   ├── サプライヤマスタ (supplier_master)
│   ├── 検査項目マスタ (inspection_items)
│   └── 検査基準マスタ (acceptance_criteria)
├── トランザクションデータ
│   ├── 入荷ロット情報 (incoming_lots)
│   ├── 検査結果 (inspection_results)
│   └── 不適合処理 (non_conformance)
└── 分析ビュー
    ├── サプライヤ品質スコアカード
    ├── ロット合格率推移
    └── サイレントチェンジ検出
```

### 主要テーブル設計

> **ISO 9001:2015, Clause 8.6 "Release of products and services" に基づく要件:**
> "The organization shall implement planned arrangements, at appropriate stages, to verify that the product and service requirements have been met. [...] The organization shall retain documented information on the release of products and services."

#### incoming_lots（入荷ロット）

| カラム | 型 | 説明 |
|--------|-----|------|
| lot_id | VARCHAR(30) PK | ロットID |
| part_id | VARCHAR(20) FK | 部品番号 |
| supplier_id | VARCHAR(20) FK | サプライヤID |
| po_number | VARCHAR(20) | 発注番号 |
| received_date | DATE | 入荷日 |
| lot_qty | INT | ロット数量 |
| sample_qty | INT | サンプル数 |
| **fw_version** | VARCHAR(20) | **FWバージョン（サイレントチェンジ監視用）** |
| **hw_version** | VARCHAR(20) | **HWバージョン** |
| lot_judgment | ENUM | ACCEPT / REJECT / CONDITIONAL |

#### inspection_results（検査結果）

| カラム | 型 | 説明 |
|--------|-----|------|
| inspection_id | BIGINT PK | 検査ID |
| lot_id | VARCHAR(30) FK | ロットID |
| item_code | VARCHAR(20) FK | 検査項目コード |
| measured_value | DECIMAL(12,4) | 測定値 |
| lower_spec_limit | DECIMAL(12,4) | 下限規格値 (LSL) |
| upper_spec_limit | DECIMAL(12,4) | 上限規格値 (USL) |
| judgment | ENUM | PASS / FAIL |
| equipment_id | VARCHAR(20) | 使用測定器ID |

### サイレントチェンジ検出クエリ例

```sql
-- FW/HWバージョン変更時の品質変動を検出
SELECT
    il.part_id,
    il.fw_version,
    MIN(il.received_date) AS first_received,
    COUNT(*) AS lot_count,
    AVG(CASE WHEN il.lot_judgment = 'REJECT' THEN 1.0 ELSE 0.0 END) AS reject_rate
FROM incoming_lots il
GROUP BY il.part_id, il.fw_version
ORDER BY il.part_id, first_received;
```

### M3 推奨段取り

```
Step 1: 要件定義・スキーマ設計
  ├── 現行の検査運用フロー調査（紙？Excel？）
  ├── 管理項目の洗い出し（特にサイレントチェンジ監視要件）
  ├── DBスキーマ設計
  └── 【成果物】要件定義書、ERダイアグラム

Step 2: DB構築・データ移行
  ├── RDBMS選定（PostgreSQL推奨 / SQLiteで小規模開始もあり）
  ├── テーブル作成・マスタデータ投入
  ├── 過去データ移行（Excel等からのインポート）
  └── 【成果物】稼働DB

Step 3: 分析機能・UI・運用開始
  ├── 分析クエリ・ダッシュボードの構築
  ├── 入力フォームの構築（Webアプリ or Excel VBA）
  └── 【成果物】運用マニュアル
```

---

## M4: 工程不良データベース化

### 不良コード体系（3階層構造）

> **IPC-9261A "In-Process DPMO and Estimated Yield for PWAs" の考え方を参考に:**
> 不良コードは「不良現象（What）」「不良箇所（Where）」「不良原因（Why）」の3軸で分類

**フォーマット: XX-XX-XX**

| Level 1 (大分類) | Level 2 (中分類) | Level 3 例 |
|-----------------|-----------------|-----------|
| **EL: 電気系** | EL-SO: はんだ付け | EL-SO-01: はんだブリッジ |
| | EL-CM: 部品実装 | EL-CM-01: 部品欠品 |
| | EL-FN: 機能不良 | EL-FN-01: 通信異常 |
| **ME: 機構系** | ME-AS: 組立 | ME-AS-01: ネジ締め不良 |
| | ME-DM: 損傷 | ME-DM-01: キズ |
| **SW: ソフトウェア系** | SW-FW: FW | SW-FW-01: 書込異常 |
| | SW-CL: キャリブレーション | SW-CL-01: 校正値異常 |
| **SE: センサー系** | SE-LI: Lidar | SE-LI-01: 測距精度不良 |
| | SE-GN: GNSS | SE-GN-01: 測位精度不良 |

### 原因コード体系（4M1E分類）

> **石川馨（1968）「品質管理入門」に基づく特性要因（4M）分類を拡張:**

| コード | 分類 | 原因例 |
|-------|------|--------|
| M1-xx | Man（人） | 手順逸脱、スキル不足、不注意 |
| M2-xx | Machine（設備） | 設備故障、治具摩耗、校正切れ |
| M3-xx | Material（材料） | 部品不良、ロット間ばらつき、保管劣化 |
| M4-xx | Method（方法） | 作業手順不備、設計起因、基準不明確 |
| E1-xx | Environment（環境） | 温度影響、湿度影響、静電気 |

### SPC連携

> **ISO 22514-1:2014 より:**
> "Process capability is the statistical measure of the inherent process variability for a given characteristic."

```
工程能力指数:
  Cp  = (USL - LSL) / (6σ)           — ばらつきのみ評価
  Cpk = min[(USL - μ)/(3σ), (μ - LSL)/(3σ)]  — 偏りも考慮

判定基準:
  Cpk ≥ 1.67  → 十分（新規工程の目標）
  Cpk ≥ 1.33  → 良好（量産維持の基準）
  1.00 ≤ Cpk < 1.33 → 要改善
  Cpk < 1.00  → 能力不足（工程改善必須）
```

### パレート分析用ビュー

```sql
-- 不良コード別パレート（累積%付き）
SELECT
    dc.defect_code,
    dc.defect_name,
    COUNT(*) AS occurrence_count,
    SUM(COUNT(*)) OVER (ORDER BY COUNT(*) DESC)
        * 100.0 / SUM(COUNT(*)) OVER () AS cumulative_pct
FROM defect_records dr
JOIN defect_code_master dc ON dr.defect_code = dc.defect_code
GROUP BY dc.defect_code, dc.defect_name
ORDER BY occurrence_count DESC;
```

### M4 推奨段取り

```
Step 1: 不良コード体系の設計
  ├── 現行の不良記録方法の調査
  ├── 上記3階層コード体系 + 4M1E原因コードの策定
  ├── 現場へのヒアリング・コード体系のレビュー
  └── 【成果物】不良コード体系表

Step 2: DB構築・SPC連携
  ├── 不良記録テーブル・SPCテーブルの作成
  ├── 管理図自動生成の仕組み構築
  ├── パレート分析ビューの作成
  └── 【成果物】稼働DB、ダッシュボード

Step 3: 運用・改善サイクル確立
  ├── 現場への展開・教育
  ├── 月次品質レビューへの組み込み
  └── 【成果物】運用マニュアル、月次レポートテンプレート
```

---

## M3とM4の共通設計ポイント

受入検査DBと工程不良DBは**ロットIDで紐付け可能**にすべき。これにより：

- 受入時に合格したロットの部品が、工程で不良を起こした場合にトレースできる
- サプライヤ品質と工程不良の相関分析が可能になる

```
受入検査DB                    工程不良DB
incoming_lots ──── lot_id ────→ defect_records.lot_id
  │                               │
  ├── supplier_id                 ├── defect_code
  ├── fw_version                  ├── cause_code
  └── lot_judgment                └── severity
```

**藤田さんのソフト開発スキルが活きるポイント**:
- DB設計（ER図、正規化）はソフト開発の経験がそのまま使える
- SQLクエリによるデータ分析もプログラミングスキルの延長
- ダッシュボード構築（Python + Dash/Streamlit等）もソフトスキルで対応可能

---

## 参考文献・ソース一覧

### 規格

| 規格 | 適用ミッション |
|------|-------------|
| ASTM E2938-15 (3Dイメージング測距性能評価) | M1 |
| ISO 17123-9:2018 (TLSフィールド評価) | M1 |
| IEC 60825-1:2014 (レーザー安全) | M1 |
| ASPRS Positional Accuracy Standards (Ed.2, 2023) | M2 |
| USGS Lidar Base Specification v2.1 | M2 |
| ISO 9001:2015 (Clause 8.6) | M3, M4 |
| ISO 22514-1:2014 (工程能力) | M4 |
| JIS Z 9015-1 (AQL抜取検査) | M3 |
| IPC-9261A (工程DPMO) | M4 |

### Webソース

| ソース | URL |
|-------|-----|
| Microdrones — LiDAR Point Cloud QC | https://www.microdrones.com/en/content/lidar-point-cloud-quality-control-automating-accuracy-and-precision-testing/ |
| YellowScan — LiDAR Accuracy Standards | https://www.yellowscan.com/knowledge/lidar-accuracy-standards-what-industry-tests-prove/ |
| Juniper Systems — GNSS Accuracy Explained | https://junipersys.com/support/article/6614 |
| GNSS SDR — Accuracy | https://gnss-sdr.org/design-forces/accuracy/ |
| Map Library — Point Cloud Quality | https://www.maplibrary.org/10350/how-to-evaluate-point-cloud-accuracy-and-quality/ |
| ASPRS Standards PDF | https://www.asprs.org/wp-content/uploads/2015/01/ASPRS_Positional_Accuracy_Standards_Edition1_Version100_November2014.pdf |
| QTEC — AQLの知識 | https://www.qtec.or.jp/knowledge/inspection/aql/ |
| ITトレンド — SPC | https://it-trend.jp/process_management/article/464-0007 |
