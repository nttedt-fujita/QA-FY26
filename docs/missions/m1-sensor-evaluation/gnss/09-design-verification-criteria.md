# GNSS設計検証基準の調査レポート

**作成**: Session 53 (2026-03-09)
**目的**: 設計検証の合格基準に対するエビデンス収集

---

## 重要: 評価の文脈

**IMPORTANT**: この評価は**受入検査ではない**。

| 項目 | 内容 |
|------|------|
| **対象製品** | Holybro H-RTK F9P Ultralightの国内機能コピー製品 |
| **フェーズ** | **原理試作に対する設計検証**（3回目の試作） |
| **目的** | 検証項目が適切かどうかの検証 |
| **現状の合格基準** | **曖昧**（明確な基準がない） |

出典: [Session 16 小板橋さんヒアリング](../../../../sessions/session16/gnss-hearing-koitabashi-01.md)

> 今は原理試作に対しての設計検証をしているが、**受入検査でも何をするかを整理しておいたほうが良い**かも

→ 受入検査は**将来の課題**として言及されている。本調査は設計検証の基準が主目的だが、
  技術的な閾値（C/N0等）は設計検証でも参考になる。

---

## 調査の背景

Session 17-18で作成した「合格基準案（叩き台）」は、No.5の異常値を排除する形で逆算したもの。
業界標準・学術的根拠・メーカー仕様を調査し、基準の妥当性を検証する。

---

## 1. C/N0（受信感度）の閾値

### 業界標準

