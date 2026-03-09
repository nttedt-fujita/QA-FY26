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
   - sessions/ の元ファイルは作成経緯の記録として残す

3. **二重管理を防ぐ**
   - 同じ情報を複数の場所に書かない
   - 詳細は1箇所に置き、他からは参照リンクで済ませる

4. **正式ドキュメントの配置先**
   - M3関連: `docs/missions/m3-incoming-inspection-db/`
   - M4関連: `docs/missions/m4-defect-db/`
   - ツール: `tools/`

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
- **M3/M4の方向性**: タブレット操作可能なアプリ化を検討、kintone vs 自前開発を比較検討中
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

| ADR | タイトル | 影響範囲 | 状態 |
|-----|---------|---------|------|
| [ADR-001](docs/adr/ADR-001-error-handling.md) | エラーハンドリング方針 | 全API | 承認済み |
| [ADR-002](docs/adr/ADR-002-api-contract.md) | API契約とFE/BE整合性 | 型定義・API設計 | 承認済み |
| [ADR-003](docs/adr/ADR-003-lot-list-view.md) | ロット一覧画面の設計判断 | FE画面・ナビ | 承認済み |
| [ADR-004](docs/adr/ADR-004-gnss-tool-approach.md) | GNSS評価ツールのアプローチ選択 | M1-B GNSS評価 | 承認済み |

**ADR詳細ファイル配置先**: `docs/adr/`

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

### 重要な参照資料

| 資料 | 内容 |
|------|------|
| [sessions/session25/quality-framework-research.md](sessions/session25/quality-framework-research.md) | 品質管理フレームワーク調査（IQC/PQC/OQC、8D等） |
| [docs/missions/m3-incoming-inspection-db/hearing-items.md](docs/missions/m3-incoming-inspection-db/hearing-items.md) | ヒアリング項目統合版（P0〜P3優先度付き） |
| [prototype/m3/docs/implementation-plan.md](prototype/m3/docs/implementation-plan.md) | プロトタイプ実装計画（Session 42-47） |
| [sessions/session41/all-screens-mockup.drawio](sessions/session41/all-screens-mockup.drawio) | 全画面モックアップ |
