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
| [ADR-001](prototype/docs/adr/ADR-001-error-handling.md) | エラーハンドリング方針 | 全API | 承認済み |

**ADR詳細ファイル配置先**: `prototype/docs/adr/`

## 過去セッションで確認した資料の索引

### Session 1（2026-03-04）
| 資料 | 内容 |
|------|------|
| [sessions/session1/qa-fundamentals-report.md](sessions/session1/qa-fundamentals-report.md) | QA基礎知識調査レポート |
| [sessions/session1/mission-approach-report.md](sessions/session1/mission-approach-report.md) | ミッション別アプローチ調査レポート |

### Session 2（2026-03-04）
| 資料 | 内容 |
|------|------|
| [sessions/session2/company-qa-qc-report.md](sessions/session2/company-qa-qc-report.md) | 自社文脈でのQA/QC活動整理レポート |
| [sessions/session2/mission-priority-analysis.md](sessions/session2/mission-priority-analysis.md) | ミッション優先順位分析 |
| [docs/index.md](docs/index.md) | ドキュメントインデックス（ミッション別に整理） |

### Session 3（2026-03-04）
| 資料 | 内容 |
|------|------|
| [docs/images/quality-flow.svg](docs/images/quality-flow.svg) | 品質管理フロー図（製品別フロー + DB横断構造） |
| [sessions/session3/m3m4-requirements-memo.md](sessions/session3/m3m4-requirements-memo.md) | M3/M4要件の初期メモ |
| [docs/qa-overview-slide.md](docs/qa-overview-slide.md) | スライド（SVG画像埋め込み済み） |

### Session 4（2026-03-04）
| 資料 | 内容 |
|------|------|
| [docs/fujita-mission-slide.md](docs/fujita-mission-slide.md) | ミッション特化スライド（Need+M1〜M4、石川さん向け） |
| [docs/missions/m3m4-development-approach.md](docs/missions/m3m4-development-approach.md) | M3/M4の進め方（開発サイクル・ドメインモデリング・保守戦略） |

### Session 5（2026-03-04）
| 資料 | 内容 |
|------|------|
| [sessions/session5/aws-cost-estimation.md](sessions/session5/aws-cost-estimation.md) | AWS構成案・コスト試算（3パターン比較） |
| [sessions/session5/kintone-evaluation.md](sessions/session5/kintone-evaluation.md) | kintone調査・評価レポート |
| [sessions/session5/platform-comparison.md](sessions/session5/platform-comparison.md) | プラットフォーム比較表（3案比較+推奨） |
| [sessions/session5/maintenance-strategy.md](sessions/session5/maintenance-strategy.md) | 保守戦略（保守範囲・外注スコープ・合意項目） |
| [sessions/session5/hearing-plan.md](sessions/session5/hearing-plan.md) | 現行Excel運用ヒアリング計画（質問リスト21問） |
| [sessions/session5/domain-modeling-guide.md](sessions/session5/domain-modeling-guide.md) | ドメインモデリング学習・準備ガイド |

### Session 6（2026-03-04）
| 資料 | 内容 |
|------|------|
| [sessions/session6/excel-review.md](sessions/session6/excel-review.md) | 現行Excel（受入検査作業集計）全8シートの客観的レビュー・課題抽出 |

### Session 7（2026-03-04）
| 資料 | 内容 |
|------|------|
| [sessions/session7/ears-requirements-hypotheses.md](sessions/session7/ears-requirements-hypotheses.md) | EARS形式 要求仮説19個（途中・次セッションで継続） |

### Session 8（2026-03-05）
| 資料 | 内容 |
|------|------|
| [sessions/session8/session8-integration-policy.md](sessions/session8/session8-integration-policy.md) | 統合方針（過去セッション + 別セッション資料の扱い） |
| [sessions/session8/hearing-items-integrated.md](sessions/session8/hearing-items-integrated.md) | ヒアリング項目統合版（約40項目、P0〜P3優先度付き） |
| [sessions/session8/resource-index.md](sessions/session8/resource-index.md) | 確認した資料のインデックス（逆引き含む） |
| [sessions/session8/another-session-Need-requirements-files/](sessions/session8/another-session-Need-requirements-files/) | 別セッション資料（AWS自前開発の詳細検討、5ファイル） |

