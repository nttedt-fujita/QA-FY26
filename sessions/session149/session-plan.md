# Session 149 計画

**目的**: 定期出力の無効化をUART1用キーで修正

**前提**: Session 148で原因特定済み（USB用キーでは無効化されない）

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|--------------------|
| 1 | UART1用キーを cfg_valset.rs に追加 | prototype/m1-gnss/backend/src/ubx/cfg_valset.rs |
| 2 | disable_periodic_output でUART1用キーを使用 | 同上 |
| 3 | 動作確認（統合APIのタイムアウトが減少するか） | logs/ |
| 4 | ADR-012を更新 | docs/adr/m1/ADR-012-periodic-output-and-unified-api.md |

---

## 詳細

### 1. UART1用キーの追加

u-blox仕様書から UART1用のキーIDを確認して追加:
- CFG-MSGOUT-UBX_NAV_PVT_UART1: 0x20910007
- CFG-MSGOUT-UBX_NAV_STATUS_UART1: 0x2091001b
- CFG-MSGOUT-UBX_NAV_SAT_UART1: 0x20910016
- CFG-MSGOUT-UBX_NAV_SIG_UART1: 0x20910346
- CFG-MSGOUT-UBX_MON_SPAN_UART1: 0x2091038c
- CFG-MSGOUT-UBX_MON_RF_UART1: 0x2091035a

### 2. disable_periodic_output の修正

USB用キーに加えてUART1用キーも設定するか、UART1用のみに切り替える。

### 3. 動作確認

- 屋内環境で統合APIを呼び出し
- タイムアウト頻度を確認
- 「不明データ」の混入が減少しているか確認

---

## 参照

- [Session 148 summary](../session148/session-summary.md)
- [Session 148 log-analysis-report](../session148/log-analysis-report.md)
- [cfg_valset.rs](../../prototype/m1-gnss/backend/src/ubx/cfg_valset.rs)
