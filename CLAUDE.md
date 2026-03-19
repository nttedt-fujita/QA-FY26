# QA-FY26 プロジェクト

## 概要

品質保証グループ（品質G）のFY2026取り組み管理プロジェクト。
藤田さんがソフト開発チームからソフトQA担当として2026年3月に異動。

## セッションディレクトリ

- セッション資料: `sessions/session{N}/`
- セッション履歴: `sessions/session-history/`

## ドキュメント配置ルール

**IMPORTANT**: 以下のルールを遵守すること。

1. **sessions/で作成したドキュメントはドラフト扱い**
   - セッション中の作業記録・検討メモとして作成
   - 正式なドキュメントの置き場所ではない

2. **区切りのいいところでdocs/に配置**
   - 技術方針、設計書、分析結果などは `docs/missions/` に移動
   - 以降の更新は `docs/` 側を更新する

3. **正式配置後はsessions/から削除**
   - 上位互換のドキュメントが存在する場合は削除
   - 「廃止マーク」ではなく削除が適切
   - session-plan.md、session-summary.md は管理用として残す

4. **二重管理を防ぐ**
   - 同じ情報を複数の場所に書かない
   - 詳細は1箇所に置き、他からは参照リンクで済ませる

5. **正式ドキュメントの配置先**
   - M1関連: `docs/missions/m1-sensor-evaluation/`
   - M3関連: `docs/missions/m3-incoming-inspection-db/`
   - M4関連: `docs/missions/m4-defect-db/`
   - ADR: `docs/adr/{m1,m3,common}/`

## Makeコマンドを伝える際のルール

