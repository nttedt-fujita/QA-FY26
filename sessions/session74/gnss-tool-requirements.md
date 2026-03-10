# GNSS評価ツール 要件定義（Requirements）

**作成日**: 2026-03-10
**作成元**: Session 73 要求定義（gnss-tool-needs.md）
**ステータス**: ドラフト

---

## 概要

Session 73で定義した10件の要求（Needs）を元に、具体的な要件（Requirements）を定義する。

---

## 機能要件（FR: Functional Requirements）

### FR1: 装置自動検出

**対応要求**: N2（接続確認）

| 項目 | 内容 |
|------|------|
| 機能 | USB接続されたGNSS装置を自動検出する |
| 入力 | USBシリアルポートのホットプラグイベント |
| 出力 | 装置一覧（ポート名、シリアル番号） |
| 詳細 | F9P（VID:PID = 1546:01A9）をフィルタリング |

**実現方法**:
- Linux: `udev`ルールまたはポーリング
- シリアル番号取得: `UBX-SEC-UNIQID`

---

### FR2: 検査項目の自動実行

**対応要求**: N1（自動検査の開始）、N3（屋内検査）

| 項目 | 内容 |
|------|------|
| 機能 | 検査項目を自動的に実行する |
| 入力 | 検査開始トリガー（ボタン押下）|
| 出力 | 各検査項目の結果（Pass/Fail + 取得値）|

**検査項目一覧**:

| No | 検査項目 | UBXメッセージ | 判定基準 |
|----|----------|--------------|----------|
| 1 | 通信疎通 | 応答確認 | 1秒以内に応答あり |
| 2 | FWバージョン | UBX-MON-VER | 期待値と一致 |
| 3 | チップシリアル | UBX-SEC-UNIQID | 読み取り成功 |
| 4 | 設定: 出力レート | UBX-CFG-RATE | 期待値と一致 |
| 5 | 設定: ポート設定 | UBX-CFG-PRT | 期待値と一致 |

---

### FR3: 検査結果の保存

**対応要求**: N4（検査結果の保存）

| 項目 | 内容 |
|------|------|
| 機能 | 検査結果をSQLiteに保存する |
| 入力 | 検査結果データ |
| 出力 | SQLiteへのINSERT成功/失敗 |

**データモデル**:

```sql
-- 検査セッション
CREATE TABLE inspection_sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    started_at DATETIME NOT NULL,
    completed_at DATETIME,
    operator TEXT
);

-- 装置
CREATE TABLE devices (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    serial_number TEXT UNIQUE NOT NULL,
    first_seen_at DATETIME NOT NULL
);

-- 検査結果
CREATE TABLE inspection_results (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id INTEGER NOT NULL REFERENCES inspection_sessions(id),
    device_id INTEGER NOT NULL REFERENCES devices(id),
    item_name TEXT NOT NULL,
    result TEXT NOT NULL,  -- 'pass' | 'fail' | 'error'
    expected_value TEXT,
    actual_value TEXT,
    inspected_at DATETIME NOT NULL
);
```

---

### FR4: レポート出力

**対応要求**: N5（レポート出力）、N6（ファイル名にシリアル番号）

| 項目 | 内容 |
|------|------|
| 機能 | 検査結果をPDF/CSV形式で出力する |
| 入力 | 検査セッションID または 装置シリアル番号 |
| 出力 | PDFファイル、CSVファイル |

**ファイル名規則**:
- PDF: `inspection_{serial_number}_{YYYYMMDD_HHMMSS}.pdf`
- CSV: `inspection_{serial_number}_{YYYYMMDD_HHMMSS}.csv`

---

### FR5: 過去データ検索

**対応要求**: N7（過去データの再取得）

| 項目 | 内容 |
|------|------|
| 機能 | 過去の検査結果を検索・表示する |
| 入力 | 検索条件（シリアル番号、日付範囲）|
| 出力 | 検査結果一覧 |

**検索条件**:
- シリアル番号（部分一致）
- 検査日（範囲指定）
- 結果（Pass/Fail）

---

### FR6: 複数装置の比較表示

**対応要求**: N8（複数装置の比較）

| 項目 | 内容 |
|------|------|
| 機能 | 複数装置の検査結果を並べて表示する |
| 入力 | 装置シリアル番号リスト（2〜5件）|
| 出力 | 比較表 |

---

### FR7: 複数装置の同時検査

