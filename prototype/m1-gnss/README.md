# GNSS評価ツール

F9P GNSS受信機の評価用Webツール。

## 技術スタック

- **言語**: Rust
- **Webフレームワーク**: Actix-web
- **シリアル通信**: serialport crate
- **UBXパース**: 自前実装

詳細: [ADR-005](../../docs/adr/ADR-005-gnss-tool-tech-stack.md)

## 開発環境（Docker + Dev Container）

### 前提条件

- Docker Desktop
- VSCode + Dev Containers拡張機能

### 起動方法

1. VSCodeでこのディレクトリを開く
2. コマンドパレット（Ctrl+Shift+P）→「Dev Containers: Reopen in Container」
3. コンテナ内でターミナルを開き:

```bash
cargo run
```

4. ブラウザで http://localhost:8080/health にアクセス

### Docker単体で起動する場合

```bash
# イメージビルド
docker build -f Dockerfile.dev -t m1-gnss-dev .

# コンテナ起動（ソースをマウント）
docker run -it --rm \
  -v $(pwd):/workspace \
  -p 8080:8080 \
  m1-gnss-dev \
  cargo run
```

## APIエンドポイント

| エンドポイント | メソッド | 説明 |
|---------------|---------|------|
| `/health` | GET | ヘルスチェック |
| `/api/gnss/status` | GET | GNSSステータス取得（ダミー） |

## 今後の実装予定

- [ ] UBXメッセージパース（NAV-SIG, NAV-SAT, NAV-PVT等）
- [ ] シリアルポート接続
- [ ] WebSocket対応（リアルタイム更新）
- [ ] スカイプロット表示（フロントエンド）
- [ ] C/N0グラフ表示
- [ ] CSV出力
