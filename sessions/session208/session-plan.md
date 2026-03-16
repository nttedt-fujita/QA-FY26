# Session 208 計画

**目的**: 古い機のFWバージョン取得問題の調査

**前提**: Session 207で一括検査機能を実装、古い機でFWバージョンがエラーになる問題を発見

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | バックエンドログでMON-VER応答を確認 | - |
| 2 | 古い機のMON-VER応答形式を分析 | `backend/src/ubx/mon_ver.rs` |
| 3 | パース処理の修正（必要な場合） | - |
| 4 | 再テストで動作確認 | - |

---

## 調査観点

- 古い機のMON-VER応答のペイロード長は？
- extensionが30バイト単位か？
- FWVERがextensionに含まれているか？

---

## 参照

- [Session 207 summary](../session207/session-summary.md)
- [mon_ver.rs](../../prototype/m1-gnss/backend/src/ubx/mon_ver.rs)
