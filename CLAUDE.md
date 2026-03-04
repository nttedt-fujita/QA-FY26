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
