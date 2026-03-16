# Session 203 計画

**目的**: 無効化リスト追加 + 複数台同時接続の完全動作確認

**前提**: Session 202でメッセージスキャンAPI実装済み、古い機から流れるメッセージを特定済み

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | 無効化リストに追加 | `backend/src/ubx/cfg_valset.rs` |
| 2 | 3台同時接続で動作確認 | - |
| 3 | FEのデバイス選択UIで切り替え確認 | `frontend/src/app/inspections/outdoor/page.tsx` |

---

## 詳細

### 1. 無効化リスト追加

Session 202のスキャン結果で特定されたメッセージ：
- NAV-POSECEF (0x01, 0x01) - 既に追加済みのはず？要確認
- NAV-GEOFENCE (0x01, 0x39)
- NAV-RELPOSNED (0x01, 0x3C)
- NAV-EOE (0x01, 0x61)
- NAV-TIMEUTC (0x01, 0x21)
- NAV-VELECEF (0x01, 0x11)
- UNKNOWN (0x01, 0x42) - NAV-ORB？

これらのCFG-MSGOUTキーを仕様書で確認し、disable_periodic_output()に追加する。

### 2. 複数台同時接続確認

無効化リスト更新後：
- 3台同時接続
- 全装置からデータ取得できることを確認
- パースエラーが解消されていることを確認

### 3. FEデバイス選択UI確認

屋外検査画面でデバイス選択ドロップダウンを使い、装置を切り替えてデータ表示できることを確認。

---

## 参照

- [Session 202 summary](../session202/session-summary.md)
- [cfg_valset.rs](../../prototype/m1-gnss/backend/src/ubx/cfg_valset.rs)
- [32-cfg-msgout-periodic-output.md](../../docs/missions/m1-sensor-evaluation/gnss/32-cfg-msgout-periodic-output.md)
