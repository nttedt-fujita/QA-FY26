# Session 35 計画

**目的**: プロトタイプ環境構築（Go + Next.js）

---

## 方針

**ローカル開発優先**: プロトタイプはまずローカルで動かす。AWSは後回し。

**確認すべき点**:
- AWSコスト構成が本当に安いか再チェック（Session 8の試算を検証）
- 不要なサービスを使っていないか
- ローカルDBはSQLite or PostgreSQL（Docker）

---

## やること

### 1. プロジェクト構成の確定

```
prototype/
├── backend/     # Go バックエンド
├── frontend/    # Next.js フロントエンド
└── infra/       # AWS CDK
```

### 2. Go バックエンド初期セットアップ

- go mod init
- ディレクトリ構成（cmd/api, internal/）
- 最小限のHTTPサーバー

### 3. Next.js フロントエンド初期セットアップ

- create-next-app
- shadcn/ui + Tailwind CSS

### 4. DB設計の叩き台

To-Beドメインモデル（Session 33）をベースにテーブル設計:
- [to-be-model.drawio](../../docs/missions/m3-incoming-inspection-db/to-be/to-be-model.drawio)

---

## 参照資料

- [session34/session-summary.md](../session34/session-summary.md) — 技術選定変更の決定
- [to-be-model.drawio](../../docs/missions/m3-incoming-inspection-db/to-be/to-be-model.drawio) — ドメインモデル
