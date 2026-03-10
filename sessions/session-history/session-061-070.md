# セッション履歴: Session 61〜70

## Session 61 (2026-03-09)

**概要**: GNSS評価ツールのドメインモデリング・技術選定

**実施内容**:
1. **フロントエンド技術選定** — Next.js（React）に決定
2. **要求の再確認** — 10-tool-design-notes.mdで要求を再確認
3. **DB設計案の検討** — SQLite + テーブル構造案

**重要な決定**:
- フロントエンド: Next.js（React）
- バックエンド: Rust + Actix-web（Session 60で決定済み）
- 永続化: SQLite

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [session61/session-summary.md](../session61/session-summary.md) | セッションサマリー |
| [session62/session-plan.md](../session62/session-plan.md) | 次セッション計画 |

**未実施（Session 62へ持ち越し）**:
- DB設計の最終決定
- ディレクトリ構成の整理
- TDD Phase 3-4（NAV-PVTパーサー実装）

**次セッション（Session 62）でやること**:
- DB設計最終決定
- ディレクトリ構成整理
- NAV-PVTパーサーのTDD実装

---

## Session 62 (2026-03-10)

**概要**: GNSS評価ツールのドメインモデリング + hooks振り返り仕組み作り

**実施内容**:
1. **ドメインモデリング** — 作業フロー追跡による概念整理、ドメインモデル確定
2. **学びの記録** — ドメインモデリング作業履歴のドキュメント化（スキル化参考）
3. **hooks振り返り仕組み作り** — 観察記録ファイル、毎セッション振り返りルール追加

**重要な決定**:
- ドメインモデル: デバイス → 計測セッション → 計測データの階層構造
- スコープ: 集計値は都度計算、評価基準は設定ファイル、レポート出力はスコープ外

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [prototype/m1-gnss/docs/domain-model.md](../../prototype/m1-gnss/docs/domain-model.md) | ドメインモデル |
| [session62/domain-modeling-learnings.md](../session62/domain-modeling-learnings.md) | ドメインモデリング作業履歴 |
| [session62/session-summary.md](../session62/session-summary.md) | セッションサマリー |
| [session63/session-plan.md](../session63/session-plan.md) | 次セッション計画 |
| [~/.claude/hooks-observations.md] | Hooks導入ニーズ観察記録 |
| [~/.claude/rules/11-hooks-review.md] | 毎セッションhooks振り返りルール |

**未実施（Session 63へ持ち越し）**:
- テーブル設計
- ディレクトリ構成の整理
- TDD Phase 3-4（NAV-PVTパーサー実装）

**次セッション（Session 63）でやること**:
- テーブル設計
- ディレクトリ構成整理
- NAV-PVTパーサーのTDD実装

---

## Session 63 (2026-03-10)

**概要**: GNSS評価ツールのテーブル設計・ディレクトリ構成整理・NAV-PVTパーサー実装

**実施内容**:
1. **テーブル設計** — `db/schema.sql`作成（9テーブル）
2. **ディレクトリ構成整理** — src/ → backend/に移動
3. **NAV-PVTパーサー実装** — TDDでテスト13件を含むパーサー実装

**重要な決定**:
- ペイロードのバイトオフセットは仕様書推測に基づく（実機検証待ち）
- テストはテーブルテスト形式で統一

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [db/schema.sql](../../prototype/m1-gnss/db/schema.sql) | SQLiteスキーマ定義 |
| [backend/src/lib.rs](../../prototype/m1-gnss/backend/src/lib.rs) | ライブラリルート |
| [backend/src/ubx/nav_pvt.rs](../../prototype/m1-gnss/backend/src/ubx/nav_pvt.rs) | NAV-PVTパーサー |
| [session63/session-summary.md](../session63/session-summary.md) | セッションサマリー |

**未実施（Session 64へ持ち越し）**:
- DevContainer内でのテスト実行確認
- 他のUBXメッセージパーサー
- Next.jsフロントエンド作成

**次セッション（Session 64）でやること**:
- テスト実行確認
- NAV-STATUS, NAV-DOPパーサー
- Next.jsプロジェクト作成

---

## Session 64 (2026-03-10)

**概要**: PDF抽出ツール作成 + UBXメッセージ仕様抽出

**実施内容**:
1. **PDF抽出ツール作成** — `tools/pdf_page_extractor.py`（pymupdf使用）
2. **UBX仕様抽出** — NAV-STATUS、NAV-DOP、MON-RFの仕様をPDFから抽出
3. **仕様整理** — `ubx-messages-spec.md` 作成

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [tools/pdf_page_extractor.py](../../tools/pdf_page_extractor.py) | PDF抽出ツール |
| [session64/ubx-messages-spec.md](../session64/ubx-messages-spec.md) | 整理済みUBX仕様書 |
| [session64/session-summary.md](../session64/session-summary.md) | セッションサマリー |
| [session65/session-plan.md](../session65/session-plan.md) | 次セッション計画 |

