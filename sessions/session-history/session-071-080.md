# セッション履歴: Session 71〜80

## Session 71 (2026-03-10)

**概要**: AS-DT1質問リストの最終レビュー・背景補強

**実施内容**:
1. **質問リストv6のレビュー** — 背景・理由が不十分であることを指摘
2. **質問リストv7作成** — 農薬散布用途・みかん畑環境を明記、仕様書の「避けるべき環境」を引用、エビデンスURL追加
3. **正式配置先への反映**

**重要な改訂**:
- 想定用途（農薬散布、みかん畑）を概要に明記
- Q01/Q02を統合、電線・みかんの木の枝など具体例を追加
- Q3/Q4に仕様書の「ほこりの多い所は避ける」「激しく振動する所は避ける」を引用
- 質問番号を連番化（11件→10件）

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [session71/as-dt1-spec-questions-v7.md](../session71/as-dt1-spec-questions-v7.md) | 質問リストv7 |
| [session71/session-summary.md](../session71/session-summary.md) | セッションサマリー |
| [session72/session-plan.md](../session72/session-plan.md) | 次セッション計画 |

**反省点**:
- session-managementスキルを使ったが、前セッションの資料をきちんと読んでいなかった
- 放熱板の仕様の出典（Session 69のスクリーンショット）を見落とした

**次セッション（Session 72）でやること**:
- GNSS評価ツール: UBXパーサー実装（NAV-STATUS/NAV-DOP/MON-RF）— TDD
- DevContainer内でのテスト実行確認

---

## Session 72 (2026-03-10)

**概要**: GNSS評価ツール UBXパーサー実装（TDD）

**実施内容**:
1. **DevContainer環境整備** — Dockerfile.devのRustバージョンを`latest`に更新
2. **NAV-STATUSパーサー実装** — TTFF、RTK状態、FIX有効性判定
3. **NAV-DOPパーサー実装** — 精度劣化係数（スケール0.01変換）
4. **MON-RFパーサー実装** — ジャミング状態監視、可変長ブロック対応

**テスト結果**: 15テスト全パス

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [nav_status.rs](../../prototype/m1-gnss/backend/src/ubx/nav_status.rs) | NAV-STATUSパーサー |
| [nav_dop.rs](../../prototype/m1-gnss/backend/src/ubx/nav_dop.rs) | NAV-DOPパーサー |
| [mon_rf.rs](../../prototype/m1-gnss/backend/src/ubx/mon_rf.rs) | MON-RFパーサー |
| [session72/ubx-parser-test-scenarios.md](../session72/ubx-parser-test-scenarios.md) | テストシナリオ文書 |
| [session72/session-summary.md](../session72/session-summary.md) | セッションサマリー |

**Hooks観察**:
- TDD Phase 1（振る舞い記述）で仕様書フィールドのヌケモレ発生
- ユーザー指摘で発覚、`~/.claude/hooks-observations.md`に記録

---

## Session 74 (2026-03-10)

**概要**: GNSS評価ツール要件定義・アーキテクチャ設計（ADR乖離問題発覚）

**実施内容**:
1. **要件定義作成** — FR8件/NFR5件/IFR4件を定義
2. **アーキテクチャ設計** — コンポーネント構成、データフロー、状態遷移
3. **実装計画** — 3フェーズ（MVP/レポート/複数台）+ 実機テスト計画
4. **ドキュメント配置** — Session 73の要求定義を正式配置先に移動

**発生した問題**:
- ADR-005を読まずにTauri + SvelteKitと書いた
- ADR-005を読んでも古い情報（静的HTML）で、Session 61の決定（Next.js）を見落とした
- 結果、アーキテクチャを3回修正

**重要な発見**:
- ADR-005とSession 61の決定が乖離している
- ADR運用ルールの見直しが必要

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [session74/gnss-tool-requirements.md](../session74/gnss-tool-requirements.md) | 要件定義 |
| [session74/gnss-tool-architecture.md](../session74/gnss-tool-architecture.md) | アーキテクチャ設計 |
| [session74/gnss-tool-implementation-plan.md](../session74/gnss-tool-implementation-plan.md) | 実装計画 + 実機テスト計画 |
| [session74/session-summary.md](../session74/session-summary.md) | セッションサマリー |
| [session75/session-plan.md](../session75/session-plan.md) | 次セッション計画 |
| [14-gnss-tool-needs.md](../../docs/missions/m1-sensor-evaluation/gnss/14-gnss-tool-needs.md) | 要求定義の正式配置 |

**Hooks観察**: 4件記録（ADR未読、履歴未読、ADR乖離、3回修正）

