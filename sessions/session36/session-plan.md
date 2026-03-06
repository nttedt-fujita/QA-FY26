# Session 36 計画

**目的**: TDDでGoバックエンド実装（ロットCRUD）

---

## 方針

- **TDDスキル**に従い、Phase 0から進める
- 古典派TDD（モック最小限）
- テーブルテスト形式（`rules/06-test-style.md`準拠）

---

## やること

### 1. TDD Phase 0-2: テストシナリオ設計

- 振る舞いの記述（ロット登録・一覧取得）
- テストシナリオリスト作成
- ユーザー承認

### 2. TDD Phase 3-4: テスト → 実装

- ロットCRUD
  - POST /api/v1/lots（登録）
  - GET /api/v1/lots（一覧）
  - GET /api/v1/lots/{id}（詳細）

### 3. DB接続

- SQLite接続（ローカル開発用）
- マイグレーション実行

---

## 参照資料

- [prototype/db/schema.sql](../../prototype/db/schema.sql) — DB設計
- [to-be-model.drawio](../../docs/missions/m3-incoming-inspection-db/to-be/to-be-model.drawio) — ドメインモデル
- TDDスキル: `~/.claude/skills/tdd-practice/SKILL.md`
