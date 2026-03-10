# Session 66 サマリー

**日時**: 2026-03-10
**実施環境**: Windows PC（WSL2）

---

## 目的

M3（受入検査DB）プロトタイプの起動・動作確認

## 実施内容

### 1. M3プロトタイプ起動

- `docker compose up -d` でDB（PostgreSQL 16）+ Backend（Go）を起動
- Next.jsフロントエンド（`npm run dev`）を起動（Node v20 via nvm）
- 全サービス正常稼働を確認

### 2. API動作確認

以下のエンドポイントをcurlで確認（全て正常レスポンス）:
- `GET /health` → `{"status":"ok"}`
- `GET /api/v1/parts` → 部品10件
- `GET /api/v1/lots` → ロット2件
- `GET /api/v1/workers` → 作業者3名
- `GET /api/v1/dashboard/summary` → 検査記録0件（シードデータに検査記録なし）

### 3. 別PCからのアクセス設定

WSL2はデフォルトで外部からアクセス不可のため、Windowsでポートフォワーディングを設定:

```powershell
# ポートフォワーディング
netsh interface portproxy add v4tov4 listenport=3000 listenaddress=0.0.0.0 connectport=3000 connectaddress=172.20.171.75
netsh interface portproxy add v4tov4 listenport=8080 listenaddress=0.0.0.0 connectport=8080 connectaddress=172.20.171.75

# ファイアウォール
netsh advfirewall firewall add rule name="WSL2 M3 Frontend" dir=in action=allow protocol=TCP localport=3000
netsh advfirewall firewall add rule name="WSL2 M3 Backend" dir=in action=allow protocol=TCP localport=8080
```

別PCから `http://192.168.100.57:3000` でフロントエンド表示を確認済み。

### 4. 解除コマンド（メモ）

```powershell
netsh interface portproxy delete v4tov4 listenport=3000 listenaddress=0.0.0.0
netsh interface portproxy delete v4tov4 listenport=8080 listenaddress=0.0.0.0
```

---

## 本セッションでの注意点

- 今回はWindows PC上のWSL2で作業（普段のUbuntu PCとは別）
- テストデータ投入等の本格作業はUbuntu PCで実施予定

## 未実施（次セッションへ持ち越し）

- UBXパーサー実装（NAV-STATUS/NAV-DOP/MON-RF）— TDD
- DevContainer内でのテスト実行確認

## 次セッションでやること

- Ubuntu PCでM3プロトタイプの作業を再開
- Session 66計画のタスク（UBXパーサー実装）を実施
