# tools/ ディレクトリ

受入検査データ分析ツールを集約したディレクトリ。

## ディレクトリ構成

```
tools/
├── README.md                       # このファイル
├── incoming_inspection/            # 受入検査分析モジュール
│   ├── __init__.py
│   ├── extract_csv.py              # Excel→CSV変換
│   ├── name_normalizer.py          # 名寄せモジュール
│   ├── monthly_analysis.py         # 月別分析
│   ├── mapping/                    # 名寄せルール
│   │   ├── 検査内容_名寄せルール.csv  # 検査内容の名寄せ（39件）
│   │   └── 品名_名寄せルール.csv      # 品名の名寄せ（18件）
│   ├── output/                     # 分析結果出力
│   │   ├── 月別サマリー.csv
│   │   └── detail/
│   │       ├── 月別×部品.csv
│   │       ├── 月別×作業者.csv
│   │       ├── 月別×カテゴリ.csv
│   │       └── 月別×検査内容.csv
│   └── scripts/                    # ユーティリティスクリプト
│       ├── analyze_product_names.py    # 品名の表記揺れ分析
│       └── extract_data_anomalies.py   # データ異常抽出
└── tests/                          # テストコード
    ├── conftest.py                 # pytest設定
    └── incoming_inspection/
        ├── test_csv_integrity.py       # CSV整合性テスト
        ├── test_name_normalizer.py     # 名寄せテスト（検査内容）
        ├── test_product_name_normalizer.py # 名寄せテスト（品名）
        └── test_monthly_analysis.py    # 月別分析テスト
```

## 使い方

### 1. Excel→CSV変換（初回のみ）

```bash
python tools/incoming_inspection/extract_csv.py
```

**入力**: `docs/excel-orgn/受入検査作業集計.xlsx`
**出力**: `sessions/session19/csv-output/`

### 2. 月別分析

```bash
python tools/incoming_inspection/monthly_analysis.py
```

**入力**: `sessions/session19/csv-output/raw/`
**出力**: `tools/incoming_inspection/output/`

### 3. テスト実行

```bash
# プロジェクトルートから実行
cd /path/to/QA-FY26

# 全テスト
python -m pytest tools/tests/ -v

# 特定のテスト
python -m pytest tools/tests/incoming_inspection/test_name_normalizer.py -v
```

## モジュール説明

### extract_csv.py

Excelの受入検査作業集計から生データCSVを抽出。

- 6シート（メカ、エレキ、Api、時間かかる、買い物、副資材）を処理
- 日付のシリアル値を `YYYY-MM-DD` 形式に変換
- カテゴリ列を追加
- 品名別CSVも出力

### name_normalizer.py

表記揺れを名寄せするモジュール。

```python
from tools.incoming_inspection import NameNormalizer

normalizer = NameNormalizer("mapping/検査内容_名寄せルール.csv")
normalized = normalizer.normalize("外観のみ")  # → "外観"
```

### monthly_analysis.py

月別の工数分析を行う。

- 月別サマリー（件数、工数、パレート指標）
- 月別×部品
- 月別×作業者
- 月別×カテゴリ
- 月別×検査内容（名寄せ適用）

## 名寄せルール

| ファイル | 対象 | ルール数 |
|----------|------|----------|
| 検査内容_名寄せルール.csv | 検査内容列 | 39 |
| 品名_名寄せルール.csv | 品名列 | 18 |

### ルール形式

```csv
変換前,変換後,適用理由,登録日
外観のみ,外観,意味同一,2026-03-06
```

## 開発履歴

- **Session 19**: extract_csv.py 作成
- **Session 20**: monthly_analysis.py 作成
- **Session 21**: name_normalizer.py 作成、検査内容の名寄せ
- **Session 22**: 品名の名寄せ、tools/への集約
