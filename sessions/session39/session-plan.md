# Session 39 計画

**目的**: プロトタイプ実装（Phase A: バックエンド拡張）

---

## 前提

- 実装計画書: [prototype-implementation-plan.md](../../prototype/docs/prototype-implementation-plan.md)
- DB設計: [schema.sql](../../prototype/db/schema.sql)
- 既存API: ロットCRUD（10テスト通過）

---

## やること

### 1. シードデータ作成

**ファイル**: `prototype/db/seed.sql`

デモ用のサンプルデータ:
- サプライヤ: 3社（小峰無線、GREPOW、SKYDROID）
- 部品: 10種類
- 検査項目: 5種類（外観/通電/寸法/重量/動作）
- 作業者: 3名（杉山、石川、藤田）
- ロット: 20件（過去3ヶ月分）
- 検査記録: 50件

### 2. マスタデータAPI

**ファイル**: `prototype/backend/internal/handler/master_handler.go`

| エンドポイント | メソッド | 用途 |
|---------------|---------|------|
| `/api/v1/parts` | GET | 部品一覧（ドロップダウン用） |
| `/api/v1/inspection-items` | GET | 検査項目一覧 |
| `/api/v1/workers` | GET | 作業者一覧 |

### 3. 検査記録API

**ファイル**: `prototype/backend/internal/handler/inspection_handler.go`

| エンドポイント | メソッド | 用途 |
|---------------|---------|------|
| `/api/v1/inspection-records` | POST | 登録 |
| `/api/v1/inspection-records` | GET | 一覧 |
| `/api/v1/inspection-records/{id}` | GET | 詳細 |

**TDDテストシナリオ**:
1. 検査記録登録_正常 → 201
2. 検査記録登録_必須項目欠落 → 400
3. 検査記録一覧取得_正常 → 200
4. 検査記録詳細取得_正常 → 200
5. 検査記録詳細取得_存在しない → 404

---

## 参照資料

- [実装計画書](../../prototype/docs/prototype-implementation-plan.md)
- [DB設計](../../prototype/db/schema.sql)
- [ロットAPI実装](../../prototype/backend/internal/handler/lot_handler.go)
- [TDDスキル](~/.claude/skills/tdd-practice/SKILL.md)
