# Session 122 計画

**目的**: TDDレビュー再実施 → 屋内/屋外検査ページ分離

---

## 背景

Session 121でスカイプロット機能を実装完了。TDDレビューを実施したが、TDDスキルを読まずに実施したためルール違反。再実施が必要。

---

## やること（優先順）

### 1. TDDレビュー再実施

**手順**:
1. TDDスキル（`~/.claude/skills/tdd-practice/SKILL.md`）を読む
2. テストスタイルルール（`~/.claude/rules/06-test-style.md`）を確認
3. 以下の機能をレビュー:
   - TTFF測定機能（Session 120）
   - スカイプロット機能（Session 121）

**レビュー観点**（スキルに従う）:
- 振る舞いベースのテストになっているか
- テーブルテスト形式で書かれているか
- should_succeedパラメータがあるか
- 境界値・異常系のカバー

### 2. 屋内/屋外検査ページ分離

**必要な作業**:
1. `/inspections/indoor` — 既存5項目（RATE, UART1, UART2, USB, NAV）
2. `/inspections/outdoor` — L2受信率、RTK FIX率、MON-SPAN、TTFF、スカイプロット
3. 検査開始→自動判定→結果記録のフロー整備

**バグ修正**:
- 検査画面から他ページへ遷移できない問題
- 原因: 常時ポーリングがページ遷移をブロック
- 対処: 「検査開始→指定時間だけポーリング→停止」に変更

---

## 参照資料

- [tdd-practice/SKILL.md](~/.claude/skills/tdd-practice/SKILL.md) — TDDスキル
- [06-test-style.md](~/.claude/rules/06-test-style.md) — テストスタイルルール
- [ADR-008](../../docs/adr/m1/ADR-008-outdoor-inspection-criteria.md) — 屋外検査合格基準
- [session121/session-summary.md](../session121/session-summary.md) — 前セッション

---

*計画作成: 2026-03-12 Session 121終了時*
