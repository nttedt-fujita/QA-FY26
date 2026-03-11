# DB Repository 振る舞い記述

**作成日**: 2026-03-11 Session 85
**Phase**: TDD Phase 1（振る舞い記述）

---

## 概要

検査結果をSQLiteに保存・取得するリポジトリ。
プロトタイプ段階では最小限の機能に絞る。

---

## 振る舞い定義

### 保存（Save）

**When** 検査が完了した時
**Then** 検査結果をDBに保存する

保存する情報:
- 装置シリアル番号（SEC-UNIQID）
- 検査日時
- 各検査項目の結果（Verdict, 実測値, 期待値）
- 全体判定（Pass/Fail）

### 取得（Get）

**When** シリアル番号で検索する時
**Then** その装置の全検査履歴を取得する

**When** 日付範囲で検索する時
**Then** 該当期間の検査結果を取得する

**When** 最新N件を取得する時
**Then** 新しい順でN件取得する

---

## データモデル

### InspectionRecord（検査レコード）

```
inspection_records
├── id: INTEGER PRIMARY KEY
├── device_serial: TEXT NOT NULL        -- SEC-UNIQIDの16進文字列
├── inspected_at: TEXT NOT NULL         -- ISO8601形式
├── overall_verdict: TEXT NOT NULL      -- "Pass", "Fail", "Error"
├── fw_version: TEXT                    -- 実測FWバージョン
├── connectivity_verdict: TEXT          -- 各項目の判定
├── fw_version_verdict: TEXT
├── serial_number_verdict: TEXT
├── output_rate_verdict: TEXT
├── port_config_verdict: TEXT
└── notes: TEXT                         -- 備考（オプション）
```

### 設計判断

1. **1テーブル構成**:
   - プロトタイプ段階では正規化より単純さを優先
   - 検査項目ごとのテーブル分割は Phase 2 以降

2. **SQLite**:
   - 軽量、組み込み用途に適合
   - 単一ファイルでバックアップ容易
   - rusqliteクレートを使用

3. **日時形式**:
   - ISO8601文字列で保存（ソート可能）
   - `2026-03-11T08:54:47+09:00`

---

## テストシナリオ（TDD Phase 2）

### 基本操作

| ID | シナリオ | 確認内容 | 優先度 |
|----|----------|----------|--------|
| R1 | 新規保存 | InspectionRecordを保存できる | P0 |
| R2 | ID取得 | 保存後にIDが返る | P0 |
| R3 | シリアル検索 | シリアル番号で検索できる | P0 |
| R4 | 全件取得 | 全レコードを取得できる | P0 |
| R5 | 最新N件取得 | 新しい順でN件取得できる | P1 |
| R6 | 日付範囲検索 | from-to で検索できる | P1 |

### DB初期化

| ID | シナリオ | 確認内容 | 優先度 |
|----|----------|----------|--------|
| D1 | DB作成 | ファイルが存在しない場合、作成する | P0 |
| D2 | テーブル作成 | テーブルが存在しない場合、作成する | P0 |
| D3 | マイグレーション | 既存DBで起動できる | P1 |

### 異常系

| ID | シナリオ | 確認内容 | 優先度 |
|----|----------|----------|--------|
| E1 | 存在しないID | get_by_idでNoneが返る | P1 |
| E2 | 空の結果 | 検索結果が空の場合、空Vecが返る | P0 |

---

## 実装優先順位

**P0（今回実装）**:
1. D1, D2: DB/テーブル作成
2. R1, R2: 新規保存
3. R3, R4: 検索
4. E2: 空の結果

**P1（Phase 2以降）**:
- R5, R6: 高度な検索
- D3: マイグレーション
- E1: 存在しないID

---

## インターフェース設計

```rust
pub struct InspectionRecord {
    pub id: Option<i64>,
    pub device_serial: String,
    pub inspected_at: String,
    pub overall_verdict: String,
    pub fw_version: Option<String>,
    pub item_verdicts: HashMap<String, String>,
    pub notes: Option<String>,
}

pub trait InspectionRepository {
    fn save(&self, record: &InspectionRecord) -> Result<i64, RepositoryError>;
    fn get_all(&self) -> Result<Vec<InspectionRecord>, RepositoryError>;
    fn find_by_serial(&self, serial: &str) -> Result<Vec<InspectionRecord>, RepositoryError>;
}

pub struct SqliteRepository {
    conn: Connection,
}
```

---

## 参照

- [session82/inspection-engine-behavior.md](../session82/inspection-engine-behavior.md) — 検査エンジンの振る舞い
- [inspection/types.rs](../../prototype/m1-gnss/backend/src/inspection/types.rs) — 既存型定義
