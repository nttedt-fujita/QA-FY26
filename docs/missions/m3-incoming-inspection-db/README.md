# M3: 受入検査データベース化

**担当**: ふじた
**ステータス**: プロトタイプ完成 → ヒアリング準備中

---

## プロトタイプ（Session 41-46で実装）

**ディレクトリ**: [prototype/](../../../prototype/)

### クイックスタート

```bash
cd prototype
make up              # DB + Backend起動
make frontend-dev    # Frontend起動（別ターミナル）
# http://localhost:3000 を開く
```

### 画面構成

| 画面 | URL | 機能 |
|------|-----|------|
| ロット登録 | `/` | 部品選択 → ロット登録 → 検査開始 |
| 検査入力 | `/inspection` | カウンター方式（OK/NG/SKIP） |
| 検査一覧 | `/records` | フィルター、CSVエクスポート |
| ダッシュボード | `/dashboard` | KPI、月別グラフ、分析 |

### ドキュメント

| ファイル | 内容 |
|----------|------|
| [quickstart.md](../../../prototype/docs/quickstart.md) | 起動手順 |
| [demo-guide.md](../../../prototype/docs/demo-guide.md) | デモ手順・ヒアリングポイント |
| [implementation-plan.md](../../../prototype/docs/implementation-plan.md) | 実装計画（Session別） |
| [ADR-001](../../../prototype/docs/adr/ADR-001-error-handling.md) | エラーハンドリング方針 |
| [ADR-002](../../../prototype/docs/adr/ADR-002-api-contract.md) | API契約とFE/BE整合性 |

### 技術スタック

- **Backend**: Go + net/http + pgx（PostgreSQL）
- **Frontend**: Next.js 15 + TypeScript + Tailwind CSS
- **DB**: PostgreSQL 16

---

## 概要

現在Excel管理されている受入検査データをデータベース化する。
品質管理フレームワーク（IQC/PQC/OQC）においては**IQC（受入品質管理）**に位置づけられる。

---

## 品質管理における位置づけ

```
[サプライヤ] → [IQC: 受入検査] → [製造工程] → [IPQC/OQC: 工程検査]
                     ↓                              ↓
                M3: 受入検査DB               M4: 工程不良DB
                     ↓                              ↓
                「この部品に問題」 ←────────→ 「この部品起因？」
```

### M3/M4の紐づけ

| 紐づきの軸 | M3での役割 | M4での役割 |
|-----------|-----------|-----------|
| **部品（品名）** | 検査対象 | 不良発生箇所 or 原因部品 |
| **ロット番号** | 入荷ロット | 使用部品のロット（トレース用） |
| **時系列** | 入荷日・検査日 | 不良発生日 |

---

## 技術方針

**kintone + 外部分析** を推奨

詳細: [to-be/platform-comparison.md](to-be/platform-comparison.md)

> **更新ルール**: 技術方針の変更はこのファイルを更新すること（sessions/は記録のみ）

---

## Phase分け

詳細: [to-be/prototype-approach.md](to-be/prototype-approach.md)

| Phase | 内容 | 状態 |
|-------|------|------|
| **Phase 1** | 分析・可視化（現行Excelデータ） | ✅ 完了 |
| **Phase 2** | 入力のデジタル化（ヒアリング後） | **プロトタイプ完成** |
| Phase 3 | M3/M4統合 + トレーサビリティ | 品質協定書締結後 |

> **Note**: Session 41-46でPhase 2のプロトタイプを先行実装。ヒアリングでフィードバックを得て修正する方針。

### 制約

| 項目 | 課題 |
|------|------|
| **品質協定書** | ナカヨ、渡辺製作所と未締結 |
| **M4の実効性** | 協定書なしでは委託先にツール使用を強制できない |
| **ロット概念** | 現行Excelにはロット概念がない（To-Beで導入必要） |

---

## 現行Excelの課題