### Session 13（2026-03-05）
| 資料 | 内容 |
|------|------|
| [sessions/session13/hearing-notes-ishikawa-02.md](sessions/session13/hearing-notes-ishikawa-02.md) | 石川さんヒアリングメモ #02（スケジュール・M1〜M4方針の更新） |
| [sessions/session13/project-status-reorg.md](sessions/session13/project-status-reorg.md) | プロジェクト状況 再整理（Session 13時点の全ミッション状況） |
| [sessions/session13/【藤田】品質G　2026取り組み.pdf](sessions/session13/【藤田】品質G　2026取り組み.pdf) | スライドPDF最終版（FY26取り組み全体＋M3/M4ヒアリング資料） |

### Session 17（2026-03-06）
| 資料 | 内容 |
|------|------|
| [sessions/session17/pdf-excel-analysis.md](sessions/session17/pdf-excel-analysis.md) | PDF（UBX仕様書）・Excel（小峰無線GPS確認）全シート詳細分析・未確認項目リスト |
| [sessions/session17/files-reviewed.md](sessions/session17/files-reviewed.md) | 今セッションで確認したファイル一覧（PDF読み込み範囲・Excel未読画像一覧） |
| [sessions/session17/session-summary.md](sessions/session17/session-summary.md) | セッションサマリー（発見事項・次セッション計画） |

### Session 18（2026-03-06）
| 資料 | 内容 |
|------|------|
| [docs/missions/m1-sensor-evaluation/gnss/](docs/missions/m1-sensor-evaluation/gnss/) | **GNSS評価統合ドキュメント（8ファイル+README）** — Session 17テキスト+Session 18画像を統合 |
| [docs/missions/m1-sensor-evaluation/gnss/07-cross-sheet-findings.md](docs/missions/m1-sensor-evaluation/gnss/07-cross-sheet-findings.md) | 横断発見事項・合格基準叩き台・末永さんヒアリング項目10問 |
| [sessions/session18/session-summary.md](sessions/session18/session-summary.md) | セッションサマリー |

### Session 19（2026-03-06）
| 資料 | 内容 |
|------|------|
| [sessions/session19/extract_csv.py](sessions/session19/extract_csv.py) | 受入検査Excel→CSV変換スクリプト（6シート、574件） |
| [sessions/session19/test_csv_integrity.py](sessions/session19/test_csv_integrity.py) | 整合性テスト（62テスト、TDDレビュー済み） |
| [sessions/session19/analysis-plan.md](sessions/session19/analysis-plan.md) | 月別分析方針（パレート+4M、出力ファイル構成） |
| [sessions/session19/csv-output/summary/品名別_検査工数集計.csv](sessions/session19/csv-output/summary/品名別_検査工数集計.csv) | 品名別検査工数集計（222品名、合計530.5時間） |

### Session 20（2026-03-06）
| 資料 | 内容 |
|------|------|
| [sessions/session20/monthly_analysis.py](sessions/session20/monthly_analysis.py) | 月別分析スクリプト（39テスト全パス） |
| [sessions/session20/naming-normalization-policy.md](sessions/session20/naming-normalization-policy.md) | 名寄せルール方針 |
| [sessions/session20/csv-output/月別サマリー.csv](sessions/session20/csv-output/月別サマリー.csv) | 月別サマリー（2024-09〜2026-03） |

### Session 21（2026-03-06）
| 資料 | 内容 |
|------|------|
| [sessions/session21/name_normalizer.py](sessions/session21/name_normalizer.py) | 名寄せモジュール（NameNormalizerクラス、17テスト） |
| [sessions/session21/monthly_analysis_v2.py](sessions/session21/monthly_analysis_v2.py) | 月別分析スクリプトv2（名寄せ対応、変換率39%） |
| [sessions/session21/mapping/検査内容_名寄せルール.csv](sessions/session21/mapping/検査内容_名寄せルール.csv) | 検査内容の名寄せルール（39件） |

