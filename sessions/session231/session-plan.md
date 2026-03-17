# Session 231 計画

**目的**: Living Documentation断捨離作業（フェーズ2）

**前提**: Session 230で方針確定済み

---

## やること

| # | 作業 | 読むべきファイル | 参照コマンド |
|---|------|-----------------|--------------|
| 1 | 重複ファイル削除（10件） | session230/living-documentation-concept.md | - |
| 2 | 📝 sessionマーク移動（2件） | docs/gnss/README.md | - |
| 3 | 未掲載抽出物の確認・判断 | session230/living-documentation-concept.md | - |
| 4 | ルールファイル改善 | ~/.claude/rules/14-document-management.md | - |

---

## 詳細

### 1. 重複ファイル削除

sessions/側を削除（docs/に同一がある）:

| 削除対象 |
|----------|
| session52/m2-obstacle-detection-report.md |
| session52/m2-confirmation-checklist.md |
| session14/closed-questions-m3m4.md |
| session14/mockup-concepts.md |
| session5/platform-comparison.md |
| session5/maintenance-strategy.md |
| session5/kintone-evaluation.md |
| session5/aws-cost-estimation.md |
| session9/typescript-vs-go-report.md |
| session9/quicksight-report.md |

### 2. 📝 sessionマーク移動

| 対象 | 現在地 | 移動先 |
|------|--------|--------|
| CFG-CFG | session211/cfg-cfg-spec.md | docs/gnss/39-cfg-cfg-spec.md |
| CFG-VALSET/VALGET | session214/cfg-valget-spec.md | docs/gnss/40-cfg-valget-spec.md |

### 3. 未掲載抽出物の確認

以下を確認し、移動 or 削除を判断:

- session165/cfg-uart1-spec.md
- session76/ubx-mon-ver-sec-uniqid-spec.md
- session64/ubx-messages-spec.md

### 4. ルールファイル改善

`14-document-management.md` に「仕様抽出のライフサイクル」セクション追加。

---

## 参照

- [session230/living-documentation-concept.md](../session230/living-documentation-concept.md) — 方針と作業詳細
- [session230/session-summary.md](../session230/session-summary.md) — 前セッションサマリー

---

*作成: Session 230 (2026-03-17)*
