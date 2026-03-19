# Session 264 サマリー

**日付**: 2026-03-19
**目的**: ドメインモデルと課題の整合性確認

---

## 実施内容

### 1. 現在のM3ドメインモデルの確認

**確認対象**:
- [prototype/m3/db/schema.sql](../../prototype/m3/db/schema.sql)（スキーマ定義）
- [prototype/m3/backend/internal/repository/*.go](../../prototype/m3/backend/internal/repository/)（Goコード）

**ドメインモデル構成**:

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

**主なフィールド**:
- `lots`: lot_id, part_id, po_number, arrival_date, quantity, fw_version, hw_version
- `inspection_records`: record_id, lot_id, item_id, worker_id, inspection_date, sample_qty, result, defect_qty, work_time_min, note
- `defect_reports`: report_id, record_id, defect_type, detail, root_cause, corrective_action, status, closed_date

---

### 2. Excelの課題とドメインモデルの対応確認

**確認対象**:
- [sessions/session6/excel-review.md](../session6/excel-review.md)（Excel課題詳細）
- [sessions/session262/pre-extracted-info-from-excel.md](../session262/pre-extracted-info-from-excel.md)（課題の具体例）

**Excel課題（Session 6より）**:
1. 列構成の不統一（メカのみ発注番号あり）
2. **ロット番号の欠落**
3. 作業者名の表記揺れ（「西村・西浦」「西村　西浦」など5パターン）
4. サプライヤー名の表記揺れ
5. 検査数量列に複数の情報が混在（検査方法、数量、作業者名）
6. 不良数量の記録が不完全（空欄でも備考に不良記述）
7. 「検査結果・備考」列に8種類の情報が混在

**対応状況の整理**（詳細は [domain-model-review.md](domain-model-review.md) 参照）:

| 課題 | ドメインモデル対応 | 判定 |
|------|----------------|------|
| 列構成の不統一 | nullable で吸収 | ✅ 対応済み |
| **ロット番号の欠落** | `lots.lot_id` のみ（システムID） | ⚠️ **部分対応** |
| 作業者名の表記揺れ | workers テーブルで正規化 | ✅ 対応済み（単独作業）<br>⚠️ 複数人作業は未対応 |
| サプライヤー名の表記揺れ | suppliers テーブルで正規化 | ✅ 対応済み |
| 検査数量列の混在 | 検査方法・作業者は分離済み | ⚠️ 検査方法詳細は未対応 |
| 不良数量の記録が不完全 | INTEGER型で型制約 | ⚠️ DEFAULT 0 で NULL と区別できない |
| 「検査結果・備考」の混在 | defect_reports で分離済み | ✅ 主要項目は分離済み<br>⚠️ note に一部混在 |

---

### 3. ドメインモデル修正の必要性判断

**修正優先度の分類**:

#### 優先度A（必須修正）

| 課題 | 修正内容 | 理由 |
|------|---------|------|
| **サプライヤーロット番号の欠落** | `lots.supplier_lot_number` を追加 | トレーサビリティに必須 |

**判断理由**:
- 現在の `lots.lot_id` はシステム内部のID（例: `LOT-12345678`）
- **サプライヤーが発行したロット番号**を記録するフィールドがない
- 不良発生時のトレーサビリティに必須

#### 優先度B（ワークショップ後に判断）

| 課題 | 修正内容 | 判断が必要な点 |
|------|---------|--------------|
| 複数人作業の記録 | 中間テーブル追加 or 運用ルール化 | 複数人記録が必要か確認 |
| 検査方法詳細の記録 | `inspection_method_detail` 追加 or レコード分割 | 検査記録の粒度を確認 |
| defect_qty のDEFAULT値 | DEFAULT 0 を削除 | 0 と NULL の運用ルール確認 |

**確認タイミング**: SIPOCワークショップ（Session 263計画）で現場担当者に確認

#### 優先度C（将来的に検討）

| 課題 | 修正内容 | 備考 |
|------|---------|------|
| 測定結果の構造化 | `measurement_results` テーブル追加 | プロトタイプでは note で代用可 |
| 改善提案の記録 | `improvement_suggestions` テーブル追加 | プロトタイプでは不要 |

---

## 成果物

| ファイル | 内容 |
|----------|------|
| [domain-model-review.md](domain-model-review.md) | ドメインモデルと課題の対応表（詳細） |
| [domain-model-revision-proposal.md](domain-model-revision-proposal.md) | 優先度A修正の実施計画（サプライヤーロット番号追加） |

---

## 主な発見

### 1. ドメインモデルは基本的に課題に対応している

**良い点**:
- 表記揺れ問題（作業者名、サプライヤー名）はマスタテーブルで正規化済み
- 「検査結果・備考」の混在は `defect_reports` で分離済み
- 不問判定は `waivers` テーブルで管理済み

**課題**:
- サプライヤーロット番号の欠落（必須修正）
- 複数人作業の記録方法が未確定（ワークショップで確認）
- 検査記録の粒度が未確定（ワークショップで確認）

### 2. 優先度Aは1件のみ（修正範囲は小さい）

**修正内容**:
- `lots.supplier_lot_number` フィールド追加のみ
- スキーマ、Goコード、テスト、フロントエンドの修正が必要だが、影響範囲は限定的

**実施計画**:
1. スキーマ修正 → マイグレーション実行
2. Goコード修正 → バックエンドビルド確認
3. テスト修正 → テスト実行・合格確認
4. フロントエンド修正 → 動作確認

### 3. 優先度Bはワークショップで判断

**確認すべきこと**:
- 「西村・西浦」のペアは何を意味するか？（作業分担? 記録係+実施係?）
- 検査記録の粒度は？（「外観全数、寸法抜き取り20個」を1レコードにまとめる? 2レコードに分ける?）
- 不良数量の運用ルールは？（0 と NULL の使い分け）

**ワークショップ実施予定**: Session 263で計画済み（[sipoc-workshop-execution-plan.md](../session263/sipoc-workshop-execution-plan.md)）

---

## 残った課題

### 優先度A修正の実施

**実施内容**:
- [ ] `lots.supplier_lot_number` フィールドを追加
- [ ] マイグレーションスクリプトを作成（`db/migrations/001_add_supplier_lot_number.sql`）
- [ ] Goコードの修正（Lot構造体、リポジトリ、API）
- [ ] テストの修正（lot_handler_test.go）
- [ ] フロントエンドの修正（型定義、作成フォーム、一覧画面）

**実施タイミング**: 次セッション（Session 265）

---

### ワークショップでの確認（優先度B）

**確認項目**:
- [ ] 複数人作業の記録が必要か（西村・西浦のペアの意味）
- [ ] 検査記録の粒度（1ロット1検査項目1レコード? or まとめて1レコード?）
- [ ] 不良数量の運用ルール（0 vs NULL の使い分け）

**実施タイミング**: SIPOCワークショップ（未定）

---

## 次セッションへの引き継ぎ

### Session 265: サプライヤーロット番号フィールドの追加

**前提**:
- Session 264でドメインモデルと課題の対応を確認
- 優先度Aの修正内容を決定（サプライヤーロット番号の追加）

**実施すること**:
1. スキーマ修正（schema.sql、マイグレーションスクリプト）
2. Goコード修正（Lot構造体、リポジトリ、API）
3. テスト修正
4. フロントエンド修正

**読むべきファイル**:
- [domain-model-revision-proposal.md](domain-model-revision-proposal.md)（実施計画）
- [prototype/m3/db/schema.sql](../../prototype/m3/db/schema.sql)（スキーマ）
- [prototype/m3/backend/internal/repository/lot.go](../../prototype/m3/backend/internal/repository/lot.go)（Goコード）

---

## 学び・気づき

### 1. ドメインモデルの事前設計は有効だった

**Session 33-37でTo-Beモデルを設計**した結果、主要な課題は既に対応されていた:
- マスタテーブルでの正規化（表記揺れ対策）
- defect_reports での不良詳細分離
- waivers での不問判定管理

**未対応だった項目**:
- サプライヤーロット番号（Excelには記録されていないため、設計時に想定外だった）

### 2. 課題の優先度づけが重要

**優先度A**: トレーサビリティに必須（サプライヤーロット番号）
**優先度B**: 運用次第で回避可能（複数人作業の記録、検査方法詳細）
**優先度C**: プロトタイプ段階では不要（測定結果の構造化、改善提案記録）

→ プロトタイプ段階では優先度Aのみ実施し、優先度Bはワークショップで確認してから判断

### 3. ワークショップの重要性

**ドメインモデルの設計だけでは分からないこと**:
- 複数人作業の記録は必要か?（システムで管理? 運用ルールで対応?）
- 検査記録の粒度は?（1検査項目1レコード? まとめて1レコード?）

→ 現場担当者とのワークショップで確認する必要がある

---

## 参照

### プロジェクト内資料

- [sessions/session6/excel-review.md](../session6/excel-review.md) — Excel課題の詳細
- [sessions/session262/pre-extracted-info-from-excel.md](../session262/pre-extracted-info-from-excel.md) — 課題の具体例
- [sessions/session263/session-summary.md](../session263/session-summary.md) — SIPOCワークショップ準備
- [prototype/m3/db/schema.sql](../../prototype/m3/db/schema.sql) — 現在のスキーマ

### スキル

- `~/.claude/skills/domain-modeling/SKILL.md` — ドメインモデリングスキル

---

*作成: Session 264 (2026-03-19)*
