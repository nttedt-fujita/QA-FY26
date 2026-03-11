# セッション履歴: Session 81〜90

## Session 81 (2026-03-10)

**概要**: DeviceManager実装（TDD Phase 3〜5）

**実施内容**:
1. **TDD Phase 3** — テストコード作成（22テスト）
   - status.rs: 状態遷移テスト（A1-A11）
   - filter.rs: フィルタリングテスト（B1-B5）
   - manager.rs: DeviceManagerテスト（C1-C4）
2. **TDD Phase 4** — 実装（Red → Green）— 全22テストパス
3. **TDD Phase 5** — リファクタリング（未使用フィールド削除、警告解消）

**テスト結果**: 50テスト全パス（device 22 + ubx 28）

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [device/mod.rs](../../prototype/m1-gnss/backend/src/device/mod.rs) | deviceモジュール定義 |
| [device/status.rs](../../prototype/m1-gnss/backend/src/device/status.rs) | 状態遷移ロジック |
| [device/filter.rs](../../prototype/m1-gnss/backend/src/device/filter.rs) | ポートフィルタ |
| [device/manager.rs](../../prototype/m1-gnss/backend/src/device/manager.rs) | DeviceManager |
| [session81/session-summary.md](../session81/session-summary.md) | セッションサマリー |
| [session82/session-plan.md](../session82/session-plan.md) | 次セッション計画 |

**進捗**: Phase 1 Step 2（DeviceManager）完了 ✅

**次セッション（Session 82）でやること**:
- InspectionEngine実装（TDD）
- 検査シーケンス制御、UBX送受信とパース、検査結果の構造化

---

## Session 82 (2026-03-10)

**概要**: InspectionEngine設計（TDD Phase 1-2）

**実施内容**:
1. **TDD Phase 1** — 振る舞い記述
   - 検査シーケンス、結果判定、状態連携、異常系を定義
   - 検査項目は5項目（要件定義FR2に準拠）
   - NAV-STATUSは検査項目に含めない判断
2. **TDD Phase 2** — テストシナリオリスト作成
   - 24シナリオを導出
   - 実装優先順位を決定

**設計判断**:
- 検査項目数: 5項目（FR2準拠）
- NAV-STATUS: 検査項目に含めない（FR2に記載なし）
- 進捗通知・キャンセル: Phase 1では省略

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [session82/inspection-engine-behavior.md](../session82/inspection-engine-behavior.md) | 振る舞い記述＋テストシナリオ |
| [session82/session-summary.md](../session82/session-summary.md) | セッションサマリー |
| [session83/session-plan.md](../session83/session-plan.md) | 次セッション計画 |

**進捗**: Phase 1 Step 3（InspectionEngine）設計完了

**次セッション（Session 83）でやること**:
- InspectionEngine実装（TDD Phase 3-5）
- 24シナリオのテストコード作成と実装

---

## Session 83 (2026-03-10)

**概要**: InspectionEngine実装開始 + F9P実機テスト

**実施内容**:
1. **inspectionモジュール作成**
   - types.rs: 型定義（InspectionItem, InspectionResult, Verdict等）
   - judge.rs: 結果判定ロジック
2. **テストコード作成（一部）**
   - D1-D3: 構造体テスト（7テスト）
   - C1-C5: 結果判定ロジックテスト（9テスト）
3. **F9P実機テスト**
   - FTDI経由UART接続（VID=0x0403, PID=0x6015）
   - ボーレート38400bpsでNMEA受信成功
   - 115200に統一したい（ボードごとに設定が違う）

**設計への影響**:
- 接続方式: USB直接 → UART経由
- フィルタリング: FTDI対応追加 or 手動ポート指定
- ボーレート: 設定可能にする（デフォルト115200）

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [inspection/mod.rs](../../prototype/m1-gnss/backend/src/inspection/mod.rs) | モジュール定義 |
| [inspection/types.rs](../../prototype/m1-gnss/backend/src/inspection/types.rs) | 型定義 + D1-D3テスト |
| [inspection/judge.rs](../../prototype/m1-gnss/backend/src/inspection/judge.rs) | 判定ロジック + C1-C5テスト |
| [session83/session-summary.md](../session83/session-summary.md) | セッションサマリー |

**進捗**: Phase 1 Step 3（InspectionEngine）TDD Phase 3 一部完了

**次セッション（Session 84）でやること**:
- lib.rsにinspectionモジュール追加、engine.rs作成
- 残りテストシナリオ（A1-A4, G1-G5, B1-B2, E1-E2, F1-F3）
- TDD Phase 4-5（実装→リファクタリング）
- 実機対応（FTDI、ボーレート設定）

---

## Session 84 (2026-03-11)

**概要**: InspectionEngine実装完了（TDD Phase 3-4）

**実施内容**:
1. **ビルド可能化**
   - lib.rsにinspectionモジュール追加
   - engine.rs作成
