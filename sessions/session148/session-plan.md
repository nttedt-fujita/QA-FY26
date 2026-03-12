# Session 148 計画

**目的**: 統合APIの屋外環境動作確認 + ADR更新

**前提**: Session 147でNMEA制御問題を修正済み

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|--------------------|
| 1 | 屋外環境での統合API動作確認 | prototype/m1-gnss/backend/logs/ |
| 2 | タイムアウト問題が継続する場合はタイミング調整 | gnss_state_api.rs |
| 3 | ADR-012を更新（定期出力無効化の経緯を追記） | docs/adr/m1/ADR-012-periodic-output-and-unified-api.md |

---

## 詳細

### 1. 屋外環境での統合API動作確認

Session 147で修正した内容:
- 統合APIでNMEA OFFを毎回送信
- 屋内検査終了時のNMEA ON戻し処理を削除

屋外環境（衛星信号あり）で動作確認を行い、タイムアウト頻度を確認する。

### 2. タイミング調整（必要な場合）

Session 147のログで500エラー/タイムアウトが発生していた。
屋内環境の影響（衛星信号なし）の可能性もあるが、
屋外でも頻発する場合はタイミング調整を検討:
- `poll_and_parse!` マクロのスリープ時間（現在50ms）
- `receive_ubx` のタイムアウト時間（現在500ms）

### 3. ADR-012更新

以下の経緯を追記:
- Session 145: 定期出力を無効化
- Session 146: NMEA出力を無効化
- Session 147: 屋内検査終了時のNMEA ON戻しを削除

---

## 参照

- [Session 147 summary](../session147/session-summary.md)
- [ADR-012](../../docs/adr/m1/ADR-012-periodic-output-and-unified-api.md)
- [gnss_state_api.rs](../../prototype/m1-gnss/backend/src/web/gnss_state_api.rs)
