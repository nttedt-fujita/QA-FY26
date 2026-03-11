# Session 100 計画

**目的**: 開発環境整理 + UBX通信タイミング問題のデバッグ

---

## 背景

Session 99でデバッグログを追加したが、プロセス競合で実機テストが中断。
開発環境の整理が必要。

---

## やること

### 1. 開発環境の整理

- 既存プロセスの停止・整理
- Docker Compose 環境の検討（バックエンド + フロントエンド）

### 2. 実機テストでログ確認

```bash
# 既存プロセスを停止
pkill -f m1-gnss
pkill -f "next dev"

# バックエンド起動（デバッグログ付き）
cd prototype/m1-gnss/backend
RUST_LOG=debug cargo run

# 別ターミナルでフロントエンド起動
cd prototype/m1-gnss/frontend
npm run dev
```

### 3. 根本原因の特定

ログで確認すべき点:
- [ ] 各項目で何バイト受信したか
- [ ] タイムアウトしている項目
- [ ] drain_buffer で読み捨てたバイト数
- [ ] 受信データの hex dump（UBXフレームとして正しいか）

### 4. 対策実装

原因に応じて:
- 待機時間の調整
- リトライ機構の追加
- UBXフレームの完全性チェック

---

## 参照資料

- [Session 99 サマリー](../session99/session-summary.md)
- [engine.rs](../../prototype/m1-gnss/backend/src/inspection/engine.rs)
- [manager.rs](../../prototype/m1-gnss/backend/src/device/manager.rs)

---

*計画作成: 2026-03-11 Session 99終了時*
