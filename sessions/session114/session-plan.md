# Session 114 計画

**目的**: 屋内/屋外検査ページ分離

---

## 背景

Session 113でNAV-SIG FE実装完了。ユーザー要望：
- 屋外検査と屋内検査はページを分けたい

現状は `/inspections` 1ページに全て統合されている。

---

## やること

### 1. ページ分離設計

**案**:
- `/inspections/indoor` — 屋内検査（既存5項目）
- `/inspections/outdoor` — 屋外検査（NAV-SIG、RTK FIX率など）
- `/inspections` — 検査選択画面 or リダイレクト

### 2. 実装

- ルーティング変更
- 屋内検査ページ（既存コード移動）
- 屋外検査ページ（NavSigPanel + 将来の項目）

### 3. 検査ロジック整理

- 屋内: 既存5項目
- 屋外: L2受信率、RTK FIX率など（ADR-008基準）

---

## 参照資料

- [session113/session-summary.md](../session113/session-summary.md) — NAV-SIG FE実装
- [outdoor-inspection-priority.md](../session111/outdoor-inspection-priority.md) — Phase 1.5優先度
- [ADR-008](../../docs/adr/m1/ADR-008-outdoor-inspection-criteria.md) — 屋外検査合格基準

---

*計画作成: 2026-03-11 Session 113終了時*
