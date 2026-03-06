# Session 31 計画

**目的**: M3ドキュメント整理（サブディレクトリ分類 + READMEインデックス化）

---

## タスク

| 優先度 | タスク | 詳細 |
|--------|--------|------|
| 1 | サブディレクトリ作成・ファイル移動 | as-is/, to-be/, hearing/ に分類 |
| 2 | README.mdをインデックス化 | 現在の内容は to-be/database-design.md に移動 |
| 3 | Session 25-28の内容をREADMEに反映 | 品質管理フレームワーク、Phase分け |

---

## 整理方針

### サブディレクトリ構成

```
m3-incoming-inspection-db/
├── README.md                    # インデックス + 概要
├── as-is/                       # 現状分析
│   ├── excel-review.md
│   ├── excel-analysis-summary.md
│   ├── as-is-model.drawio
│   ├── qa-gap-analysis.*
│   └── domain-modeling-approach.md
├── to-be/                       # あるべき姿・設計
│   ├── database-design.md       # 現READMEの内容
│   ├── analysis-what-to-build.md
│   ├── mockup-concepts.md
│   └── ears-*.md
└── hearing/                     # ヒアリング関連
    ├── hearing-items.md
    ├── hearing-items-integrated.md
    └── closed-questions-m3m4.*
```

### READMEに含めるべき内容

1. **概要**: M3の目的（1-2行）
2. **現状と課題**: Session 6, 24, 30で発見した問題
3. **品質管理フレームワーク**: IQC/PQC/OQC、8D、AQL（Session 25）
4. **Phase分け**: 分析→入力デジタル化→M3/M4統合（Session 25）
5. **ドキュメント一覧**: サブディレクトリごとのファイル一覧

---

## 参照資料

- [Session 30: 整理方針の検討](../session30/session-summary.md)
- [Session 25: 品質フレームワーク調査](../session25/quality-framework-research.md)
- [Session 6: Excelレビュー](../session6/excel-review.md)