**未実施（Session 65へ持ち越し）**:
- NAV-STATUS/NAV-DOP/MON-RFパーサー実装（TDD）
- DevContainer内でのテスト実行確認
- Next.jsフロントエンド作成

**次セッション（Session 65）でやること**:
- UBXパーサー実装（TDD）
- テスト実行確認

---

## Session 65 (2026-03-10)

**概要**: 受入検査Excel分析資料の整理（石川さんからの質問対応）

**実施内容**:
1. **工数上位10品目の詳細資料作成** — 作業者・件ごとの工数を抽出
2. **検査内容別の工数詳細資料作成** — 「外観＋通電」「動作確認」の内訳
3. **退職者情報の整理** — 若原、加藤、角田が退職済み

**重要な発見**:
- PCM Assy 24hは電源基板にヒートシンクを貼る作業（小笠原さん情報）。担当者（若原・加藤）は退職済み
- チャージャ 16.7hは備考の計算と合致、杉山に確認可能
- 長野さんは2人（長野将、長野佳）

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [session65/top10-inspection-details.md](../session65/top10-inspection-details.md) | 工数上位10品目の詳細 |
| [session65/inspection-type-details.md](../session65/inspection-type-details.md) | 検査内容別の工数内訳 |
| [session65/session-summary.md](../session65/session-summary.md) | セッションサマリー |
| [session66/session-plan.md](../session66/session-plan.md) | 次セッション計画 |

**未実施（Session 66へ持ち越し）**:
- UBXパーサー実装（TDD）
- DevContainer内でのテスト実行確認

**次セッション（Session 66）でやること**:
- NAV-STATUS/NAV-DOP/MON-RFパーサー実装（TDD）
- テスト実行確認

---

## Session 66 (2026-03-10)

**概要**: M3プロトタイプの起動・動作確認（Windows PC / WSL2環境）+ AS-DT1納入仕様質問リスト作成

**実施内容**:
1. **M3プロトタイプ起動** — Docker Compose（PostgreSQL + Go Backend）+ Next.jsフロントエンド
2. **API動作確認** — 部品・ロット・作業者の取得、ダッシュボードAPIを確認
3. **別PCアクセス設定** — WSL2ポートフォワーディング + ファイアウォール設定
4. **AS-DT1納入仕様調査** — 既存プロジェクト（SONY-LiDAR-TestMinutes）の情報確認、質問リスト初版作成

**重要な情報**:
- WSL2 IP: 172.20.171.75（Windows PC: 192.168.100.57）
- ポートフォワーディング設定済み（3000, 8080）
- 解除コマンドはsession-summaryに記載

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [session66/session-summary.md](../session66/session-summary.md) | セッションサマリー |
| [session66/task_sony_lidar_as-dt1.md](../session66/task_sony_lidar_as-dt1.md) | AS-DT1タスク定義 |
| [session66/as-dt1-spec-questions.md](../session66/as-dt1-spec-questions.md) | AS-DT1質問リスト初版 |
| [session67/session-plan.md](../session67/session-plan.md) | 次セッション計画 |

**未実施（次セッションへ持ち越し）**:
- AS-DT1納入仕様: Web調査・エビデンス整備・質問リスト再構成
- UBXパーサー実装（NAV-STATUS/NAV-DOP/MON-RF）— TDD
- DevContainer内でのテスト実行確認

**次セッション（Session 67）でやること**:
- AS-DT1納入仕様調査の続き（Web調査、エビデンス整備）
- Ubuntu PCでM3プロトタイプの作業を再開（時間があれば）

---

## Session 67 (2026-03-10)

**概要**: AS-DT1仕様書（ユーザーズガイド FW1.00）の確認とドキュメント化

**実施内容**:
1. **PDF抽出スクリプト作成** — 目次確認→ユーザー確認→抽出のフローで作業
2. **仕様書からの情報抽出** — 6カテゴリに分けてMarkdown生成
3. **質問リスト更新（v2）** — 仕様書から判明した事項を整理、残り15件に絞り込み

**主な発見**:
- 電源仕様、物理仕様、耐衝撃性などが仕様書から判明
- 「APIマニュアル」への参照が複数箇所あり、別途確認が必要
- 反射率10%未満、IP等級、耐振動などは仕様書に記載なし

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [session67/extract_as_dt1_spec.py](../session67/extract_as_dt1_spec.py) | PDF抽出スクリプト |
| [session67/extracted/](../session67/extracted/) | 抽出済みMarkdown（6ファイル） |
| [session67/as-dt1-spec-questions-v2.md](../session67/as-dt1-spec-questions-v2.md) | 質問リスト更新版 |
| [session67/session-summary.md](../session67/session-summary.md) | セッションサマリー |
| [session68/session-plan.md](../session68/session-plan.md) | 次セッション計画 |

**Hooks観察**:
- PDFを直接読みに行った問題 → PDF作業フローのルール化を提案

**次セッション（Session 68）でやること**:
- APIマニュアルPDFの目次確認・抽出
- 抽出済みファイルの正式配置（`docs/missions/m1-sensor-evaluation/lidar/as-dt1/`）

---