2. **TDD Phase 3-4** — 残り16シナリオのテスト＋実装
   - A1-A4: 検査シーケンス実行（4テスト）
   - B1-B2: 通信疎通（2テスト）
   - G1-G5: 各検査項目UBX送信（5テスト）
   - F1-F3: 異常系（3テスト）
   - E1-E2: 状態連携 → Phase 1では省略

**テスト結果**: 81テスト全パス（device 22 + ubx 28 + inspection 31）

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [inspection/engine.rs](../../prototype/m1-gnss/backend/src/inspection/engine.rs) | InspectionEngine本体 + テスト |
| [session84/session-summary.md](../session84/session-summary.md) | セッションサマリー |
| [session85/session-plan.md](../session85/session-plan.md) | 次セッション計画 |

**進捗**: Phase 1 Step 3（InspectionEngine）完了 ✅

**実機テストまでのロードマップ**:
```
Session 85: DB Repository実装
Session 86-87: 基本UI実装
Session 88: FTDI対応＋ボーレート設定
Session 89: Phase 1 統合テスト（実機）T1-1〜T1-7
```

**次セッション（Session 85）でやること**:
- DB Repository実装（Phase 1 Step 4）
- FTDI対応方針決定

---

## Session 85 (2026-03-11)

**概要**: DB Repository実装 → **ドメインモデリング未実施の問題発覚で途中終了**

**実施内容**:
1. **DB Repository実装（途中）**
   - rusqlite/chronoをCargo.tomlに追加
   - repositoryモジュール作成（types.rs, sqlite.rs）
   - 93テスト全パス
2. **問題発覚**: ドメインモデリングをやらずにDB設計に入っていた
3. **ドメインモデリング開始（途中）**
   - 要求（Needs）から名詞抽出
   - 5軸チェックリスト
   - **ロット**という概念の必要性が判明
4. **運用イメージの修正**
   - 当初: ロットを先に登録、期待値を決める
   - 実際: 最初の1台が仮の期待値、複数台検査で多数派がわかる

**学び**:
- ドメインモデリングを省略してDB設計に入ると手戻りが発生する
- 運用イメージの確認が重要

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [session85/incoming-inspection-domain-model.md](../session85/incoming-inspection-domain-model.md) | 受入検査ドメインモデル（途中） |
| [session85/db-repository-behavior.md](../session85/db-repository-behavior.md) | DB Repository振る舞い記述（見直し必要） |
| [repository/types.rs](../../prototype/m1-gnss/backend/src/repository/types.rs) | 型定義（見直し必要） |
| [repository/sqlite.rs](../../prototype/m1-gnss/backend/src/repository/sqlite.rs) | SQLite実装（見直し必要） |
| [session85/session-summary.md](../session85/session-summary.md) | セッションサマリー |
| [session86/session-plan.md](../session86/session-plan.md) | 次セッション計画 |

**進捗**: Phase 1 Step 4（DB Repository）**途中** — ドメインモデリング要修正

**次セッション（Session 86）でやること**:
- ドメインモデリング完了（運用イメージ反映、屋外確認項目整理）
- DB設計見直し

---

## Session 86 (2026-03-11)

**概要**: ドメインモデリング完了 + 屋内/屋外統合ドメインモデル作成

**実施内容**:
1. **ドメインモデリング修正**
   - FW判定方式を明確化（FWはRecorded、他はPass/Fail）
2. **過去コンテキスト確認**
   - Session 62（屋外計測向け）とSession 85（屋内検査向け）で別々のモデルが存在
3. **統合ドメインモデル作成**
   - 運用フロー: 屋内検査 → 屋外計測
   - 装置の紐づけ: シリアル番号で紐づける
   - M3との統合: 後で（両方プロトタイプ）
4. **統合DB設計（ドラフト）**
   - lots, indoor_inspections, inspection_item_results を追加

**重要な決定**:
| 項目 | 決定 |
|------|------|
| 運用フロー | 屋内検査 → 屋外計測 |
| 装置の紐づけ | シリアル番号で紐づける |
| M3との統合 | 後で（両方プロトタイプ） |
| FW判定 | Recorded（全記録、後で比較） |

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [session86/incoming-inspection-domain-model-v2.md](../session86/incoming-inspection-domain-model-v2.md) | 屋内検査ドメインモデル（FW判定修正） |
| [session86/m1-m3-relationship.md](../session86/m1-m3-relationship.md) | M1とM3の関係性整理 |
| [session86/domain-model-integration.md](../session86/domain-model-integration.md) | 統合検討 |
| [session86/gnss-unified-domain-model.md](../session86/gnss-unified-domain-model.md) | **統合ドメインモデル（決定版）** |
| [session86/unified-db-schema.md](../session86/unified-db-schema.md) | 統合DB設計（ドラフト） |
| [session86/session-summary.md](../session86/session-summary.md) | セッションサマリー |
| [session87/session-plan.md](../session87/session-plan.md) | 次セッション計画 |