**対応要求**: N9（複数台同時接続）

| 項目 | 内容 |
|------|------|
| 機能 | 複数装置の検査を並行実行する |
| 入力 | 接続された装置一覧 |
| 出力 | 各装置の検査進捗・結果 |

**制約**:
- 同時接続台数: 2〜5台

---

### FR8: 装置状態の一覧表示

**対応要求**: N10（認知負荷の軽減）

| 項目 | 内容 |
|------|------|
| 機能 | 接続された装置の状態を一覧表示する |
| 入力 | 装置一覧 |
| 出力 | 状態一覧（接続/切断、検査中/完了/エラー）|

**表示項目**:
- ポート名
- シリアル番号
- 状態（接続中/検査中/完了/エラー）
- 最終検査結果（Pass/Fail）

---

## 非機能要件（NFR: Non-Functional Requirements）

### NFR1: 応答性

| 項目 | 要件 |
|------|------|
| 装置検出 | 接続から3秒以内に表示 |
| 検査開始 | ボタン押下から1秒以内に開始 |
| UI更新 | 検査進捗は500ms以内に反映 |

---

### NFR2: 同時接続数

| 項目 | 要件 |
|------|------|
| 最小 | 2台 |
| 推奨 | 3台 |
| 最大 | 5台 |

---

### NFR3: データ保持期間

| 項目 | 要件 |
|------|------|
| 保持期間 | 無制限（SQLiteファイルで永続化）|
| バックアップ | 手動（ファイルコピー）|

---

### NFR4: 可用性

| 項目 | 要件 |
|------|------|
| 稼働環境 | 評価室のPC（Linux）|
| 依存サービス | なし（スタンドアロン動作）|

---

### NFR5: 操作性

| 項目 | 要件 |
|------|------|
| 習熟時間 | 10分以内で基本操作を習得できる |
| エラー表示 | 何が問題か明確に表示する |

---

## インターフェース要件（IFR: Interface Requirements）

### IFR1: シリアル通信

| 項目 | 仕様 |
|------|------|
| プロトコル | UBX（u-bloxバイナリプロトコル）|
| ボーレート | 115200bps（デフォルト）|
| フロー制御 | なし |

---

### IFR2: UBXメッセージ

| メッセージ | Class/ID | 用途 |
|------------|----------|------|
| UBX-MON-VER | 0x0A 0x04 | FWバージョン取得 |
| UBX-SEC-UNIQID | 0x27 0x03 | チップシリアル取得 |
| UBX-CFG-RATE | 0x06 0x08 | 出力レート設定確認 |
| UBX-CFG-PRT | 0x06 0x00 | ポート設定確認 |

---

### IFR3: データ形式

| 形式 | 用途 | 仕様 |
|------|------|------|
| SQLite | データ保存 | SQLite 3.x |
| PDF | レポート出力 | A4サイズ、日本語対応 |
| CSV | データエクスポート | UTF-8、カンマ区切り |

---

### IFR4: API（バックエンド-フロントエンド間）

| エンドポイント | メソッド | 用途 |
|---------------|----------|------|
| /api/devices | GET | 装置一覧取得 |
| /api/devices/{id}/inspect | POST | 検査開始 |
| /api/inspections | GET | 検査結果一覧 |
| /api/inspections/{id} | GET | 検査結果詳細 |
| /api/reports/pdf | POST | PDF出力 |
| /api/reports/csv | POST | CSV出力 |

---

## トレーサビリティマトリクス

| 要求 | 機能要件 | 非機能要件 |
|------|---------|-----------|
| N1: 自動検査の開始 | FR2 | NFR1 |
| N2: 接続確認 | FR1 | NFR1 |
| N3: 屋内検査 | FR2 | - |
| N4: 検査結果の保存 | FR3 | NFR3 |
| N5: レポート出力 | FR4 | - |
| N6: ファイル名にシリアル番号 | FR4 | - |
| N7: 過去データの再取得 | FR5 | - |
| N8: 複数装置の比較 | FR6 | - |
| N9: 複数台同時接続 | FR7 | NFR2 |
| N10: 認知負荷の軽減 | FR8 | NFR5 |

---

## 参照資料

- [gnss-tool-needs.md](../session73/gnss-tool-needs.md) — 要求定義
- [ADR-005](../../docs/adr/ADR-005-gnss-tool-tech-stack.md) — 技術スタック

---

*作成: 2026-03-10 Session 74*
