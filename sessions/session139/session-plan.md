# Session 139 計画

**目的**: 実機でGGA正式実装・IOエラーの動作確認

**前提**: Session 138でGGA正式実装完了（F9P位置を使用）

---

## 確認項目

| # | 項目 | 確認方法 |
|---|------|----------|
| 1 | GGA正式実装の動作 | NTRIP接続してログ確認（位置が反映されているか） |
| 2 | IOエラーの発生有無 | 屋外検査 + NTRIP接続で操作し、エラーログを確認 |
| 3 | RTK FIXの動作 | VRS補正データが正しく反映されるか |

---

## IOエラーが発生する場合の調査

1. エラーログからどの操作で発生したか特定
2. シリアルポート競合の詳細分析
3. 対策検討（リトライ、排他制御強化など）

---

## 残タスク（Session 138から引き継ぎ）

| # | タスク | 状態 |
|---|--------|------|
| 1 | IOエラー調査 | 今回 |
| 2 | NTRIPライフサイクル整理 | IOエラー調査後 |
| 3 | u-center照合 | 最後 |

---

## 参照

- [Session 138 summary](../session138/session-summary.md)
- [20-ntrip-rtk-implementation.md](../../docs/missions/m1-sensor-evaluation/gnss/20-ntrip-rtk-implementation.md)
- [21-ntrip-protocol-spec.md](../../docs/missions/m1-sensor-evaluation/gnss/21-ntrip-protocol-spec.md)

---

*計画作成: 2026-03-12 Session 138終了時*
