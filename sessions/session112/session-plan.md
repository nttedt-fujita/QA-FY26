# Session 112 計画

**目的**: NAV-SIG API/FE連携の設計・実装開始

---

## 背景

Session 111でPhase 1.5の優先度を整理。
最優先はNAV-SIG API/FE連携（L1/L2別C/N0一覧、L2受信率表示）。

---

## やること

### 1. 設計確認

- 既存のAPI構造を確認（Axum）
- エンドポイント設計
  - `GET /api/nav-sig` — NAV-SIG取得
  - `GET /api/outdoor-inspection` — 屋外検査結果（L2受信率等）

### 2. バックエンド実装

**優先順位**:

1. `/api/nav-sig` エンドポイント追加
2. signal_stats呼び出し（既存関数）
3. OutdoorInspector（閾値判定）

**テスト**:
- 単体テスト（TDD）
- 実機テスト（NAV-SIG取得確認）

### 3. （時間があれば）フロントエンド設計

- 画面モックアップ
- コンポーネント構成

---

## 参照資料

- [outdoor-inspection-priority.md](../session111/outdoor-inspection-priority.md) — 優先度整理
- [ADR-008](../../docs/adr/m1/ADR-008-outdoor-inspection-criteria.md) — 合格基準
- [nav-sig-behavior-spec.md](../session107/nav-sig-behavior-spec.md) — NAV-SIG振る舞い仕様

---

## 成果物（予定）

- `/api/nav-sig` エンドポイント
- OutdoorInspector（または関数）
- 単体テスト

---

*計画作成: 2026-03-11 Session 111終了時*
