# QA-FY26 プロジェクト

## 概要

品質保証グループ（品質G）のFY2026取り組み管理プロジェクト。
藤田さんがソフト開発チームからソフトQA担当として2026年3月に異動。

## セッションディレクトリ

- セッション資料: `sessions/session{N}/`
- セッション履歴: `sessions/session-history/`

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
