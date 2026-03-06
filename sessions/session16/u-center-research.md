# u-center 調査結果

**調査日**: 2026-03-06
**目的**: GNSS評価ツール設計の前提情報収集

---

## u-centerとは

**提供元**: u-blox社（スイス）
**種類**: GNSS評価・設定ソフトウェア（Windows向け）
**対応チップ**: M8, M9, F9シリーズ（**F9P対応**）

### バージョン

| バージョン | 対応世代 | 備考 |
|------------|----------|------|
| u-center（レガシー） | M8, M9, F9等 | 現在使用中 |
| u-center 2 | Gen 10, Gen 20（F10, M10, X20） | 新世代向け |

---

## 主要機能

### 1. 衛星追跡・表示

| 機能 | 説明 |
|------|------|
| Sky View | 衛星の空間配置を円形図で表示 |
| Satellite Level | 衛星ごとの信号強度をバーグラフ表示 |
| SNR表示 | Signal-to-Noise Ratio（信号対雑音比） |

**小板橋さんの言う「スペアナ機能」**:
→ おそらく **Satellite Level表示**（衛星信号強度のバーグラフ）を指している可能性あり
→ 本格的なスペクトラムアナライザーとは異なる

### 2. データログ機能

| 機能 | 説明 |
|------|------|
| UBXログ | u-blox独自形式でのログ記録 |
| NMEAログ | 標準形式でのログ記録 |
| RTCMログ | RTK補正データのログ |

### 3. データエクスポート

| 形式 | 用途 |
|------|------|
| ASCII形式 | スプレッドシートへのインポート可能 |
| KML形式 | Google Earth表示用 |
| Database Export | 「ファイル」メニューから実行 |

**課題**: CSVへの直接エクスポートは明確に記載されていない → 要実機確認

### 4. 設定機能

| 機能 | 説明 |
|------|------|
| MSG設定 | NMEAメッセージの有効/無効切替 |
| PRT設定 | ボーレート、I2Cアドレス、プロトコル変更 |
| インターフェース | USB, I2C, SPI, UART独立設定 |

### 5. RTK関連

| 機能 | 説明 |
|------|------|
| NTRIPクライアント | RTK補正データの受信 |
| NTRIPサーバー | RTK補正データの配信 |

---

## L1/L2波について

| 帯域 | 周波数 | 用途 |
|------|--------|------|
| L1 | 1575.42 MHz | 基本信号 |
| L2 | 1227.60 MHz | 精度向上用 |
| L5 | 1176 MHz | 新世代用 |

**F9Pの特徴**: L1+L2のデュアルバンド対応 → 高精度RTK測位

---

## 調査で判明した課題

| 課題 | 状況 |
|------|------|
| CSVエクスポート | 直接対応不明、実機確認必要 |
| ログデータの自動解析 | ツール作成が必要 |
| L1/L2波形の個別表示 | u-center単体では難しい可能性 |

---

## 次のアクション

1. **u-centerを実際に触る** → 機能の詳細確認
2. **ログファイルの形式確認** → 解析ツール設計のため
3. **アプリケーションノート確認** → F9Pの詳細仕様

---

## 参考リンク

- [u-center | u-blox](https://www.u-blox.com/en/product/u-center)
- [u-center 2 | u-blox](https://www.u-blox.com/en/u-center-2)
- [Getting Started with U-Center - SparkFun](https://learn.sparkfun.com/tutorials/getting-started-with-u-center-for-u-blox/all)
- [11 things you probably didn't know about u-center | u-blox](https://www.u-blox.com/en/blogs/tech/11-things-you-probably-didnt-know-about-u-center)
