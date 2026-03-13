# セッション履歴: Session 171〜180

## Session 171 (2026-03-13)

**概要**: ドキュメント整理・現状把握・優先度整理

**実施内容**:
1. Session 155-170のファイル整理
2. PDF目次を統合して33番に配置
3. 仕様抽出ファイルを34-36番として配置
4. デバッグログのメンテナンス（info! → debug!）

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [33-toc-ublox-f9-interface-description.md](../../docs/missions/m1-sensor-evaluation/gnss/33-toc-ublox-f9-interface-description.md) | u-blox F9 HPG PDF目次 |
| [34-ubx-mon-comms.md](../../docs/missions/m1-sensor-evaluation/gnss/34-ubx-mon-comms.md) | UBX-MON-COMMS仕様 |
| [35-ubx-uart-config.md](../../docs/missions/m1-sensor-evaluation/gnss/35-ubx-uart-config.md) | CFG-UART1/2仕様 |
| [36-ntrip-rtk-findings.md](../../docs/missions/m1-sensor-evaluation/gnss/36-ntrip-rtk-findings.md) | NTRIP/RTK調査まとめ |

**次セッション（Session 172）でやること**:
1. RTK屋外テスト実施
2. FE 30秒設定の見直し検討

---

## Session 172 (2026-03-13)

**概要**: 生データ保存機能の設計計画立案

**実施内容**:
1. **問題の認識**: RTKフィックス状態がFE経由でしかわからない、DBからスカイプロット再表示したい
2. **現状調査**: 既存資料・コードを確認（ドメインモデル、スキーマ、BE/FE実装）
3. **重要な発見**: 既存スキーマに詳細データ用テーブル（outdoor_measurements系）が存在するが未実装
4. **設計計画書作成**: 3つのオプションを検討、ハイブリッド案（オプションC）を推奨

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [raw-data-storage-plan.md](../session172/raw-data-storage-plan.md) | 生データ保存機能の設計計画書 |
| [session-summary.md](../session172/session-summary.md) | セッションサマリー |

**確認した資料・コード**:
- 19-gnss-unified-domain-model.md
- schema.sql
- session105/outdoor-inspection-design.md
- session106/outdoor-inspection-needs.md
- repository/types.rs
- outdoor_inspection_api.rs
- useOutdoorInspection.ts

**次セッション（Session 173）でやること**:
1. 計画書のレビュー・精査
2. オプションCで進めてよいか確認
3. 実装開始（Phase 1: BE）

---

## Session 173 (2026-03-13)

**概要**: 生データ保存機能 Phase 1（BE）実装

**実施内容**:
1. オプションB vs C の工数比較 → **オプションC（ハイブリッド）採用**
2. DBスキーマ: `outdoor_inspection_snapshots`テーブル追加
3. リポジトリ: insert/get_snapshots実装
4. API: POST拡張 + GET snapshots新規追加

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [session-summary.md](../session173/session-summary.md) | セッションサマリー |

**変更ファイル**:
| ファイル | 変更内容 |
|----------|----------|
| `repository/types.rs` | OutdoorInspectionSnapshot構造体追加 |
| `repository/sqlite.rs` | テーブル作成 + CRUD |
| `web/outdoor_inspection_api.rs` | POST拡張 + GET snapshots |

**確認した問題**:
- 既存テストコードでserialportクレートAPI変更によるエラー（今回の変更とは無関係）

**次セッション（Session 174）でやること**:
1. Phase 2: FE実装（スナップショット蓄積・送信）
2. または: テストコード修正（serialport API対応）

---

## Session 174 (2026-03-13)

**概要**: 生データ保存機能 Phase 2（FE）実装

**実施内容**:
1. FE実装: `snapshots` state追加、`addSnapshot()`関数追加、`saveResult`でスナップショット送信
2. NTRIP位置更新の修正: `gnss_state_api`がNAV-PVT取得成功時に`current_position`を更新

**変更ファイル**:
| ファイル | 変更内容 |
|----------|----------|
| `hooks/useOutdoorInspection.ts` | スナップショット蓄積・送信 |
| `app/inspections/outdoor/page.tsx` | useEffectでaddSnapshot()呼び出し |
| `web/gnss_state_api.rs` | current_position更新 |

---

## Session 175 (2026-03-13)

**概要**: 生データ保存動作確認 → 競合問題調査・方針検討

**実施内容**:
1. 屋外テスト実施: 検査10件、スナップショット15件保存
2. **問題発覚**: 30秒検査で1-2回しかスナップショット取れない
3. BEログ分析: 1回のAPI呼び出しに16秒（タイムアウト連発、NTRIP競合）
4. 解決方針検討: 案A（読み書き分離）と案D（タイムアウト短縮）

**重要な発見**:
- gnss_state_apiとntrip_apiが同じMutexを取り合い
- serialport v4の`try_clone()`で読み書き分離が可能

**確認した1次情報**:
| ファイル | 内容 |
|----------|------|
| gnss_state_api.rs:63 | タイムアウト2秒設定 |
| gnss_state_api.rs:184 | ロック取得（読み込み側） |
| ntrip_api.rs:502 | ロック取得（書き込み側） |
| ADR-012 | 統合API採用経緯 |

