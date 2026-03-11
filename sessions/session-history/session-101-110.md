# セッション履歴: Session 101〜110

## Session 101 (2026-03-11)

**概要**: UBX通信タイミング問題の対策実装（案A: B5 62同期）

**実施内容**:
1. **案A: receive_ubx で B5 62 同期を実装**
   - 受信データから `B5 62` を探す同期処理を追加
   - `extract_ubx_frame` ヘルパー関数を追加
   - NMEAが先に届いてもUBXフレームを正しく読み取れるようになった
2. **テスト追加（+6テスト）**
   - C5-1〜C5-6: UBXフレーム同期テスト
3. **実機テスト**
   - 改善を確認（4/5項目Pass）
   - ただしタイミング依存で失敗するケースあり

**テスト結果**: 144テスト全パス（138 → 144）

**変更ファイル**:
| ファイル | 変更内容 |
|----------|----------|
| [src/device/manager.rs](../../prototype/m1-gnss/backend/src/device/manager.rs) | receive_ubx B5 62同期、extract_ubx_frame追加、テスト+6 |

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [session101/session-summary.md](../session101/session-summary.md) | セッションサマリー |
| [session102/session-plan.md](../session102/session-plan.md) | 次セッション計画 |

**残課題**:
- 案Bの実装（検査中のNMEA OFF/ON）

**次セッション（Session 102）でやること**:
- 案B: cfg_valset.rs 作成（NMEA OFF/ONメッセージ生成）
- 案B: engine.rs で検査開始/終了時に NMEA OFF/ON
- 実機テストで効果確認（5項目安定Pass）

---

## Session 102 (2026-03-11)

**概要**: UBX通信タイミング問題の対策実装（案B: NMEA OFF/ON）

**実施内容**:
1. **cfg_valset.rs 作成**
   - CFG-VALSETメッセージ生成モジュール新規作成
   - `set_uart1_nmea_output(enable, layer)` 関数実装
   - テスト5件追加
2. **engine.rs でNMEA制御追加**
   - run()冒頭でNMEA OFF、最後でNMEA ON
   - CFG-VALSET送信後に50ms待機
3. **テスト修正・追加**
   - G1-G5テスト修正（NMEA制御メッセージを考慮）
   - H1-H3テスト追加（NMEA制御検証）
4. **実機テスト**
   - 効果は限定的（エラー率あまり変わらず）
   - 通信疎通/FWバージョンでエラー発生

**テスト結果**: 152テスト全パス（144 → 152、+8テスト）

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [cfg_valset.rs](../../prototype/m1-gnss/backend/src/ubx/cfg_valset.rs) | CFG-VALSETメッセージ生成 |
| [session102/session-summary.md](../session102/session-summary.md) | セッションサマリー |
| [session103/session-plan.md](../session103/session-plan.md) | 次セッション計画 |

**変更ファイル**:
| ファイル | 変更内容 |
|----------|----------|
| engine.rs | NMEA OFF/ON制御追加、テスト修正・追加 |
| ubx/mod.rs | cfg_valsetモジュール追加 |

**残課題**:
- タイミング問題が根本解決していない
- CFG-VALSETのACK確認、または待機時間調整が必要

**次セッション（Session 103）でやること**:
- タイミング問題の原因をさらに調査
- ACK確認の実装、または待機時間調整
- 安定して5項目Passする状態を目指す

---

## Session 103 (2026-03-11)

**概要**: UBX通信タイミング問題の根本解決（ACK確認実装）

**実施内容**:
1. **デバッグログのメンテナンス**
   - 仮説検証可能なログ出力を追加
   - drain_bufferで読み捨てデータの内容をログ出力
   - receive_ubxでUBX前の蓄積データを詳細ログ
2. **ubx/ack.rs 新規作成**
   - `parse_ack()`: ACK-ACK/ACK-NAK判定
   - `is_ack_for()`: 指定class/idのACK判定
   - テスト7件追加
3. **manager.rs に wait_for_ack() 追加**
   - CFG-VALSET送信後にACK-ACKを待つ
   - 500msタイムアウト
4. **engine.rs の NMEA OFF 処理変更**
   - 変更前: send_ubx() → 50ms待機
   - 変更後: send_ubx() → wait_for_ack()

**テスト結果**: 159テスト全パス（152 → 159、+7テスト）

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [ack.rs](../../prototype/m1-gnss/backend/src/ubx/ack.rs) | ACK/NAKメッセージ関連 |
| [session103/session-summary.md](../session103/session-summary.md) | セッションサマリー |
| [session104/session-plan.md](../session104/session-plan.md) | 次セッション計画 |

**変更ファイル**:
| ファイル | 変更内容 |
|----------|----------|
| engine.rs | ログ強化、ACK確認追加 |
| manager.rs | ログ強化、wait_for_ack追加 |
| ubx/mod.rs | ackモジュール追加 |

**実機テスト結果**: 100回以上テスト、99%以上Pass
- ACK-ACK受信成功 → NMEA OFF適用確認
- 1回だけFwVersionエラー（NMEA ONのACK-ACKが遅れて届いた）

**発見した問題**:
- NMEA ON送信後にACK-ACKを待っていない
- 次の検査でACK-ACKがConnectivityの応答として処理される
- 応答が1つずつズレてパースエラー

**次セッション（Session 104）でやること**:
- NMEA ON送信後もACK-ACKを待つ修正

---

## Session 104 (2026-03-11)

**概要**: NMEA ON後のACK待ち追加（残り1%エラー対策）

**実施内容**:
1. **engine.rs 修正**
   - NMEA ON送信後に `wait_for_ack()` を追加
   - 500msタイムアウトでACK-ACKを待機
   - 次回検査でACK-ACKが遅れて届く問題を解消
2. **テスト修正**
   - `all_pass_responses()` にNMEA ON ACK-ACK追加
   - 関連テスト（B1, F2）にもACK応答追加
3. **連続テストツール追加**
   - `make stress-test` で100回連続テスト
   - エラー時は `/tmp/gnss-stress-test/` にログ保存

**テスト結果**: 159テスト全パス

**変更ファイル**:
| ファイル | 変更内容 |
|----------|----------|
| engine.rs | NMEA ON後ACK待ち追加、テスト修正 |
| api.mk | stress-test ターゲット追加 |

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [session104/session-summary.md](../session104/session-summary.md) | セッションサマリー |
| [session105/session-plan.md](../session105/session-plan.md) | 次セッション計画 |

**次セッション（Session 105）でやること**:
- 実機テスト結果確認（100回連続テスト0%エラー）
- 屋外検査項目の調査・設計

---
