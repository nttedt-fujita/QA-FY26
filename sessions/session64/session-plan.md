# Session 64 計画

**日時**: 2026-03-XX（予定）
**前提**: Session 63でテーブル設計・NAV-PVTパーサー完了

---

## 目的

1. DevContainer内でテスト実行確認
2. 他のUBXメッセージパーサー実装
3. Next.jsフロントエンドプロジェクト作成

---

## タスク

### 1. テスト実行確認

DevContainer内で以下を実行:

```bash
cd backend
cargo test --lib
```

全13テストが通ることを確認。

### 2. 追加UBXパーサー

優先度順:

1. **NAV-STATUS** — TTFF、Fix状態
2. **NAV-DOP** — 精度劣化係数
3. **MON-RF** — ジャミング状態

各パーサーでテストケースを作成（テーブルテスト形式）。

### 3. Next.jsプロジェクト作成

```bash
cd frontend
npx create-next-app@latest . --typescript --tailwind --eslint --app --src-dir --no-import-alias
```

最小限の構成:
- ヘルスチェック表示
- GNSSステータス表示（ダミー）

---

## 参照資料

- [Session 63 サマリー](../session63/session-summary.md)
- [ドメインモデル](../../prototype/m1-gnss/docs/domain-model.md)
- [NAV-PVT設計判断](../../prototype/m1-gnss/docs/nav-pvt-design-decisions.md)
