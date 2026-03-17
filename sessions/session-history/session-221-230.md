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

## Session 224 (2026-03-17)

**概要**: Flash永続化問題の真因特定（BBR優先順位問題）

**実施内容**:
1. CFG-VALGET実機テスト
   - Flashに値あり（value=1）
   - USB抜き差し後もFlash維持 → **Flash搭載確認**
   - しかしmessage-scan 0件
2. 仮説検証ツール作成
   - cfg-valget-ram, cfg-valget-bbr, connect-raw
3. 真因特定
   - BBRに0が残っていて、Flashの1より優先されていた
   - u-blox優先順位: RAM > BBR > Flash > Default

**発見事項**:
- **BBRに値があるとFlashが無視される**
- 接続時のdisable_periodic_outputがBBRに0を書き込んでいた

**変更ファイル**:
| ファイル | 内容 |
|----------|------|
| device_api.rs | skip_disableパラメータ追加 |
| api.mk | cfg-valget-ram, cfg-valget-bbr, connect-raw追加 |

**残タスク**: set-periodic-outputをRamBbrFlashで書くように変更

**次セッション**: [session225/session-plan.md](../session225/session-plan.md)

---

## Session 225 (2026-03-17)

**概要**: BBR優先順位問題の解決 + 仕様理解の深掘り

**実施内容**:
1. いじったデバイス vs いじっていないデバイスの比較
   - いじったデバイス: BBR=0（値あり）
   - いじっていないデバイス: BBR=エラー（値なし）
2. 重要な発見
   - BBRの「値の存在」がポイント
   - 値がなければスキップされ、次のレイヤーが参照される
   - `disable_periodic_output`がBBRに0を書くことで問題発生
3. 定期出力の謎を解決
   - cfg-valget NAV_PVT_USB=0なのにNAV-PVTが出力されていた
   - 原因: デバイスは内部的にUART1として接続されている
   - NAV_PVT_UART1=1が有効だった

**発見事項（重要）**:
- **BBRレイヤーの「値の存在」**: 未設定=エラー、一度でも書き込み=値あり（0でも）
- **USB vs UART1の接続形態**: 組み込みボードはPCにUSB経由で接続するが、内部的にはUART1として動作

**変更ファイル**:
| ファイル | 内容 |
|----------|------|
| [api.mk](../../prototype/m1-gnss/makefiles/api.mk) | cfg-valget-default追加 |
| [cfg_valget_api.rs](../../prototype/m1-gnss/backend/src/web/cfg_valget_api.rs) | NAV_PVT_UART1キー追加 |

**残タスク**:
- BBR優先順位問題の解決（disable_periodic_outputをRAMのみに変更 or CFG-VALDELでBBR削除）
- 仕様書との整合性確認（config-layers-spec.md、32-cfg-msgout-periodic-output.mdへの追記）

**次セッション**: [session226/session-plan.md](../session226/session-plan.md)

---

## Session 226 (2026-03-17)

**概要**: BBR優先順位問題の対策実装 + テスト手順書作成

**実施内容**:
1. 対策実装
   - `disable_periodic_output`を`Layer::RamAndBbr`から`Layer::Ram`に変更
   - BBRに0を書き込まないようにする
2. 現状確認（Step 1のみ）
   - BBR: 0, Flash: 1 を確認
3. テスト手順書作成
   - USB抜き差し→BE再起動→状態確認→message-scanをセットで行う方針

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [test-procedure.md](../session226/test-procedure.md) | テスト手順書 |

**変更ファイル**:
| ファイル | 内容 |
|----------|------|
| [device_api.rs](../../prototype/m1-gnss/backend/src/web/device_api.rs) | disable_periodic_outputをRAMのみに変更 |

**残タスク**: Step 2-3テスト実施、ドキュメント更新

**次セッション**: [session227/session-plan.md](../session227/session-plan.md)

---
