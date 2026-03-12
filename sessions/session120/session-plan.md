# Session 120 計画

**目的**: TTFF/スカイプロット実装 → 検査機能整理 → 動作確認

---

## 背景

Session 119でMON-SPAN FE実装完了。

**現状の問題**:
- 「垂れ流し」モニタリング状態
- 検査機能として整理されていない
- u-centerとのデータ照合（信頼性評価）が未実施

---

## やること（優先順）

### 1. TTFF測定

**仕様確認**:
- [ttff-monrf-spec.md](../session111/ttff-monrf-spec.md) を読む
- NAV-STATUS.ttff（ミリ秒）で取得可能

**実装**:
- NAV-STATUSパーサー追加
- TTFF表示パネル

**課題**:
- Cold/Warm/Hot Startの再現手順

### 2. NAV-SAT スカイプロット

**仕様確認**:
- NAV-SAT（UBX 0x01 0x35）の仕様を調査

**実装**:
- NAV-SATパーサー
- スカイプロット表示（仰角/方位角の極座標グラフ）

### 3. 屋内/屋外検査ページ分離

**必要な作業**:
1. `/inspections/indoor` と `/inspections/outdoor` にページ分離
2. 屋内検査: 既存5項目（RATE, UART1, UART2, USB, NAV）
3. 屋外検査: L2受信率、RTK FIX率、MON-SPAN PGA判定、TTFF、スカイプロット
4. 検査開始→自動判定→結果記録のフロー整備

### 4. u-centerとのデータ照合（信頼性評価）

**目的**: 今回作ったツールの出力がu-centerと一致するか検証

**確認項目**:
- NAV-SIG: C/N0値、L1/L2分類
- MON-SPAN: スペクトラム波形、PGAゲイン
- NAV-SAT: 仰角/方位角
- TTFF: 測定値

### 5. 屋外動作確認

1〜4が完了してから実施:
- NAV-SIG: L2受信率50%以上で合格判定されるか
- MON-SPAN: PGAゲイン54dB程度で正常判定されるか
- TTFF: 測定値の妥当性
- スカイプロット: 表示の妥当性

---

## 参照資料

- [ttff-monrf-spec.md](../session111/ttff-monrf-spec.md) — TTFF仕様
- [outdoor-inspection-priority.md](../session111/outdoor-inspection-priority.md) — 優先度整理
- [ADR-008](../../docs/adr/m1/ADR-008-outdoor-inspection-criteria.md) — 屋外検査合格基準

---

*計画作成: 2026-03-12 Session 119終了時*
*計画修正: 2026-03-12 優先順変更（TTFF/スカイプロット→検査整理→動作確認）*