| 閾値 | 根拠 | ソース |
|------|------|--------|
| **30 dBHz** | トラッキングループ（DLL）がロックを失う閾値 | [Inside GNSS](https://insidegnss.com/measuring-gnss-signal-strength/)、[学術論文](https://pmc.ncbi.nlm.nih.gov/articles/PMC5038690/) |
| 35-60 dBHz | 実環境での典型的な測定レンジ | [GNSS Accuracy Metrics](https://gnsssimulator.com/gnss-accuracy-metrics-dop-cn0-ttff/) |
| 15 dBHz | 高感度受信機の限界値（特殊技術が必要） | 学術論文 |

### u-blox F9P推奨値

| 項目 | 値 | ソース |
|------|-----|--------|
| 良好な設計での高仰角衛星 | **40-50 dBHz** | [ZED-F9P Integration Manual](https://content.u-blox.com/sites/default/files/ZED-F9P_IntegrationManual_UBX-18010802.pdf) |
| 標準アクティブアンテナ | **47 dBHz** 容易に達成可能 | 同上 |
| 追尾感度 | -167 dBm | [ZED-F9P Datasheet](https://content.u-blox.com/sites/default/files/ZED-F9P-04B_DataSheet_UBX-21044850.pdf) |

### 学術的根拠

> "Tracking loops, such as DLLs, lose lock rapidly below C/N0 levels of 30 dBHz"
> （トラッキングループは30 dBHz以下で急速にロックを失う）

出典: [Weak and Dynamic GNSS Signal Tracking Strategies](https://pmc.ncbi.nlm.nih.gov/articles/PMC5038690/)

### 叩き台との比較

| 項目 | 叩き台 | 業界標準 | 判定 |
|------|--------|---------|------|
| L1受信感度 | ≥25 dBHz | **≥30 dBHz** | 叩き台は甘すぎる |

**結論**: 30 dBHzが業界標準。叩き台の25 dBHzは甘すぎる可能性。

---

## 2. RTK FIX時間

### 業界標準

| 条件 | 典型値 | ソース |
|------|--------|--------|
| 理想環境（開空） | **2-3秒** | [marXact](https://support.marxact.com/article/108-why-and-how-long-do-i-have-to-wait-for-a-rtk-fix) |
| 良好環境 | **1-10秒** | 同上 |
| 一般的KPI | **<30秒、FIX率>95%** | [RTK KPIs](https://rtkdata.com/blog/kpis-that-matter-for-rtk/) |
| 困難環境 | **30秒以上** | 同上 |
| 最新モジュール（初期化時間） | **<5秒** | [GNSS RTK Timing](https://medium.com/@ericco236/performance-analysis-of-gnss-rtk-timing-8072f3a1b9cb) |

### 叩き台との比較

| 項目 | 叩き台 | 業界標準 | 判定 |
|------|--------|---------|------|
| RTK FIX時間 | ≤3分 | **≤30秒** が一般的KPI | 叩き台は緩すぎる |

**結論**: 30秒以内が一般的なKPI。3分は緩すぎる可能性。ただし環境依存が大きい。

---

## 3. 水平精度

### RTK精度の標準

| 方式 | 精度 | ソース |
|------|------|--------|
| RTK（単一基地局） | **8mm + 1ppm 水平** | [Wikipedia RTK](https://en.wikipedia.org/wiki/Real-time_kinematic_positioning) |
| RTK（垂直） | **15mm + 1ppm 垂直** | 同上 |
| ネットワークRTK | **8mm + 0.5ppm 水平** | [Connecticut DOT](https://portal.ct.gov/dot/-/media/dot/aec/const_inspection/a3gpsguidelinesforrtnrtkgnssfieldworkinctpdf.pdf) |
| UAV測量 | **1-2 cm** | [MDPI UAV Positioning](https://www.mdpi.com/2504-446X/10/2/91) |

### UAV向け精度要件

| 要件 | 値 | ソース |
|------|-----|--------|
| Remote ID（FAA） | 高度精度 **±4.6m（15フィート）** 95%信頼度 | [FAA Remote ID](https://pilotinstitute.com/remote-id-gps-requirements/) |
| BVLOS飛行 | **cm級** が必要 | [u-blox BVLOS](https://www.u-blox.com/en/blogs/insights/cm-level-accuracy-bvlos-drones) |

### 叩き台との比較

| 項目 | 叩き台 | 業界標準 | 判定 |
|------|--------|---------|------|
| 飛行中水平精度 | ≤1.0m | RTK標準は**cm級** | 用途による |

**結論**: RTK本来の精度はcm級。1.0mは「RTKが機能していれば達成可能」な値であり、合否判定としては妥当かもしれないが、要確認。

---

## 4. 衛星数

### 推奨値

| 項目 | 値 | ソース |
|------|-----|--------|
| GPS単独 | **最低6衛星** | [Connecticut DOT](https://portal.ct.gov/dot/-/media/dot/aec/const_inspection/a3gpsguidelinesforrtnrtkgnssfieldworkinctpdf.pdf) |
| GNSS（複数系） | **7-8衛星** | 同上 |
| PDOP | **2-3以下** 推奨 | 同上 |

### 叩き台との比較

| 項目 | 叩き台 | 業界標準 | 判定 |
|------|--------|---------|------|
| 飛行中衛星数 | ≥20 | 最低6-8 | 叩き台は厳しめ（良い方向） |

**結論**: 叩き台の≥20は厳しめだが、F9Pは4系（GPS/GLONASS/Galileo/BeiDou）対応なので妥当かもしれない。

---

## 5. 関連規格

### IEC 61108シリーズ

| 規格 | 対象 |
|------|------|
| IEC 61108-1 | GPS受信機（船舶用） |
| IEC 61108-7 | SBAS L1受信機 |

出典: [Wikipedia IEC 61108](https://en.wikipedia.org/wiki/IEC_61108)

**注意**: 船舶用規格であり、UAV/ドローンには直接適用できない。

### ISO 24245:2023

GNSS受信機のクラスコードを定義。

出典: [ISO 24245:2023](https://www.iso.org/standard/80939.html)

---

## 6. 叩き台に含まれていない一般的な検査項目

### 追加調査で判明した検査項目

以下は一般的なGNSS性能評価で確認されるが、**現在の叩き台には含まれていない**項目。

| カテゴリ | 項目 | 説明 | 典型的な基準 | ソース |
|---------|------|------|-------------|--------|
| **TTFF** | Cold Start | 電源OFF長時間後の初回FIX時間 | **2-4分** | [Safran](https://safran-navigation-timing.com/basic-gnss-tests-time-to-first-fix/) |
| **TTFF** | Warm Start | 位置・時刻情報あり、エフェメリス再取得 | **≤45秒** | [Syntony](https://syntony-gnss.com/news/tech/understanding-gnss-receiver-start-modes-cold-warm-hot-and-direct) |
| **TTFF** | Hot Start | 全情報保持からの再起動 | **≤22秒（数秒も可）** | 同上 |
| **再捕捉** | Reacquisition Time | 信号遮断後の復帰時間（トンネル通過等） | 明確な基準なし（テスト推奨） | [GNSS SDR](https://gnss-sdr.org/design-forces/availability/) |
| **DOP** | PDOP | 位置精度の劣化係数 | **≤2-3**（理想は≤1） | [GIS Geography](https://gisgeography.com/gps-accuracy-hdop-pdop-gdop-multipath/) |
| **DOP** | HDOP | 水平精度の劣化係数 | **≤2** | 同上 |
| **耐干渉** | ジャミング耐性 | RF干渉時の動作 | テスト推奨（定量基準なし） | [GPS World](https://www.gpsworld.com/testing-gnss-receivers-against-jamming-and-spoofing-attacks/) |
| **耐干渉** | マルチパス耐性 | 反射波による誤差 | 環境依存（テスト推奨） | [Bench Mark USA](https://rtkgpssurveyequipment.com/surveying-with-rtk-what-does-multipath-mean/) |
| **設定** | ファームウェアバージョン | 期待バージョンとの一致 | 仕様書と照合 | [u-blox u-center](https://www.u-blox.com/en/product/u-center) |
| **設定** | 内部コンフィグ | GNSS設定パラメータの確認 | 仕様書と照合 | 同上 |
| **物理** | 外観検査 | 物理的損傷、汚れ | 目視確認 | [Safran](https://safran-navigation-timing.com/gnss-testing-what-tests-should-you-use/) |

### TTFF（Time To First Fix）の詳細

GNSSテストで最も基本的かつ重要な項目の一つ。**現在の叩き台には含まれていない**。

| 起動モード | 条件 | 典型値 | 測定方法 |
|-----------|------|--------|---------|
| Cold Start | 位置・時刻・エフェメリス全てなし | 2-4分 | 電源OFF→ON後、初回FIXまでの時間 |
| Warm Start | 位置・時刻あり、エフェメリスなし | ≤45秒 | 同上 |
| Hot Start | 全情報保持 | ≤22秒 | 同上 |

> "A minimum of 200 TTFF tests per receiver for each unit start condition (i.e., cold, warm, and hot) is recommended"
> （信頼性のある結果を得るには、各起動条件で最低200回のテストを推奨）

出典: [Safran - TTFF Testing](https://safran-navigation-timing.com/basic-gnss-tests-time-to-first-fix/)

### DOP（Dilution of Precision）の詳細

衛星配置による精度劣化の指標。**RTK FIX率や精度に直接影響**。

| DOP値 | 品質 |
|-------|------|
| <1 | 理想的 |
| 1-2 | 優秀 |
| 2-5 | 良好 |
| 5-10 | 中程度 |
| >10 | 不良 |

出典: [Wikipedia - Dilution of Precision](https://en.wikipedia.org/wiki/Dilution_of_precision_(navigation))

### 小板橋さんのExcelにあるが叩き台に反映されていない項目

[07-cross-sheet-findings.md](07-cross-sheet-findings.md) を確認すると、以下がExcelにはあるが叩き台の合格基準に反映されていない:

| 項目 | Excel記載 | 叩き台に反映 | コメント |
|------|----------|-------------|---------|
| Hot Start確認 | 電池確認シートで記録あり | ❌ | 手順自体が曖昧 |
| ジャミングスパイク | 飛行試験で観測 | ❌ | 閾値設定困難と記載 |
| イノベーション比率 | 飛行試験で記録 | ❌ | 必要性要確認 |
| スペアナ波形 | 画像キャプチャ | ❌ | 異常検出に使用したが基準化されていない |

---

## 参照ソース一覧

### C/N0関連
- [Inside GNSS - Measuring GNSS Signal Strength](https://insidegnss.com/measuring-gnss-signal-strength/)
- [GNSS Accuracy Metrics](https://gnsssimulator.com/gnss-accuracy-metrics-dop-cn0-ttff/)
- [PMC - Weak Signal Tracking](https://pmc.ncbi.nlm.nih.gov/articles/PMC5038690/)

### RTK FIX時間
- [marXact - RTK FIX Wait Time](https://support.marxact.com/article/108-why-and-how-long-do-i-have-to-wait-for-a-rtk-fix)
- [RTK KPIs That Matter](https://rtkdata.com/blog/kpis-that-matter-for-rtk/)

### TTFF・起動モード
- [Safran - TTFF Testing](https://safran-navigation-timing.com/basic-gnss-tests-time-to-first-fix/)
- [Syntony - GNSS Receiver Start Modes](https://syntony-gnss.com/news/tech/understanding-gnss-receiver-start-modes-cold-warm-hot-and-direct)
- [Wikipedia - Time to First Fix](https://en.wikipedia.org/wiki/Time_to_first_fix)

### DOP（精度劣化係数）
- [GIS Geography - GPS Accuracy HDOP PDOP GDOP](https://gisgeography.com/gps-accuracy-hdop-pdop-gdop-multipath/)
- [Wikipedia - Dilution of Precision](https://en.wikipedia.org/wiki/Dilution_of_precision_(navigation))

### 再捕捉・干渉耐性
- [GNSS SDR - Availability](https://gnss-sdr.org/design-forces/availability/)
- [GPS World - Testing Against Jamming and Spoofing](https://www.gpsworld.com/testing-gnss-receivers-against-jamming-and-spoofing-attacks/)
- [Bench Mark USA - RTK Multipath](https://rtkgpssurveyequipment.com/surveying-with-rtk-what-does-multipath-mean/)

### u-blox F9P
- [ZED-F9P Integration Manual](https://content.u-blox.com/sites/default/files/ZED-F9P_IntegrationManual_UBX-18010802.pdf)
- [ZED-F9P Datasheet](https://content.u-blox.com/sites/default/files/ZED-F9P-04B_DataSheet_UBX-21044850.pdf)
- [u-blox u-center](https://www.u-blox.com/en/product/u-center)

### UAV/ドローン
- [MDPI - UAV Positioning Using GNSS](https://www.mdpi.com/2504-446X/10/2/91)
- [FAA Remote ID GPS Requirements](https://pilotinstitute.com/remote-id-gps-requirements/)
- [u-blox BVLOS Drones](https://www.u-blox.com/en/blogs/insights/cm-level-accuracy-bvlos-drones)

### 規格
- [Wikipedia - IEC 61108](https://en.wikipedia.org/wiki/IEC_61108)
- [ISO 24245:2023](https://www.iso.org/standard/80939.html)
- [Connecticut DOT RTK Guidelines](https://portal.ct.gov/dot/-/media/dot/aec/const_inspection/a3gpsguidelinesforrtnrtkgnssfieldworkinctpdf.pdf)

### GNSSテスト一般
- [Safran - GNSS Testing: What Tests Should You Use?](https://safran-navigation-timing.com/gnss-testing-what-tests-should-you-use/)
- [Safran - Creating a GNSS Test Plan](https://safran-navigation-timing.com/creating-a-gnss-test-plan/)
