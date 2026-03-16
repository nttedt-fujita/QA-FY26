# Session 201 計画

**目的**: 追加MSGOUTキーの実装と動作確認

**前提**: Session 200で仕様書から正確なKey IDを確認済み

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | cfg_valset.rsに追加キーを実装 | `prototype/m1-gnss/backend/src/ubx/cfg_valset.rs`, `docs/.../gnss/32-cfg-msgout-periodic-output.md` |
| 2 | disable_periodic_outputに追加キーを含める | 同上 |
| 3 | USB1で動作確認 | - |
| 4 | 装置画面のシリアル表示修正 | `prototype/m1-gnss/frontend/src/components/DeviceCard.tsx` |

---

## 詳細

### 1-2. 追加するMSGOUTキー

Session 200で確認したKey ID:

| メッセージ | Class/ID | USB Key ID | UART1 Key ID |
|-----------|----------|------------|--------------|
| NAV-CLOCK | 0x01 0x22 | 0x20910068 | 0x20910066 |
| NAV-POSLLH | 0x01 0x02 | 0x2091002c | 0x2091002a |
| NAV-HPPOSECEF | 0x01 0x13 | 0x20910031 | 0x2091002f |
| NAV-TIMEGPS | 0x01 0x20 | 0x2091004a | 0x20910048 |
| NAV-TIMEQZSS | 0x01 0x27 | 0x20910389 | 0x20910387 |
| NAV-SBAS | 0x01 0x32 | 0x2091006d | 0x2091006b |

### 4. 装置画面のシリアル表示問題

**現状**:
- DeviceCard.tsxで`serial_number`（FTDIのシリアル）を表示
- F9Pのシリアル（`f9p_serial`）と異なる値が出て混乱の原因

**対策**:
- `f9p_serial`を優先表示、なければ`serial_number`をフォールバック

---

## 参照

- [Session 200 summary](../session200/session-summary.md)
- [32-cfg-msgout-periodic-output.md](../../docs/missions/m1-sensor-evaluation/gnss/32-cfg-msgout-periodic-output.md)