**次セッションでやること（KISS原則順）**:
1. **案D（タイムアウト短縮）を先に検討** - 最もシンプルな解決策
2. 案Dで不十分なら案A（try_clone）
3. async化（mio-serial/tokio-serial）は最後の手段

---

## Session 176 (2026-03-13)

**概要**: 整理作業（要求の再確認・ログ分析方針策定）

**実施内容**:
1. **要求の再確認**: 競合問題解決に飛びついていたが、そもそもの要求を見失っていた
2. **問題の再認識**: FEの30秒終了後もBEが数十秒動いている問題
3. **ログ分析方針策定**: 全検査のログを分割・抽出して傾向を把握する方針を文書化

**重要な学び**:
- 解決策に飛びつかない（問題を正しく理解してから）
- 根拠なく結論づけない（FE側ファイル書き出し可能だった）
- ログ分析は時間をかけてやる

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [log-analysis-plan.md](../session176/log-analysis-plan.md) | ログ分析の方針・手順 |
| [mutex-contention-analysis.md](../session176/mutex-contention-analysis.md) | Mutex競合分析（要修正） |

**次セッション（Session 177）でやること**:
1. 全10件の検査ログを分割・抽出
2. 各検査の時間データを表にまとめる
3. 傾向を分析
4. 根本原因の特定と対策案検討

---

## Session 177 (2026-03-13)

**概要**: ログ分析実施・根本原因特定

**実施内容**:
1. 全17件の検査ログを分割・抽出（当初10件と思っていたが17件）
2. 各検査の時間データを表にまとめた
3. 傾向を分析
4. 根本原因の特定と対策案検討

**主な発見**:
- POST後BE処理回数: 平均8.2回
- ずれ時間: 8秒〜214秒（典型: 60-80秒）
- ロック待機警告: 100件
- **根本原因**: FEポーリング間隔（1秒）とBE処理能力（6秒/回）の不整合

**残った課題**:
- 処理の流れが不明確（FE→BEのポーリングの仕組みが説明できていない）

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [log-analysis-results.md](../session177/log-analysis-results.md) | ログ分析結果・傾向・対策案 |
| [logs/](../session177/logs/) | 17件の検査ログ（分割済み） |
| [session-summary.md](../session177/session-summary.md) | セッションサマリー |

**次セッション（Session 178）でやること**:
1. 図解作成（drawio）: 概念説明図、シーケンス図
2. 対策案A実装（ポーリング間隔6秒化）

---

## Session 178 (2026-03-13)

**概要**: FE/BEポーリング処理の図解作成と対策実装

**実施内容**:
1. 概念説明図作成（polling-architecture.drawio）
2. シーケンス図作成（request-lifecycle.drawio）
3. 対策案A実装: ポーリング間隔を1秒→6秒に変更
4. 動作確認: POST後のBE処理時間が60-80秒→5秒に改善

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [polling-architecture.drawio](../session178/polling-architecture.drawio) | ポーリング構造概要図 |
| [request-lifecycle.drawio](../session178/request-lifecycle.drawio) | リクエストライフサイクル図 |
| [session-summary.md](../session178/session-summary.md) | セッションサマリー |

**変更ファイル**:
| ファイル | 変更内容 |
|----------|----------|
| `frontend/src/app/inspections/outdoor/page.tsx` | pollIntervalMs: 1000 → 6000 |

**動作確認結果**:
- gnss-state呼び出し: 30回 → 6回
- POST後BE処理時間: 60-80秒 → 5秒
- ロック待機警告: 100件 → 0件

**残課題**: POST後1件分（約5秒）の処理結果をFEに反映すべき

**次セッション（Session 179）でやること**:
1. FE表示タイミング調整: POST後のBE処理完了を待ってから結果表示
2. スナップショット可視化: DBから取得したデータでスカイプロット再表示

---

## Session 179 (2026-03-13)

**概要**: レスポンス駆動ポーリング実装・検査終了タイミング改善

**実施内容**:
1. ポーリング方式を固定間隔→レスポンス駆動に変更
2. 検査終了フローを分割（startCompleting/completeInspection）
3. completing状態で最後のレスポンスを待つ処理を追加
4. NTRIP-RTCMとのMutex競合対策（delayAfterResponseMs=1000）

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [session-summary.md](../session179/session-summary.md) | セッションサマリー |

**変更ファイル**:
| ファイル | 変更内容 |
|----------|----------|
| `hooks/useGnssState.ts` | レスポンス駆動ポーリング実装 |
| `hooks/useOutdoorInspection.ts` | 検査終了フロー分割、addFinalSample追加 |
| `app/inspections/outdoor/page.tsx` | completing状態で最後のレスポンス待ち |

**技術的決定**:
- レスポンス駆動: リクエスト完了後に次を送信（BEの処理速度に自動追従）
- delayAfterResponseMs=1000: NTRIP-RTCMがロックを取得する隙間を確保
- requestCountで最後のレスポンスを検知

**次セッション（Session 180）でやること**:
1. テスト結果の確認（ユーザー実施中）
2. 問題があれば修正
3. （余裕があれば）スナップショット可視化

---
