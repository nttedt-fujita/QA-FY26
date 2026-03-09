# GNSS View Web API 調査レポート

**作成日:** 2026年3月6日  
**調査目的:** GNSS ViewのデータをWeb APIで取得できるかの可否調査

---

## 1. 結論サマリー

- QZSS（みちびき）公式のWeb APIは**無料で公開**されており、QZSS・GPSの軌道データ（Almanac、Ephemeris等）をHTTP GETで取得可能。
- ただし**非商用目的のみ**利用可。APIキーや事前登録は不要。
- GNSS Viewアプリが表示するような**スカイプロット画像を直接返すAPIは存在しない**。仰角マスクを指定して画像を取得する機能もない。
- 同等の表示を実現するには、APIから軌道データを取得し、**自前で衛星位置（方位角・仰角）を計算してプロットする**必要がある。

---

## 2. GNSS Viewとは

GNSS Viewは、公表されている測位衛星の軌道情報をもとに、任意の時間・任意の場所における衛星の天球上の配置（コンステレーション）を画面上に再現するソフトウェアである。2014年から公開されている。

### 対応プラットフォーム

- **Web版:** https://app.qzss.go.jp/GNSSView/gnssview.html
- **iOS版:** App Store
- **Android版:** Google Play

### 対応衛星システム（6系統）

- QZSS（みちびき）
- GPS（米国）
- GLONASS（ロシア）
- BeiDou（中国）
- Galileo（欧州）
- SBAS（衛星航法補強システム）

### データソース

- QZSS・GPSの軌道情報: QZSS公式のWeb APIから取得
- GLONASS・BeiDou・Galileo・SBASの軌道情報: NORAD（北米航空宇宙防衛司令部）が公開している軌道情報を使用

### エビデンス

> 「GNSS Viewで表示するGNSS衛星はスマートフォンが直接受信した衛星の情報を表示するものではなく、外部サーバーから取得したGNSS衛星の軌道情報を元に計算した衛星配置を表示します。」

**出典:** GNSS View マニュアル  
**URL:** https://qzss.go.jp/technical/gnssview/manual.html

> 「みちびきとGPS以外の4システムの衛星配置は、NORAD（North American Aerospace Defense Command, 北米航空宇宙防衛司令部）が公開している軌道情報を用いて計算しています。」

**出典:** みちびきの技術 - 衛星配置表示アプリ（GNSS View）  
**URL:** https://qzss.go.jp/technical/gnssview/index.html

---

## 3. QZSS Web API の詳細

### 3.1 概要

> 「準天頂衛星システムでは、QZSS・GPSに関わるデータ取得機能をWeb APIとして公開しています。さまざまなツール開発やサービスの開発にお役立てください。」

**出典:** Application Programming Interface | 準天頂衛星システム(QZSS) 公式サイト  
**URL:** https://sys.qzss.go.jp/dod/api.html

### 3.2 リクエスト仕様

- **プロトコル:** HTTP GET
- **パラメータ:** URLエンコード必須
- **レスポンス（検索）:** XML形式
- **レスポンス（取得）:** 公開データファイル（Content-Type: application/octet-stream）

### エビデンス

> 「全てHTTPプロトコルのGETメソッドによって行われます。リクエストパラメータはURLエンコードされている必要があります。レスポンスについては検索の場合はXML形式にて返却されます。」

**出典:** 同上  
**URL:** https://sys.qzss.go.jp/dod/api.html

### 3.3 利用条件

- **料金:** 無料
- **APIキー:** 不要
- **利用回数制限:** 現時点ではなし（ただし事前告知なく制限がかかる可能性あり）
- **利用目的:** 非商用目的のみ
- **保証:** APIの動作の完全性やデータの正確性は保証されない
- **免責:** 公開アーカイブデータの利用には免責条項への同意が必要

### エビデンス

> 「提供する全てのAPIは、非商用の目的でのみご利用頂けます。APIの動作の完全性やデータの正確性を保証するものではありません。現在、APIの利用回数に関する制限は設けておりませんが、状況に応じて事前の告知なしに制限をかける可能性があります。」

**出典:** 同上  
**URL:** https://sys.qzss.go.jp/dod/api.html

### 3.4 取得可能なデータ一覧

