# M3/M4ドメインモデリング再確認ノート（中間）

**作成日**: 2026-03-18（Session 244）
**ステータス**: 中間ドキュメント（次セッションで継続）

---

## 1. 確認した過去の決定事項

### M3/M4の位置づけ

| ミッション | 品質管理での位置づけ | 役割 |
|-----------|-------------------|------|
| M3 | IQC（受入品質管理） | 受入検査記録 |
| M4 | IPQC（工程品質管理） | 工程不良記録 |

### M3/M4の紐づけ

| 紐づきの軸 | M3での役割 | M4での役割 |
|-----------|-----------|-----------|
| **部品（品名）** | 検査対象 | 不良発生箇所 or 原因部品 |
| **ロット番号** | 入荷ロット | 使用部品のロット（トレース用） |
| **時系列** | 入荷日・検査日 | 不良発生日 |

### ロット概念の定義

- **現状**: 現行Excelにはロット概念がない
- **方針**: **入荷タイミングをロットとして定義する**
- **スキーマ**: `lots`テーブルに`arrival_date`（入荷日）を持つ

---

## 2. ドメインモデル vs ER図（整理必要）

**次セッションで詳細確認が必要**

| 観点 | ドメインモデル | ER図 |
|------|---------------|------|
| 目的 | ビジネス概念を整理 | DB設計を表現 |
| 抽象度 | 高い（概念レベル） | 低い（実装レベル） |
| 内容 | 「何を扱うか」 | 「どう保存するか」 |

**このプロジェクトでの対応**:
- `to-be-model.drawio` → ドメインモデル（概念）
- `schema.sql` → ER図相当（テーブル定義）

**確認すべきこと**:
- to-be-model.drawioの中身がドメインモデルなのかER図なのか
- 両者の関係を明確化
- 解説ドキュメントを作成すべきか

---

## 3. 関連ファイル一覧

### ドメインモデル関連

| ファイル | 内容 |
|----------|------|
| [to-be-model.drawio](../../docs/missions/m3-incoming-inspection-db/to-be/to-be-model.drawio) | To-Beドメインモデル（Session 33作成） |
| [as-is-model.drawio](../../docs/missions/m3-incoming-inspection-db/as-is/as-is-model.drawio) | As-Is概念図 |
| [qa-gap-analysis.drawio](../../docs/missions/m3-incoming-inspection-db/as-is/qa-gap-analysis.drawio) | ギャップ分析図 |

### 実装（スキーマ）

| ファイル | 内容 |
|----------|------|
| [schema.sql](../../prototype/m3/db/schema.sql) | M3プロトタイプDBスキーマ |

---

## 4. 気づき・TODO

### created_at / updated_at の追加

**ユーザー指摘**: 必要なところに`created_at`/`updated_at`がない可能性

→ **次セッションでスキーマを確認し、必要なテーブルに追加を検討**

### AI検査システムとの連携

- AI検査は**IQC（受入検査）**の効率化に位置
- AI検査結果を**ロット単位**で記録
- 不良発生時のトレースバックに使用

→ **次セッションで連携設計を継続**

---

## 5. 次セッションでやること

1. **to-be-model.drawio**を開いて中身を確認
2. ドメインモデルとER図の関係を整理・解説作成
3. created_at / updated_at の追加箇所を確認
4. AI検査システムとM3の連携設計

---

## 参照資料

| 資料 | 場所 |
|------|------|
| M3 README | [docs/missions/m3-incoming-inspection-db/README.md](../../docs/missions/m3-incoming-inspection-db/README.md) |
| M4 README | [docs/missions/m4-defect-db/README.md](../../docs/missions/m4-defect-db/README.md) |
| Session 243 AI検査要件 | [ai-inspection-requirements-draft.md](../session243/ai-inspection-requirements-draft.md) |
| Session 243 QAフレームワーク考慮 | [qa-framework-considerations.md](../session243/qa-framework-considerations.md) |

---

*作成: Session 244 (2026-03-18)*