**次セッション（Session 75）でやること**:
- ADR-005のメンテナンス
- ADR運用ルールの見直し（adr-management/SKILL.md、rules/10-adr-enforcement.md）
- CLAUDE.mdのADR一覧更新

---

## Session 75 (2026-03-10)

**概要**: ADR・ドキュメント整理（実装前の立ち止まり）

**実施内容**:
1. **ADR-005メンテナンス** — Session 61の決定（Next.js）を反映、変更履歴追加
2. **ADR運用ルール見直し** — 変更時はADR更新を必須化、確認順序を明確化
3. **CLAUDE.md更新** — ADR一覧に「最終更新」列追加
4. **Session 74ドキュメント正式配置** — 要件定義・アーキテクチャ・実装計画

**重要な決定**:
| 項目 | 決定 |
|------|------|
| ADR更新タイミング | 決定変更時は同セッション内で更新 |
| ADR確認順序 | CLAUDE.md → ADR詳細 → セッション履歴 |

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [15-gnss-tool-requirements.md](../../docs/missions/m1-sensor-evaluation/gnss/15-gnss-tool-requirements.md) | 要件定義（正式配置） |
| [16-gnss-tool-architecture.md](../../docs/missions/m1-sensor-evaluation/gnss/16-gnss-tool-architecture.md) | アーキテクチャ（正式配置） |
| [17-gnss-tool-implementation-plan.md](../../docs/missions/m1-sensor-evaluation/gnss/17-gnss-tool-implementation-plan.md) | 実装計画（正式配置） |
| [session75/session-summary.md](../session75/session-summary.md) | セッションサマリー |
| [session76/session-plan.md](../session76/session-plan.md) | 次セッション計画 |

**次セッション（Session 76）でやること**:
- UBXパーサー追加実装（MON-VER, SEC-UNIQID）— TDD

---

## Session 76 (2026-03-10)

**概要**: UBXパーサー追加実装（MON-VER, SEC-UNIQID）— TDD Phase 1

**実施内容**:
1. **仕様抽出** — PDFからMON-VER、SEC-UNIQIDの仕様を抽出
2. **TDD Phase 1** — 振る舞い記述（MON-VER 8件、SEC-UNIQID 6件）

**発生した問題**:
- 仕様書の探し方で手戻り（セッション履歴を先に確認すべきだった）
- Session 64でPDF抽出ツールとubx-messages-spec.mdを作成済みだったが見落とした

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [session76/ubx-mon-ver-sec-uniqid-spec.md](../session76/ubx-mon-ver-sec-uniqid-spec.md) | MON-VER、SEC-UNIQID仕様 + 振る舞い記述 |
| [session76/ubx-mon-ver-sec-uniqid-raw.md](../session76/ubx-mon-ver-sec-uniqid-raw.md) | PDFから抽出した生データ |
| [session76/session-summary.md](../session76/session-summary.md) | セッションサマリー |
| [session77/session-plan.md](../session77/session-plan.md) | 次セッション計画 |

**Hooks観察**: 1件記録（セッション履歴を先に確認すべきパターン）

**次セッション（Session 77）でやること**:
- TDD Phase 2〜5: テストシナリオ作成 → テストコード → 実装 → リファクタリング

---

## Session 77 (2026-03-10)

**概要**: UBXパーサー実装（MON-VER, SEC-UNIQID）— TDD Phase 2〜5

**実施内容**:
1. **TDD Phase 2** — テストシナリオリスト作成・承認
2. **TDD Phase 3** — テストコード作成（テーブルテスト形式）
3. **TDD Phase 4** — 実装（Red → Green）— 全20テストパス
4. **TDD Phase 5** — リファクタリング（common.rsに共通処理を抽出）

**テスト結果**: 20テスト全パス

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [mon_ver.rs](../../prototype/m1-gnss/backend/src/ubx/mon_ver.rs) | MON-VERパーサー |
| [sec_uniqid.rs](../../prototype/m1-gnss/backend/src/ubx/sec_uniqid.rs) | SEC-UNIQIDパーサー |
| [common.rs](../../prototype/m1-gnss/backend/src/ubx/common.rs) | 共通定義（checksum, ヘッダー定数）|
| [session77/session-summary.md](../session77/session-summary.md) | セッションサマリー |
| [session78/session-plan.md](../session78/session-plan.md) | 次セッション計画 |

**進捗**: Phase 1 Step 1（UBXパーサー）5/7完了

**次セッション（Session 78）でやること**:
- CFG-RATE, CFG-PRT パーサー実装（TDD）
- または DeviceManager実装に着手

---

## Session 73 (2026-03-10)

**概要**: GNSS評価ツールの要求整理・要求定義作成

