# Session 219 計画

**目的**: reset-config効果確認テスト（USB抜き差し含む完全版）

**前提**: Session 218でset-periodic-output API作成完了、基本動作確認済み

---

## やること

| # | 作業 | 読むべきファイル | 参照コマンド |
|---|------|-----------------|--------------|
| 1 | Phase 1完了: USB抜き差し3回で定期出力が維持されることを確認 | - | make connect, make message-scan |
| 2 | Phase 2: reset-config → 定期出力が消えることを確認 | - | make reset-config, make message-scan |
| 3 | Phase 2完了: USB抜き差し3回で消えた状態が維持されることを確認 | - | make message-scan |
| 4 | テスト結果を記録 | session218/session-summary.md | - |

---

## テスト手順

### Phase 1: 定期出力設定 → 出ることを確認（継続）

| # | 操作 | 期待結果 |
|---|------|----------|
| 1 | `make connect` | 接続成功 |
| 2 | `make set-periodic-output` | ACK受信 |
| 3 | `make message-scan` | 定期出力あり |
| 4 | USB抜き差し1回目 | - |
| 5 | `make connect` + `make message-scan` | 定期出力あり |
| 6 | USB抜き差し2回目 | - |
| 7 | `make connect` + `make message-scan` | 定期出力あり |
| 8 | USB抜き差し3回目 | - |
| 9 | `make connect` + `make message-scan` | 定期出力あり |

### Phase 2: reset-config → 消えることを確認

| # | 操作 | 期待結果 |
|---|------|----------|
| 10 | `make reset-config` | ACK受信 |
| 11 | `make message-scan` | 定期出力なし（0件） |
| 12 | USB抜き差し1回目 | - |
| 13 | `make connect` + `make message-scan` | 定期出力なし |
| 14 | USB抜き差し2回目 | - |
| 15 | `make connect` + `make message-scan` | 定期出力なし |
| 16 | USB抜き差し3回目 | - |
| 17 | `make connect` + `make message-scan` | 定期出力なし |

---

## 参照

- [Session 218 summary](../session218/session-summary.md)
- [32-cfg-msgout-periodic-output.md](../../docs/missions/m1-sensor-evaluation/gnss/32-cfg-msgout-periodic-output.md)
