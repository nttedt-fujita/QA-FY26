# Session 245 計画

**目的**: to-be-model.drawio確認 + ドメインモデル/ER図の整理 + AI検査連携設計

## 背景

- Session 244でM3/M4ドメインモデリングの過去決定事項を確認
- ドメインモデルとER図の違いが曖昧 → 整理が必要
- created_at / updated_at の追加が必要な箇所を確認

## やること

| # | 作業 | 読むべきファイル | 参照コマンド |
|---|------|-----------------|--------------|
| 1 | to-be-model.drawioの中身を確認 | to-be/to-be-model.drawio | - |
| 2 | ドメインモデル vs ER図の関係を整理 | - | - |
| 3 | created_at/updated_at の追加箇所を確認 | schema.sql | - |
| 4 | AI検査システムとM3の連携設計 | domain-model-review-notes.md | - |

## 確認ポイント

### to-be-model.drawio

- 何が描かれているか（概念モデル？ER図？）
- 品質管理フロー（IQC→PQC→OQC）の表現
- ロット概念の表現

### created_at / updated_at

- 現在のschema.sqlで付いているテーブル
- 追加が必要なテーブル

### AI検査連携

- AI検査結果をどのテーブルに記録するか
- ロットとの紐づけ方

## 成果物

- ドメインモデルとER図の関係解説ドキュメント
- created_at/updated_at 追加計画
- AI検査システム連携設計（仮）

## 参照

- [session244/domain-model-review-notes.md](../session244/domain-model-review-notes.md)
- [to-be-model.drawio](../../docs/missions/m3-incoming-inspection-db/to-be/to-be-model.drawio)
- [schema.sql](../../prototype/m3/db/schema.sql)

---

*作成: Session 244 (2026-03-18)*