### Session 22（2026-03-06）
| 資料 | 内容 |
|------|------|
| [tools/](tools/) | **統合ツールディレクトリ** — Session 19-21のスクリプトを集約 |
| [tools/README.md](tools/README.md) | ツール使用ガイド（使い方・モジュール説明） |
| [tools/incoming_inspection/mapping/品名_名寄せルール.csv](tools/incoming_inspection/mapping/品名_名寄せルール.csv) | 品名の名寄せルール（18件） |
| [sessions/session22/data-anomaly-report.md](sessions/session22/data-anomaly-report.md) | データ異常レポート（未来日付10件、入荷日不明34件） |

### Session 23（2026-03-06）
| 資料 | 内容 |
|------|------|
| [tools/incoming_inspection/data_cleaner.py](tools/incoming_inspection/data_cleaner.py) | データクレンジングモジュール（未来日付修正 + 入荷日フォールバック） |
| [tools/tests/incoming_inspection/test_data_cleaner.py](tools/tests/incoming_inspection/test_data_cleaner.py) | テストコード（26テスト） |
| [tools/incoming_inspection/monthly_analysis.py](tools/incoming_inspection/monthly_analysis.py) | 月別分析スクリプトv3（名寄せ + クレンジング統合） |

### Session 24（2026-03-06）
| 資料 | 内容 |
|------|------|
| [docs/missions/m3-incoming-inspection-db/domain-modeling-approach.md](docs/missions/m3-incoming-inspection-db/domain-modeling-approach.md) | ドメインモデリング方針（As-Is/To-Be分離、プロトタイプ判断基準） |
| [docs/missions/m3-incoming-inspection-db/excel-analysis-summary.md](docs/missions/m3-incoming-inspection-db/excel-analysis-summary.md) | 分析結果サマリー（小笠原さん報告用、データ限界・品質問題含む） |

### Session 25（2026-03-06）
| 資料 | 内容 |
|------|------|
| [sessions/session25/quality-framework-research.md](sessions/session25/quality-framework-research.md) | **品質管理フレームワーク調査レポート**（IQC/PQC/OQC、ロット管理、AQL、8D、品質協定書、QC7つ道具） |
| [sessions/session25/prototype-approach.md](sessions/session25/prototype-approach.md) | M3/M4プロトタイプ方針（Phase 1-3の段階的アプローチ） |
| [docs/missions/m3-incoming-inspection-db/hearing-items.md](docs/missions/m3-incoming-inspection-db/hearing-items.md) | **ヒアリング項目統合版**（Session 5,8,24,25の統合、P0〜P3優先度付き） |
| [sessions/session25/meeting-notes-am.md](sessions/session25/meeting-notes-am.md) | 午前打ち合わせメモ（宇枝さん要望、品質協定書課題） |

### Session 26（2026-03-06）
| 資料 | 内容 |
|------|------|
| [docs/missions/m3-incoming-inspection-db/excel-analysis-summary.md](docs/missions/m3-incoming-inspection-db/excel-analysis-summary.md) | 分析結果サマリー **更新**（前処理の記載追加、データ品質問題拡充） |

### Session 27（2026-03-06）
| 資料 | 内容 |
|------|------|
| [docs/missions/m3-incoming-inspection-db/as-is-model.drawio](docs/missions/m3-incoming-inspection-db/as-is-model.drawio) | As-Isモデル概念図（現行Excel構造）— 品質管理視点の追加が必要 |
| [sessions/session27/session-summary.md](sessions/session27/session-summary.md) | セッションサマリー（品質管理視点欠落の発見、qa-design-reviewスキル作成方針） |

### Session 28（2026-03-06）
| 資料 | 内容 |
|------|------|
| [.claude/skills/qa-design-review/SKILL.md](.claude/skills/qa-design-review/SKILL.md) | **品質管理設計レビュースキル** — M3/M4設計時の品質管理視点チェックリスト |
| [docs/missions/m3-incoming-inspection-db/qa-gap-analysis.drawio](docs/missions/m3-incoming-inspection-db/qa-gap-analysis.drawio) | 品質管理視点のギャップ分析図（現行Excel vs 理想的IQC） |

### Session 29（2026-03-06）
| 資料 | 内容 |
|------|------|
| [tools/incoming_inspection/data_cleaner.py](tools/incoming_inspection/data_cleaner.py) | **ArrowResolverクラス追加** — 矢印記号（↓↑）を参照先の数値に変換 |
| [tools/tests/incoming_inspection/test_data_cleaner.py](tools/tests/incoming_inspection/test_data_cleaner.py) | テストコード（33テスト、ArrowResolver 7テスト追加） |

