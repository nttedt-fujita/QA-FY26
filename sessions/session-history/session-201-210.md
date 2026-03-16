# セッション履歴: Session 201〜210

## Session 201 (2026-03-16)

**概要**: NAV-TIMEQZSS実装 + 実機テスト3台

**実施内容**:
1. NAV-TIMEQZSSキーの実装（Session 200で特定済みだが実装漏れ）
   - cfg_valset.rsにUSB/UART1用キー定義追加
   - `disable_periodic_output()`を22キー→24キーに更新
   - テストを24キー対応に更新（39テストパス）

2. 実機テスト（3台すべて正常動作）
   - 検証用: D30I4QFD / 9543F2097F
   - 試作機1: D30I4QFD / A5400AEB1F
   - 試作機2: D30I4QFD / A44052ED9D

**変更ファイル**:
| ファイル | 内容 |
|----------|------|
| [cfg_valset.rs](../../prototype/m1-gnss/backend/src/ubx/cfg_valset.rs) | NAV-TIMEQZSS追加、24キー対応 |

**次セッション**: [session202/session-plan.md](../session202/session-plan.md)

---

## Session 202 (2026-03-16)

**概要**: シリアル表示修正 + 古い機テスト + メッセージスキャンAPI実装

**実施内容**:
1. DeviceCard.tsxでf9p_serial優先表示に修正
2. query_f9p_serial()のバグ修正（呼び出し順序を定期出力無効化後に変更）
3. 古い機2台の単体テスト完了
4. 3台同時接続テスト → 古い機でパースエラー発生
5. メッセージスキャンAPI実装 `GET /api/devices/{path}/message-scan`
6. スキャン結果：古い機はRTK基準局向け設定、無効化リストに無いメッセージが出力

**変更ファイル**:
| ファイル | 内容 |
|----------|------|
| [DeviceCard.tsx](../../prototype/m1-gnss/frontend/src/components/DeviceCard.tsx) | f9p_serial優先表示 |
| [device_api.rs](../../prototype/m1-gnss/backend/src/web/device_api.rs) | query_f9p_serial()順序修正、message-scanルート追加 |
| [message_scan_api.rs](../../prototype/m1-gnss/backend/src/web/message_scan_api.rs) | 新規：メッセージスキャンAPI |

**次セッション**: [session203/session-plan.md](../session203/session-plan.md)

---
