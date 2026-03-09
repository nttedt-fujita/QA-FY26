# クイックスタート

M3 受入検査DB プロトタイプの起動手順。

## 前提条件

- Docker / Docker Compose
- Node.js 18以上
- Make

## 起動手順

### 1. バックエンド起動（DB + API）

```bash
cd prototype
make up
```

起動確認:
```bash
make status
```

### 2. フロントエンド起動

```bash
make frontend-dev
```

ブラウザで http://localhost:3000 を開く。

## よく使うコマンド

| コマンド | 説明 |
|---------|------|
| `make help` | コマンド一覧表示 |
| `make up` | 全サービス起動 |
| `make down` | 全サービス停止 |
| `make status` | 状態確認 |
| `make db-psql` | DB接続 |
| `make db-counts` | レコード数確認 |
| `make demo-flow` | デモフロー表示 |

## トラブルシューティング

### ポートが使用中

```bash
# 既存コンテナ停止
make down

# ポート確認
lsof -i :5432  # DB
lsof -i :8080  # Backend
lsof -i :3000  # Frontend
```

### DBリセット

```bash
make db-reset
```

### 全クリーンアップ

```bash
make clean
```