| データ種別 | 検索URL | 取得URL |
|-----------|---------|---------|
| QZSS+GPS Almanac | /api/search/almanac | /api/get/almanac |
| QZSS Ephemeris（RINEX拡張版） | /api/search/ephemeris-qzss | /api/get/ephemeris-qzss |
| QZSS Ephemeris | /api/search/ephemeris | /api/get/ephemeris |
| Ultra Rapid Orbit (QZU) | /api/search/ultra-rapid-sp3 | /api/get/ultra-rapid-sp3 |
| Ultra Rapid ERP (QZU) | /api/search/ultra-rapid-erp | /api/get/ultra-rapid-erp |
| Rapid Orbit (QZR) | /api/search/rapid-sp3 | /api/get/rapid-sp3 |
| Rapid Clock (QZR) | /api/search/rapid-clk | /api/get/rapid-clk |
| Rapid ERP (QZR) | /api/search/rapid-erp | /api/get/rapid-erp |
| Final Orbit (QZF) | /api/search/final-sp3 | /api/get/final-sp3 |
| Final Clock (QZF) | /api/search/final-clk | /api/get/final-clk |
| Final ERP (QZF) | /api/search/final-erp | /api/get/final-erp |
| Final 局座標（日次） | /api/search/final-snx | /api/get/final-snx |
| Final 局座標（週次） | /api/search/final-snx-w | /api/get/final-snx-w |
| L1S信号（サブメータ級補強） | /api/search/l1s | /api/get/l1s |
| L6信号（センチメータ級補強） | /api/search/l6 | /api/get/l6 |
| NAQU情報 | /api/search/naqu | /api/get/naqu |
| 安否確認（Anpi） | /api/search/anpi | /api/get/anpi |

**ベースURL:** `https://sys.qzss.go.jp/dod`

### 3.5 APIの使用例

#### 検索（Search）

```
GET https://sys.qzss.go.jp/dod/api/search/almanac?since_datetime=2010-12-14%2000:00:00&until_datetime=2010-12-16%2000:00:00
```

レスポンス例:
```xml
<search>
  <id>qg2010352.alm</id>
  <id>qg2010351.alm</id>
  <id>qg2010350.alm</id>
</search>
```

#### 取得（Get）

```
GET https://sys.qzss.go.jp/dod/api/get/almanac?id=qg2010352.alm
```

※ `id` パラメータを省略した場合は最新のデータが返却される。

### 3.6 リクエストパラメータ（検索用）

| パラメータ | 型 | 説明 |
|-----------|------|------|
| since_datetime | 文字列（任意） | 検索開始日時(UTC) `YYYY-MM-DD hh:mm:ss` |
| until_datetime | 文字列（任意） | 検索終了日時(UTC) `YYYY-MM-DD hh:mm:ss` |
| since_week | 整数（任意） | 検索開始 GPS Week |
| until_week | 整数（任意） | 検索終了 GPS Week |
| since_weeksec | 整数（任意） | 検索開始 週内秒 |
| until_weeksec | 整数（任意） | 検索終了 週内秒 |

### 3.7 データ公開時期

> 「衛星測位サービスの公開アーカイブデータはApr.14, 2017よりダウンロードできるようになりました。Apr.1, 2017からのデータを利用できます。」

**出典:** 同上  
**URL:** https://sys.qzss.go.jp/dod/api.html

---

## 4. 仰角指定によるスカイプロット画像取得について

### 結論: 不可

QZSS Web APIには、仰角マスクを指定してスカイプロット画像を返すエンドポイントは存在しない。APIが提供するのは生の軌道データファイル（Almanac、Ephemerisなど）のみである。

### 代替手段

GNSS Viewと同等のスカイプロットを生成するには、以下の手順で自前実装が必要：

1. **軌道データの取得:** QZSS APIからAlmanac/Ephemerisを取得。GLONASS等も含める場合はCelesTrak等からTLE（二行軌道要素）を取得。
2. **衛星位置計算:** 観測地点の緯度経度・時刻を指定し、各衛星の方位角（Azimuth）・仰角（Elevation）を計算。
3. **仰角マスク適用:** 指定した仰角以下の衛星を除外。
4. **極座標プロット:** matplotlibなどでスカイプロット（極座標レーダー図）として描画。

### 推奨ライブラリ（Python）

- `skyfield` — 衛星位置の高精度計算
- `sgp4` — TLEからの軌道伝播
- `matplotlib` — スカイプロット描画

---

## 5. 参考URL一覧

| リソース | URL |
|---------|-----|
| GNSS View 公式ページ（日本語） | https://qzss.go.jp/technical/gnssview/index.html |
| GNSS View 公式ページ（英語） | https://qzss.go.jp/en/technical/gnssview/index.html |
| GNSS View Web版 | https://app.qzss.go.jp/GNSSView/gnssview.html |
| GNSS View マニュアル | https://qzss.go.jp/technical/gnssview/manual.html |
| QZSS Web API 仕様 | https://sys.qzss.go.jp/dod/api.html |
| QZSS 公開アーカイブデータ免責条項 | https://sys.qzss.go.jp/dod/api/agree.html |
| みちびきの技術トップ | https://qzss.go.jp/technical/ |
| GNSS View (iOS) App Store | https://apps.apple.com/us/app/gnss-view/id924350018 |
| GNSS View (Android) Google Play | https://play.google.com/store/apps/details?id=com.nec.android.qzss.gnssview |

---

## 6. 備考

- GNSS Viewの開発元は**NEC Corporation**。
- 2025年7月18日にみちびき6号機がサービスを開始し、7機体制運用に向けてGNSS Viewもアップデートされている（Ver.6で7機体制のダミー表示に対応）。
- GNSS View Web版はブラウザ上で仰角マスク（Mask Angle）設定が可能だが、その結果を画像として自動取得するAPIは提供されていない。
