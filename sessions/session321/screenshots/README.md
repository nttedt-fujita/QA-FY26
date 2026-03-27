# GEN画面スクショ整理

**目的**: GEN画面構成の把握・PSI以外のDX化ポイント発見（QA-19）

## ディレクトリ構成

GENトップナビのモジュール順に整理。

| Dir | モジュール | 注目ポイント |
|-----|-----------|-------------|
| `00-overview/` | 全体（ROUTE MAP等） | モジュール間の関連 |
| `01-sales/` | 販売管理 | 受注→納品→入金フロー |
| `02-purchasing/` | 購買管理 | **P-6発注情報の謎**（バッテリ発注はここ？） |
| `03-production/` | 生産管理 | P-4生産数との関係 |
| `04-inventory/` | 在庫管理 | P-5在庫データとの関係 |
| `05-quality/` | 品質管理 | トップナビに見えたが中身不明 |
| `06-reports/` | レポート | どんなレポートが出せるか |
| `07-master/` | マスタ | 品目・取引先等の基盤データ（サブディレクトリあり） |
| `08-settings/` | 設定 | 必要に応じて |

## 命名ルール

`{連番}-{画面名の短縮}.png`

例: `01-order-list.png`, `02-order-detail.png`

探索的に撮るので、後から並べ替えやすいよう連番付き。

## 探索の進め方

1. まずトップナビの各モジュールを開いて一覧画面を撮る
2. 気になる画面はドリルダウンして詳細も撮る
3. **特に購買管理（02）を重点的に** — P-6の謎解きの鍵

## 撮影記録

| # | ファイル | 画面 | メモ |
|---|---------|------|------|
| 1 | `00-overview/GEN-ROUTE-MAP.png` | ROUTE MAP | 5モジュールのフロー全体像 |
| 2 | `01-sales/01-route-map-sales.png` | 販売管理フロー | 見積→受注→納品→締め請求書→入金 |
| 3 | `02-purchasing/01-route-map-purchasing.png` | 購買管理フロー | 発注→発注受入→購買締め→支払 ★P-6関連 |
| 4 | `03-production/01-route-map-production.png` | 生産管理フロー | 計画→所要量計算→発注→受入 / 受注→製造指示→製造実績 |
| 5 | `04-inventory/01-route-map-inventory.png` | 在庫管理フロー | 発注受入/外製受入/製造実績→ロケーション移動→棚卸→出庫 |
| 6 | `02-purchasing/02-order-list.png` | 発注登録一覧 | 発注データ実在確認。バッテリ発注あり ★P-6解決 |
| 7 | `02-purchasing/03-order-list-wide.png` | 発注登録一覧（横幅広） | 全レコード表示 |
| 8 | `02-purchasing/04-columns-left.png` | カラムヘッダー左 | 選択〜受入状況 |
| 9 | `02-purchasing/05-columns-center.png` | カラムヘッダー中央 | 従業員名〜瑕疵担保期間 |
| 10 | `02-purchasing/06-columns-right.png` | カラムヘッダー右 | 検収日〜発注メモ |
| 11 | `07-master/01-master-menu.png` | マスタメニュー | 品目/取引先/構成表/ロケーション等16種 |
| 12 | `01-sales/02-columns-left.png` | 受注登録カラム左 | 選択〜受注備考1 |
| 13 | `01-sales/03-columns-center.png` | 受注登録カラム中央 | 在庫推移〜メール送信日時 |
| 14 | `01-sales/04-columns-right.png` | 受注登録カラム右 | 納品希望日〜有効在庫数 |
| 15 | `04-inventory/02-stock-list-columns.png` | 在庫リストカラム | 品目名〜製造可能数 |
| 16-18 | `07-master/item/01〜03` | 品目マスタ一覧+カラム | 4,078品目 |
| 19 | `07-master/bom/01-bom-overview.png` | 構成表マスタ | データあり。親品目選択→子部品展開 |
| 20 | `07-master/location/01-location-columns.png` | ロケーションマスタカラム | 28拠点 |

## 分析成果物

| ファイル | 内容 |
|---------|------|
| [gen-system-analysis.md](../gen-system-analysis.md) | GENシステム1次分析（客観的事実ベース） |
| [confidence-update.md](../confidence-update.md) | 確度更新（S317ドメインモデルのA(G)更新） |
| [gen-er-diagram.drawio](../gen-er-diagram.drawio) | GEN ER図（確度色分け） |
| 16 | `07-master/item/01-item-list.png` | 品目マスタ一覧 | 全品目リスト |
| 17 | `07-master/item/02-columns-left.png` | 品目マスタカラム左 | 品目コード〜理論在庫数・工数 |
| 18 | `07-master/item/03-columns-right.png` | 品目マスタカラム右 | 作業工賃単価〜メーカー型番 |
| 19 | `07-master/bom/01-bom-overview.png` | 構成表マスタ | データあり。親品目選択→子部品展開の形式 |
