# Session 121 計画

**目的**: NAV-SATスカイプロット実装 → 検査ページ分離

---

## 背景

Session 120でTTFF測定機能を完了。NAV-STATUSパネルで以下を表示可能に：
- TTFF（Time to First Fix）
- Fix状態・RTK状態
- 起動からの経過時間

---

## やること（優先順）

### 1. NAV-SATスカイプロット

**仕様確認**:
- [ubx-nav-messages.md](../../docs/missions/m1-sensor-evaluation/gnss/ubx-nav-messages.md) を読む
- NAV-SAT（UBX 0x01 0x35）の構造確認
- 仰角（elev）、方位角（azim）フィールドの位置

**実装**:
1. NAV-SATパーサー（`nav_sat.rs`）
2. NAV-SAT API（`nav_sat_api.rs`）
3. スカイプロット表示パネル（極座標グラフ）

**スカイプロット表示要件**:
- 極座標: 中心が天頂（仰角90°）、外周が水平線（仰角0°）
- 方位角: 北=0°, 東=90°, 南=180°, 西=270°
- 衛星をドットで表示、C/N0で色分け

### 2. 屋内/屋外検査ページ分離

**必要な作業**:
1. `/inspections/indoor` — 既存5項目（RATE, UART1, UART2, USB, NAV）
2. `/inspections/outdoor` — L2受信率、RTK FIX率、MON-SPAN、TTFF、スカイプロット
3. 検査開始→自動判定→結果記録のフロー整備

**バグ修正**:
- 検査画面から他ページへ遷移できない問題
- 原因: 常時ポーリングがページ遷移をブロック
- 対処: 「検査開始→指定時間だけポーリング→停止」に変更

### 3. u-centerとのデータ照合

Session 120で未着手。

---

## 参照資料

- [ubx-nav-messages.md](../../docs/missions/m1-sensor-evaluation/gnss/ubx-nav-messages.md) — NAV-SAT仕様
- [ADR-008](../../docs/adr/m1/ADR-008-outdoor-inspection-criteria.md) — 屋外検査合格基準
- [session120/session-summary.md](../session120/session-summary.md) — 前セッション

---

## 補足（藤田さんより）

スカイプロット機能完成後、TDDスキルを使って以下をレビューする：
- TTFF測定機能（Session 120で実装）
- スカイプロット機能

**目的**: ヌケモレがないかの確認

---

*計画作成: 2026-03-12 Session 120終了時*
