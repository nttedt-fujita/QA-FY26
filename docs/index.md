# QA-FY26 ドキュメントインデックス

**最終更新**: 2026-03-09 (Session 47)

---

## M3 プロトタイプ（Session 41-46で実装）

**ディレクトリ**: [prototype/](../prototype/)

受入検査DBのプロトタイプ。カウンター方式の検査入力、ダッシュボード等を実装。

### クイックスタート

```bash
cd prototype
make up              # DB + Backend起動
make frontend-dev    # Frontend起動（別ターミナル）
# http://localhost:3000 を開く
```

### ドキュメント

| ファイル | 内容 |
|----------|------|
| [quickstart.md](../prototype/docs/quickstart.md) | 起動手順 |
| [demo-guide.md](../prototype/docs/demo-guide.md) | デモ手順・ヒアリングポイント |
| [implementation-plan.md](../prototype/docs/implementation-plan.md) | 実装計画（Session別） |

### 設計判断（ADR）

| ADR | 内容 |
|-----|------|
| [ADR-001](../prototype/docs/adr/ADR-001-error-handling.md) | エラーハンドリング方針 |
| [ADR-002](../prototype/docs/adr/ADR-002-api-contract.md) | API契約とFE/BE整合性 |

---

## ミッション別ドキュメント

| ミッション | 担当 | 概要 | ディレクトリ |
|-----------|------|------|-------------|
| **M1** | ふじた | [AirGrow搭載センサー評価手法策定（Lidar/GNSS）](missions/m1-sensor-evaluation/README.md) | `missions/m1-sensor-evaluation/` |
| **M2** | ふじた | [点群データ検証方法策定](missions/m2-pointcloud-verification/README.md) | `missions/m2-pointcloud-verification/` |
| **M3** | ふじた | [受入検査データベース化](missions/m3-incoming-inspection-db/README.md) | `missions/m3-incoming-inspection-db/` |
| **M4** | ふじた | [工程不良データベース化](missions/m4-defect-db/README.md) | `missions/m4-defect-db/` |

---

## 【集約】M1/M2 進捗・調査資料（Session 10-12）

### 最新の集約ドキュメント

| ドキュメント | 内容 | 更新Session |
|-------------|------|-------------|
| **[schedule-and-m1m2-summary-v2.md](../sessions/session12/schedule-and-m1m2-summary-v2.md)** | スケジュール・M1/M2進め方の集約（PPTX転記用） | Session 12 |

### M1-A: Lidar評価

| ドキュメント | 内容 | Session |
|-------------|------|---------|
| [lidar-evaluation-research.md](../sessions/session12/lidar-evaluation-research.md) | Lidar評価環境Web調査（規格・国内サービス・機材・アプローチ3案） | Session 12 |
| [missions/m1-sensor-evaluation/README.md](missions/m1-sensor-evaluation/README.md) | M1詳細（指標・試験方法・規格） | Session 2 |

**現状**: 評価環境ゼロ。外部委託 / 自社構築 / ハイブリッドの3案を検討中。

### M1-B: GNSS評価

| ドキュメント | 内容 | Session |
|-------------|------|---------|
| [gnss-hearing-items.md](../sessions/session12/gnss-hearing-items.md) | 小板橋さん向けGNSSヒアリング項目（16問） | Session 12 |
| [missions/m1-sensor-evaluation/README.md](missions/m1-sensor-evaluation/README.md) | M1詳細（指標・試験方法・規格） | Session 2 |

**現状**: 小板橋さんが既存知見を持っている。ヒアリング待ち。

### M2: 点群データ検証

| ドキュメント | 内容 | Session |
|-------------|------|---------|
| [m1m2-current-status-and-next-steps.md](../sessions/session11/m1m2-current-status-and-next-steps.md) | M1/M2現状整理・次セッション方針 | Session 11 |
| [missions/m2-pointcloud-verification/README.md](missions/m2-pointcloud-verification/README.md) | M2詳細（品質指標・検証ツール） | Session 2 |

**現状**: 別プロジェクトでFalse Alarm低減作業を実施中。これを整理→検証手法として体系化する方針。

---

## 【集約】M3/M4 技術検討資料（Session 5-10）

### 技術選定・比較

| ドキュメント | 内容 | Session |
|-------------|------|---------|
| [platform-comparison.md](../sessions/session5/platform-comparison.md) | プラットフォーム比較表（kintone/AWS/ハイブリッド） | Session 5 |
| [kintone-vs-aws-report.md](../sessions/session9/kintone-vs-aws-report.md) | kintone vs AWS自前開発 詳細比較 | Session 9 |
| [modular-monolith-report.md](../sessions/session10/modular-monolith-report.md) | モジュラーモノリス調査（M3/M4への適用評価） | Session 10 |

### AWS自前開発の詳細検討（別セッション資料）

| ドキュメント | 内容 | Session |
|-------------|------|---------|
| [01_architecture_discussion.md](../sessions/session8/another-session-Need-requirements-files/01_architecture_discussion.md) | アーキテクチャ議論（PWA, モジュラーモノリス等） | Session 8統合 |
| [02_data_growth_analysis.md](../sessions/session8/another-session-Need-requirements-files/02_data_growth_analysis.md) | データ膨張予測（3シナリオ） | Session 8統合 |
| [05_techstack_plan.md](../sessions/session8/another-session-Need-requirements-files/05_techstack_plan.md) | 技術スタック・開発計画 | Session 8統合 |

### コスト・保守

