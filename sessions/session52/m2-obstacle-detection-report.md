# M2: 障害物検知評価 調査レポート

**作成日**: 2026-03-09 (Session 52)
**目的**: AirGrowの障害物検知システム評価方法策定のための事前調査

---

## 1. 調査背景

M2「点群データ検証方法の策定」の実態を確認したところ、AirGrowのLidar用途は**測量・マッピングではなく、リアルタイム障害物検知**であることが判明。

従来のREADME.mdに記載していた評価方法（ASPRS精度基準、GCP設置等）は測量用途向けであり、AirGrowには適用できない可能性が高い。

**現在の課題**: False Alarm（誤検知）低減作業が進行中

---

## 2. 評価指標（調査結果）

### 2.1 検知精度に関する指標

| 指標 | 定義 | 調査で得られた参考値 |
|------|------|---------------------|
| **検知精度 (Detection Accuracy)** | 障害物を正しく検知する割合 | 深層学習系: 98.10% |
| **Precision** | 検知したもののうち正しいものの割合 | 97.52% |
| **Recall** | 実際の障害物のうち検知できた割合 | 97.54% |
| **F1-score** | PrecisionとRecallの調和平均 | 97.28% |
| **IoU (Intersection over Union)** | 検知領域と実際の領域の重なり | 96.1% |

