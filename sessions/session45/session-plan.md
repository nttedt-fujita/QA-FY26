# Session 45 計画

**日時**: 2026-03-09（予定）
**前提**: Session 44でカウンター画面実装完了

---

## 目的

1. **工数表示の修正**（優先）
2. ロット登録画面の実装

---

## タスク

### 0. CLAUDE.md整理（軽作業）

**問題**: 「過去セッションで確認した資料の索引」セクションがセッション履歴（session-history/）と重複している

**対処**:
- CLAUDE.mdから「過去セッションで確認した資料の索引」セクションを削除
- 詳細はsession-historyを参照するルールに統一

### 1. 工数表示の修正（優先）

**問題**: 検査終了時のalertで工数が `0.24025752946666667分` のように小数点以下が長すぎる

**対処方針**:
- バックエンドで `math.Round()` を使って整数に丸める
- または `fmt.Sprintf("%.1f", workTimeMin)` で小数点1桁に

**修正箇所**:
- `prototype/backend/internal/handler/inspection_session_handler.go` の Finish 関数

### 2. ロット登録画面の実装

**実装計画より（Session 41）**:
- 部品選択（ドロップダウン、サプライヤー自動絞り込み）
- 入荷数量入力
- 入荷日（デフォルト今日）
- 登録後は検査記録入力画面へ遷移

**現状**:
- `prototype/frontend/src/app/page.tsx` に既存のロット登録フォームあり
- ただし部品IDが手入力になっている

**改善点**:
1. 部品をドロップダウンで選択
2. サプライヤーを自動表示
3. ロットIDを自動採番（`LOT-YYYYMMDD-NNN`形式を検討）

---

## 参照資料

- [Session 44 サマリー](../session44/session-summary.md)
- [実装計画](../../prototype/docs/implementation-plan.md)
- [全画面モックアップ](../session41/all-screens-mockup.drawio)
