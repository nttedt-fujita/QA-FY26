# M3 ドメインモデルと課題の対応レビュー

**作成日**: 2026-03-19 (Session 264)
**目的**: 現在のM3ドメインモデル（schema.sql）が、判明しているExcelの課題に対応できているかを確認
**出典**:
- [prototype/m3/db/schema.sql](../../prototype/m3/db/schema.sql)（ドメインモデル）
- [sessions/session6/excel-review.md](../session6/excel-review.md)（課題）
- [sessions/session262/pre-extracted-info-from-excel.md](../session262/pre-extracted-info-from-excel.md)（課題の具体例）

---

## ドメインモデル概要

現在のM3（受入検査DB）のエンティティ構成:

```
マスタデータ:
- suppliers（サプライヤマスタ）
- parts（部品マスタ）
- inspection_items（検査項目マスタ）
- workers（作業者マスタ）

トランザクションデータ:
- lots（ロット）
- inspection_records（検査記録）
- defect_reports（不良レポート・8D対応）
- waivers（不問判定）
```

---

## 課題とドメインモデルの対応表

### A. 列構成の不統一（メカ/エレキ/Apiでヘッダーが異なる）

| Excel課題 | ドメインモデル対応 | 判定 | 備考 |
|---------|----------------|------|------|
| メカのみ発注番号（PO番号）がある | `lots.po_number` (TEXT, nullable) | ✅ 対応済み | 全ロットでPO番号を記録できる |
| エレキ・ApiにはPO番号なし | nullable なので記録なしでもOK | ✅ 対応済み | NULL許容で設計されている |

**結論**: 列構成の不統一は、DB化すれば解消される（NULLで吸収）

---

### B. ロット番号の欠落

| Excel課題 | ドメインモデル対応 | 判定 | 備考 |
|---------|----------------|------|------|
| ロット番号が記録されていない | `lots.lot_id` (TEXT, PK) | ⚠️ **部分対応** | システムが自動発番する想定<br>サプライヤーのロット番号は？ |

**問題点**:
- `lots.lot_id` はシステム内部のID（例: `LOT-12345678`）
- **サプライヤーが発行したロット番号**（トレーサビリティに必要）を記録するフィールドがない

**修正提案**:
```sql
ALTER TABLE lots ADD COLUMN supplier_lot_number TEXT;
```

---

### C. 作業者名の表記揺れ

| Excel課題 | ドメインモデル対応 | 判定 | 備考 |
|---------|----------------|------|------|
| 「西村・西浦」「西村　西浦」など5パターンに分散 | `workers` テーブルで正規化済み | ✅ 対応済み | worker_id で紐付けるため表記揺れは発生しない |
| 複数人作業の記録方法が不明確 | `inspection_records.worker_id` は1人のみ | ⚠️ **対応不足** | 複数人の記録ができない |

**問題点**:
- `inspection_records.worker_id` は単一FK
- 複数人作業（「西村・西浦」等）を記録できない

**修正提案**（選択肢）:
1. **中間テーブルを追加**（多対多）:
   ```sql
   CREATE TABLE inspection_record_workers (
       record_id TEXT,
       worker_id TEXT,
       PRIMARY KEY (record_id, worker_id),
       FOREIGN KEY (record_id) REFERENCES inspection_records(record_id),
       FOREIGN KEY (worker_id) REFERENCES workers(worker_id)
   );
   ```
2. **メイン担当者1人のみ記録**（現行のまま）:
   - 運用で「代表者のみ記録」とルール化
   - 複数人作業は `inspection_records.note` に記載

**判断が必要**: ワークショップで「複数人作業の記録が必要か」を確認

---

### D. サプライヤー名の表記揺れ

| Excel課題 | ドメインモデル対応 | 判定 | 備考 |
|---------|----------------|------|------|
| 「金森産業（株）」「金森産業株式会社」など複数表記 | `suppliers` テーブルで正規化済み | ✅ 対応済み | supplier_id で紐付けるため表記揺れは発生しない |

**結論**: DB化すれば解消される

---

### E. 検査数量列に複数の情報が混在