### Session 30（2026-03-06）
| 資料 | 内容 |
|------|------|
| [docs/missions/m3-incoming-inspection-db/qa-gap-analysis-slide.md](docs/missions/m3-incoming-inspection-db/qa-gap-analysis-slide.md) | ギャップ分析スライド（Marp形式） |
| [docs/missions/m3-incoming-inspection-db/qa-gap-analysis.svg](docs/missions/m3-incoming-inspection-db/qa-gap-analysis.svg) | ギャップ分析図（SVGエクスポート） |
| [docs/missions/m3-incoming-inspection-db/excel-analysis-summary.md](docs/missions/m3-incoming-inspection-db/excel-analysis-summary.md) | 分析サマリー **更新**（構造的な問題8項目を追記） |

### Session 31（2026-03-06）
| 資料 | 内容 |
|------|------|
| [docs/missions/m3-incoming-inspection-db/README.md](docs/missions/m3-incoming-inspection-db/README.md) | **M3新README**（インデックス + 概要、サブディレクトリ整理後） |

### Session 32（2026-03-06）
| 資料 | 内容 |
|------|------|
| [tools/incoming_inspection/dashboard.py](tools/incoming_inspection/dashboard.py) | **分析ダッシュボード**（Streamlit、Phase 1プロトタイプ） |
| [docs/missions/m3-incoming-inspection-db/to-be/ears-requirements-hypotheses.md](docs/missions/m3-incoming-inspection-db/to-be/ears-requirements-hypotheses.md) | EARS要求 **更新**（Excel分析結果を反映、Phase 1検証可能な要求を明確化） |

### Session 33（2026-03-06）
| 資料 | 内容 |
|------|------|
| [docs/missions/m3-incoming-inspection-db/to-be/to-be-model.drawio](docs/missions/m3-incoming-inspection-db/to-be/to-be-model.drawio) | **To-Beドメインモデル**（品質管理フレームワーク反映、IQC/PQC/OQC、ロット、8D対応） |

### Session 34（2026-03-06）
| 資料 | 内容 |
|------|------|
| [sessions/session34/session-summary.md](sessions/session34/session-summary.md) | **技術選定変更**（バックエンド: TypeScript → Go） |

**重要な決定**: バックエンドをGoに変更（パフォーマンス・依存関係・脆弱性リスクで有利）

### Session 35（2026-03-06）
| 資料 | 内容 |
|------|------|
| [prototype/db/schema.sql](prototype/db/schema.sql) | **DB設計**（To-Beモデル準拠、8テーブル） |
| [prototype/backend/](prototype/backend/) | Goバックエンド初期化（最小HTTPサーバー） |
| [prototype/frontend/](prototype/frontend/) | Next.jsフロントエンド（TypeScript, Tailwind） |

**重要な決定**: プロトタイプ環境構築完了。次セッションでDocker化 + TDD実装

### Session 36（2026-03-06）
| 資料 | 内容 |
|------|------|
| [prototype/docker-compose.yml](prototype/docker-compose.yml) | **Docker開発環境**（Go + PostgreSQL） |
| [prototype/docker-compose.test.yml](prototype/docker-compose.test.yml) | テスト環境（ポート5433） |
| [prototype/backend/Dockerfile](prototype/backend/Dockerfile) | Goバックエンド（マルチステージビルド） |
| [prototype/db/init.sql](prototype/db/init.sql) | PostgreSQL初期化スクリプト |

**重要な決定**: To-Beモデルで先に実装（ヒアリング後の変更コストは受け入れる）。TDDテストシナリオ（ロットCRUD 5件）承認済み

### Session 37（2026-03-06）
| 資料 | 内容 |
|------|------|
| [prototype/backend/internal/handler/lot_handler_test.go](prototype/backend/internal/handler/lot_handler_test.go) | **ロットCRUD統合テスト**（10ケース、全Green） |
| [prototype/backend/internal/handler/lot_handler.go](prototype/backend/internal/handler/lot_handler.go) | ロットAPIハンドラー |
| [prototype/backend/internal/repository/lot.go](prototype/backend/internal/repository/lot.go) | ロットリポジトリ |
| [prototype/docs/architecture-concerns.md](prototype/docs/architecture-concerns.md) | **DDD/CA懸念点**（将来対応計画） |

