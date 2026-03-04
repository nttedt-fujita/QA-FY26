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

## Session 4 (2026-03-04)

**概要**: ミッション特化スライド作成、Need（要求）の明確化、M3/M4の開発進め方を別資料として整理。

**背景**: 全体スライド（qa-overview-slide.md）とは別に、藤田さん担当のM1〜M4に絞ったスライドが必要だった。また、M3/M4の開発サイクル・フォーマット・ドメインモデリングなどの進め方を事前に整理する方針となった。

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [docs/fujita-mission-slide.md](../../docs/fujita-mission-slide.md) | ミッション特化スライド（11ページ、Need+M1〜M4+スケジュール） |
| [docs/missions/m3m4-development-approach.md](../../docs/missions/m3m4-development-approach.md) | M3/M4の進め方（開発サイクル・フィードバック・ドメインモデリング・保守） |
| [session4/session-summary.md](../session4/session-summary.md) | セッションサマリー |
| [session5/session-plan.md](../session5/session-plan.md) | Session 5計画 |

**重要な発見**:
- Need（変えられないもの）を1つに絞る:「自社製品の品質を定量的に保証できる体制を作る」
- M1〜M4はすべてHow（要件）、Needとの紐づけを明示して方針説明に使う
- DB設計の前にドメインモデリング（ユビキタス言語→モデル→DB設計）が必要
- AWS使用希望、コスト試算が必要
- 保守問題: 設計段階で保守計画・範囲を合意すべき（「作った人が一生保守」を回避）

## Session 5 (2026-03-04)

**概要**: M3/M4のアーキテクチャ検討。AWS構成・コスト試算、kintone調査・評価、プラットフォーム比較表、保守戦略、ヒアリング計画、ドメインモデリング準備を実施。

**背景**: Session 4でNeedと開発の進め方が定まったため、技術選定の判断材料を揃える必要があった。kintone vs AWS自前開発の比較、保守体制の設計、現場ヒアリングの準備が主な課題。

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [session5/aws-cost-estimation.md](../session5/aws-cost-estimation.md) | AWS構成案・コスト試算（3パターン比較） |
| [session5/kintone-evaluation.md](../session5/kintone-evaluation.md) | kintone調査・評価レポート |
| [session5/platform-comparison.md](../session5/platform-comparison.md) | プラットフォーム比較表（3案比較+推奨） |
| [session5/maintenance-strategy.md](../session5/maintenance-strategy.md) | 保守戦略（保守範囲・外注スコープ・合意項目） |
| [session5/hearing-plan.md](../session5/hearing-plan.md) | 現行Excel運用ヒアリング計画（質問リスト21問） |
| [session5/domain-modeling-guide.md](../session5/domain-modeling-guide.md) | ドメインモデリング学習・準備ガイド |
| [session5/session-summary.md](../session5/session-summary.md) | セッションサマリー |

**重要な発見**:
- **推奨: kintone+外部分析(Python)の段階的アプローチ**
  - Phase 1: kintoneで記録・閲覧（数日で稼働）
  - Phase 2: Python分析追加（SPC・パレート）
  - Phase 3: 必要に応じて自前開発に移行
- AWS自前開発: サーバレス構成で月額~$25(~3,750円)、Lightsailなら~$5(~750円)
- kintone: スタンダード1,800円/ユーザー/月、SPC/管理図は不可→Python連携必須
- 保守工数: kintone+外部分析で年20-40時間、自前開発で年100-200時間
- 石川さんと合意すべき保守計画7項目を整理
- ヒアリングがドメインモデリングの入力になる → ヒアリング→モデリング→DB設計の順
- **藤田さんの所感**: 一人運用前提がそもそもおかしい、kintoneでも保守は実質無理、自前開発はクラウド経験として価値あり、保守をきちんと設計したい

## Session 6 (2026-03-04)

**概要**: 一人運用問題の議論、技術選定トレードオフの継続議論、現行Excel（受入検査作業集計.xlsx）の客観的レビュー・課題抽出を実施。

**背景**: Session 5で技術選定の判断材料は揃えたが、藤田さんの「一人運用前提がおかしい」という問題意識と、技術選定の前に分析要件を明確にする必要があった。石川さんとの短い会話で新情報も得ていた。

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [session6/excel-review.md](../session6/excel-review.md) | 現行Excel全8シートの客観的レビュー・課題抽出 |
| [session6/session-summary.md](../session6/session-summary.md) | セッションサマリー |

**重要な発見**:
- **技術選定は分析要件が見えてから** — ヒアリング→ドメインモデリング→技術選定の順
- **lot_idはHow（要件）であってWhat（要求）ではない** — 紐付けの手段は現場に聞いて決める
- **現行Excelは「記録はあるがデータになっていない」** — 構造化されていないため分析不可能
- 横断的課題8項目: 列構成の不統一、検査数量の表記混在（87パターン）、作業者名・サプライヤ名の表記揺れ、不良数量の記録不完全、備考列への情報混在、検査基準書の不足、不具合発生シートの情報不足
- 全567件/約17ヶ月、1日1〜2件ペースの記録
- 一人運用リスク: 負のループ回避が藤田さんの主な動機。リスク文書化+属人化しない設計で自分側でコントロールできる部分を固める方針
