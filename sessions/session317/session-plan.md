# Session 317 計画

**目的**: 要求への立ち返り + ドメインモデリング

---

## 背景

- Session 316: kintoneマニュアル分析・CSV収集・ER図作成が完了
- 本来の要求（M3/M4/PSI）をほぼ忘れかけている状態
- 農業問合せは「市場不具合」でありM4「工程不良」ではない → データソース不明
- kintone設計者の背景（GENを残しながら現在の形にした理由）も整理したい

---

## やること

| # | 作業 | 内容 |
|---|------|------|
| 0 | **Session 316残課題の処理** | ER図・リレーション図の矢印追加、参照キー表更新、凡例追加、ファイル整理（R1〜R5） |
| 1 | **本来の要求（What）に立ち返って整理** | M3/M4/PSI各ミッションの要求を再確認。何を改善したいのか、達成したいことは何か |
| 2 | **ドメインモデリング** | kintoneのCSV構造からドメインモデルを抽出。設計意図のリバースエンジニアリング。管理されていない領域の特定 |
| 3 | **M4データソースの特定** | 工程不良の記録がどこにあるか整理（GEN？別Excel？未管理？） |
| 4 | **ヒアリング用の図と質問整理** | 全体フロー図をベースに「何で困っているか」「改善で達成したいことは何か」の質問リスト |
| 5 | **CSV削除** | 01_fukyuu_juchu, 03_seikyu_meisaiの分析完了後にCSV削除。gitの履歴にも残さない |

---

## 参照資料

| 資料 | 内容 |
|------|------|
| [session316/kintone-system-analysis.md](../session316/kintone-system-analysis.md) | マニュアル7冊の統合分析 |
| [session316/csv-analysis-summary.md](../session316/csv-analysis-summary.md) | CSV 1次分析結果 |
| [session316/confidence-update.md](../session316/confidence-update.md) | 確度更新 |
| [session316/diagrams/kintone-app-relations.drawio](../session316/diagrams/kintone-app-relations.drawio) | アプリ間リレーション図 |
| [session316/diagrams/kintone-er-diagram.drawio](../session316/diagrams/kintone-er-diagram.drawio) | ER図 |
| [session314/confidence-matrix.md](../session314/confidence-matrix.md) | 確度整理（元版） |

---

## 注意

- Session 316のCSV（01_fukyuu_juchu, 03_seikyu_meisai）は分析完了後に削除予定
- gitの履歴にも残さない

---

*作成: Session 316終了時 (2026-03-26)*
