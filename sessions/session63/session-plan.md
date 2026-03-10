# Session 63 計画

**日時**: 2026-03-XX（予定）
**前提**: Session 62でドメインモデリング完了

---

## 目的

1. テーブル設計（ドメインモデルをDBに落とし込み）
2. ディレクトリ構成の整理
3. NAV-PVTパーサーのTDD実装

---

## タスク

### 1. テーブル設計

[prototype/m1-gnss/docs/domain-model.md](../../prototype/m1-gnss/docs/domain-model.md) をベースに：

- devices テーブル
- measurement_sessions テーブル
- nav_pvt_records テーブル
- nav_status_records テーブル
- nav_dop_records テーブル
- satellites テーブル
- signals テーブル
- mon_span_records テーブル
- mon_rf_records テーブル

スキーマファイル: `db/schema.sql`

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

- [Session 62 サマリー](../session62/session-summary.md)
- [ドメインモデル](../../prototype/m1-gnss/docs/domain-model.md)
- [NAV-PVT設計判断](../../prototype/m1-gnss/docs/nav-pvt-design-decisions.md)
