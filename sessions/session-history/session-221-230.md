# セッション履歴: Session 221〜230

## Session 221 (2026-03-17)

**概要**: reset-configテスト → Flash保存されない問題発覚 + MON-VER API作成

**実施内容**:
1. Phase 1テスト開始
   - set-periodic-output → ACK成功
   - message-scan → NAV-PVT等検出
   - USB抜き差し → **0件**（消失）
2. 問題調査
   - Flashレイヤー仕様確認（IF p.224）
   - 「外部Flashメモリがある場合のみ」使用可能
3. MON-VER API作成
   - モジュール情報取得（ZED-F9P, HPG 1.32確認）

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [mon_ver_api.rs](../../prototype/m1-gnss/backend/src/web/mon_ver_api.rs) | MON-VER API（新規） |

**残タスク**: 評価ボードのFlash有無調査、対応策検討

**次セッション**: [session222/session-plan.md](../session222/session-plan.md)

---

## Session 222 (2026-03-17)

**概要**: Flash/BBR仕様調査 + layerパラメータ追加

**実施内容**:
1. 仕様調査（公式ドキュメントから事実確認）
   - Interface Description p.224: BBR/Flash仕様
   - Integration Manual p.88, p.104: V_BCKP要件
2. Holybro H-RTK F9P仕様確認
   - 公式サイトにV_BCKP/Flash情報なし
3. 実機テスト
   - set-periodic-output-flash（layers=0x04）→ ACK
   - USB抜き差し後 message-scan → 0件

**結論（仮説）**:
- V_BCKPバッテリー: なし（BBR消失から推測）
- 外部Flash: 不明（ACKは返るが維持されない）

**変更ファイル**:
| ファイル | 内容 |
|----------|------|
| set_periodic_output_api.rs | layerクエリパラメータ追加 |
| api.mk | set-periodic-output-flashコマンド追加 |

**次セッション**: [session223/session-plan.md](../session223/session-plan.md)

---

## Session 223 (2026-03-17)

**概要**: CFG-VALGET API作成

**実施内容**:
1. CFG-VALGET仕様確認（Session 214で抽出済み、IF p.95-96）
2. CFG-VALGET API作成
   - `cfg_valget.rs`: メッセージ生成・パース（14テスト全パス）
   - `cfg_valget_api.rs`: REST API
   - `manager.rs`: wait_for_ubx_messageメソッド追加
3. Makeコマンド追加（cfg-valget-flash）

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [cfg_valget.rs](../../prototype/m1-gnss/backend/src/ubx/cfg_valget.rs) | CFG-VALGETメッセージ生成・パース |
| [cfg_valget_api.rs](../../prototype/m1-gnss/backend/src/web/cfg_valget_api.rs) | CFG-VALGET API |

**変更ファイル**:
| ファイル | 内容 |
|----------|------|
| device/manager.rs | wait_for_ubx_messageメソッド追加 |
| api.mk | cfg-valget-flashコマンド追加 |

**残タスク**: 実機テスト（Flashレイヤーから値が読めるか確認）

**次セッション**: [session224/session-plan.md](../session224/session-plan.md)

---
