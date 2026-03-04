# セッション履歴: Session 1〜10

## Session 1 (2026-03-04)

**概要**: 初回セッション。プロジェクト基盤構築とQA基礎知識・ミッション別アプローチの調査レポートを作成。

**背景**: 藤田さんがソフト開発チームからソフトQAとして品質Gに異動。品質保証の知識がないため、基礎知識の習得とミッション遂行のアプローチ調査が必要。

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [session1/session-plan.md](../session1/session-plan.md) | Session 1計画 |
| [session1/qa-fundamentals-report.md](../session1/qa-fundamentals-report.md) | QA基礎知識調査レポート（QA/QC、ISO 9001、AQL、FMEA等） |
| [session1/mission-approach-report.md](../session1/mission-approach-report.md) | ミッション別アプローチ調査レポート（Lidar/GNSS/点群/受入検査DB/工程不良DB） |
| [session1/files-reviewed.md](../session1/files-reviewed.md) | 確認した情報源一覧 |
| [session1/session-summary.md](../session1/session-summary.md) | セッションサマリー |

**重要な発見**:
- ソフト開発スキルはQA業務に直接応用可能（DB設計、データ分析、テスト設計）
- M3(受入検査DB)とM4(工程不良DB)はlot_idで紐付けて一体設計すべき
- 国際規格（ASTM E2938, ISO 17123-9, ASPRS）が各ミッションの参照基準として使える

## Session 2 (2026-03-04)

**概要**: 自社文脈でのQA/QC活動整理、ドキュメントのミッション別分割・整理、認識すり合わせ用スライド初版作成。

**背景**: Session 1で調査した知識を自社の文脈に落とし込み、チームメンバーに共有するための資料を作成する必要があった。

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [session2/company-qa-qc-report.md](../session2/company-qa-qc-report.md) | 自社QA/QC活動整理レポート |
| [session2/mission-priority-analysis.md](../session2/mission-priority-analysis.md) | ミッション優先順位分析 |
| [session2/session-summary.md](../session2/session-summary.md) | セッションサマリー |
| [docs/index.md](../../docs/index.md) | ドキュメントインデックス |
| [docs/missions/m1〜m4/README.md](../../docs/missions/) | ミッション別README（4ファイル） |
| [docs/qa-knowledge/](../../docs/qa-knowledge/) | QA基礎知識・自社QA/QC（要約版2ファイル） |
| [docs/qa-overview-slide.md](../../docs/qa-overview-slide.md) | 認識すり合わせ用スライド（Marp形式・15ページ） |
| [docs/slide-workflow.md](../../docs/slide-workflow.md) | Marp→PPTXワークフロー整理 |
| [session3/session-plan.md](../session3/session-plan.md) | Session 3計画 |

**重要な発見**:
- 藤田さんの役割は「検査する人」ではなく「検査の仕組みを作る人（QA）」
- 品質管理フローはSQM→IQC→IPQC→OQCの4段階
- 現状はExcel管理 → タブレットアプリ化を検討
- チーム構成: 小笠原さん(課長)、石川さん(上司)、小板橋さん(先輩)、宇枝さん(部長)
- marp-slidesスキルを追加

## Session 3 (2026-03-04)

**概要**: 品質管理フロー図（SVG）を作成し、スライドに画像として埋め込み。M3/M4の要件初期ヒアリングを実施。

**背景**: Session 2で作成したスライドのASCIIアートフロー図を、本格的なSVG画像に置き換える必要があった。会話の中でフローが製品ごとに存在すること、M3/M4の方針が明らかになった。

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [docs/images/quality-flow.svg](../../docs/images/quality-flow.svg) | 品質管理フロー図（製品別フロー + DB横断構造） |
| [docs/images/quality-flow.drawio](../../docs/images/quality-flow.drawio) | フロー図の編集用ソース |
| [session3/m3m4-requirements-memo.md](../session3/m3m4-requirements-memo.md) | M3/M4要件の初期メモ |
| [session3/session-summary.md](../session3/session-summary.md) | セッションサマリー |
| [session4/session-plan.md](../session4/session-plan.md) | Session 4計画 |

**重要な発見**:
- 品質管理フローは**製品ごと**に存在する（AirGrow、H12G等）
- QA手法（M1/M2）は全製品共通、DB（M3/M4）は全製品データを集約
- M3/M4: kintone（社内導入済み）vs 自前開発（AI活用）の比較検討が必要
- M3/M4: 「プロトタイプ → 外注」が現実的なアプローチ
- 製品追加時にテーブルを増やさない設計が求められる
- 作業者は藤田さん一人 → シンプルさ・メンテ最小化が最優先
