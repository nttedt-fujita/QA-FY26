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

**実機テスト結果**: 200回連続テスト Pass（0%エラー）

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
- 屋外検査項目の調査・設計

---

## Session 105 (2026-03-11)

**概要**: 屋外検査項目の調査・要求整理

**実施内容**:
1. **セッション履歴の修正**
   - Session 104の「200回連続テストPass」記載漏れを修正
2. **過去資料の確認**
   - 元Excel（小峰無線GPS確認.xlsx）で確認していた情報を整理
   - 07-cross-sheet-findings.md、03-spectrum-analyze.md等を確認
3. **発見した問題**
   - NAV-SAT vs NAV-SIG: 07-cross-sheet-findings.mdでは**NAV-SIG**推奨
   - MON-SPANの仕様未確認（128MHz/256点の取得方法不明）
   - 要求整理が先 — 合格基準・ヒアリング結果を確認してから設計すべき

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [session105/session-summary.md](../session105/session-summary.md) | セッションサマリー |
| [session105/outdoor-inspection-design.md](../session105/outdoor-inspection-design.md) | 屋外検査設計（ドラフト、不完全） |
| [session106/session-plan.md](../session106/session-plan.md) | 次セッション計画 |

**次セッション（Session 106）でやること**:
- u-blox仕様書確認（MON-SPAN p.134）
- 屋外検査の要求整理（What）

---

## Session 106 (2026-03-11)

**概要**: MON-SPAN仕様確認 + 屋外検査要求整理

**実施内容**:
1. **UBX仕様書からの情報抽出**
   - PyMuPDFでPDFから目次抽出→該当ページ読み取り
   - MON-SPAN (p.134), NAV-SAT (p.150-151), NAV-SIG (p.152-154)を抽出
2. **MON-SPAN仕様の確認**
   - 256点のスペクトラムデータ（dB単位）
   - 中心周波数計算: f(i) = center + span × (i - 127) / 256
   - 128MHz/256点の場合、分解能500kHz
3. **NAV-SAT vs NAV-SIG比較**
   - 結論: 屋外検査では**NAV-SIG**を使用（L1/L2別のC/N0が必要）
4. **屋外検査の要求整理**
   - 優先度の高い要求（What）を整理
   - 実装優先順位を決定

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [session106/ubx-spec-memo.md](../session106/ubx-spec-memo.md) | MON-SPAN, NAV-SAT, NAV-SIG仕様メモ |
| [session106/outdoor-inspection-needs.md](../session106/outdoor-inspection-needs.md) | 屋外検査の要求整理（What） |
| [session106/session-summary.md](../session106/session-summary.md) | セッションサマリー |
| [session107/session-plan.md](../session107/session-plan.md) | 次セッション計画 |

**hooks観察**:
- PDF抽出スクリプトを毎回新規作成している問題を記録

**次セッション（Session 107）でやること**:
- NAV-SIGパーサー実装（TDD）
- ドキュメント整理（直近セッションの資料メンテナンス）

---

## Session 107 (2026-03-11)

**概要**: NAV-SIG仕様調査 + TDD Phase 1振る舞い記述

**実施内容**:
1. **sigId定義の正確な抽出**
   - PDF p.18-21からsigId定義を抽出
   - L1/L2判定ロジックの正確な仕様を確認
   - **重要発見**: 既存コードにバグあり（GPS sigId 3をL1Cと誤認、実際はL2 CL）
2. **NAV-SAT/NAV-SIG仕様の正式配置**
   - docs/missions/m1-sensor-evaluation/gnss/に正式ドキュメント作成
3. **TDD Phase 1: 振る舞い記述**
   - NAV-SIGパーサーの振る舞い仕様を作成
   - テストシナリオリストを設計

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [ubx-signal-identifiers.md](../../docs/missions/m1-sensor-evaluation/gnss/ubx-signal-identifiers.md) | sigId定義（正式版） |
| [ubx-nav-messages.md](../../docs/missions/m1-sensor-evaluation/gnss/ubx-nav-messages.md) | NAV-SAT/NAV-SIG仕様 |
| [ubx-mon-messages.md](../../docs/missions/m1-sensor-evaluation/gnss/ubx-mon-messages.md) | MON-SPAN/MON-RF仕様 |
| [session107/nav-sig-behavior-spec.md](../session107/nav-sig-behavior-spec.md) | NAV-SIG振る舞い仕様（TDD Phase 1） |
| [session107/ubx-spec-extract.md](../session107/ubx-spec-extract.md) | PDF抽出結果（中間ファイル） |
| [session107/session-summary.md](../session107/session-summary.md) | セッションサマリー |
| [session108/session-plan.md](../session108/session-plan.md) | 次セッション計画 |
| [nav_sig.rs](../../prototype/m1-gnss/backend/src/ubx/nav_sig.rs) | NAV-SIGパーサー（ドラフト、sigId判定バグあり） |

**残課題**:
- 屋外検査要求の確定（ヒアリング未完了部分）
- nav_sig.rsのsigId判定バグ修正
- TDD Phase 2-5（テスト実装・コード修正）

**次セッション（Session 108）でやること**:
- 屋外検査要求の確定（業界標準ベースで合格基準設定）
- NAV-SIGパーサー実装（TDD Phase 2-5）

---

## Session 108 (2026-03-11)

**概要**: 屋外検査合格基準のADR作成 + TDDで振る舞い仕様の抜け漏れ確認

**実施内容**:
1. **屋外検査合格基準のADR作成（ADR-008）**
   - 業界標準調査（Session 53）をベースに合格基準を確定
   - L1受信感度: ≥30 dBHz、L2受信率: GPS 50%以上
   - RTK FIX時間: ≤30秒、RTK FIX率: >95%
2. **TDDスキルで要求・振る舞いの抜け漏れ確認**
   - qualityInd ≥ 5 を「L2受信中」の定義として決定
   - 集計ロジックはフリー関数（signal_stats）として実装決定
   - 閾値判定は検査ロジック（OutdoorInspector等）の責任と決定

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [ADR-008](../../docs/adr/m1/ADR-008-outdoor-inspection-criteria.md) | 屋外検査の合格基準 |
| [session108/session-summary.md](../session108/session-summary.md) | セッションサマリー |
| [session109/session-plan.md](../session109/session-plan.md) | 次セッション計画 |

**変更ファイル**:
| ファイル | 変更内容 |
|----------|----------|
| CLAUDE.md | ADR一覧にADR-008追加 |

**決定事項（ADR-008に記録）**:
- qualityInd ≥ 5（搬送波ロック以上）を「L2受信中」とする
- 集計ロジックはフリー関数（案C）で実装
- 閾値判定はパーサーではなく検査ロジックに持たせる

**次セッション（Session 109）でやること**:
- NAV-SIG振る舞い仕様の更新（ADR-008の決定を反映）
- NAV-SIGパーサー + signal_stats実装（TDD Phase 3-5）

---
