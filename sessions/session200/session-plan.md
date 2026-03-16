# Session 200 計画

**目的**: 全定期出力メッセージの無効化実装

**前提**: Session 199でUSB1のパースエラー原因を特定。現在の`disable_periodic_output`は6種類のメッセージのみ無効化しており、他のメッセージ（NAV-CLOCK, NAV-POSLLH等）が流れ続けていた。

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | MSGOUTキー一覧を仕様書から確認 | `docs/missions/m1-sensor-evaluation/gnss/` 内の仕様書 |
| 2 | cfg_valset.rsに追加キーを実装 | `backend/src/ubx/cfg_valset.rs` |
| 3 | USB1で動作確認 | - |
| 4 | 装置画面のシリアル表示修正 | `frontend/src/components/DeviceCard.tsx` |

---

## 詳細

### 1. 追加が必要なMSGOUTキー

Session 199のログで確認した追加メッセージ:
- NAV-CLOCK (0x01, 0x22)
- NAV-POSLLH (0x01, 0x02)
- NAV-HPPOSECEF (0x01, 0x13)
- NAV-TIMEGPS (0x01, 0x27)
- NAV-SBAS (0x01, 0x32)

これらのUSB用/UART1用のMSGOUTキーIDを仕様書から確認する。

### 4. 装置画面のシリアル表示問題

**現状**:
- DeviceCard.tsxで`serial_number`（FTDIのシリアル）を表示
- F9Pのシリアル（`f9p_serial`）と異なる値が出て混乱の原因

**対策**:
- `f9p_serial`を優先表示、なければ`serial_number`をフォールバック

---

## 参照

- [Session 199 summary](../session199/session-summary.md)
- [ubx-nav-messages.md](../../docs/missions/m1-sensor-evaluation/gnss/ubx-nav-messages.md)
- [ubx-mon-messages.md](../../docs/missions/m1-sensor-evaluation/gnss/ubx-mon-messages.md)
