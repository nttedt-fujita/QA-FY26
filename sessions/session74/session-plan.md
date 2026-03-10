# Session 74 計画

**目的**: GNSS評価ツール要件定義・アーキテクチャ設計

---

## やること

### 1. 要件定義（Requirement）作成

Session 73で作成した要求定義（gnss-tool-needs.md）を元に、要件を定義する。

| 種別 | 内容 |
|------|------|
| 機能要件（FR） | 各要求をどう実現するか |
| 非機能要件（NFR） | パフォーマンス、UI応答性、同時接続数 |
| インターフェース要件（IFR） | API、データ形式、UBXメッセージ |

### 2. アーキテクチャ設計

| 内容 | 成果物 |
|------|--------|
| コンポーネント構成 | シリアル層、パーサー層、API層、DB層 |
| データフロー | F9P → シリアル → パーサー → API → UI |
| 状態管理 | 接続/切断、検査中/完了 |

### 3. 実装計画

| 内容 | 成果物 |
|------|--------|
| フェーズ分け | v1 / v2 のスコープ |
| 優先順位 | 何を先に実装するか |

---

## 参照資料

- [session73/gnss-tool-needs.md](../session73/gnss-tool-needs.md) — 要求定義
- [ADR-005](../../docs/adr/ADR-005-gnss-tool-tech-stack.md) — 技術スタック
- [10-tool-design-notes.md](../../docs/missions/m1-sensor-evaluation/gnss/10-tool-design-notes.md) — 設計メモ

---

*計画作成: 2026-03-10 Session 73終了時*
