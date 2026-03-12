# Session 147 計画

**目的**: NMEA OFF修正後の動作確認

**前提**: Session 146でデバイス接続時にNMEA出力もOFFにする修正を実施

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | バックエンド再起動後の動作確認 | logs/gnss-backend.log.* |
| 2 | 問題があれば修正 | 該当ファイル |
| 3 | ADR-012を更新 | docs/adr/m1/ADR-012-periodic-output-and-unified-api.md |

---

## 詳細

### 1. 動作確認

- 屋内検査 → 屋外検査の流れで実行
- 統合APIが安定して動作するか確認（500エラーが発生しないか）
- ログファイルで確認

### 2. ADR-012を更新

Session 145-146で決定した内容を追記:
- 定期出力を無効化した理由（Session 145）
- NMEA出力を無効化した理由（Session 146）

---

## 参照

- [Session 146 サマリー](../session146/session-summary.md)
- [ADR-012](../../docs/adr/m1/ADR-012-periodic-output-and-unified-api.md)