Session 6, 24, 30で発見した主な問題:

| カテゴリ | 問題 |
|---------|------|
| **データ品質** | 未来日付10件、入荷日不明34件、矢印記号15件 |
| **構造的問題** | 検査基準書不足、判定基準が属人的、不良数量の記録不完全 |
| **設計上の欠落** | ロット概念なし、サプライヤID/検査員IDなし |

詳細: [as-is/excel-analysis-summary.md](as-is/excel-analysis-summary.md)

---

## ドキュメント一覧

### as-is/（現状分析）

| ファイル | 内容 |
|----------|------|
| [excel-review.md](as-is/excel-review.md) | Session 6: Excelレビュー（8シート構成の分析） |
| [excel-analysis-summary.md](as-is/excel-analysis-summary.md) | 分析結果サマリー（報告用、問題一覧） |
| [domain-modeling-approach.md](as-is/domain-modeling-approach.md) | As-Is/To-Be分離方針 |
| [as-is-model.drawio](as-is/as-is-model.drawio) | As-Is概念図（現行Excel構造） |
| [qa-gap-analysis.drawio](as-is/qa-gap-analysis.drawio) | 品質管理視点のギャップ分析図 |
| [qa-gap-analysis.svg](as-is/qa-gap-analysis.svg) | ギャップ分析図（SVG） |
| [qa-gap-analysis-slide.md](as-is/qa-gap-analysis-slide.md) | ギャップ分析スライド（Marp形式） |

### to-be/（あるべき姿・設計）

| ファイル | 内容 |
|----------|------|
| [platform-comparison.md](to-be/platform-comparison.md) | **技術方針**（kintone vs 自前開発） |
| [prototype-approach.md](to-be/prototype-approach.md) | **Phase分け方針**（M3/M4の進め方） |
| [ears-requirements-hypotheses.md](to-be/ears-requirements-hypotheses.md) | EARS形式要求仮説（Excel分析から導出） |
| [mockup-concepts.md](to-be/mockup-concepts.md) | 入力画面ワイヤーフレーム（**Phase 2用**） |

### hearing/（ヒアリング関連）

| ファイル | 内容 |
|----------|------|
| [hearing-items.md](hearing/hearing-items.md) | ヒアリング項目（P0〜P3優先度付き、統合版） |
| [closed-questions-m3m4.md](hearing/closed-questions-m3m4.md) | クローズド質問（M3/M4共通） |
| [closed-questions-m3m4.csv](hearing/closed-questions-m3m4.csv) | クローズド質問（CSV） |

---

## 関連リソース

### スキル

- [qa-design-review](.claude/skills/qa-design-review/SKILL.md) — M3/M4設計時の品質管理視点チェック

### 調査資料

- [Session 25: 品質フレームワーク調査](../../sessions/session25/quality-framework-research.md)
- [Session 25: プロトタイプ方針](../../sessions/session25/prototype-approach.md)

### ツール

- [tools/incoming_inspection/](../../tools/incoming_inspection/) — 分析ツール群
- [tools/README.md](../../tools/README.md) — ツール使用ガイド

---

## 品質管理フレームワーク参照

M3/M4に関連する品質管理の概念:

| 概念 | 説明 | M3/M4での適用 |
|------|------|--------------|
| **IQC/PQC/OQC** | 受入/工程/出荷の品質管理 | M3=IQC、M4=IPQC |
| **ロット管理** | 同条件で製造された単位 | To-Beで導入必要 |
| **AQL** | 抜取検査の合格判定基準 | 検査基準策定時に参照 |
| **8Dレポート** | 問題解決の8ステップ | M4の原因分析に適用 |
| **トレーサビリティ** | 製品の追跡可能性 | M3→M4の紐づけで実現 |

詳細: [Session 25: 品質フレームワーク調査](../../sessions/session25/quality-framework-research.md)

---

*更新日: 2026-03-09 (Session 47)*
