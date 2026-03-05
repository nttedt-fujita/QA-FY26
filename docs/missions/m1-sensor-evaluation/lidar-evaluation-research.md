# Lidar評価環境 Web調査結果

**目的**: M1-A（Lidar評価）の環境構築に必要な情報収集

---

## 1. 関連規格

### ASTM E2938-15
- **概要**: 3Dイメージングシステムの相対距離測定性能を評価する試験方法
- **測定範囲**: 2m〜150m（中距離）
- **対象**: Time-of-Flight方式のレーザースキャナー
- **参照**: [ASTM E2938](https://store.astm.org/e2938-15.html)

### ASTM E3125-17
- **概要**: 自動車用LiDARセンサーの性能評価
- **対象**: MEMS式自動車用LiDAR
- **AEC-Q100 Grade 2**: 車載電子部品の信頼性試験規格
- **参照**: [MDPI論文](https://www.mdpi.com/1424-8220/23/6/3113)

### ISO/CD 13228（策定中）
- **概要**: 自動車用LiDARの試験方法を定める国際規格
- **状況**: 現在Committee Draft段階
- **参照**: [ISO](https://www.iso.org/standard/84347.html)

### ISO/TS 19159-2
- **概要**: 航空LiDARセンサーの校正プロセス（幾何・放射）
- **用途**: UAV搭載LiDARの精度評価

---

## 2. 国内の評価サービス候補

### 試験受託サービス会社

| 会社 | サービス内容 | 備考 |
|------|-------------|------|
| **HORIBA Techno Service** | 車載センサー評価（HTS Automotive Testing Center） | 2025年本格稼働、京都 |
| **エスペック** | 環境試験・信頼性試験 | LiDAR特化ではないが汎用試験可能 |
| **神戸清光** | 測量機器の点検・校正 | 3Dスキャナー対応 |
| **JTL** | 各種受託評価 | 風洞・音響等 |
| **Leica Geosystems** | 校正サービス（ISO 9001認証） | グローバル対応 |

### LiDAR関連会社（国内）

| 会社 | 事業内容 |
|------|----------|
| **北陽電機** | 測域センサ（LiDAR）メーカー |
| **岡谷エレクトロニクス (OEC)** | LiDAR評価スターターキット提供 |
| **Locus Blue** | 点群処理・データ解析 |
| **Terra Drone** | UAV + LiDAR測量 |

---

## 3. 評価機材

### リファレンスターゲット

| 製品 | メーカー | 用途 |
|------|----------|------|
| **Permaflect® Targets** | Labsphere | LiDAR試験用反射ターゲット |
| **Ground Control Targets** | Routescene | UAV LiDAR精度検証用 |
| **3D Towered Checkerboard** | 学術研究 | カメラ-LiDAR校正用 |

### 校正設備の目安

- **UAV LiDAR**: 500m×500m調査に4〜8個のGCPターゲットが必要
- **地上3Dスキャナー**: FARO Focus Premiumで±1mm精度
- **国土地理院基準**: 航空レーザー測量で標高誤差5〜10cm

---

## 4. 評価アプローチの選択肢

### Option A: 外部委託

| メリット | デメリット |
|----------|-----------|
| 専門知識不要 | コスト高 |
| 設備投資不要 | 依存度が上がる |
| 早期開始可能 | ノウハウ蓄積しにくい |

**候補**: HORIBA Techno Service（車載LiDAR評価）

### Option B: 自社構築

| メリット | デメリット |
|----------|-----------|
| ノウハウ蓄積 | 設備投資必要 |
| 柔軟な評価可能 | 立ち上げに時間 |
| 長期的コスト減 | 専門人材必要 |

**必要なもの**:
- リファレンスターゲット
- 既知点（GCP）の設置
- 評価用ソフトウェア

### Option C: ハイブリッド

| 初期 | 中長期 |
|------|--------|
| 外部委託で規格準拠を学ぶ | 自社で簡易評価を実施 |
| 評価基準・手順を取得 | 定期的な外部校正のみ委託 |

---

## 5. 概算コスト（推定）

**注意**: 具体的な見積もりは各社への問い合わせが必要

| 項目 | 概算 | 備考 |
|------|------|------|
| リファレンスターゲット | 数万〜数十万円 | サイズ・反射率による |
| GCP設置・測量 | 数十万円〜 | 面積・精度による |
| 外部試験委託（1回） | 要問い合わせ | HORIBA等 |
| 評価スターターキット | 要問い合わせ | OEC等 |

---

## 6. 次のアクション

### 短期（今週〜来週）

- [ ] HORIBA Techno Service に問い合わせ（車載LiDAR評価サービスの有無・費用）
- [ ] OEC に問い合わせ（LiDAR評価スターターキットの内容・価格）
- [ ] 北陽電機に問い合わせ（評価支援の有無）

### 中期（今月中）

- [ ] 社内で使用予定のLiDARセンサーの型番・仕様を確認
- [ ] 評価要件の明確化（何を測りたいか）
- [ ] Option A/B/Cの比較検討資料作成

---

## 7. 情報ソース

- [ASTM E2938 - LiDAR News](https://lidarnews.com/articles/astm-e2938-15-putting-laser-scanners-to-the-test/)
- [NIST Performance Evaluation](https://tsapps.nist.gov/publication/get_pdf.cfm?pub_id=930840)
- [YellowScan - LiDAR Accuracy Standards](https://www.yellowscan.com/knowledge/lidar-accuracy-standards-what-industry-tests-prove/)
- [HORIBA Techno Service](https://www.horiba.com/ind/mobility/service/testing/techno-service/)
- [OEC LiDAR評価スターターキット](https://www.oec.okaya.co.jp/solution/iot_sol/3dlidar/)
- [Labsphere Permaflect Targets](https://www.labsphere.com/product/permaflect-targets-rigid-lidar-commercial-and-custom/)
- [kumonos - 3Dレーザースキャナの精度](https://kumonos.co.jp/media/3d-laser-scanner-precision/)
