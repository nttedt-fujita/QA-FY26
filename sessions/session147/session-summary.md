# Session 147 サマリー

**日付**: 2026-03-12
**目的**: 統合APIのNMEA問題を修正

---

## 実施内容

### 1. ログ分析で問題特定

屋内検査後に統合APIで500エラー・タイムアウトが頻発していた。

**原因**: 屋内検査終了時にNMEA ONに戻していたため、
屋外検査の統合APIポーリング時にNMEAデータがバッファに混入。

### 2. 修正1: 統合APIでNMEA OFFを毎回送信

- `send_disable_nmea_output` をpubに変更
- `gnss_state_api.rs` でメッセージ取得前にNMEA OFF送信

### 3. 修正2: 屋内検査終了時のNMEA ON戻しを削除

- `engine.rs` のNMEA ON送信処理を削除
- テスト `test_h3_nmea_off_only` に更新

---

## 決定事項

| 項目 | 決定 |
|------|------|
| NMEA制御 | 屋内検査終了時にONに戻さない |

---

## 変更ファイル

| ファイル | 内容 |
|----------|------|
| [device_api.rs](../../prototype/m1-gnss/backend/src/web/device_api.rs) | send_disable_nmea_outputをpubに変更 |
| [gnss_state_api.rs](../../prototype/m1-gnss/backend/src/web/gnss_state_api.rs) | NMEA OFF送信を追加 |
| [engine.rs](../../prototype/m1-gnss/backend/src/inspection/engine.rs) | NMEA ON戻し処理を削除 |

---

## 残った課題

- 統合APIでタイムアウトが多い（屋内環境の影響？タイミング問題？）
- ADR-012の更新が未実施

---

## 次セッション（Session 148）でやること

- 屋外環境での統合API動作確認
- タイムアウト問題が継続する場合はタイミング調整
- ADR-012を更新

---

## 参照

- [Session 147 plan](session-plan.md)
- [ADR-012](../../docs/adr/m1/ADR-012-periodic-output-and-unified-api.md)
