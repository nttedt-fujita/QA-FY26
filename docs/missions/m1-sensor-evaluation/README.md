# M1: AirGrow搭載センサーの定量的評価手法の策定

**担当**: ふじた
**難易度**: 高
**スケジュール**: Step1(1Q) → Step2(2Q-3Q) → Step3(3Q-FY27 1Q)

---

## 概要

AirGrowに搭載されるLidarおよびGNSSセンサーの性能を、定量的に評価するための手法を策定する。

---

## M1-A: Lidarセンサー評価

### 性能評価の主要指標

| 評価指標 | 定義 | UAV用Lidarの典型値 |
|---------|------|-------------------|
| **測距精度 (Range Accuracy)** | 真値に対する測距値の系統的偏差（バイアス） | 1〜3 cm @ 100m |
| **測距精密度 (Range Precision)** | 同一ターゲットに対する繰り返し測定のばらつき (1σ) | 0.5〜2 cm |
| **角度精度 (Angular Accuracy)** | スキャンミラー角度の系統的誤差 | 0.01°〜0.05° |
| **レンジノイズ (Range Noise)** | 短時間測距値の確率的変動 | 表面反射率・距離に依存 |
| **最大検出距離** | 所定の反射率ターゲットを検出可能な最大距離 | 100〜450m (反射率80%) |
| **点群密度** | 単位面積当たりの計測点数 | 50〜400 pts/m² (高度50m) |
| **マルチエコー分離能** | 複数の反射を分離できる最小距離差 | 0.5〜1.5m |

### 評価試験方法

#### (a) 室内リファレンスターゲット試験

> **ASTM E2938-15** "Standard Test Method for Evaluating the Relative-Range Measurement Performance of 3D Imaging Systems in the Medium Range":
> "This test method covers procedures for evaluating the relative-range measurement performance of 3D imaging systems over a range of 2 m to 150 m."

手順:
1. 校正済みの拡散反射板（反射率10%, 50%, 80%の3種以上）を既知距離に配置
2. 各距離・各反射率で最低100回の測距を実施
3. 平均誤差（バイアス）と標準偏差（精密度）を算出
4. 距離依存性・反射率依存性をプロットしてスペック適合を判定

#### (b) 屋外フィールド試験

> **出典**: [Microdrones — LiDAR Point Cloud QC](https://www.microdrones.com/en/content/lidar-point-cloud-quality-control-automating-accuracy-and-precision-testing/)
> "LiDAR accuracy is commonly evaluated using ground control points (GCPs) with known coordinates"

1. GCPを測量級トータルステーション or RTK-GNSSで設置（基準精度 < 1cm）
2. 既知座標のGCPを含む領域をUAV-Lidarでスキャン
3. GCP座標と点群座標の差分から評価:
   - 絶対精度: RMSE_z（鉛直）, RMSE_xy（水平）
   - 相対精度: 近接GCP間の距離誤差

#### (c) 環境条件試験

外乱要因として以下を評価:
- 日照条件（直射日光 vs 曇天 vs 夜間）
- 降雨条件（0〜40mm/h）
- 観測角度（0°〜60°）
- 温度条件（-10°C〜50°C）

> **出典**: [PMC — Automotive LiDAR Performance Test](https://pmc.ncbi.nlm.nih.gov/articles/PMC10147061/)

### 関連規格

| 規格 | 内容 | 適用 |
|------|------|------|
| **ASTM E2938-15** | 中距離3Dイメージングの測距性能評価 | 室内精度試験 |
| **ASTM E3125-17** | 点間距離精度評価 | 精度試験 |
| **ISO 17123-9:2018** | TLSのフィールド評価手順 | 屋外精度試験（UAV-Lidarに準用可能） |
| **IEC 60825-1:2014** | レーザー製品の安全クラス分類 | レーザー安全 |
| **VDI/VDE 2634 Part 3** | 3D光学計測システムの受入検査 | 受入評価 |

---

## M1-B: GNSS評価

### 精度指標

| 指標 | 定義 | 確率 |
|------|------|------|
| **CEP** | 水平測位誤差の50%が収まる円の半径 | 50% |
| **DRMS** | 水平誤差の二乗平均平方根 | ~65% |
| **2DRMS** | DRMSの2倍 | ~95% |
| **R95** | 水平誤差の95%が収まる円の半径 | 95% |
| **SEP** | 三次元測位誤差の50%が収まる球の半径 | 50% |

> **出典**: [Juniper Systems — GNSS Accuracy Explained](https://junipersys.com/support/article/6614)

### RTK-GNSS評価項目

| 試験項目 | 方法 | 合格基準例 |
|---------|------|-----------|
| **Fix率** | 既知基線で一定時間観測し、Fix解の割合 | > 95% (開放環境) |
| **TTFF** | RTK Fix解を最初に得るまでの時間 | < 30秒（再初期化時） |
| **Fix精度 (水平)** | Fix解の2DRMS | < 2cm + 1ppm × 基線長 |
| **Fix精度 (垂直)** | Fix解の垂直方向2σ | < 3cm + 1ppm × 基線長 |
| **Wrong Fix率** | Fix解の誤解率 | < 0.1% |

### 基準局との比較試験

1. 国土地理院の電子基準点（GEONET）を利用、または自社基準局を設置
2. 基線解析（PPK: Post-Processing Kinematic）
3. 解析ツール: **RTKLIB**（オープンソース）— ratio値 > 3.0 で Fix解と判定

---

## 推奨段取り

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

## 参考文献

| トピック | ソース |
|---------|--------|
| ASTM E2938-15 | 中距離3Dイメージング測距性能評価 |
| ISO 17123-9:2018 | TLSフィールド評価 |
| Microdrones LiDAR QC | [URL](https://www.microdrones.com/en/content/lidar-point-cloud-quality-control-automating-accuracy-and-precision-testing/) |
| GNSS Accuracy | [Juniper Systems](https://junipersys.com/support/article/6614) |
| GNSS SDR | [gnss-sdr.org](https://gnss-sdr.org/design-forces/accuracy/) |
| RTKLIB | T. Takasu, 2013 |

---

*元データ: [session1/mission-approach-report.md](../../sessions/session1/mission-approach-report.md)*
