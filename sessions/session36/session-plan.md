# Session 36 計画

**目的**: Docker環境構築 + TDDでGoバックエンド実装

---

## 方針

- **Docker化が環境構築の完了条件**
- 本番環境とテスト環境を分離
- TDDスキルに従い、Phase 0から進める
- 古典派TDD（モック最小限）
- テーブルテスト形式（`rules/06-test-style.md`準拠）

---

## やること

### 1. Docker環境構築（最優先）

```
prototype/
├── docker-compose.yml       # 開発環境（backend + db）
├── docker-compose.test.yml  # テスト環境
├── backend/
│   └── Dockerfile
└── db/
    └── init.sql             # 初期化スクリプト
```

- Go + PostgreSQL（本番想定）のコンテナ構成
- テスト用コンテナ（DBを分離）
- ホットリロード対応（air等）

### 2. TDD Phase 0-2: テストシナリオ設計

- 振る舞いの記述（ロット登録・一覧取得）
- テストシナリオリスト作成
- ユーザー承認

### 3. TDD Phase 3-4: テスト → 実装

- ロットCRUD
  - POST /api/v1/lots（登録）
  - GET /api/v1/lots（一覧）
  - GET /api/v1/lots/{id}（詳細）

### 4. DB接続

- PostgreSQL接続（Docker内）
- マイグレーション実行

---

## 参照資料

- [prototype/db/schema.sql](../../prototype/db/schema.sql) — DB設計
- [to-be-model.drawio](../../docs/missions/m3-incoming-inspection-db/to-be/to-be-model.drawio) — ドメインモデル
- TDDスキル: `~/.claude/skills/tdd-practice/SKILL.md`