**ソース**: [Ensuring UAV Safety: A Vision-only and Real-time Framework for Collision Avoidance](https://arxiv.org/html/2405.06749v1)

> "Recent deep learning-based approaches achieved 98.10% obstacle detection accuracy, alongside high precision (97.52%), recall (97.54%), F1-score (97.28%), and Intersection over Union (96.1%)."

---

### 2.2 反応時間に関する指標

| 指標 | 定義 | 調査で得られた参考値 |
|------|------|---------------------|
| **反応時間 (Reaction Time)** | 検知から回避動作開始までの遅延 | Vision系: 50-200ms |
| **システムレイテンシ** | センサ入力から制御出力までの全体遅延 | 高速システム: 3.5ms |

**ソース**: [Dynamic obstacle avoidance for quadrotors with event cameras](https://www.science.org/doi/10.1126/scirobotics.aaz9712)

> "Today's autonomous drones have reaction times of tens of milliseconds, which is not enough for navigating fast in complex dynamic environments."
>
> "Event cameras, which are bioinspired sensors with reaction times of microseconds, can achieve an overall latency of only 3.5 milliseconds."

---

### 2.3 検知距離に関する指標

| 指標 | 定義 | 調査で得られた参考値 |
|------|------|---------------------|
| **検知距離 (Detection Range)** | 障害物を検知できる最大距離 | Vision系: 15-30m |
| **最小安全距離** | 衝突回避に必要な最小距離 | 4-6m（中型ドローン） |

**ソース**: [Drone Obstacle Avoidance Calculator - Fly Eye](https://www.flyeye.io/drone-calcaulators-obstacle-avoidance/)

> "Vision-based obstacle detection systems typically have a range of 15 to 30 meters."
>
> "For most mid-range drones, staying at least 4–6 meters from solid objects is a safe rule."

---

### 2.4 速度と検知距離の関係

| 飛行速度 | 必要検知距離 | 利用可能時間 | 備考 |
|---------|------------|------------|------|
| 4 m/s | 7m | 1.75s | 反応時間350ms想定 |
| 15 m/s | 6m（超音波） | 0.4s | ギリギリ |
| 15 m/s（対向） | 6m | 0.2s | 不十分 |

**ソース**: [Obstacle Detection and Collision Avoidance for a UAV With Complementary Low-Cost Sensors](https://ieeexplore.ieee.org/document/7105819/)

> "When considering a UAV traveling at maximum speed (15 m/s), ultrasonic sensors with a 6 m range provide only 0.4 s traversal time, and in the worst case with an obstacle moving at 15 m/s head on, the available reaction time of just 0.2 s was clearly insufficient."

---

### 2.5 処理性能に関する指標

| 指標 | 定義 | 調査で得られた参考値 |
|------|------|---------------------|
| **処理時間** | 1回の最適化にかかる時間 | Nonlinear Constraint: 0.89s |
| **位置誤差** | 追跡精度 | 0.15m（制約あり）→ 0.51m（緩和時） |

**ソース**: [Predictive Modeling for UAV Collision Avoidance](https://anvil.so/post/predictive-modeling-uav-collision-avoidance)

> "The 'Nonlinear Constraint' method delivered the best collision avoidance performance with a processing time of 0.89 seconds per optimization."
>
> "When collision constraints were relaxed into a 'penalty cost' to save computational resources, tracking accuracy dropped significantly, with mean position errors increasing from 0.15 meters to 0.51 meters."

---

## 3. 業界規格

### 3.1 EUROCAE ED-267（欧州）

| 項目 | 内容 |
|------|------|
| **正式名称** | OSED for Detect and Avoid in Very Low Level Operations |
| **発行** | 2020年、EUROCAE WG-105 SG-13 |
| **対象** | 低高度UAS運用におけるDAA（Detect and Avoid） |
| **検知対象** | 交通、固定/移動障害物、地形、気象、野生動物、地上の人 |

**ソース**: [EUROCAE ED-267](https://www.eurocae.net/ed-267-osed-for-detect-and-avoid-in-very-low-level-operations/)

> "ED-267 provides the Operational Services and Environment Definition (OSED) for Detect and Avoid (DAA) capabilities to support very low level (VLL) operations conducted by unmanned aircraft systems (UAS)."
>
> "It is intended to be used as a supporting document for implementation of ED-78A/DO-264 process in the definition of Safety and Performance Requirements (SPR) and Interoperability requirements (INTEROP)."

**位置づけ**:
- MASPS（最小航空システム性能標準）の基礎
- MOPS（最小運用性能仕様）の基礎

---

### 3.2 FAA Part 108（米国、策定中）

| 項目 | 内容 |
|------|------|
| **対象** | BVLOS（目視外飛行）向けDAA要件 |
| **状況** | 2025年後半に追加規定（Part 74）リリース予定 |

**ソース**: [Part 108: What We Know So Far about the FAA's BVLOS Rule](https://www.dronepilotgroundschool.com/part-108/)

> "Minimum operational standards for drone performance will likely include specifications for flight endurance, communication links, and navigation accuracy."
>
> "Detect-and-Avoid (DAA) or Sense-and-Avoid systems are required for BVLOS drones since they don't rely on human visual awareness."

---

## 4. 商用ドローンの実装例

### 4.1 Skydio 2

| 項目 | 仕様 |
|------|------|
| センサ | 6台の4Kカメラ（360°全方位、魚眼レンズ） |
| プロセッサ | NVIDIA Tegra X2 |
| 回避可能速度 | 最大36 mph（約16 m/s） |
| 更新レート | 100万点/秒以上 |
| 処理方式 | 9つのカスタムDNNで3Dモデル構築 |

**ソース**: [Skydio's obstacle avoidance and self-flying capability explained](https://dronedj.com/2019/10/01/skydios-obstacle-avoidance-self-flying-capability/)

> "The Skydio 2 uses six 4K cameras to collect data about its surroundings, with a super fisheye lens for 360° view, giving the Skydio 2 true omnidirectional obstacle detection including above and below."
>
> "The Skydio 2 uses an NVIDIA Tegra X2 to process all the data that is collected by the six cameras. This processor helps the drone avoid obstacles up to a speed of 36 mph and runs nine custom deep networks to create a 3D model of its surroundings with an update rate of more than 1 million points per second."

**注意**: 公式の障害物回避仕様値は「推定値」であり、環境やフライトモードにより変動

---

### 4.2 DJI

**ソース**: [Introduction to the Aircraft Obstacle Avoidance System](https://support.dji.com/help/content?customId=en-us03400006547&spaceId=34&re=US&lang=en&documentType=artical&paperDocType=paper)

- 複数センサ（Vision + ToF + 超音波）の組み合わせ
- 機種ごとに検知範囲・対応角度が異なる

---

### 4.3 実機比較テスト

**ソース**: [DJI Mavic 3 vs Skydio 2: Obstacle Avoidance Detailed Testing](https://www.dcrainmaker.com/2021/11/dji-mavic-3-skydio-2-obstacle-avoidance-testing.html)

実際の環境でのテスト動画・結果が公開されている。評価方法の参考になる可能性あり。

---

## 5. 学術研究

### 5.1 LiDARセンサフュージョン

**ソース**: [Autonomous aerial obstacle avoidance using LiDAR sensor fusion](https://pmc.ncbi.nlm.nih.gov/articles/PMC10306222/)

LiDARを使った自律的な障害物回避に関する研究。センサフュージョンのアプローチが参考になる可能性。

### 5.2 障害物検知サーベイ

**ソース**: [A Survey on Obstacle Detection and Avoidance Methods for UAVs](https://www.mdpi.com/2504-446X/9/3/203)

UAVの障害物検知・回避手法の包括的なサーベイ論文。

---

## 6. AirGrow向け評価項目（案）

### 優先度高

| 項目 | 評価方法（案） | 合格基準（要検討） |
|------|--------------|-----------------|
| **False Alarm率** | 障害物なし環境での誤検知回数を計測 | TBD |
| **検知距離** | 既知距離に障害物を配置、検知可否を確認 | TBD |
| **反応時間** | 障害物出現から回避開始までの時間を計測 | TBD |

### 優先度中

| 項目 | 評価方法（案） | 合格基準（要検討） |
|------|--------------|-----------------|
| **見逃し率** | 障害物あり環境での見逃し回数を計測 | TBD |
| **速度依存性** | 飛行速度を変えて回避成功率を測定 | TBD |

---

## 7. 参照リンク一覧

| # | トピック | URL |
|---|---------|-----|
| 1 | EUROCAE ED-267 | https://www.eurocae.net/ed-267-osed-for-detect-and-avoid-in-very-low-level-operations/ |
| 2 | DAA規格サーベイ | https://www.mdpi.com/2226-4310/12/4/344 |
| 3 | 障害物検知サーベイ | https://www.mdpi.com/2504-446X/9/3/203 |
| 4 | Skydio技術説明 | https://dronedj.com/2019/10/01/skydios-obstacle-avoidance-self-flying-capability/ |
| 5 | LiDARセンサフュージョン研究 | https://pmc.ncbi.nlm.nih.gov/articles/PMC10306222/ |
| 6 | DJI vs Skydio比較テスト | https://www.dcrainmaker.com/2021/11/dji-mavic-3-skydio-2-obstacle-avoidance-testing.html |
| 7 | DJI障害物回避システム | https://support.dji.com/help/content?customId=en-us03400006547 |
| 8 | 障害物回避計算機 | https://www.flyeye.io/drone-calcaulators-obstacle-avoidance/ |
| 9 | FAA Part 108説明 | https://www.dronepilotgroundschool.com/part-108/ |
| 10 | イベントカメラ研究 | https://www.science.org/doi/10.1126/scirobotics.aaz9712 |
| 11 | 衝突回避予測モデリング | https://anvil.so/post/predictive-modeling-uav-collision-avoidance |
| 12 | 低コストセンサ研究 | https://ieeexplore.ieee.org/document/7105819/ |
| 13 | UAV安全性フレームワーク | https://arxiv.org/html/2405.06749v1 |
