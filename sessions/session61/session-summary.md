# Session 61 サマリー

**日時**: 2026-03-09
**目的**: GNSS評価ツールのドメインモデリング・技術選定

---

## 実施内容

1. **フロントエンド技術選定**
   - Vue / React / Next.js の比較検討
   - **決定: Next.js（React）**
   - 理由: 要求を満たせる、表現力が高い、エコシステムが豊富

2. **要求の再確認**
   - [10-tool-design-notes.md](../../docs/missions/m1-sensor-evaluation/gnss/10-tool-design-notes.md) を再確認
   - 衛星配置（仰角・方位角）、衛星数、C/N0（L1/L2別）、RTK状態、精度の表示が必要

3. **DB設計案の検討**
   - SQLite採用を決定
   - テーブル構造案を作成（measurement_sessions, nav_pvt_records, satellite_records, signal_records）

---

## 決定事項

| 項目 | 決定 |
|------|------|
| フロントエンド | Next.js（React） |
| バックエンド | Rust + Actix-web（Session 60で決定済み） |
| 永続化 | SQLite |

---

## 未実施（Session 62へ持ち越し）

- DB設計の最終決定
- ディレクトリ構成の整理（backend/ frontend/ db/ 分離）
- TDD Phase 3: NAV-PVTテストコード作成
- TDD Phase 4: NAV-PVTパーサー実装

---

## 振り返り

- 要求（10-tool-design-notes.md）を確認せずに技術選定の議論に入ってしまった
- 「何のためのツールを作るのか」を最初に確認すべきだった

---

## 参照資料

- [10-tool-design-notes.md](../../docs/missions/m1-sensor-evaluation/gnss/10-tool-design-notes.md) — ツール設計メモ（要求整理）
- [ADR-004](../../docs/adr/ADR-004-gnss-tool-approach.md) — 直接UBX通信アプローチ選択
- [ADR-005](../../docs/adr/ADR-005-gnss-tool-tech-stack.md) — 技術スタック選定
