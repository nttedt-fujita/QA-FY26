# Session 246 計画

**目的**: M3/M4ドメインモデル（概念図）の新規作成

## 背景

- Session 245でto-be-model.drawioは「ER図」であることを確認
- 純粋なドメインモデル（概念図）がM3/M4とも欠如している
- ビジネス概念の理解・共有のためにドメインモデルが必要

## やること

| # | 作業 | 読むべきファイル | 参照コマンド |
|---|------|-----------------|--------------|
| 1 | M3ドメインモデル（概念図）の新規作成 | domain-model-vs-er-diagram.md | - |
| 2 | M4ドメインモデル（概念図）の新規作成 | M4 README.md | - |
| 3 | （任意）to-be-model.drawioのリネーム検討 | - | - |

## M3ドメインモデルに含めるべき内容

1. **エンティティと関係**（概念レベル）
   - サプライヤ、部品、ロット、検査記録、不良レポート、不問判定
   - PK/FKは書かない

2. **ビジネスルールの記述**
   - 「ロット = 入荷タイミング単位」
   - 「検査記録はロット単位で管理」
   - 「不問判定には承認者が必要」

3. **M4との紐づき**
   - lot_idで連携

## M4ドメインモデルに含めるべき内容

1. **エンティティ**
   - 工程不良記録

2. **不良コード体系（3階層）の表現**
   - Level 1: 大分類（EL/ME/SW/SE）
   - Level 2: 中分類
   - Level 3: 詳細

3. **原因コード体系（4M1E）の表現**
   - Man, Machine, Material, Method, Environment

4. **M3との紐づき**
   - lot_id → ロット

## 成果物

- `docs/missions/m3-incoming-inspection-db/domain/domain-model.drawio`（新規）
- `docs/missions/m4-defect-db/domain/domain-model.drawio`（新規）

## 参照

- [Session 245: ドメインモデル vs ER図解説](../session245/domain-model-vs-er-diagram.md)
- [to-be-model.drawio](../../docs/missions/m3-incoming-inspection-db/to-be/to-be-model.drawio)
- [M4 README](../../docs/missions/m4-defect-db/README.md)

---

*作成: Session 245 (2026-03-18)*
