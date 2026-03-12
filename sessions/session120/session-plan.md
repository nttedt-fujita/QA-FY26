# Session 120 計画

**目的**: 屋内/屋外検査ページ分離 + TTFF/スカイプロット実装

---

## 背景

Session 119でMON-SPAN FE実装完了。Phase 1.5は残りRTK FIX率のみ。

**未着手の重要タスク**:
- **屋内/屋外検査ページ分離**: ずっと言っているがまだできていない
- 現状は「垂れ流し」のモニタリング状態で、検査機能として整理されていない

---

## やること（優先順）

### 0. 屋内/屋外検査ページ分離（最優先）

**現状の問題**:
- 検査実行画面が「モニタリング画面」になっている
- NAV-SIG、MON-SPANはリアルタイム表示のみで検査ロジックに組み込まれていない

**必要な作業**:
1. `/inspections/indoor` と `/inspections/outdoor` にページ分離
2. 屋内検査: 既存5項目（RATE, UART1, UART2, USB, NAV）
3. 屋外検査: L2受信率、RTK FIX率、MON-SPAN PGA判定
4. 検査開始→自動判定→結果記録のフロー整備

### 1. 屋外動作確認

- NAV-SIG: L2受信率が50%以上で合格判定されるか
- MON-SPAN: PGAゲインが54dB程度（正常機）で正常判定されるか

### 2. TTFF測定（優先）

**仕様確認**:
- [ttff-monrf-spec.md](../session111/ttff-monrf-spec.md) を読む
- NAV-STATUS.ttff（ミリ秒）で取得可能

**実装**:
- NAV-STATUSパーサー追加
- TTFF表示パネル

**課題**:
- Cold/Warm/Hot Startの再現手順

### 3. NAV-SAT スカイプロット（優先）

**仕様確認**:
- NAV-SAT（UBX 0x01 0x35）の仕様を調査

**実装**:
- NAV-SATパーサー
- スカイプロット表示（仰角/方位角）

---

## 参照資料

- [ttff-monrf-spec.md](../session111/ttff-monrf-spec.md) — TTFF仕様
- [outdoor-inspection-priority.md](../session111/outdoor-inspection-priority.md) — 優先度整理
- [ADR-008](../../docs/adr/m1/ADR-008-outdoor-inspection-criteria.md) — 屋外検査合格基準

---

*計画作成: 2026-03-12 Session 119終了時*
