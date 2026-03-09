# M2: 点群データ検証方法の策定

**担当**: ふじた
**難易度**: 中〜高
**依存関係**: M1と並行可能
**ステータス**: 方針転換中（Session 52）

---

## ⚠️ 方針転換（Session 52）

**問題**: 本README.mdに記載の評価方法（ASPRS精度基準、GCP設置等）は**測量・マッピング用途向け**。

**AirGrowの実際の用途**: **リアルタイム障害物検知**（みかん畑の上を飛行し、木を検知して避ける）

**結論**: 「点群データ検証」ではなく**「障害物検知システムの評価」**として再定義が必要。

### 障害物検知評価（新アプローチ）

詳細は [obstacle-detection/](obstacle-detection/) を参照。

| ファイル | 内容 |
|----------|------|
| [obstacle-detection/m2-obstacle-detection-report.md](obstacle-detection/m2-obstacle-detection-report.md) | 調査レポート（業界規格・指標、ソース併記） |
| [obstacle-detection/m2-confirmation-checklist.md](obstacle-detection/m2-confirmation-checklist.md) | FA率評価方法の確認リスト |

### 障害物検知の評価指標（調査結果）

| 指標 | 定義 | 参考値 |
|------|------|--------|
| **False Alarm率** | 存在しない障害物を検知する割合 | ← AirGrowの現在の課題 |
| **検知距離** | 障害物を検知できる最大距離 | Vision系: 15-30m |
| **反応時間** | 検知から回避動作開始までの遅延 | Vision系: 50-200ms |
| **見逃し率** | 存在する障害物を見逃す割合 | - |

### 関連規格

- **EUROCAE ED-267**: 欧州の低高度UAS向けDAA（Detect and Avoid）運用要件

### 次のアクション

1. 開発側にFA率評価方法を確認（確認リスト参照）
2. 評価方法の具体化
3. 本README.mdの再構成

---

## 概要（従来）

> **注意**: 以下の内容は測量・マッピング用途向け。AirGrowには適用しない可能性が高い。

AirGrowが取得する点群データの品質を検証するための方法を策定する。

---

## 品質指標

| 品質指標 | 定義 | 目標値例 |
|---------|------|---------|
| **点密度** | 単位面積当たりの点数 (pts/m²) | ≥ 8 pts/m² (USGS QL1) |
| **点密度の均一性** | 点密度の変動係数 (CV) | CV < 30% |
| **ノイズ率** | 全点数に対する外れ値の割合 | < 0.5% |
| **絶対精度** | GCPに対する系統的誤差 (RMSE) | RMSE_z < 10cm |
| **相対精度** | ストリップ間の整合性 | < 7cm |
| **網羅性** | 計測対象エリアのカバー率 | データボイド < 4 × NPS² |
| **分類精度** | 点群分類の正答率 | 地表面分類 > 95% |

---

## ASPRS精度基準

> **ASPRS Positional Accuracy Standards for Digital Geospatial Data (Edition 2, 2023)**:
> "Vertical accuracy shall be tested by comparing the elevations of the dataset with elevations of higher-accuracy check points."
> "A minimum of 30 checkpoints are required by ASPRS for 'Tested to Meet ...' accuracy reporting."

| 精度クラス | RMSE_z (cm) | NVA 95% (cm) |
|-----------|-------------|-------------|
| 1 cm | 1.0 | 2.0 |
| 5 cm | 5.0 | 9.8 |
| 10 cm | 10.0 | 19.6 |
| 20 cm | 20.0 | 39.2 |

**計算式**:
- `RMSE_z = √[ Σ(z_data,i - z_check,i)² / n ]`
- `NVA 95% = 1.9600 × RMSE_z`

---

## 点群品質評価の7つの方法

> **出典**: [Map Library — 7 Ways to Evaluate Point Cloud Accuracy](https://www.maplibrary.org/10350/how-to-evaluate-point-cloud-accuracy-and-quality/)

| # | 方法 | 説明 |
|---|------|------|
| 1 | **統計分析による幾何学的精度測定** | RMSE・点間距離分析で参照座標との偏差を定量化 |
| 2 | **点群密度とカバレッジ評価** | グリッドセル内の点密度統計で均一性を検証 |
| 3 | **ノイズレベルと外れ値検出** | 統計的フィルタリングで局所表面フィッティングとの変動を測定 |
| 4 | **表面再構成品質の検証** | ハウスドルフ距離・曲率分析 |
| 5 | **登録・位置合わせ精度分析** | 複数スキャンの重複領域で変換行列の精度を評価 |
| 6 | **自動品質制御ワークフロー** | 閾値ベースの自動検証プロセス導入 |
| 7 | **専門ソフトウェア活用** | CloudCompare等で標準化レポート生成 |

---

## 検証ツール

| ツール | 用途 | ライセンス |
|-------|------|-----------|
| **CloudCompare** | 点群比較・解析（M3C2, ICP） | GPL v2 (OSS) |
| **PDAL** | 点群処理パイプライン | BSD (OSS) |
| **LAStools** | LASファイル処理・品質チェック | 一部商用 |
| **PCL** | 点群処理（フィルタ、特徴量等） | BSD (OSS) |
| **RTKLIB** | GNSS後処理解析 | BSD-2 (OSS) |

---

## 推奨段取り

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

## 参考文献

| トピック | ソース |
|---------|--------|
| ASPRS Standards | [ASPRS PDF](https://www.asprs.org/wp-content/uploads/2015/01/ASPRS_Positional_Accuracy_Standards_Edition1_Version100_November2014.pdf) |
| 点群品質評価 | [Map Library](https://www.maplibrary.org/10350/how-to-evaluate-point-cloud-accuracy-and-quality/) |
| USGS Lidar Base Spec | v2.1 |

---

*元データ: [session1/mission-approach-report.md](../../sessions/session1/mission-approach-report.md)*
