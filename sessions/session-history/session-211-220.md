# セッション履歴: Session 211〜220

## Session 211 (2026-03-16)

**概要**: 古い機の定期出力問題の根本対処を検討（仕様理解）

**実施内容**:
1. CFG-CFG仕様の確認
   - u-blox F9 HPG 1.32 Interface Description p.64から抽出
   - clearMask/saveMask/loadMask の動作を確認
2. u-center メニューとUBXコマンドの対応調査
   - u-center User Guide (UBX-13005250) R30を確認
   - Receiver → Action → Save/Load/Revert Config = CFG-CFG
   - Tools → Receiver Configuration... = CFG-VALGET（設定バックアップ）
3. 設定レイヤーの仕組みを理解
   - ROM/Flash/BBR/RAMの4レイヤー
   - 起動時の優先順位: Flash > BBR > ROM → RAM
4. ドキュメント作成
   - [38-ublox-config-management.md](../../docs/missions/m1-sensor-evaluation/gnss/38-ublox-config-management.md) を作成

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [38-ublox-config-management.md](../../docs/missions/m1-sensor-evaluation/gnss/38-ublox-config-management.md) | u-blox設定管理の仕組み解説（新規） |
| [gnss/README.md](../../docs/missions/m1-sensor-evaluation/gnss/README.md) | ドキュメント一覧に38を追加 |
| [cfg-cfg-spec.md](../session211/cfg-cfg-spec.md) | CFG-CFG仕様PDF抽出（p.63-68） |
| [ucenter-*.md](../session211/) | u-center User Guide各セクション抽出 |

**結論**: CFG-CFG（clearMask + loadMask）でBBRクリアが根本対処として有効

**次セッション**: [session212/session-plan.md](../session212/session-plan.md)

---

## Session 212 (2026-03-16)

**概要**: CFG-CFGによる設定リセットAPIの実装

**実施内容**:
1. CFG-CFGコマンド実装（ubx/cfg_cfg.rs）
   - CFG-CFG (0x06 0x09) メッセージ生成機能
   - `reset_config_to_default()`: BBRクリア + ROMデフォルト復元
   - `save_config_to_bbr()`: RAM → BBR保存
   - テスト4件追加（全パス）
2. 設定リセットAPI実装（web/reset_config_api.rs）
   - `POST /api/devices/{path}/reset-config`
   - CFG-CFG送信 → ACK/NAK確認

**変更ファイル**:
| ファイル | 内容 |
|----------|------|
| [cfg_cfg.rs](../../prototype/m1-gnss/backend/src/ubx/cfg_cfg.rs) | CFG-CFGメッセージ生成（新規） |
| [reset_config_api.rs](../../prototype/m1-gnss/backend/src/web/reset_config_api.rs) | 設定リセットAPI（新規） |

**メモ**: PX4のubx.cpp/ubx.hを調査することで、FCがどのメッセージを定期出力設定しているか分かる可能性がある。

**次セッション**: [session213/session-plan.md](../session213/session-plan.md)

---

