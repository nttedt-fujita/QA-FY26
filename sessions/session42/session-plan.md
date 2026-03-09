# Session 42 計画

**目的**: カウンター画面に必要なバックエンド準備

---

## 前提

- Session 41でモックアップ完成、パターンB（カウンター方式）採用
- 実装計画: [implementation-plan.md](../session41/implementation-plan.md)

---

## タスク

### 1. シードデータ作成

**ファイル**: `prototype/db/seed.sql`

| データ | 件数 |
|--------|------|
| サプライヤー | 3社 |
| 部品 | 10種類 |
| 検査項目 | 5種類 |
| 作業者 | 3名 |

### 2. マスタデータAPI

| エンドポイント | 説明 |
|---------------|------|
| `GET /api/v1/parts` | 部品一覧 |
| `GET /api/v1/inspection-items` | 検査項目一覧 |
| `GET /api/v1/workers` | 作業者一覧 |

### 3. 動作確認

- Docker環境起動
- シードデータ投入
- ロットAPI動作確認（Session 37で実装済み）

---

## 成果物

- `prototype/db/seed.sql`
- `prototype/backend/internal/handler/master_handler.go`
- `prototype/backend/internal/handler/master_handler_test.go`

---

## 参照

- [all-screens-mockup.drawio](../session41/all-screens-mockup.drawio) — モックアップ
- [lot_handler.go](../../prototype/backend/internal/handler/lot_handler.go) — 既存ロットAPI
