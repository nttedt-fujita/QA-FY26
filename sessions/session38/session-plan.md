# Session 38 計画

**目的**: フロントエンド開発（ロット登録画面）

---

## 前提

- バックエンドAPI完成（ロットCRUD、10テスト通過）
- Next.js初期化済み（TypeScript, Tailwind）

---

## やること

### 選択肢A: フロントエンド開発（優先）

1. **ロット登録フォーム**
   - part_id選択（ドロップダウン）
   - quantity入力
   - オプション項目（po_number, arrival_date, fw/hw_version）

2. **ロット一覧表示**
   - テーブル形式
   - 詳細リンク

3. **API連携**
   - fetch または SWR/TanStack Query

### 選択肢B: テストデータ充実

- 分析ダッシュボード用のサンプルデータ作成
- seed.sql の作成

### 選択肢C: マイグレーションツール導入

- goose または golang-migrate
- スキーマバージョン管理

---

## 参照資料

- [Session 37 サマリー](../session37/session-summary.md)
- [DB設計](../../prototype/db/schema.sql)
- [バックエンドAPI](../../prototype/backend/cmd/api/main.go)
