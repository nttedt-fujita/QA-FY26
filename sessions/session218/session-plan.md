# Session 218 計画

**目的**: reset-config効果確認方法の再検討（Session 217 planの残タスク）

**前提**: Session 217でコマンド引き継ぎ問題を解決（ルール・スキル拡張完了）

---

## やること

| # | 作業 | 読むべきファイル | 参照コマンド |
|---|------|-----------------|--------------|
| 1 | 定期出力有効化の既存実装確認 | cfg_valset.rs | - |
| 2 | reset-config効果確認方法を決定 | session216/session-summary.md | make message-scan, make reset-config |
| 3 | 確認テスト実施（方法が決まれば） | api.mk | make dev-backend |

---

## 詳細

### 1. 定期出力有効化の既存実装確認

`set_periodic_output`関数が定義されているが使われていない理由を確認。
検査フロー（inspection）で定期出力がどう扱われているか確認。

### 2. reset-config効果確認方法

以下の選択肢を検討:
- A: テスト用API追加（set-periodic-output）
- B: 既存の検査フローを利用
- C: 別の方法

---

## 参照

- [Session 217 summary](../session217/session-summary.md)
- [Session 216 summary](../session216/session-summary.md)