| Excel課題 | ドメインモデル対応 | 判定 | 備考 |
|---------|----------------|------|------|
| 検査数量に「外観全数寸法なみ検20」など検査方法が混在 | `inspection_records.sample_qty` (INTEGER, nullable)<br>`inspection_items.type` (TEXT, 全数/抜取) | ⚠️ **部分対応** | 検査方法は検査項目ごとに定義されているが、実施方法の詳細は記録できない |
| 「西村」など作業者名が混在 | `inspection_records.worker_id` で分離済み | ✅ 対応済み | 作業者は別フィールド |

**問題点**:
- `inspection_records` に「検査方法の詳細」を記録するフィールドがない
- 例: 「外観全数、寸法抜き取り20個」 → 複数の検査を1レコードで記録できない

**修正提案**（選択肢）:
1. **検査記録を細分化**:
   - 1つのロットに対して、複数の `inspection_records` を作成
   - 例: 「外観検査（全数）」+ 「寸法検査（抜き取り20個）」 → 2レコード
2. **検査方法詳細フィールドを追加**:
   ```sql
   ALTER TABLE inspection_records ADD COLUMN inspection_method_detail TEXT;
   ```

**判断が必要**: 検査記録の粒度を確認（1ロット1検査項目1レコード? or 複数検査まとめて1レコード?）

---

### F. 不良数量の記録が不完全

| Excel課題 | ドメインモデル対応 | 判定 | 備考 |
|---------|----------------|------|------|
| 不良数量が空欄でも備考に不良の記述 | `inspection_records.defect_qty` (INTEGER, DEFAULT 0)<br>`inspection_records.note` (TEXT, nullable) | ⚠️ **運用依存** | 空欄 vs 0 の区別ができる（NULLにしていないため） |
| 不良数量に「保留」「1個不足」などテキストが混在 | INTEGER型なので不可 | ✅ 対応済み | 型制約でテキスト混入を防げる |
| 「不問」判定の基準が曖昧 | `waivers` テーブルで管理 | ✅ 対応済み | 不問理由、承認者、承認日時を記録可能 |

**問題点**:
- `defect_qty` のDEFAULT値が0
- 「記録なし」（NULL）と「0個」の区別ができない

**修正提案**:
```sql
-- DEFAULT 0 を削除し、NULLを許容
ALTER TABLE inspection_records ALTER COLUMN defect_qty DROP DEFAULT;
```

**運用ルール**:
- 不良あり → defect_qty に数値を記録
- 不良なし → defect_qty = 0
- 未記入 → defect_qty = NULL

---

### G. 「検査結果・備考」列に8種類の情報が混在

Excelの「検査結果・備考」列には以下が混在:

| 情報の種類 | Excel例 | ドメインモデル対応 | 判定 |
|-----------|--------|----------------|------|
| 合否判定 | 「良」「良好」 | `inspection_records.result` (TEXT, NOT NULL) | ✅ 分離済み |
| 不良の詳細 | 「インサート逆入れあり」 | `defect_reports.detail` (TEXT) | ✅ 分離済み |
| 対処内容 | 「代品入庫済」 | `defect_reports.corrective_action` (TEXT) | ✅ 分離済み |
| 測定結果 | 「板厚1.71～1.73㎜」 | `inspection_records.note` (TEXT) | ⚠️ 別フィールド化? |
| 検査方法メモ | 「ネジを入れて確認」 | `inspection_items.description` (TEXT) | ⚠️ 項目ごと? 記録ごと? |
| 判断保留 | 「要協議」「メーカー確認要」 | `defect_reports.status` (TEXT, '要協議'/'対処中'/'完了') | ✅ 分離済み |
| 基準書要望 | 「基準書に入れてほしい」 | （対応なし） | ❌ フィールドなし |
| 作業備忘録 | 「箱入り箱出し含む」 | `inspection_records.note` (TEXT) | ⚠️ noteに混在 |

**問題点**:
- **測定結果**: 数値データとして記録すべきか、自由記述か
- **基準書要望**: 改善提案を記録するフィールドがない
- **note**: まだ複数の用途が混在（測定結果、検査方法メモ、作業備忘録）

**修正提案**（選択肢）:
1. **測定結果を構造化**（測定項目が定まっている場合）:
   ```sql
   CREATE TABLE measurement_results (
       measurement_id TEXT PRIMARY KEY,
       record_id TEXT NOT NULL,
       measurement_item TEXT NOT NULL,
       measured_value REAL,
       unit TEXT,
       FOREIGN KEY (record_id) REFERENCES inspection_records(record_id)
   );
   ```
