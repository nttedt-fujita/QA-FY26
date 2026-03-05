# Session 8 確認資料インデックス

**目的**: 次セッションで参照しやすいように、今回確認した資料の内容を一覧化

---

## 過去セッション資料

### Session 5

| 資料 | 主な内容 | キーワード |
|------|---------|-----------|
| [platform-comparison.md](../session5/platform-comparison.md) | kintone vs kintone+外部分析 vs AWS自前開発の3案比較 | 比較表、コスト、M3/M4適合度、**推奨: kintone+外部分析** |
| [hearing-plan.md](../session5/hearing-plan.md) | ヒアリング計画21問 | A1-A4(運用), B1-B4(Excel構成), C1-C4(検査項目), D1-D4(活用), E1-E4(工程不良), F1-F4(不良管理), G1-G5(共通) |
| [aws-cost-estimation.md](../session5/aws-cost-estimation.md) | AWS構成案・コスト試算（3パターン） | サーバレス、Lightsail、月額$5-$25 |
| [kintone-evaluation.md](../session5/kintone-evaluation.md) | kintone調査・評価 | ユーザー課金、SPC不可、API連携 |
| [maintenance-strategy.md](../session5/maintenance-strategy.md) | 保守戦略 | 保守範囲、外注スコープ、合意項目7つ |
| [domain-modeling-guide.md](../session5/domain-modeling-guide.md) | ドメインモデリング準備ガイド | ユビキタス言語、エンティティ |

### Session 6

| 資料 | 主な内容 | キーワード |
|------|---------|-----------|
| [excel-review.md](../session6/excel-review.md) | 現行Excel全8シートの客観的レビュー | 567件、17ヶ月、横断的課題8項目、表記揺れ87パターン |
| [session-summary.md](../session6/session-summary.md) | 一人運用問題、技術選定議論 | **結論: 技術選定は分析要件が見えてから** |

### Session 7

| 資料 | 主な内容 | キーワード |
|------|---------|-----------|
| [ears-requirements-hypotheses.md](../session7/ears-requirements-hypotheses.md) | EARS形式 要求仮説19個 | R-01〜R-19、確認済み6件、事実ベース6件、仮説5件 |

---

## 別セッション資料（session8/another-session-Need-requirements-files/）

| ファイル | 主な内容 | キーワード |
|----------|---------|-----------|
| [01_architecture_discussion.md](another-session-Need-requirements-files/01_architecture_discussion.md) | アーキテクチャ判断の記録 | PWAファースト、**モジュラーモノリス**、TypeScript統一の理由、RDS選定理由 |
| [02_data_growth_analysis.md](another-session-Need-requirements-files/02_data_growth_analysis.md) | データ膨張予測（3シナリオ） | 保守的/成長/急成長、10年累計30万〜3500万件、**ヒアリング項目G-1〜G-16** |
| [03_discussion_log.md](another-session-Need-requirements-files/03_discussion_log.md) | 議論ログ・検討メモ | 要件整理、ロット定義3案、ツール選定5案 |
| [04_domain_model_prep.md](another-session-Need-requirements-files/04_domain_model_prep.md) | ドメインモデル設計準備 | **ヒアリング項目A〜F**、コアエンティティ9個、「ヒアリング不要」判定 |
| [05_techstack_plan.md](another-session-Need-requirements-files/05_techstack_plan.md) | 技術スタック・開発計画 | Next.js, Lambda, RDS PostgreSQL, Cognito, **Phase 0〜5開発計画** |

---

## キーワード別 逆引き

| 調べたいこと | 参照する資料 |
|-------------|-------------|
| **kintone vs AWS自前開発の比較** | S5/platform-comparison.md |
| **コスト試算** | S5/aws-cost-estimation.md, S5/kintone-evaluation.md |
| **現行Excelの課題** | S6/excel-review.md |
| **要求仮説（EARS形式）** | S7/ears-requirements-hypotheses.md |
| **ヒアリング項目（統合版）** | S8/hearing-items-integrated.md |
| **データ膨張予測** | 別S/02_data_growth_analysis.md |
| **アーキテクチャ判断（PWA, モジュラーモノリス等）** | 別S/01_architecture_discussion.md |
| **技術スタック詳細** | 別S/05_techstack_plan.md |
| **ドメインモデル・エンティティ** | 別S/04_domain_model_prep.md |
| **開発計画（Phase 0〜5）** | 別S/05_techstack_plan.md |
| **保守戦略** | S5/maintenance-strategy.md |
| **一人運用問題** | S6/session-summary.md |

---

## 未解決・要調査の懸念点

| 懸念点 | 関連資料 | 調査内容 |
|--------|---------|---------|
| TypeScript vs Go言語 | 別S/01_architecture_discussion.md | パフォーマンス比較、バグ発生率、TS 7.0のGoコンパイラ |
| モジュラーモノリスとは | 別S/01_architecture_discussion.md | 定義・メリット・デメリットの説明が必要 |
| QuickSightで何ができるか | 別S/05_techstack_plan.md | 分析・可視化の実装範囲に影響 |
| 工程不良のExcel | S6/excel-review.md（受入検査のみ） | 工程不良側のExcelは未確認 |
