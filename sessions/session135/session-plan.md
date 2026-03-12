# Session 135 計画

**目的**: 別ネットワークでNTRIP接続テスト または 残タスク整理

**前提**: Session 134でNTRIP接続試行 → DNS解決成功、TCP接続タイムアウト（会社NWブロックの可能性）

---

## やること（候補）

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | 別ネットワーク（モバイル回線）でNTRIP接続テスト | - |
| 2 | 残タスク優先度確認 | sessions/session131/session-summary.md |

---

## Session 134で確認できたこと

- DNS解決: `d-gnss.jp -> 153.246.40.50:2101` ✅
- TCP接続: タイムアウト（会社NWがポート2101ブロック？）

**次のアクション**:
- モバイルテザリング等でTCP接続を再テスト
- 成功すればRTCM受信→ZED-F9P転送まで確認

---

## 残タスク一覧

1. **NTRIP接続テスト**（別ネットワークで再テスト）
2. device_id紐付け実装
3. 自動保存に変更（手動保存から）
4. u-center照合

---

## 参照

- [Session 134 summary](../session134/session-summary.md)
- [Session 131 summary](../session131/session-summary.md)（残タスク整理）

---

*計画作成: 2026-03-12 Session 134終了時*
