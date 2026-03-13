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
