# Session 216 計画

**目的**: reset-config API実機テスト完了 + Session 215計画の続き

**前提**: Session 215でloadMask=NONEに修正済み（未テスト）

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|------------------|
| 1 | reset-config API実機テスト | - |
| 2 | テスト成功後、デバッグログ削除 | manager.rs, reset_config_api.rs |
| 3 | 全体の進捗整理 | CLAUDE.md, docs/index.md |
| 4 | M1の作業範囲確認 | docs/missions/m1-sensor-evaluation/ |
| 5 | M3/M4のAI見積作成時期を決定 | - |

---

## 詳細

### 1. reset-config API実機テスト

- loadMask=NONE版で実機テスト
- 古い機を接続し、message-scanで確認
- reset-config実行 → ACK受信確認
- USB抜き差し → 定期出力が消えていることを確認

### 2. デバッグログ削除

テスト成功後、以下のデバッグログを削除:
- manager.rs: B5 62検索位置ログ
- reset_config_api.rs: CFG-CFG送信データログ

### 3-5. Session 215計画の続き

Session 215で予定していた作業を実施。

---

## 参照

- [Session 215 summary](../session215/session-summary.md)
- [Session 211-214 履歴](../session-history/session-211-220.md)
