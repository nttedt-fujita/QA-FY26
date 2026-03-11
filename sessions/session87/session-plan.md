# Session 87 計画

**目的**: 統合DB設計の確定・実装

---

## 背景

Session 86で統合ドメインモデルと統合DB設計（ドラフト）を作成。
次はDB設計を確定し、既存コードを修正する。

---

## やること

### 1. 統合DB設計の確定

- [unified-db-schema.md](../session86/unified-db-schema.md) をレビュー
- `db/schema.sql` を更新

### 2. 既存コードの修正

- `src/repository/types.rs` — 新しいエンティティ追加
- `src/repository/sqlite.rs` — 新しいテーブル対応
- `src/inspection/engine.rs` — 結果保存先の変更

### 3. 実装計画更新

- Phase 1の計画を更新（屋内+屋外統合を反映）

---

## 完了条件

- [ ] 統合DB設計が確定している
- [ ] schema.sqlが更新されている
- [ ] 既存テストがパスする

---

## 参照資料

- [session86/gnss-unified-domain-model.md](../session86/gnss-unified-domain-model.md) — 統合ドメインモデル
- [session86/unified-db-schema.md](../session86/unified-db-schema.md) — 統合DB設計
- [prototype/m1-gnss/db/schema.sql](../../prototype/m1-gnss/db/schema.sql) — 既存スキーマ

---

*計画作成: 2026-03-11 Session 86終了時*
