# Session 61 計画

**日時**: 2026-03-09
**前提**: Session 60でTDD Phase 0-2完了、環境構築完了

---

## 目的

1. ドメインモデリング・DB設計の検討
2. NAV-PVTパーサーのテストコード作成と実装

---

## タスク

### 0. ドメインモデリング・ディレクトリ構成整理

**検討事項**:
- データ管理にDBを使うか（SQLite等）
- 測定データの永続化方法
- フロントエンド（静的HTML? React?）

**ディレクトリ構成の整理**:
```
prototype/m1-gnss/
├── backend/         ← Rustバックエンド（現在のsrc/を移動）
├── frontend/        ← フロントエンド（新規）
├── db/              ← DB定義（必要に応じて）
├── docs/            ← 設計ドキュメント
└── docker-compose.yml
```

### 1. TDD Phase 3: テストコード作成

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

**成果物**:
- `src/ubx/mod.rs`
- `src/ubx/parser.rs`
- `src/ubx/messages.rs`
- テストコード（同ファイル内 or `tests/`）

### 2. TDD Phase 4: 実装（Red → Green）

- テストを1つずつ通す
- 最小限のコードで実装

### 3. 他のUBXメッセージの設計（時間があれば）

- NAV-SAT（衛星方位角・仰角）
- NAV-SIG（L1/L2別C/N0）

---

## 参照資料

- [Session 60 サマリー](../session60/session-summary.md)
- [NAV-PVT設計判断](../../prototype/m1-gnss/docs/nav-pvt-design-decisions.md)
- [UBXプロトコル索引](../../docs/missions/m1-sensor-evaluation/gnss/08-ubx-protocol-index.md)
