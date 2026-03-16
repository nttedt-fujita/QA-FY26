# Session 202 サマリー

**目的**: 装置画面シリアル表示修正 + 古い機テスト

**実施内容**:

1. DeviceCard.tsxのシリアル表示修正
   - `f9p_serial`を優先表示、なければ`serial_number`にフォールバック

2. `query_f9p_serial()`のバグ修正
   - 定期出力無効化の**前**にSEC-UNIQID pollを送っていたためタイムアウトしていた
   - 定期出力・NMEA無効化の**後**に移動して修正

3. 古い機2台のテスト
   - 古い機1: D30I4WYP / 4D439AC85F → ✅ 正常
   - 古い機2: D30EDS81 / 4D436ACF5B → ✅ 正常

**変更ファイル**:

| ファイル | 内容 |
|----------|------|
| [DeviceCard.tsx](../../prototype/m1-gnss/frontend/src/components/DeviceCard.tsx) | f9p_serial優先表示 |
| [device_api.rs](../../prototype/m1-gnss/backend/src/web/device_api.rs) | query_f9p_serial()の呼び出し順序修正 |

**テスト結果**: 古い機2台で正常動作確認

---

## 全テスト済み機体一覧

| 機体 | FTDIシリアル | F9Pシリアル | セッション |
|------|-------------|------------|-----------|
| 検証用 | D30I4QFD | 9543F2097F | 201 |
| 試作機1 | D30I4QFD | A5400AEB1F | 201 |
| 試作機2 | D30I4QFD | A44052ED9D | 201 |
| 古い機1 | D30I4WYP | 4D439AC85F | 202 |
| 古い機2 | D30EDS81 | 4D436ACF5B | 202 |