2. **改善提案を記録**:
   ```sql
   CREATE TABLE improvement_suggestions (
       suggestion_id TEXT PRIMARY KEY,
       record_id TEXT,
       category TEXT, -- '基準書修正', '検査方法改善', etc.
       suggestion TEXT NOT NULL,
       status TEXT DEFAULT '未対応',
       created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
   );
   ```

**判断が必要**: プロトタイプ段階でどこまで構造化するか

---

### H. 検査基準書の不足

| Excel課題 | ドメインモデル対応 | 判定 | 備考 |
|---------|----------------|------|------|
| エレキは検査基準書がない | `parts.spec_doc_url` (TEXT, nullable) | ⚠️ **部分対応** | 基準書URLは記録できるが、未整備の部品が多い |
| 検査項目ごとの手順説明 | `inspection_items.description` (TEXT, nullable) | ⚠️ **部分対応** | 手順説明を記録できるが、詳細度は不明 |

**問題点**:
- ドメインモデルは「基準書がある」前提で設計されている
- 「基準書がない部品の運用」が明確でない

**対応不要（運用で解決）**:
- プロトタイプ段階では `spec_doc_url` が NULL でも許容
- 今後、基準書整備と並行してURLを追加

---

### I. 不具合発生シートの情報不足

| Excel課題 | ドメインモデル対応 | 判定 | 備考 |
|---------|----------------|------|------|
| 不具合発生シートに日付・数量・対策がない | `defect_reports` テーブルで管理 | ✅ 対応済み | 日付: created_at, closed_date<br>対策: corrective_action<br>ステータス: status |
| 受入検査と不具合の紐付けがない | `defect_reports.record_id` (FK) | ✅ 対応済み | inspection_records と紐付く |

**結論**: DB化すれば解消される

---

## まとめ: 修正が必要な項目

### 優先度A（必須修正）

| # | 課題 | 修正内容 | 理由 |
|---|------|---------|------|
| 1 | サプライヤーロット番号の欠落 | `lots.supplier_lot_number` を追加 | トレーサビリティに必須 |

### 優先度B（ワークショップ後に判断）

| # | 課題 | 修正内容 | 判断が必要な点 |
|---|------|---------|--------------|
| 2 | 複数人作業の記録 | 中間テーブル追加 or 運用ルール化 | 複数人記録が必要か確認 |
| 3 | 検査方法詳細の記録 | `inspection_method_detail` 追加 or レコード分割 | 検査記録の粒度を確認 |
| 4 | defect_qty のDEFAULT値 | DEFAULT 0 を削除 | 0 と NULL の運用ルール確認 |

### 優先度C（将来的に検討）

| # | 課題 | 修正内容 | 備考 |
|---|------|---------|------|
| 5 | 測定結果の構造化 | `measurement_results` テーブル追加 | プロトタイプでは note で代用可 |
| 6 | 改善提案の記録 | `improvement_suggestions` テーブル追加 | プロトタイプでは不要 |

---

## 次のアクション

### 1. 必須修正の実施（優先度A）

- [ ] `lots.supplier_lot_number` フィールドを追加
- [ ] マイグレーションスクリプトを作成
- [ ] Goコードの修正（Lot構造体、リポジトリ、API）

### 2. ワークショップでの確認（優先度B）

SIPOCワークショップ（Session 263計画）で以下を確認:

- [ ] 複数人作業の記録が必要か（西村・西浦のペアは何を意味するか）
- [ ] 検査記録の粒度（1ロット1検査項目1レコード? or まとめて1レコード?）
- [ ] 不良数量の運用ルール（0 vs NULL の使い分け）

### 3. 設計判断の記録

- [ ] ADR作成: 「サプライヤーロット番号の追加」
- [ ] ADR作成（必要に応じて）: 「複数人作業の記録方式」

---

## 参照

- [prototype/m3/db/schema.sql](../../prototype/m3/db/schema.sql) — 現在のスキーマ
- [sessions/session6/excel-review.md](../session6/excel-review.md) — Excel課題の詳細
- [sessions/session262/pre-extracted-info-from-excel.md](../session262/pre-extracted-info-from-excel.md) — 課題の具体例
- [sessions/session263/session-summary.md](../session263/session-summary.md) — SIPOCワークショップ準備

---

*作成: Session 264 (2026-03-19)*
