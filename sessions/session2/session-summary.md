# Session 2 サマリー

**日付**: 2026-03-04
**セッション種別**: 知識整理・ドキュメント整備

---

## 実施内容

1. **自社文脈でのQA/QC活動の整理**
   - 部品購入型ドローンメーカーとしての品質管理フロー（SQM→IQC→IPQC→OQC）
   - QAとQCの違い、藤田さんの立ち位置の明確化（QA=仕組みを作る側）
   - 中国サプライヤの特有課題（サイレントチェンジ、ロット間ばらつき）
   - 品質Gの全活動をQA/QCにマッピング

2. **ドキュメントの整理**
   - ミッション別ディレクトリ構造（docs/missions/m1〜m4/）作成
   - Session 1のレポートをミッション別READMEに分割
   - QA基礎知識の要約版作成
   - 全体インデックス（docs/index.md）作成

3. **認識すり合わせ用スライド作成（初版）**
   - Marp形式で15ページのスライド作成
   - 用語定義、全体フロー、活動マッピング、ミッション説明、優先順位案を含む
   - Marp→PPTXのワークフローも整理

4. **ミッション優先順位の分析**
   - M3/M4先行→M1並行の推奨案を作成
   - ただし正式決定は次セッション以降

## 主な発見・判断

- 藤田さんの役割は「検査する人」ではなく「検査の仕組みを作る人（QA）」
- 現在の受入検査・工程不良はExcel管理 → タブレットアプリ化を検討
- marp-slidesスキルを新規追加（次セッションで活用）

## チーム構成の確認

| メンバー | 役職・立場 |
|----------|-----------|
| 小笠原さん | 課長 |
| 石川さん | 上司（品質Gメンバー） |
| 小板橋さん | 先輩（品質Gメンバー） |
| 宇枝さん | 部長（DB化・受入検査関連で見せる可能性） |
| 向後さん | ソフトチーム |

## 残った課題

- スライドの内容レビュー・修正（marp-slidesスキルを使って改善）
- 優先順位の正式決定（石川さんとの認識合わせ後）
- M3/M4: 現行Excel運用の詳細ヒアリング
- M1: 規格学習の開始

## 成果物

| ファイル | 内容 |
|----------|------|
| [company-qa-qc-report.md](company-qa-qc-report.md) | 自社QA/QC活動整理レポート |
| [mission-priority-analysis.md](mission-priority-analysis.md) | ミッション優先順位分析 |
| [../../docs/index.md](../../docs/index.md) | ドキュメントインデックス |
| [../../docs/missions/m1-sensor-evaluation/README.md](../../docs/missions/m1-sensor-evaluation/README.md) | M1: センサー評価 |
| [../../docs/missions/m2-pointcloud-verification/README.md](../../docs/missions/m2-pointcloud-verification/README.md) | M2: 点群検証 |
| [../../docs/missions/m3-incoming-inspection-db/README.md](../../docs/missions/m3-incoming-inspection-db/README.md) | M3: 受入検査DB |
| [../../docs/missions/m4-defect-db/README.md](../../docs/missions/m4-defect-db/README.md) | M4: 工程不良DB |
| [../../docs/qa-knowledge/qa-fundamentals.md](../../docs/qa-knowledge/qa-fundamentals.md) | QA基礎知識（要約版） |
| [../../docs/qa-knowledge/company-qa-qc.md](../../docs/qa-knowledge/company-qa-qc.md) | 自社QA/QC（要約版） |
| [../../docs/qa-overview-slide.md](../../docs/qa-overview-slide.md) | 認識すり合わせ用スライド（Marp） |
| [../../docs/slide-workflow.md](../../docs/slide-workflow.md) | スライド作成ワークフロー |

## 次セッションでやること

1. **marp-slidesスキルを使ってスライドを改善** — 内容レビュー・デザイン調整
2. **優先順位の正式決定** — スライドの内容を踏まえて
3. **M3/M4のStep 1着手** — 現行Excel運用のヒアリング計画
