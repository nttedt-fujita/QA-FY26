# Session 36 サマリー

**日付**: 2026-03-06
**目的**: Docker環境構築 + TDDテストシナリオ設計

---

## 実施内容

### 1. Docker環境構築

- `docker-compose.yml` — 開発環境（Go + PostgreSQL）
- `docker-compose.test.yml` — テスト環境（ポート5433）
- `backend/Dockerfile` — マルチステージビルド
- `db/init.sql` — PostgreSQL初期化スクリプト

**確認済み**:
- ヘルスチェック: `curl http://localhost:8080/health` → `{"status":"ok"}`
- DBテーブル: 8テーブル作成済み

### 2. TDDルール追加

- `~/.claude/rules/09-tdd-skill-reference.md` — TDD作業時のスキル参照を強制

### 3. TDD Phase 0-2

**Phase 0**: プロジェクト文脈確認
- To-Beモデル（schema.sql）通りに実装する方針を確認
- ヒアリング後の変更は受け入れる

**Phase 1-2**: 振る舞いの記述 + テストシナリオ設計

| # | シナリオ | 入力 | 期待結果 |
|---|---------|------|---------|
| 1 | ロット登録_正常 | `{part_id, quantity}` | 201, `{lot_id}` |
| 2 | ロット一覧取得_正常 | なし | 200, `[lots...]` |
| 3 | ロット詳細取得_正常 | `lot_id` | 200, `{lot}` |
| 4 | ロット詳細取得_存在しない | 不正なID | 404 |
| 5 | ロット登録_必須項目欠落 | `{quantity}` のみ | 400 |

---

## 作成ファイル

| ファイル | 内容 |
|----------|------|
| [prototype/docker-compose.yml](../../prototype/docker-compose.yml) | 開発環境 |
| [prototype/docker-compose.test.yml](../../prototype/docker-compose.test.yml) | テスト環境 |
| [prototype/backend/Dockerfile](../../prototype/backend/Dockerfile) | Goバックエンド |
| [prototype/db/init.sql](../../prototype/db/init.sql) | PostgreSQL初期化 |
| [~/.claude/rules/09-tdd-skill-reference.md] | TDDスキル参照ルール |

---

## 重要な決定

1. **To-Beモデルで先に実装** — ヒアリング後の変更コストは受け入れる
2. **テスト戦略**: 統合テスト中心（シンプルなCRUDのため）

---

## 次セッション（Session 37）でやること

1. TDD Phase 3: テストコード作成
2. TDD Phase 4: 実装（Red → Green）
3. PostgreSQL接続実装