**進捗**: ドメインモデリング完了、DB設計ドラフト作成

**未反映の議論内容**（Session 87で要検討）:
- FWバージョンの多数派・はずれ値比較 → ロット単位集計の設計が不明確
- devicesにfw_versionカラム追加 or クエリ対応
- item_nameの具体的な値を定義

**次セッション（Session 87）でやること**:
- 統合DB設計の未反映点を解決
- 統合DB設計の確定・実装
- 既存コードの修正

---

## Session 87 (2026-03-11)

**概要**: 統合DB設計の確定・実装

**実施内容**:
1. **未反映点の解決**
   - FWバージョン: devicesテーブルにfw_versionカラム追加
   - item_name定義: communication/serial/fw/rate/port
2. **schema.sql更新**
   - lots, indoor_inspections, inspection_item_results追加
   - measurement_sessions → outdoor_measurementsリネーム
   - session_id → measurement_idリネーム
3. **repository/types.rs全面書き換え**
   - Lot, Device, IndoorInspection, InspectionItemResult
   - InspectionItemName enum, Verdict enum
4. **repository/sqlite.rs全面書き換え**
   - 全エンティティのCRUD操作実装

**テスト結果**: 110テスト全パス

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [db/schema.sql](../../prototype/m1-gnss/db/schema.sql) | 統合スキーマ |
| [repository/types.rs](../../prototype/m1-gnss/backend/src/repository/types.rs) | 新エンティティ |
| [repository/sqlite.rs](../../prototype/m1-gnss/backend/src/repository/sqlite.rs) | CRUD実装 |
| [session87/session-summary.md](../session87/session-summary.md) | セッションサマリー |

**進捗**: Phase 1 Step 4（DB Repository）屋内検査関連 完了 ✅

**次セッション（Session 88）でやること**:
- FTDI対応＋ボーレート設定
- InspectionEngineとRepositoryの統合

---

## Session 88 (2026-03-11)

**概要**: ドキュメントメンテナンス + ADR構造変更

**実施内容**:
1. **設計/実装整合性レビュー** — ドメインモデルとDB設計の整合性確認（OK）
2. **ADR構造変更** — フラット → ミッション別サブディレクトリ（m1/, m3/, common/）
3. **ドキュメント正式配置** — gnss-unified-domain-model.md, m1-m3-relationship.md
4. **不要ドキュメント削除** — Session 85-86の途中版7ファイル
5. **CLAUDE.md更新** — ドキュメント配置ルール、ADR一覧

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [docs/adr/common/ADR-006-m1-m3-integration.md](../../docs/adr/common/ADR-006-m1-m3-integration.md) | M1/M3統合方針（新規ADR） |
| [docs/missions/m1-sensor-evaluation/gnss/19-gnss-unified-domain-model.md](../../docs/missions/m1-sensor-evaluation/gnss/19-gnss-unified-domain-model.md) | 統合ドメインモデル |
| [session88/session-summary.md](../session88/session-summary.md) | セッションサマリー |

**ルール追加**:
- CLAUDE.md: 「正式配置後はsessions/から削除」（廃止マークではなく削除が適切）

**次セッション（Session 89）でやること**:
- FTDI対応 + ボーレート設定
- InspectionEngineとRepositoryの統合

---

## Session 89 (2026-03-11)

**概要**: FTDI対応 + ボーレート調査計画

**実施内容**:
1. **FTDI対応**
   - filter.rs: FTDI_VID/PID定数、is_ftdi_device(), filter_gnss_ports()追加
   - manager.rs: filter_gnss_ports使用、ボーレート設定機能追加
   - テスト追加（C1-C6）
2. **ボーレート調査計画作成**
   - ZED-F9P Integration Manualの調査計画をドキュメント化
   - Pythonスクリプトで目次抽出→該当ページ抽出の方針

**計画変更**:
- InspectionEngine → Repository統合は次回へ延期
- ボーレート自動判定の可能性を先に調査

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [filter.rs](../../prototype/m1-gnss/backend/src/device/filter.rs) | FTDI対応追加 |
| [manager.rs](../../prototype/m1-gnss/backend/src/device/manager.rs) | ボーレート設定追加 |
| [baud-rate-investigation-plan.md](../session89/baud-rate-investigation-plan.md) | ボーレート調査計画 |
| [session89/session-summary.md](../session89/session-summary.md) | セッションサマリー |

**進捗**: Phase 1 Step 5（FTDI対応）完了、Step 6（ボーレート）調査中

**次セッション（Session 90）でやること**:
- ZED-F9P Integration ManualのPDF読み込み（Pythonで目次抽出）
- ボーレート自動判定の調査・方針決定
- InspectionEngine → Repository統合（時間があれば）

---
