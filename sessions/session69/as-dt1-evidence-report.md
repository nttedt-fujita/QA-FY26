# ソニー製LiDAR AS-DT1 質問リスト エビデンスレポート

**作成日**: 2026-03-10 (Session 69)
**目的**: 質問リストの各項目について、競合製品・業界標準との比較を通じて質問の妥当性を示す

---

## 1. 競合製品との仕様比較

### 1.1 ドローン向け小型LiDARセンサー比較

| 項目 | Sony AS-DT1 | Livox Mid-360 | Livox AVIA |
|------|-------------|---------------|------------|
| **重量** | 46g | 265g | 498g |
| **寸法** | 29×29×31mm | 65×65×60mm | 91×61×65mm |
| **IP等級** | **不明** | IP67 | IP67 |
| **動作温度** | -5〜+45℃ | -20〜+55℃ | -20〜+65℃ |
| **耐衝撃** | 200G | 不明 | 不明 |
| **耐振動** | **不明** | 不明 | 不明 |
| **検出距離** | 40m (10%反射率) | 40m (10%反射率) | 450m (80%反射率) |
| **消費電力** | 2.5W | 6.5W | 9W |
| **波長** | 940nm | 905nm | 905nm |
| **価格** | **不明** | $999 | $649 |

**出典**:
- [Livox Mid-360 Specs](https://www.livoxtech.com/mid-360/specs)
- [Livox AVIA Specs](https://www.livoxtech.com/avia/specs)
- Sony AS-DT1: ユーザーズガイド FW1.00 p31

### 1.2 AS-DT1の特徴

**強み**:
- 極めて軽量（46g）— 競合の1/5〜1/10
- コンパクト（29×29×31mm）— 競合の1/3以下の体積
- 低消費電力（2.5W）— 競合の1/3〜1/4
- 耐衝撃200G（ユーザーズガイドに記載）

**不明点**:
- IP等級
- 耐振動性能
- 価格

---

## 2. 業界標準・基準

### 2.1 IP等級（防塵・防水）

| IP等級 | 防塵 | 防水 | 用途例 |
|--------|------|------|--------|
| IP65 | 完全防塵 | 噴流水 | 産業機器、屋外カメラ |
| IP66 | 完全防塵 | 暴噴流水 | 過酷な屋外環境 |
| IP67 | 完全防塵 | 水没（1m/30分） | ドローンセンサー標準 |

**ドローン搭載センサーの業界傾向**:
> 「IP67 is essential for mission-critical applications, such as in defense, rescue operations, or offshore inspections」
> — [Coptrz: What is an IP Rating?](https://coptrz.com/blog/what-is-an-ip-rating-a-guide-to-understanding-drone-ip-ratings/)

**競合製品の状況**:
- Livox Mid-360: IP67
- Livox AVIA: IP67
- GreenValley LiAir X3C-H: IP67

**→ Q13（IP等級）の妥当性**: ドローン向けLiDARセンサーはIP67が標準的。AS-DT1のIP等級が不明なのは仕様確認の必須事項。

### 2.2 耐振動性能

**ドローン搭載機器の耐振動規格**:
- MIL-STD-810H Method 514.8: 軍用・航空宇宙機器の振動試験標準
- ドローン搭載機器は様々な振動・衝撃にさらされる

> 「drones are often exposed to a wide range of environmental conditions during their operational life... vibration and shock, which subjects drones to mechanical vibration and shock to test their durability」
> — [House of Testing: Military Standards Drone Test](https://houseoftesting.com/military-standards-drone-test/)

**AS-DT1の状況**:
- 耐衝撃: 200G（7-8ms）— ユーザーズガイドに記載あり
- 耐振動: **記載なし**

**→ Q14（耐振動性能）の妥当性**: 耐衝撃は記載あるが耐振動は不明。ドローン搭載には継続的な振動への耐性が重要。

### 2.3 環境光条件

| 条件 | 照度 |
|------|------|
| 室内 | 500〜1,000 lx |
| 曇天屋外 | 10,000〜25,000 lx |
| 晴天屋外 | 40,000〜80,000 lx |
| 真夏直射日光 | 100,000〜120,000 lx |

**AS-DT1の仕様書記載**:
- 屋外性能データ: 40,000 lx まで（ユーザーズガイド p27）

**Web情報**:
> 「can operate under bright conditions (up to 100,000 lux)」
> — [CNX Software](https://www.cnx-software.com/2025/04/23/sony-as-dt1-tiny-industrial-lidar-depth-sensor-can-operate-under-bright-conditions-up-to-100000-lux/)

**→ Q15（直射日光性能）の妥当性**: 仕様書では40,000 lxまでのデータのみ。Web情報では100,000 lx対応との記載あり。正式な仕様確認が必要。

---

## 3. 各質問のエビデンス整理

### Q01/Q02: 反射率10%未満の検出性能

**仕様書情報**:
- 測定データは10/50/80%反射率で提供（ユーザーズガイド p26-27）
- 10%未満のデータなし

**競合製品の記載**:
- Livox Mid-360: 「40 m @ 10% reflectivity」と明記

**Web情報**:
> 「Capable of measuring distances to low-contrast subjects and objects with low reflectivity」
> — [Newsshooter](https://www.newsshooter.com/2025/04/10/sony-as-dt1-the-worlds-smallest-and-lightest-miniature-precision-lidar-depth-sensor/)

**→ 質問の妥当性**: 黒い物体（反射率5%以下）の検出可否はドローン障害物検知に重要。具体的な下限値の確認が必要。

---

### Q09: ROS/ROS2対応ドライバー

**仕様書情報**:
- Python SDKの提供あり（APIマニュアル p61-63）
- ROS/ROS2ドライバーの記載なし

**社内実績**:
- `SONY-LiDAR-TestMinutes`プロジェクトでvendor提供のROS2ドライバー使用実績あり
- ドライバー名: `as_dt1_ros2_driver`

**Web調査結果**:
> 「Currently, there is no explicit ROS2 driver support information available for the Sony AS-DT1」
> — Web検索結果（2026-03-10）

**→ 質問の妥当性**: 社内ではvendor提供ドライバーで動作実績あり。正式提供か、提供形態（OSS/有償等）の確認が必要。

---

### Q13: IP等級

**仕様書情報**:
- 「防塵防水のためのカバーガラスを採用」（ユーザーズガイド p3）
- IP等級の明記なし
- 「直接水がかかる環境では使用しないでください」（ユーザーズガイド p8）

**Web情報（矛盾あり）**:
> 「IP65-rated enclosure (dust-tight, water-resistant)」
> — [CNX Software](https://www.cnx-software.com/2025/04/23/sony-as-dt1-tiny-industrial-lidar-depth-sensor-can-operate-under-bright-conditions-up-to-100000-lux/)

**競合製品**:
- Livox Mid-360/AVIA: **IP67**

**→ 質問の妥当性**: 仕様書にはIP等級なし。Web情報ではIP65との記載あるが信頼性不明。競合がIP67の中、正式な等級確認が必須。

---

### Q14: 耐振動性能

**仕様書情報**:
- 耐衝撃: 200G（7-8ms）— 記載あり
- 耐振動: **記載なし**

**業界標準**:
- MIL-STD-810H Method 514.8が航空宇宙機器の標準
- ドローン搭載機器は飛行中の継続的振動に耐える必要あり

**→ 質問の妥当性**: 衝撃と振動は異なる負荷。ドローン飛行中の連続振動への耐性確認が必要。

---

### Q15: 直射日光（100,000 lx超）での性能

**仕様書情報**:
- 屋外性能データ: 40,000 lxまで（ユーザーズガイド p27）

**Web情報**:
> 「can operate under bright conditions (up to 100,000 lux)」
> — [CNX Software](https://www.cnx-software.com/2025/04/23/sony-as-dt1-tiny-industrial-lidar-depth-sensor-can-operate-under-bright-conditions-up-to-100000-lux/)

**→ 質問の妥当性**: 真夏の屋外飛行では100,000 lx超の可能性あり。仕様書のデータと矛盾するWeb情報の真偽確認が必要。

---

### Q16: 高温時の復帰条件

**仕様書情報**:
- 「センサー温度が70℃を超えると寿命が低下する」（ユーザーズガイド p8）
- 「70℃を超えないように」という注意のみ
- 復帰条件（自動復帰？手動リセット？）の記載なし

**→ 質問の妥当性**: 夏場のドローン運用で高温になる可能性あり。異常時の運用手順設計に必要。

---

### Q19: ケーブル仕様

**仕様書情報**:
- コネクタ型番: JST GHR-08V-S（8ピン）
- ケーブル長・耐久性の記載なし

**→ 質問の妥当性**: 配線設計・ドローン搭載時の取り回しに必要。

---

### Q20-Q26: 納入・商務条件

**Web調査結果**:
- 価格・MOQ等の情報は公開されていない
- 発売時期: Spring 2026（2026年春）

**競合製品の参考価格**:
- Livox Mid-360: $999
- Livox AVIA: $649

**→ 質問の妥当性**: 調達計画に必須。競合製品との価格競争力も評価ポイント。

---

## 4. Web調査ソース一覧

| ソース | URL | 取得情報 |
|--------|-----|---------|
| CNX Software | https://www.cnx-software.com/2025/04/23/sony-as-dt1-tiny-industrial-lidar-depth-sensor-can-operate-under-bright-conditions-up-to-100000-lux/ | IP65、100,000 lx対応、-20〜+60℃ |
| Newsshooter | https://www.newsshooter.com/2025/04/10/sony-as-dt1-the-worlds-smallest-and-lightest-miniature-precision-lidar-depth-sensor/ | 低反射率対応、寸法・重量 |
| Advexure | https://advexure.com/blogs/news/top-7-lidar-sensors-for-drones | ドローン向けLiDAR一覧、競合製品仕様 |
| Livox Mid-360 | https://www.livoxtech.com/mid-360/specs | 競合製品仕様（IP67、重量、価格） |
| Livox AVIA | https://www.livoxtech.com/avia/specs | 競合製品仕様（IP67、重量） |
| Coptrz | https://coptrz.com/blog/what-is-an-ip-rating-a-guide-to-understanding-drone-ip-ratings/ | ドローンIP等級の業界標準 |
| House of Testing | https://houseoftesting.com/military-standards-drone-test/ | MIL-STD-810 ドローン試験 |

---

## 5. 仕様書からの原文抜粋

### 防塵・防水関連

> 「防塵防水のためのカバーガラスを採用しています。」
> — ユーザーズガイド FW1.00 p3

> 「直接水がかかる環境では使用しないでください。」
> — ユーザーズガイド FW1.00 p8

### 温度・寿命関連

> 「センサー温度が70℃を超えると、センサー寿命が低下する可能性があります。ご注意ください。70℃を超えないように放熱板の使用を推奨します。」
> — ユーザーズガイド FW1.00 p8

### 反射率関連

> 「測定被写体反射率: 10% / 50% / 80%」
> — ユーザーズガイド FW1.00 p26

---

*作成: Session 69 (2026-03-10)*
