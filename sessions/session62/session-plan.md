# Session 62 計画

**日時**: 2026-03-XX（予定）
**前提**: Session 61で技術選定完了（Next.js + Rust + SQLite）

---

## 目的

1. DB設計の最終決定
2. ディレクトリ構成の整理
3. NAV-PVTパーサーのTDD実装

---

## タスク

### 1. DB設計の最終決定

Session 61で提案したテーブル構造をレビュー・確定:
- measurement_sessions
- nav_pvt_records
- satellite_records
- signal_records

### 2. ディレクトリ構成の整理

```
prototype/m1-gnss/
├── backend/         ← Rustバックエンド（現在のsrc/を移動）
├── frontend/        ← Next.js（新規作成）
├── db/              ← SQLiteスキーマ定義
├── docs/            ← 設計ドキュメント
└── docker-compose.yml
```

### 3. TDD Phase 3-4: NAV-PVTパーサー

**テストシナリオ（Session 60で合意済み）**:

| # | シナリオ |
|---|---------|
| 1 | 正常なNAV-PVTメッセージをパースできる |
| 2 | RTK Fixed状態を正しく判定できる |
| 3 | RTK Float状態を正しく判定できる |
| 4 | ヘッダーが不正な場合エラーを返す |
| 5 | Class/IDが不一致の場合エラーを返す |
| 6 | チェックサムが不正な場合エラーを返す |
| 7 | ペイロード長が不足の場合エラーを返す |

---

## 参照資料

- [Session 61 サマリー](../session61/session-summary.md)
- [NAV-PVT設計判断](../../prototype/m1-gnss/docs/nav-pvt-design-decisions.md)
- [10-tool-design-notes.md](../../docs/missions/m1-sensor-evaluation/gnss/10-tool-design-notes.md)