**IMPORTANT - MUST**: Makeコマンドをユーザーに伝える前に、**必ず該当プロジェクトのMakefile/makefiles/*.mkを読むこと**。推測でコマンド名を伝えてはならない。

例:
- ❌ 「`make backend-dev`で起動」（推測）
- ✅ Makefileを確認 → 「`make dev-backend`で起動」（確認済み）

## 現在の進捗

詳細は [docs/progress.md](docs/progress.md) を参照。

## 藤田さんの担当ミッション

1. AirGrow搭載センサーの定量的評価手法の策定（Lidar / GNSS）
2. 点群データ検証方法の策定
3. 受入検査データベース化
4. 工程不良データベース化

## チームメンバーと担当

| メンバー | 担当 |
|----------|------|
| いしかわ | EVPEAK/GREPOW/SKYDROID継続評価、FMEA、経年劣化部品検証(メカ)、次世代バッテリ評価 |
| ふじた   | センサー評価(Lidar/GNSS)、点群データ検証、受入検査DB、工程不良DB |
| こいたばし | 帯電散布検証、経年劣化部品検証、H12G EVT/DVT量産評価、中国製品修理スキーム |
| みんな   | 東南アジア安全規格習得 |

## ファイル構成

- `powerpoint-orgn/` — 元のPowerPointファイル（原本、編集しない）
- `slides.md` — PPTXの中間Markdownファイル（位置情報付き）
- `sessions/` — セッション管理ディレクトリ
- `docs/` — 整理済みドキュメント（ミッション別・QA知識）

## ドキュメント構成（docs/）

- **[docs/index.md](docs/index.md)** — 全体インデックス
- `docs/missions/m1-sensor-evaluation/` — M1: センサー評価手法策定
- `docs/missions/m2-pointcloud-verification/` — M2: 点群データ検証方法策定
- `docs/missions/m3-incoming-inspection-db/` — M3: 受入検査DB化
- `docs/missions/m4-defect-db/` — M4: 工程不良DB化
- `docs/qa-knowledge/` — QA基礎知識・自社QA/QC整理

## 現状の業務環境

- **受入検査・工程不良の記録**: 現在はExcel管理
- **M3/M4の方向性**: Go + Next.jsで自前開発（Session 34決定）、M3は⏸️ストップ中（M4優先、Session 52決定）
- **画像ディレクトリ**: `docs/images/` — スライド用の図表を格納

## 品質管理設計ルール

**IMPORTANT**: M3/M4関連の設計・図作成時は、必ず以下を参照すること。

- **スキル**: [.claude/skills/qa-design-review/SKILL.md](.claude/skills/qa-design-review/SKILL.md)
- **調査資料**: [sessions/session25/quality-framework-research.md](sessions/session25/quality-framework-research.md)

チェックすべき観点:
1. IQC/PQC/OQCの位置づけ
2. ロット/トレーサビリティの考慮
3. AQL/抜取検査の基準
4. 8Dフレームワーク（問題→原因→対策→効果確認）
5. M3/M4の紐づき（共通の「部品」で連携）

## ADR一覧（設計判断記録）

**IMPORTANT**: ADRに関わる判断時は作業を止めて確認を取ること。詳細は `~/.claude/rules/10-adr-enforcement.md` 参照。

**ADR構造**: ミッション別サブディレクトリ（番号はグローバル連番）
- `docs/adr/m1/` — M1専用
- `docs/adr/m3/` — M3専用
- `docs/adr/common/` — ミッション横断

### M3（受入検査DB）

| ADR | タイトル | 影響範囲 | 状態 |
|-----|---------|---------|------|
| [ADR-001](docs/adr/m3/ADR-001-error-handling.md) | エラーハンドリング方針 | 全API | 承認済み |
| [ADR-002](docs/adr/m3/ADR-002-api-contract.md) | API契約とFE/BE整合性 | 型定義・API設計 | 承認済み |
| [ADR-003](docs/adr/m3/ADR-003-lot-list-view.md) | ロット一覧画面の設計判断 | FE画面・ナビ | 承認済み |

### M1（センサー評価）

| ADR | タイトル | 影響範囲 | 状態 |
|-----|---------|---------|------|
| [ADR-004](docs/adr/m1/ADR-004-gnss-tool-approach.md) | GNSS評価ツールのアプローチ選択 | M1-B GNSS評価 | 承認済み |
| [ADR-005](docs/adr/m1/ADR-005-gnss-tool-tech-stack.md) | GNSS評価ツール技術スタック選定 | M1-B GNSS評価 | 承認済み |
| [ADR-007](docs/adr/m1/ADR-007-baud-rate-detection.md) | ボーレート自動検出方式 | DeviceManager | 承認済み |
| [ADR-008](docs/adr/m1/ADR-008-outdoor-inspection-criteria.md) | 屋外検査の合格基準 | 屋外検査ツール | 承認済み |
| [ADR-009](docs/adr/m1/ADR-009-outdoor-inspection-fe-aggregation.md) | 屋外検査の集計処理をFEで実行 | FE/BE責務分担 | 承認済み |
| [ADR-010](docs/adr/m1/ADR-010-api-test-strategy.md) | APIテスト戦略（統合テストで検証） | テスト方針 | 承認済み |
| [ADR-011](docs/adr/m1/ADR-011-ntrip-implementation.md) | NTRIP実装方式（独自実装を採用） | NTRIPクライアント | 承認済み |
| [ADR-012](docs/adr/m1/ADR-012-periodic-output-and-unified-api.md) | 定期出力と統合API採用 | API並行問題対策 | 承認済み |
| [ADR-013](docs/adr/m1/ADR-013-gnss-filter-linkage.md) | GNSSフィルタ連動と周波数帯別統計 | FE状態管理 | 承認済み |
| [ADR-014](docs/adr/m1/ADR-014-multi-device-manager.md) | 複数装置同時対応の実装方式 | Phase 3 複数台対応 | 承認済み |
| [ADR-015](docs/adr/m1/ADR-015-cfg-cfg-loadmask.md) | CFG-CFGでloadMaskを使わない | 設定リセットAPI | 承認済み |

### 共通（ミッション横断）

| ADR | タイトル | 影響範囲 | 状態 |
|-----|---------|---------|------|
| [ADR-006](docs/adr/common/ADR-006-m1-m3-integration.md) | M1/M3統合方針 | M1-B、M3 | 承認済み |

## 過去セッションの重要な決定

詳細なファイル一覧は [sessions/session-history/](sessions/session-history/) を参照。

| Session | 決定 | 備考 |
|---------|------|------|
| 34 | バックエンドをGoに変更 | パフォーマンス・依存関係・脆弱性リスクで有利 |
| 36 | To-Beモデルで先に実装 | ヒアリング後の変更コストは受け入れる |
| 37 | DDD戦術的パターンは未適用 | プロトタイプ段階では意図的に省略 |
| 38 | 全体フロー実装に方向転換 | ロットCRUDだけでなく検査記録→ダッシュボード連携 |
| 39 | プロトタイプの位置づけ再定義 | 「記録基盤としての最小限ツール」 |
| 41 | 入力UIはカウンター方式 | 現場の「合格、合格、合格...」作業実態に合致 |
| 256 | 員数確認AI化から外観検査AI化へ | プロポ・ペラの外観検査の方が効果大 |
| 260 | AI化の前提崩れ・方針転換 | 現場の隠れコスト発見、作業フロー全体の可視化へ |

### 重要な参照資料

| 資料 | 内容 |
|------|------|
| [sessions/session25/quality-framework-research.md](sessions/session25/quality-framework-research.md) | 品質管理フレームワーク調査（IQC/PQC/OQC、8D等） |
| [docs/missions/m3-incoming-inspection-db/hearing-items.md](docs/missions/m3-incoming-inspection-db/hearing-items.md) | ヒアリング項目統合版（P0〜P3優先度付き） |
| [prototype/m3/docs/implementation-plan.md](prototype/m3/docs/implementation-plan.md) | プロトタイプ実装計画（Session 42-47） |
| [sessions/session41/all-screens-mockup.drawio](sessions/session41/all-screens-mockup.drawio) | 全画面モックアップ |
