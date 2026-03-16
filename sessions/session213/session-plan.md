# Session 213 計画

**目的**: 設定リセットAPIの実機テスト + PX4 ubx.cpp調査

**前提**: Session 212でCFG-CFGコマンドとAPIを実装済み

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|--------------------|
| 1 | 実機テスト（BBRクリア効果確認） | - |
| 2 | PX4 ubx.cpp/ubx.h調査 | Web検索 |

---

## 詳細

### 1. 実機テスト

1. 古い機を接続
2. message-scan APIで定期出力を確認
3. reset-config APIを実行
4. message-scan APIで定期出力が停止したことを確認
5. 再起動後も定期出力が停止していることを確認

### 2. PX4 ubx.cpp調査

PX4のソースコード（GitHub）でubx.cpp/ubx.hを確認し、どのメッセージを定期出力設定しているか調査する。

**調査目的**:
- 「なぜこの定期出力があるのか」の根本原因を理解
- 今後の無効化リストに追加すべきメッセージの予測

---

## 参照

- [Session 212 summary](../session212/session-summary.md)
- [38-ublox-config-management.md](../../docs/missions/m1-sensor-evaluation/gnss/38-ublox-config-management.md)