| ドキュメント | 内容 | Session |
|-------------|------|---------|
| [aws-cost-estimation.md](../sessions/session5/aws-cost-estimation.md) | AWS構成案・コスト試算（3パターン） | Session 5 |
| [maintenance-strategy.md](../sessions/session5/maintenance-strategy.md) | 保守戦略（保守範囲・外注スコープ） | Session 5 |

### ヒアリング・要件

| ドキュメント | 内容 | Session |
|-------------|------|---------|
| [hearing-items-integrated.md](../sessions/session8/hearing-items-integrated.md) | ヒアリング項目統合版（約40項目、優先度付き） | Session 8 |
| [ears-requirements-hypotheses.md](../sessions/session7/ears-requirements-hypotheses.md) | EARS形式 要求仮説19個 | Session 7 |
| [excel-review.md](../sessions/session6/excel-review.md) | 現行Excel全8シートのレビュー・課題抽出 | Session 6 |

### 分析・可視化

| ドキュメント | 内容 | Session |
|-------------|------|---------|
| [spc-control-chart-guide.md](../sessions/session9/spc-control-chart-guide.md) | SPC・管理図・パレート図 解説 | Session 9 |
| [quicksight-report.md](../sessions/session9/quicksight-report.md) | AWS QuickSight 調査レポート | Session 9 |

### 言語・技術調査

| ドキュメント | 内容 | Session |
|-------------|------|---------|
| [typescript-vs-go-report.md](../sessions/session9/typescript-vs-go-report.md) | TypeScript vs Go 比較レポート | Session 9 |
| [kintone-evaluation.md](../sessions/session5/kintone-evaluation.md) | kintone調査・評価レポート | Session 5 |

---

## スライド・発表資料

| ドキュメント | 内容 | Session |
|-------------|------|---------|
| **[schedule-and-m1m2-summary-v2.md](../sessions/session12/schedule-and-m1m2-summary-v2.md)** | M1/M2スライド用（最新） | Session 12 |
| [ishikawa-slide-draft.md](../sessions/session10/ishikawa-slide-draft.md) | M3/M4技術方針ドラフト（PPTX更新用） | Session 10 |
| [hearing-agenda-slide.md](../sessions/session10/hearing-agenda-slide.md) | ヒアリング・相談アジェンダ | Session 10 |

---

## QA基礎知識

| ドキュメント | 内容 | 作成Session |
|-------------|------|-------------|
| [QA基礎知識](qa-knowledge/qa-fundamentals.md) | QA/QCの違い、ISO 9001、AQL、FMEA等 | Session 1 |
| [自社QA/QC活動の整理](qa-knowledge/company-qa-qc.md) | 部品購入型メーカーとしての品質管理体制 | Session 2 |

---

## 画像・図表

| ファイル | 内容 | Session |
|----------|------|---------|
| [images/quality-flow.svg](images/quality-flow.svg) | 品質管理フロー図（製品別フロー + DB横断構造） | Session 3 |

---

## ディレクトリ構成

```
prototype/                        ← M3プロトタイプ（Session 41-46）
├── Makefile                      ← 操作コマンド（make help）
├── makefiles/                    ← .mkファイル群
├── backend/                      ← Go API
├── frontend/                     ← Next.js
├── db/                           ← PostgreSQL初期化
└── docs/                         ← プロトタイプ関連ドキュメント

docs/
├── index.md                      ← このファイル
├── missions/
│   ├── m1-sensor-evaluation/     ← M1: センサー評価
│   ├── m2-pointcloud-verification/ ← M2: 点群検証
│   ├── m3-incoming-inspection-db/  ← M3: 受入検査DB
│   └── m4-defect-db/             ← M4: 工程不良DB
├── qa-knowledge/                 ← QA基礎知識
└── images/                       ← 図表

sessions/
├── session-history/              ← セッション履歴（10件ごと）
└── session1/ 〜 session47/       ← 各セッション資料
```

---

## セッション別資料インデックス

### Session 10 (2026-03-05)

| ファイル | 内容 |
|----------|------|
| [modular-monolith-report.md](../sessions/session10/modular-monolith-report.md) | モジュラーモノリス調査レポート |
| [ishikawa-slide-draft.md](../sessions/session10/ishikawa-slide-draft.md) | 技術方針ドラフト（16ページ） |
| [hearing-agenda-slide.md](../sessions/session10/hearing-agenda-slide.md) | ヒアリングアジェンダ（7ページ） |
| [hearing-sheet-p0.md](../sessions/session10/hearing-sheet-p0.md) | ヒアリング用シート（印刷用） |

### Session 11 (2026-03-05)

| ファイル | 内容 |
|----------|------|
| [schedule-and-m1m2-summary.md](../sessions/session11/schedule-and-m1m2-summary.md) | スケジュール・M1/M2進め方（v1） |
| [m1m2-current-status-and-next-steps.md](../sessions/session11/m1m2-current-status-and-next-steps.md) | M1/M2現状整理・次セッション方針 |

### Session 12 (2026-03-05)

| ファイル | 内容 |
|----------|------|
| [schedule-and-m1m2-summary-v2.md](../sessions/session12/schedule-and-m1m2-summary-v2.md) | スケジュール・M1/M2進め方（v2、最新） |
| [gnss-hearing-items.md](../sessions/session12/gnss-hearing-items.md) | 小板橋さん向けGNSSヒアリング項目 |
| [lidar-evaluation-research.md](../sessions/session12/lidar-evaluation-research.md) | Lidar評価環境Web調査 |
