# Session 172 計画

**目的**: RTK屋外テスト実施

**前提**: Session 171でドキュメント整理・ログメンテナンス完了

---

## やること

| # | 作業 | 備考 |
|---|------|------|
| 1 | RTK屋外テスト実施 | `make rtk-debug` |
| 2 | テスト結果確認 | RTK状態、エラー有無、応答時間 |
| 3 | 問題あれば対応 | - |
| 4 | FE 30秒設定の見直し検討 | フィックス時間を考慮 |

---

## RTKテスト手順

1. ターミナル1: `cd prototype/m1-gnss && make dev-backend`
2. ターミナル2: `cd prototype/m1-gnss && make rtk-debug`

---

## 確認ポイント

- RTK状態（None / Float / Fixed）
- エラー有無
- 応答時間

---

## 参照

- [Session 171 サマリー](../session171/session-summary.md)
- [api.mk](../../prototype/m1-gnss/makefiles/api.mk)
