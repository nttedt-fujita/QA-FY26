# Session 245 サマリー

**日付**: 2026-03-18
**目的**: to-be-model.drawio確認 + ドメインモデル/ER図の整理 + AI検査連携設計

---

## 実施内容

1. **to-be-model.drawioの確認**
   - 中身は「ER図相当」であり、純粋なドメインモデル（概念図）ではない
   - PK/FK、カラム詳細まで含まれている

2. **schema.sqlとの整合性確認**
   - 8テーブル全て一致
   - created_at/updated_atは全テーブルに存在（waivers除く）
   - 図には技術詳細（created_at等）が未記載だが、これは許容範囲

3. **M3/M4図の状況確認**
   - M3: as-is-model.drawio、to-be-model.drawio、qa-gap-analysis.drawio がある
   - M4: ドメインモデル図は存在しない（READMEにコード体系のみ）

4. **ドメインモデル vs ER図の解説ドキュメント作成**
   - 違いを整理
   - Living Documentation観点での整理方針を含む
   - 再作成の方針を提案

5. **AI検査連携設計のたたき台作成**
   - 3つのアプローチを提案（案A/B/C）
   - プロトタイプ段階では案A（既存テーブル拡張）を推奨

---

## 主な発見

| 発見 | 詳細 |
|------|------|
| to-be-model.drawio = ER図 | 名前は「モデル」だが実態はER図 |
| ドメインモデル（概念図）が欠如 | M3/M4とも純粋な概念図がない |
| created_at/updated_at | 全テーブルに存在（waivers除く） |
| 致命的な整合性問題はなし | schema.sqlとto-be-model.drawioは整合 |

---

## 作成ファイル

| ファイル | 内容 |
|----------|------|
| [domain-model-vs-er-diagram.md](domain-model-vs-er-diagram.md) | ドメインモデル vs ER図の解説、整理方針 |
| [ai-inspection-m3-integration-draft.md](ai-inspection-m3-integration-draft.md) | AI検査とM3連携設計のたたき台 |

---

## 残った課題

| 課題 | 優先度 | 次アクション |
|------|:------:|-------------|
| M3ドメインモデル（概念図）作成 | 高 | 次セッションで図作成 |
| M4ドメインモデル作成 | 高 | M3の後で着手 |
| to-be-model.drawioのリネーム | 中 | er-diagram.drawioに変更 |
| AI連携設計の詳細化 | 中 | ヒアリング後 |

---

## 次セッションでやること

1. **M3ドメインモデル（概念図）の新規作成**
   - エンティティと関係（概念レベル）
   - ビジネスルールの記述
   - to-be-model.drawioとは別ファイル

2. **M4ドメインモデルの新規作成**
   - 工程不良記録のエンティティ
   - 不良コード体系の表現
   - M3との紐づき

---

## 参照

- [Session 244: ドメインモデリング再確認](../session244/domain-model-review-notes.md)
- [to-be-model.drawio](../../docs/missions/m3-incoming-inspection-db/to-be/to-be-model.drawio)
- [schema.sql](../../prototype/m3/db/schema.sql)

---

*作成: Session 245 (2026-03-18)*
