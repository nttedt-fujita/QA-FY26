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

## Session 213 (2026-03-16)

**概要**: reset-config API実機テスト → 効果なし → 仕様確認不足が判明

**実施内容**:
1. 実機テスト実施
   - 古い機を接続（/dev/ttyUSB1）
   - message-scanで定期出力確認: NAV-TIMEBDS(4), NAV-TIMEGAL(1)
   - reset-config API実行 → ACK受信（成功応答）
   - 再度message-scan → **定期出力が消えていない**
   - USB抜き差し（電源リセット）後も同じ定期出力

**問題**:
- CFG-CFG (deviceMask=BBR) でBBRクリアしたが効果なし
- 設定がBBRではなくFlashに保存されている可能性

**反省**:
- 「Flashも含めて試す」と提案したが、**仕様書確認が先**
- 推測で試そうとした → ルール違反（13-spec-first-implementation.md）

**次セッション**: [session214/session-plan.md](../session214/session-plan.md)

---

## Session 214 (2026-03-17)

**概要**: CFG-CFG仕様の再確認 + 実装修正

**実施内容**:
1. CFG-CFG仕様の再確認（p.64）
   - deviceMaskはclearMask/saveMaskにのみ適用
   - loadMaskには適用されない（下位レイヤーから再構築）
2. CFG-VALGET仕様の確認（p.95-96を新規抽出）
   - レイヤー別読み出し方法を確認
3. 問題の原因特定
   - 旧実装: deviceMask=0x01 (BBRのみ)
   - 設定がFlashに保存されていたため効果なし
4. 実装修正
   - deviceMaskを0x03 (BBR+Flash) に変更
   - テスト修正、全4件パス

**変更ファイル**:
| ファイル | 内容 |
|----------|------|
| [cfg_cfg.rs](../../prototype/m1-gnss/backend/src/ubx/cfg_cfg.rs) | deviceMaskをBBR+Flashに変更 |

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [cfg-valget-spec.md](../session214/cfg-valget-spec.md) | CFG-VALGET仕様（p.96-98抽出） |
| [cfg-valget-request-spec.md](../session214/cfg-valget-request-spec.md) | CFG-VALGETリクエスト仕様（p.95抽出） |

**残タスク**: 実機テスト（古い機でreset-config APIが効くか確認）

**次セッション**: [session215/session-plan.md](../session215/session-plan.md)

---

## Session 215 (2026-03-17)

**概要**: reset-config API実機テスト + NMEA OFF対応 + loadMask問題発見

**実施内容**:
1. reset-config API実機テスト
   - deviceMask=BBR+Flashで実機テスト → タイムアウトエラー
   - NMEAデータが大量に来てACKが受信できなかった
2. NMEA OFF対応（Session 102-103の知見活用）
   - `send_disable_nmea_output`と`wait_for_ack`を使用
   - reset-config API内でNMEA OFFを先に送信
3. loadMask問題の発見
   - loadMask=ALLでボーレートがROMデフォルト（9600bps）に戻る
   - 受信データが文字化け → loadMask=NONEに変更

**変更ファイル**:
| ファイル | 内容 |
|----------|------|
| [cfg_cfg.rs](../../prototype/m1-gnss/backend/src/ubx/cfg_cfg.rs) | loadMask=NONEに変更 |
| [reset_config_api.rs](../../prototype/m1-gnss/backend/src/web/reset_config_api.rs) | NMEA OFF送信追加、wait_for_ack使用 |
| [manager.rs](../../prototype/m1-gnss/backend/src/device/manager.rs) | デバッグログ追加 |

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [ADR-015-cfg-cfg-loadmask.md](../../docs/adr/m1/ADR-015-cfg-cfg-loadmask.md) | loadMask使用禁止の理由を記録 |

**残タスク**: loadMask=NONE版の実機テスト

**次セッション**: [session216/session-plan.md](../session216/session-plan.md)

---

