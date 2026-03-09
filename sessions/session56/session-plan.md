# Session 56 計画

**日時**: 2026-03-XX（予定）
**前提**: Session 55でGNSS資料の整理が必要と判明

---

## 目的

GNSS関連調査資料の整理・統合

---

## タスク

### 1. GNSS調査資料の整理（優先）

**背景**: Session 16-18で調査した内容が散在しており、次回以降の作業で見つけにくい

**整理方針**:

#### 1-1. 統合先

`docs/missions/m1-sensor-evaluation/gnss/` に統合

#### 1-2. 作成すべきドキュメント

| ドキュメント | 内容 | 元ネタ |
|-------------|------|--------|
| `10-ubx-protocol-reference.md` | UBXプロトコル仕様の索引・ツール化に使えるメッセージ一覧 | session17/pdf-excel-analysis.md |
| `11-tool-design-notes.md` | ツール設計メモ（自動化可能な項目、通信経路） | session17, session55調査結果 |
| `12-px4-uorb-mapping.md` | PX4 uORBトピックとUBXメッセージの対応表 | session55 Web調査結果 |

#### 1-3. 既存資料の整理

- session17/pdf-excel-analysis.md の「Phase 2以降で使う主要メッセージ」セクションを抽出
- session16/gnss-hearing-koitabashi-01.md の「調査タスク」セクションのステータス更新

---

### 2. 小板橋さんへ確認（継続）

**資料**: [session54/koitabashi-confirmation-checklist.md](../session54/koitabashi-confirmation-checklist.md)

### 3. 末永さんへ相談（継続）

**資料**: [session54/suenaga-consultation-checklist.md](../session54/suenaga-consultation-checklist.md)

### 4. M4工程不良Excel入手（継続）

---

## 参照資料

- [Session 55サマリー](../session55/session-summary.md)
- [session17/pdf-excel-analysis.md](../session17/pdf-excel-analysis.md) — UBXプロトコル調査結果
- [session16/gnss-hearing-koitabashi-01.md](../session16/gnss-hearing-koitabashi-01.md) — 小板橋さんヒアリング
