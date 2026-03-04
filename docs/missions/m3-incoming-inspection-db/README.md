# M3: 受入検査データベース化

**担当**: ふじた
**難易度**: 中
**依存関係**: 独立（M4と共通設計推奨）

---

## 概要

現在Excel管理されている受入検査データをデータベース化する。
将来的にはタブレット操作可能なアプリ化も視野に入れる。

---

## 現状と目標

| 項目 | 現状 | 目標 |
|------|------|------|
| **記録方法** | Excel管理 | DB + アプリ（タブレット対応） |
| **検索性** | ファイルを開いて検索 | SQLクエリ / ダッシュボード |
| **分析** | 手作業集計 | 自動パレート分析、傾向管理 |
| **サイレントチェンジ検出** | PO発行時のバージョン確認（手動） | FW/HWバージョン変更の自動検出 |
| **トレーサビリティ** | 限定的 | lot_idで工程不良DB（M4）と紐付け |

---

## データベースの構成

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

> **ISO 9001:2015, Clause 8.6** "Release of products and services":
> "The organization shall retain documented information on the release of products and services."

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

---

## M4との連携設計

受入検査DBと工程不良DBは**lot_idで紐付け**する。

```
受入検査DB                    工程不良DB
incoming_lots ──── lot_id ────→ defect_records.lot_id
  │                               │
  ├── supplier_id                 ├── defect_code
  ├── fw_version                  ├── cause_code
  └── lot_judgment                └── severity
```

これにより:
- 受入時に合格したロットの部品が工程で不良を起こした場合にトレースできる
- サプライヤ品質と工程不良の相関分析が可能

---

## 推奨段取り

```
Step 1: 要件定義・スキーマ設計
  ├── 現行のExcel運用フロー調査
  ├── 管理項目の洗い出し（特にサイレントチェンジ監視要件）
  ├── DBスキーマ設計（ER図作成）
  ├── アプリ化の要件整理（タブレット対応）
  └── 【成果物】要件定義書、ERダイアグラム

Step 2: DB構築・データ移行
  ├── RDBMS選定（PostgreSQL推奨 / SQLiteで小規模開始もあり）
  ├── テーブル作成・マスタデータ投入
  ├── 過去データ移行（Excelからのインポート）
  └── 【成果物】稼働DB

Step 3: アプリ・分析機能・運用開始
  ├── タブレット対応の入力アプリ構築
  ├── 分析クエリ・ダッシュボードの構築
  └── 【成果物】アプリ、運用マニュアル
```

---

## アプリ化の技術選択肢（今後検討）

| 選択肢 | 特徴 | 適合度 |
|--------|------|--------|
| **Streamlit** | Python製、高速プロトタイピング | 社内ツールとして◎ |
| **Django + DRF** | 本格Webアプリ、REST API | 拡張性◎ |
| **Power Apps** | ノーコード、Microsoft連携 | 非開発者も保守可能 |
| **React + FastAPI** | モダンSPA | 開発コスト高だが自由度◎ |

---

## 参考文献

| トピック | ソース |
|---------|--------|
| ISO 9001:2015 Clause 8.6 | 製品・サービスのリリース管理 |
| AQL抜取検査 | [QTEC](https://www.qtec.or.jp/knowledge/inspection/aql/) |
| JIS Z 9015-1 | 計数値検査に対する抜取検査手順 |

---

*元データ: [session1/mission-approach-report.md](../../sessions/session1/mission-approach-report.md)*
