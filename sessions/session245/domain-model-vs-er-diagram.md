# ドメインモデル vs ER図：解説と整理方針

**作成日**: 2026-03-18（Session 245）
**ステータス**: 解説ドキュメント

---

## 1. ドメインモデルとER図の違い

### 定義

| 観点 | ドメインモデル | ER図（Entity-Relationship Diagram） |
|------|--------------|-----------------------------------|
| **目的** | ビジネス概念の理解・共有 | DB実装の設計 |
| **読む人** | 開発者 + 業務担当者 + ステークホルダー | 開発者のみ |
| **抽象度** | 高い（概念レベル） | 低い（実装レベル） |
| **内容** | エンティティ + 関係 + ビジネスルール | テーブル + カラム + PK/FK |

### 書くべき内容の違い

| 要素 | ドメインモデル | ER図 |
|------|:-------------:|:----:|
| エンティティ名 | ✅ | ✅ |
| ビジネス上重要な属性 | ✅ | ✅ |
| 技術的属性（created_at等） | ❌ | ✅ |
| PK/FK | ❌ | ✅ |
| カーディナリティ（1:N等） | ✅（意味を説明） | ✅（記号で表記） |
| ビジネスルール | ✅ | ❌ |
| データ型 | ❌ | ✅ |

### ドメインモデルを作るなら

**良い例**（概念レベル）:
```
┌─────────────┐     入荷される     ┌─────────────┐
│   部品      │ ←─────────────── │   ロット    │
│             │      1 : N        │             │
│ ・品名      │                   │ ・入荷日    │
│ ・カテゴリ  │                   │ ・数量      │
└─────────────┘                   └─────────────┘

【ビジネスルール】
- ロットは「入荷タイミング」を単位とする
- 同じ部品でも入荷日が異なれば別ロット
```

**悪い例**（ER図をドメインモデルと呼んでいる）:
```
parts                    lots
├─ part_id (PK)          ├─ lot_id (PK)
├─ name                  ├─ part_id (FK)
├─ created_at            ├─ arrival_date
└─ updated_at            └─ created_at
```

---

## 2. 現状のM3/M4図の状況

### M3（受入検査DB）

| ファイル | 実際の内容 | 問題点 |
|----------|-----------|--------|
| [as-is-model.drawio](../../docs/missions/m3-incoming-inspection-db/as-is/as-is-model.drawio) | 現行Excelの構造図 | 概念図として作成 |
| [to-be-model.drawio](../../docs/missions/m3-incoming-inspection-db/to-be/to-be-model.drawio) | **ER図相当** | 名前は「モデル」だが実態はER図 |
| [qa-gap-analysis.drawio](../../docs/missions/m3-incoming-inspection-db/as-is/qa-gap-analysis.drawio) | ギャップ分析図 | 問題なし |
| [schema.sql](../../prototype/m3/db/schema.sql) | DBスキーマ | to-be-model.drawioと整合 |

### M4（工程不良DB）

| ファイル | 実際の内容 | 問題点 |
|----------|-----------|--------|
| ドメインモデル図 | **存在しない** | READMEにコード体系のみ |
| schema.sql | **存在しない** | プロトタイプ未実装 |

### to-be-model.drawioとschema.sqlの整合性

| 観点 | 状況 |
|------|------|
| エンティティ数 | ✅ 一致（8テーブル） |
| 属性 | ⚠️ created_at/updated_atは図に未記載（許容範囲） |
| FK関係 | ✅ 一致 |

**結論**: 致命的な整合性問題はない。ただしドメインモデル（概念図）が欠如。

---

## 3. Living Documentation観点での整理方針

### 原則

> **Living Documentation**: ドキュメントはコードと同期し、常に最新を保つ

| 分類 | 住処 | 更新タイミング |
|------|------|---------------|
| **ドメインモデル** | docs/missions/mX/domain/ | ビジネス概念が変わったとき |
| **ER図** | docs/missions/mX/to-be/ | スキーマが変わったとき |
| **スキーマ** | prototype/mX/db/schema.sql | 実装時 |

### 同期ルール

```
ドメインモデル（概念）
    ↓ 具体化
ER図（設計）
    ↓ 実装
schema.sql（コード）
    ↓ 逆引き
ER図（自動生成可能）
```

**推奨**: スキーマからER図を自動生成するツールを使えば、ER図の手動メンテは不要になる。

### 整理の優先度

| 優先度 | アクション | 理由 |
|:------:|-----------|------|
| 高 | M3ドメインモデル（概念図）を新規作成 | ビジネス理解の共有に必須 |
| 高 | M4ドメインモデルを新規作成 | M4はまだ設計段階 |
| 中 | to-be-model.drawioを「ER図」にリネーム | 名前と実態の不一致を解消 |
| 低 | ER図の自動生成検討 | 長期的なメンテコスト削減 |

---

## 4. 再作成の方針（次セッション以降）

### M3ドメインモデル新規作成

**含めるべき内容**:
1. エンティティと関係（概念レベル）
2. ビジネスルールの記述
   - 「ロット = 入荷タイミング単位」
   - 「検査記録はロット単位で管理」
   - 「不問判定には承認者が必要」
3. M4との紐づき（lot_id）

**含めない内容**:
- created_at / updated_at
- PK / FK
- データ型

### M4ドメインモデル新規作成

**含めるべき内容**:
1. 工程不良記録のエンティティ
2. 不良コード体系（3階層）の表現
3. 原因コード体系（4M1E）の表現
4. M3との紐づき（lot_id → ロット）

### ファイル構成案

```
docs/missions/m3-incoming-inspection-db/
├── domain/
│   └── domain-model.drawio    ← 新規作成（概念図）
├── to-be/
│   └── er-diagram.drawio      ← リネーム（旧to-be-model.drawio）
└── as-is/
    └── as-is-model.drawio     ← 現状維持

docs/missions/m4-defect-db/
├── domain/
│   └── domain-model.drawio    ← 新規作成（概念図）
└── to-be/
    └── er-diagram.drawio      ← スキーマ設計時に作成
```

---

## 5. 用語整理

| 用語 | 定義 | 例 |
|------|------|-----|
| **ドメインモデル** | ビジネス概念とその関係を表現した図 | 「ロット」「検査記録」の関係 |
| **ER図** | データベースのテーブル設計を表現した図 | lots、inspection_recordsテーブル |
| **スキーマ** | 実際のSQL DDL | schema.sql |
| **概念モデル** | ドメインモデルの別名 | 同上 |
| **論理モデル** | ER図の別名（DB独立） | 同上 |
| **物理モデル** | 特定DBに依存したスキーマ | SQLite用schema.sql |

---

## 参照

- [to-be-model.drawio](../../docs/missions/m3-incoming-inspection-db/to-be/to-be-model.drawio) — 現状のM3 ER図
- [schema.sql](../../prototype/m3/db/schema.sql) — M3 DBスキーマ
- [M4 README](../../docs/missions/m4-defect-db/README.md) — M4概要・コード体系

---

*作成: Session 245 (2026-03-18)*