**重要な確認**: To-BeモデルはM3のみ設計済み、M4は連携ポイントのみ。DDD戦術的パターンはプロトタイプ段階では意図的に未適用

### Session 38（2026-03-06）

**重要な決定**: プロトタイプ方向性変更。ロットCRUDだけでなく「検査記録入力→ダッシュボード連携」の全体フローを実装

※ Session 38の実装計画はSession 41で更新・統合済み

### Session 39（2026-03-09）
| 資料 | 内容 |
|------|------|
| [sessions/session39/review-direction.md](sessions/session39/review-direction.md) | **方針整理**（見直しの方向性・アプローチ、3つの仮説、プロトタイプの位置づけ再定義） |
| [sessions/session39/m3-research-key-points.md](sessions/session39/m3-research-key-points.md) | **重要ポイント抽出**（m3-research-files 9ファイルの核心的内容、著名人の視点整理） |
| [sessions/session39/m3-research-files/](sessions/session39/m3-research-files/) | **DMAICアプローチ改善調査資料**（9ファイル、別セッションで作成） |
| [sessions/session39/quality-management-glossary.md](sessions/session39/quality-management-glossary.md) | 品質管理用語集（QM/QA/QC階層、著名人リスト） |

**重要な発見**: 暗黙の前提「記録ツールを作れば改善できる」は不十分。正しくは「問題を解決するプロセスの中で、記録基盤としてのツールが必要になる」。プロトタイプの位置づけを「Phase 2（記録基盤構築）のための最小限のツール」に再定義

### Session 41（2026-03-09）
| 資料 | 内容 |
|------|------|
| [prototype/docs/implementation-plan.md](prototype/docs/implementation-plan.md) | **プロトタイプ実装計画**（セッション別、Session 42-47） |
| [sessions/session41/all-screens-mockup.drawio](sessions/session41/all-screens-mockup.drawio) | 全画面モックアップ |
| [sessions/session41/input-ui-patterns.drawio](sessions/session41/input-ui-patterns.drawio) | 入力UIパターン比較（A/B/C） |

**重要な決定**: 入力UI方式はパターンB（カウンター方式）を採用。現場の「合格、合格、合格...」という作業実態に合致

### Session 42（2026-03-09）
| 資料 | 内容 |
|------|------|
| [prototype/db/seed.sql](prototype/db/seed.sql) | シードデータ |
| [prototype/docs/adr/ADR-001-error-handling.md](prototype/docs/adr/ADR-001-error-handling.md) | エラーハンドリング方針ADR |

**実施内容**: バックエンド準備完了（シードデータ、マスタAPI）、ADRルール策定

### Session 43（2026-03-09）
| 資料 | 内容 |
|------|------|
| [sessions/session43/test-scenarios.md](sessions/session43/test-scenarios.md) | **テストシナリオ**（TDD Phase 2成果物） |
| [prototype/backend/internal/session/](prototype/backend/internal/session/) | セッション状態管理（インメモリ） |
| [prototype/backend/internal/handler/inspection_session_handler.go](prototype/backend/internal/handler/inspection_session_handler.go) | 検査セッションAPIハンドラー |

**実施内容**: 検査セッションAPI（カウンター方式）TDD実装。単体8件+統合7件テスト

### Session 44（2026-03-09）
| 資料 | 内容 |
|------|------|
| [prototype/frontend/src/app/inspection/page.tsx](prototype/frontend/src/app/inspection/page.tsx) | **カウンター画面**（検査記録入力、モックアップ準拠） |
| [prototype/frontend/src/types/inspection.ts](prototype/frontend/src/types/inspection.ts) | 検査関連の型定義 |
| [sessions/session44/session-summary.md](sessions/session44/session-summary.md) | セッションサマリー |

**実施内容**: カウンター画面（フロントエンド）実装、CORS対応、API用語統一（ok/ng/skip）

**残課題**: 工数表示の修正（小数点以下が長すぎる）、ロット登録画面改善
