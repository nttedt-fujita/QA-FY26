# Session 37 計画

**目的**: TDDでロットCRUD API実装

---

## 前提

- Docker環境構築済み（Session 36）
- テストシナリオ設計済み（Session 36）

---

## やること

### 1. TDD Phase 3: テストコード作成

テストシナリオ（Session 36で承認済み）:

| # | シナリオ | 入力 | 期待結果 |
|---|---------|------|---------|
| 1 | ロット登録_正常 | `{part_id, quantity}` | 201, `{lot_id}` |
| 2 | ロット一覧取得_正常 | なし | 200, `[lots...]` |
| 3 | ロット詳細取得_正常 | `lot_id` | 200, `{lot}` |
| 4 | ロット詳細取得_存在しない | 不正なID | 404 |
| 5 | ロット登録_必須項目欠落 | `{quantity}` のみ | 400 |

**テスト構文**: Goのテーブルテスト形式

### 2. TDD Phase 4: 実装（Red → Green）

- 1つずつテストを通す
- PostgreSQL接続を実装

### 3. テスト実行環境

```bash
# テストDB起動
docker compose -f docker-compose.test.yml up -d

# テスト実行
go test ./...
```

---

## 参照資料

- [Session 36 サマリー](../session36/session-summary.md)
- [prototype/db/schema.sql](../../prototype/db/schema.sql) — DB設計
- TDDスキル: `~/.claude/skills/tdd-practice/SKILL.md`
- テストスタイル: `~/.claude/rules/06-test-style.md`
