# Session 265 計画

**目的**: サプライヤーロット番号フィールドの追加（優先度A修正）

**前提**:
- Session 264でドメインモデルと課題の対応を確認
- 優先度Aの修正内容を決定（サプライヤーロット番号の追加）
- トレーサビリティに必須のため、必ず実施する

---

## やること

| # | 作業 | 読むべきファイル | 参照コマンド |
|---|------|-----------------|--------------|
| 1 | スキーマ修正 | sessions/session264/domain-model-revision-proposal.md<br>prototype/m3/db/schema.sql | - |
| 2 | マイグレーションスクリプト作成 | sessions/session264/domain-model-revision-proposal.md | - |
| 3 | Goコード修正 | prototype/m3/backend/internal/repository/lot.go | - |
| 4 | テスト修正 | prototype/m3/backend/internal/handler/lot_handler_test.go | - |
| 5 | フロントエンド修正 | prototype/m3/frontend/src/types/lot.ts | - |
| 6 | 動作確認 | - | make help（M3のMakefile確認） |

---

## 詳細

### 1. スキーマ修正

**修正内容**:
```sql
ALTER TABLE lots ADD COLUMN supplier_lot_number TEXT;
```

**修正ファイル**:
- `prototype/m3/db/schema.sql`（行57-68）

**フィールド仕様**:
- フィールド名: `supplier_lot_number`
- 型: TEXT
- NULL許容: ✅ Yes
- デフォルト: NULL
- 説明: サプライヤーが発行したロット番号

---

### 2. マイグレーションスクリプト作成

**作成ファイル**: `prototype/m3/db/migrations/001_add_supplier_lot_number.sql`

**内容**:
```sql
-- Migration: サプライヤーロット番号の追加
-- Date: 2026-03-19
-- Session: 264

ALTER TABLE lots ADD COLUMN supplier_lot_number TEXT;

-- コメント: 既存データはNULLのまま（後で手動入力またはインポート時に設定）
```

---

### 3. Goコード修正

**修正ファイル**: `prototype/m3/backend/internal/repository/lot.go`

**修正箇所**:
1. Lot構造体に `SupplierLotNumber` フィールドを追加
2. Create関数のINSERT文とScanに追加
3. FindAll関数のSELECT文とScanに追加
4. FindByID関数のSELECT文とScanに追加

**詳細**: [sessions/session264/domain-model-revision-proposal.md](../session264/domain-model-revision-proposal.md) Phase 2参照

---

### 4. テスト修正

**修正ファイル**: `prototype/m3/backend/internal/handler/lot_handler_test.go`

**修正内容**:
- ロット作成のテストで `supplier_lot_number` を含めたテストケースを追加
- JSON レスポンスに `supplier_lot_number` が含まれることを確認

---

### 5. フロントエンド修正

**修正ファイル**（推測）:
- `prototype/m3/frontend/src/types/lot.ts`（型定義）
- `prototype/m3/frontend/src/app/lots/new/page.tsx`（作成フォーム）
- `prototype/m3/frontend/src/app/lots/page.tsx`（一覧）

**修正内容**:
- Lot型に `supplier_lot_number?: string` を追加
- 作成フォームに入力欄を追加
- 一覧・詳細画面に表示を追加

---

### 6. 動作確認

**確認項目**:
- [ ] マイグレーション実行成功
- [ ] バックエンドビルド成功
- [ ] テスト実行・合格
- [ ] ロット作成フォームで入力可能
- [ ] ロット一覧で表示される

---

## 期待される成果物

| ファイル | 内容 |
|----------|------|
| prototype/m3/db/schema.sql（修正） | supplier_lot_number フィールド追加 |
| prototype/m3/db/migrations/001_add_supplier_lot_number.sql（新規） | マイグレーションスクリプト |
| prototype/m3/backend/internal/repository/lot.go（修正） | Lot構造体、CRUD処理の修正 |
| prototype/m3/backend/internal/handler/lot_handler_test.go（修正） | テストケース追加 |
| prototype/m3/frontend/src/types/lot.ts（修正） | 型定義修正 |
| prototype/m3/frontend/src/app/lots/new/page.tsx（修正） | 作成フォーム修正 |
| prototype/m3/frontend/src/app/lots/page.tsx（修正） | 一覧画面修正 |

---

## 参照

- [sessions/session264/session-summary.md](../session264/session-summary.md) — Session 264サマリー
- [sessions/session264/domain-model-revision-proposal.md](../session264/domain-model-revision-proposal.md) — 修正提案（詳細）
- [sessions/session264/domain-model-review.md](../session264/domain-model-review.md) — ドメインモデルと課題の対応表

---

*作成: Session 264 (2026-03-19)*