**実施内容**:
1. **ドキュメント配置** — Session 72のテストシナリオを正式配置先に移動
2. **過去の要求確認** — Session 16（小板橋さんヒアリング）、Session 56-57の内容確認
3. **ヒアリング** — 藤田さんの理想像を整理（自動検査、複数台同時接続、SQLite保存）
4. **要求定義作成** — EARSパターンで10件の要求を定義

**重要な決定**:
| 項目 | 決定 |
|------|------|
| 用途 | 受入検査（屋内で確認できる範囲）|
| RTK確認 | 一旦保留（受入検査ではやらない）|
| 同時接続台数 | 2〜5台程度（認知負荷に収まる範囲）|
| データ保存 | SQLite |
| レポート出力 | PDF + CSV |

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [session73/gnss-tool-needs.md](../session73/gnss-tool-needs.md) | 要求定義（EARS記法）|
| [session73/session-summary.md](../session73/session-summary.md) | セッションサマリー |
| [session74/session-plan.md](../session74/session-plan.md) | 次セッション計画 |
| [docs/.../13-ubx-parser-test-scenarios.md](../../docs/missions/m1-sensor-evaluation/gnss/13-ubx-parser-test-scenarios.md) | テストシナリオ正式配置 |

---

## Session 78 (2026-03-10)

**概要**: CFG-RATE/CFG-PRT パーサー実装準備（TDD Phase 1）

**実施内容**:
1. **仕様抽出** — PDFからCFG-RATE、CFG-PRTの仕様を抽出（pdf_page_extractor.py使用）
2. **仕様整理** — 抽出データを整理してMarkdownドキュメント化
3. **TDD Phase 1** — 振る舞い記述（CFG-RATE 10件、CFG-PRT 11件）

**重要な決定**:
| 項目 | 決定 | 理由 |
|------|------|------|
| CFG-PRTのスコープ | USBポート(portID=3)のみ | 受入検査ではUSB接続のみ使用 |

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [session78/cfg-rate-prt-raw.md](../session78/cfg-rate-prt-raw.md) | PDFから抽出した生データ |
| [session78/cfg-rate-prt-spec.md](../session78/cfg-rate-prt-spec.md) | 仕様書（整理済み） |
| [session78/cfg-rate-prt-behavior.md](../session78/cfg-rate-prt-behavior.md) | 振る舞い記述（TDD Phase 1） |
| [session78/session-summary.md](../session78/session-summary.md) | セッションサマリー |
| [session79/session-plan.md](../session79/session-plan.md) | 次セッション計画 |

**進捗**: Phase 1 Step 1（UBXパーサー）5/7（CFG-RATE/CFG-PRTはPhase 1のみ完了）

**次セッション（Session 79）でやること**:
- CFG-RATE/CFG-PRT の TDD Phase 2〜5（テストシナリオ → テストコード → 実装 → リファクタリング）

---

## Session 79 (2026-03-10)

**概要**: CFG-RATE/CFG-PRT パーサー実装（TDD Phase 2〜5）

**実施内容**:
1. **TDD Phase 2** — テストシナリオリスト作成、ヌケモレ確認で3件追加
2. **設計判断** — timeRef範囲外→Unknown、複数プロトコル同時→サポート、portID≠3→エラー
3. **TDD Phase 3** — テストコード作成（CFG-RATE 11件、CFG-PRT 13件）
4. **TDD Phase 4** — 実装（Red → Green）— 全28テストパス
5. **TDD Phase 5** — リファクタリング（追加の共通化は不要と判断）

**重要な設計判断**:
| 項目 | 決定 |
|------|------|
| timeRef範囲外（6以上） | Unknown として扱う（エラーではない） |
| 複数プロトコル同時 | サポートする（ビットマスクなので当然） |
| portID≠3 | UnsupportedPortエラー（USBのみ対応） |

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [cfg_rate.rs](../../prototype/m1-gnss/backend/src/ubx/cfg_rate.rs) | CFG-RATEパーサー |
| [cfg_prt.rs](../../prototype/m1-gnss/backend/src/ubx/cfg_prt.rs) | CFG-PRTパーサー |
| [session79/cfg-parser-design-decisions.md](../session79/cfg-parser-design-decisions.md) | 設計判断メモ |
| [session79/session-summary.md](../session79/session-summary.md) | セッションサマリー |
| [session80/session-plan.md](../session80/session-plan.md) | 次セッション計画 |

**進捗**: Phase 1 Step 1（UBXパーサー）7/7 完了 ✅

**次セッション（Session 80）でやること**:
- DeviceManager実装（Phase 1 Step 2）
- ポート列挙、接続管理、状態管理

---
