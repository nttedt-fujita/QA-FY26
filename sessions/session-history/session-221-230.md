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

## Session 227 (2026-03-17)

**概要**: BBR優先順位問題テスト + 仕様書確認

**実施内容**:
1. Step 2-3 テスト実施
   - Step 2: BBR=0（変化なし）、Flash=1、message-scan=0件
   - Step 3: 同上
   - **結論**: 既存のBBR=0があるためFlashが使われない
2. 仕様書確認（ユーザー指摘を受けて）
   - CFG-VALDEL仕様（p.93-97）を抽出
   - Configuration layers仕様（p.223-225）を抽出
   - **BBRを削除すればFlashが参照される**ことを仕様書で確認

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [session227/cfg-valdel-spec.md](../session227/cfg-valdel-spec.md) | CFG-VALDEL仕様抽出 |
| [session227/config-layers-spec.md](../session227/config-layers-spec.md) | Configuration layers仕様抽出 |
| [session227/session-summary.md](../session227/session-summary.md) | セッションサマリー |

**変更ファイル**:
| ファイル | 内容 |
|----------|------|
| [38-ublox-config-management.md](../../docs/missions/m1-sensor-evaluation/gnss/38-ublox-config-management.md) | CFG-VALDEL詳細仕様、レイヤー優先順位詳細を追記 |
| [README.md](../../docs/missions/m1-sensor-evaluation/gnss/README.md) | PDF仕様抽出状態テーブル更新 |

**残タスク**: CFG-VALDEL実装、BBR削除テスト

**次セッション**: [session228/session-plan.md](../session228/session-plan.md)

---

## Session 228 (2026-03-17)

**概要**: CFG-VALDEL実装 + BBR削除テスト成功

**実施内容**:
1. CFG-VALDELメッセージ実装
   - cfg_valdel.rs（9テスト全パス）
   - DeleteLayer enum: Bbr, Flash, BbrAndFlash
2. CFG-VALDEL API実装
   - DELETE /api/devices/{path}/cfg-valdel
3. Makeコマンド追加
   - cfg-valdel-bbr, cfg-valdel-flash
4. 実機テスト
   - BBR=0, Flash=1 を確認
   - cfg-valdel-bbr → ACK
   - cfg-valget-bbr → エラー（BBR削除成功）

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [cfg_valdel.rs](../../prototype/m1-gnss/backend/src/ubx/cfg_valdel.rs) | CFG-VALDELメッセージ生成 |
| [cfg_valdel_api.rs](../../prototype/m1-gnss/backend/src/web/cfg_valdel_api.rs) | CFG-VALDEL REST API |

**残タスク**: USB抜き差し後のFlash値確認

**次セッション**: [session229/session-plan.md](../session229/session-plan.md)

---

## Session 229 (2026-03-17)

**概要**: USB抜き差し後のFlash値確認成功 + 調査サマリー作成

**実施内容**:
1. USB抜き差し後のテスト
   - connect-raw → message-scan → **NAV-PVT 13件検出**
   - BBR削除後、Flashの値が正しく使われることを確認
2. ドキュメント更新
   - 38-ublox-config-management.md にセクション6追加（BBR優先順位問題）
3. 調査サマリー作成
   - Session 199-229の一連の調査を整理
4. チートシート作成
   - レイヤー設定確認コマンド一覧

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [investigation-summary.md](../session229/investigation-summary.md) | Session 199-229 調査サマリー |
| [layer-config-cheatsheet.md](../session229/layer-config-cheatsheet.md) | レイヤー設定確認コマンド一覧 |

**変更ファイル**:
| ファイル | 内容 |
|----------|------|
| [38-ublox-config-management.md](../../docs/missions/m1-sensor-evaluation/gnss/38-ublox-config-management.md) | セクション6追加 |

**結論**: BBR優先順位問題の解決手順が確立

**次セッション**: [session230/session-plan.md](../session230/session-plan.md)

---

## Session 230 (2026-03-17)

**概要**: Living Documentation方針確定 + 断捨離計画策定

**実施内容**:
1. リポジトリ全体のドキュメント構造調査
   - sessions/に657個のMarkdownファイル
   - 重複ファイル10件以上を発見
2. Living Documentationの概念整理
   - 出典: Cyrille Martraire「Living Documentation」(2019)
   - 4原則: Reliable, Low Effort, Collaborative, Insightful
3. 組み込み開発への適用方針を確定
   - 仕様書（PDF）が1次情報
   - コードが実装の真実（出典をコードに書く）
   - 抽出ドキュメントはClaude用に必要（PDFを直接読めないため）
4. 断捨離計画策定
   - 重複ファイル削除（10件）
   - 📝 sessionマーク移動（CFG-CFG, CFG-VALGET）
   - ルールファイル改善

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [living-documentation-concept.md](../session230/living-documentation-concept.md) | 方針と断捨離計画（次セッション用背景情報） |

**次セッション**: [session231/session-plan.md](../session231/session-plan.md)

---
