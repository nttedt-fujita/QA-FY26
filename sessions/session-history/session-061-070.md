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
