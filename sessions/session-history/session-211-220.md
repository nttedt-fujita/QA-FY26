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

## Session 216 (2026-03-17)

**概要**: reset-config API実機テスト → 定期出力は既に消えていた + 作業品質の問題指摘

**実施内容**:
1. message-scan確認
   - 古い機を接続してmessage-scan実行
   - USB抜き差し前後で各3回、計6回実行 → 全て0件
   - 定期出力は既に消えている
2. reset-config効果確認方法の検討
   - 「定期出力を再設定 → reset-config → 消えることを確認」を検討
   - `set_periodic_output`関数は存在するがAPIがない
   - 新API作成前に過去セッション履歴の確認が必要と判断

**問題点・反省**:
- APIメソッド（GET/POST）を確認せずに試行錯誤
- 「これは古い機ですか？」と明らかな文脈を無視した質問
- session-summary.mdを読まずに作業開始
- 過去の検討を確認せずに「新API作りましょうか？」と提案

**確認結果**:
| 条件 | 結果 |
|------|------|
| USB抜き差し前 (3回) | 全て0件 |
| USB抜き差し後 (3回) | 全て0件 |

**次セッション**: [session217/session-plan.md](../session217/session-plan.md)

---

## Session 217 (2026-03-17)

**概要**: コマンド引き継ぎ問題の解決（ルール・スキル拡張）

**実施内容**:
1. 問題分析: 複数セッションでMakefileコマンド・curlが引き継がれない
2. 新ルール `16-command-reference.md` 作成（コマンド推測禁止）
3. session-managementスキル拡張（参照コマンド・使用コマンドセクション追加）
4. M1-GNSS CLAUDE.mdにコマンドリファレンス追加
5. api.mk拡張（message-scan, reset-config, DEVICE変数）

**変更ファイル**:
| ファイル | 内容 |
|----------|------|
| [16-command-reference.md](../../.claude/rules/16-command-reference.md) | 新規: コマンド推測禁止ルール |
| [session-management/SKILL.md](../../.claude/skills/session-management/SKILL.md) | 拡張: テンプレート |
| [prototype/m1-gnss/CLAUDE.md](../../prototype/m1-gnss/CLAUDE.md) | 追加: コマンドリファレンス |
| [api.mk](../../prototype/m1-gnss/makefiles/api.mk) | 追加: DEVICE変数、新コマンド |

**結論**: 次セッションから「参照コマンド」カラム・「使用したコマンド」セクションを使用

**次セッション**: [session218/session-plan.md](../session218/session-plan.md)

---

## Session 218 (2026-03-17)

**概要**: set-periodic-output API作成 + 実機テスト（途中まで）

**実施内容**:
1. set-periodic-output API作成
   - reset-config効果確認のためのテスト用API
   - `POST /api/devices/{path}/set-periodic-output`
2. 実機テスト → USB用キーのみで検出されない問題発覚
   - 原因: `set_periodic_output`がUSB用キーのみ設定（UART1キー不足）
   - 修正: USB + UART1両方のキーを設定
3. 再テスト成功
   - NAV-PVT(3), MON-RF(1), NAV-STATUS(1) 検出

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [set_periodic_output_api.rs](../../prototype/m1-gnss/backend/src/web/set_periodic_output_api.rs) | 定期出力有効化API（新規） |

**変更ファイル**:
| ファイル | 内容 |
|----------|------|
| cfg_valset.rs | `set_periodic_output`にUART1キー追加 |
| api.mk | `make set-periodic-output` コマンド追加 |
| device_api.rs | ルーティング追加 |
| mod.rs | モジュール追加 |

**残タスク**: Phase 1/2完全テスト（USB抜き差し3回含む）

**次セッション**: [session219/session-plan.md](../session219/session-plan.md)

---

## Session 219 (2026-03-17)

**概要**: reset-configテスト → USB抜き差しでBBR消失問題発覚 + 作業品質問題

**実施内容**:
1. message-scanスキャン時間延長（3秒→12秒）
2. Phase 1テスト開始
   - set-periodic-output → 成功
   - message-scan → NAV-PVT等検出
   - USB抜き差し1回目 → **0件**（定期出力消失）
3. 問題調査
   - CFG-VALSET仕様確認（p.96-98、Session 214で抽出済み）
   - 複数レイヤー同時指定可能（ビットフィールド）

**問題点**:
- 推測と事実を分けなかった（「BBRはバッテリーがないと消える」は仮説）
- 仕様書への誘導が不足（CFG-VALSET仕様の場所を探すのに時間がかかった）

**変更ファイル**:
| ファイル | 内容 |
|----------|------|
| message_scan_api.rs | スキャン時間3秒→12秒 |
| cfg_valset.rs | `Layer::RamBbrFlash = 0x07` 追加（未テスト） |

**次セッションでやること**:
- ルール追加: 推測と事実の分離
- 仕様書索引の整備
- BBR/Flash仕様確認（p.223抽出）
- reset-configテスト再実行

**次セッション**: [session220/session-plan.md](../session220/session-plan.md)

---

## Session 220 (2026-03-17)

**概要**: 仕様書索引整備 + ルール追加 + set-periodic-output Flash対応

**実施内容**:
1. ルール追加: 推測と事実の分離（17-fact-vs-hypothesis.md）
2. PDF抽出ルール拡張: 抽出後の整備セクション追加（15-pdf-handling.md）
3. 仕様書索引の整備
   - gnss/README.mdに「PDF仕様抽出状態」セクション追加
   - 複数PDF対応（IF, IM, NTRIP, UC）
   - CLAUDE.mdを簡素化（gnss/README.mdを参照）
4. BBR/Flash仕様確認（p.223-225抽出）
   - **事実確認**: BBRはバッテリーバックアップがないと電源断で消える
   - **事実確認**: Flashは永続的
5. set-periodic-output APIをFlash対応に修正
   - Layer::RamAndBbr → Layer::RamBbrFlash

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [17-fact-vs-hypothesis.md](../../.claude/rules/17-fact-vs-hypothesis.md) | 推測と事実の分離ルール |
| [config-layers-spec.md](../session220/config-layers-spec.md) | PDF抽出: Configuration layers |

**変更ファイル**:
| ファイル | 内容 |
|----------|------|
| 15-pdf-handling.md | 抽出後の整備セクション追加 |
| gnss/README.md | PDF仕様抽出状態セクション追加 |
| prototype/m1-gnss/CLAUDE.md | 簡素化 |
| set_periodic_output_api.rs | Layer::RamBbrFlashに変更 |
| api.mk | コメント更新 |

**残タスク**: reset-configテスト再実行（Phase 1/2）

**次セッション**: [session221/session-plan.md](../session221/session-plan.md)

---

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

**変更ファイル**:
| ファイル | 内容 |
|----------|------|
| web/mod.rs, device_api.rs | MON-VER APIルーティング追加 |
| api.mk | `make mon-ver` コマンド追加 |

**問題点・反省**:
- Session 220で整備した仕様書索引（gnss/README.md）を確認せずにWeb検索した
- リポジトリ内のPDFを使うべきところでWeb調査してしまった

**残タスク**: 評価ボードのFlash有無調査、対応策検討

**次セッション**: [session222/session-plan.md](../session222/session-plan.md)

---

