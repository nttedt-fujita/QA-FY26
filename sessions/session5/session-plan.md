# Session 5 計画

**前セッション**: Session 4 (2026-03-04)

## 目的

M3/M4のアーキテクチャ検討（AWS構成・保守戦略）とドメインモデリング準備

## やること

### 1. AWS構成案・コスト試算

- M3/M4のインフラ候補を具体化（RDS, Lambda, S3, etc.）
- ランニングコストの概算を出す
- kintoneとのコスト比較（ライセンス費 vs AWS費）

### 2. 保守戦略の検討

- 一人体制で持続可能な運用とは何か
- マネージドサービスによる保守最小化
- 外注する場合のスコープ・予算感
- 設計段階で石川さんと合意すべき保守計画の項目整理

### 3. 現行Excel運用のヒアリング計画

- 誰に何を聞くか具体化
- ヒアリングシート（質問リスト）の準備
- 受入検査・工程不良のそれぞれで聞くべき項目

### 4. ドメインモデリングの学習・準備

- DDD（ドメイン駆動設計）の基本概念の理解
- ユビキタス言語収集の進め方
- 品質管理ドメインに適用する場合のポイント

### 5. kintone調査

- 社内でのkintone利用状況の確認方法
- M3/M4の要件（lot_id紐付け、パレート分析等）をkintoneで実現できるか

## 参照資料

- [session4/session-summary.md](../session4/session-summary.md) — Session 4サマリー
- [docs/missions/m3m4-development-approach.md](../../docs/missions/m3m4-development-approach.md) — M3/M4の進め方
- [docs/fujita-mission-slide.md](../../docs/fujita-mission-slide.md) — ミッション特化スライド
- [session3/m3m4-requirements-memo.md](../session3/m3m4-requirements-memo.md) — M3/M4要件メモ
