# Session 40 サマリー

**日時**: 2026-03-09
**目的**: m3-research-filesを踏まえた方針見直しの具体化

---

## 実施内容

### 1. 特性要因図の作成

「なぜ検査に300hかかるのか」を4M（Man, Machine, Method, Material）で構造化。

**成果**: [fishbone-diagram.md](./fishbone-diagram.md)

| 区分 | 確認済み | 仮説（一次調査で検証） |
|------|---------|----------------------|
| Man | 判定基準が属人的 | 検査員の技能差、教育の仕組み |
| Machine | 記録がExcel+紙混在、入力作業の手間 | 検査設備の効率 |
| Method | 検査基準書の不足、サプライヤー層別管理なし | 抜取検査計画の根拠、付帯作業の多さ |
| Material | 部品種類222種、品質協定書が未締結 | サプライヤー品質の実績 |

### 2. プロトタイプ設計見直し

m3-research-files（07, 08, 09）とprototype-implementation-plan.mdを比較し、ギャップを分析。

**成果**: [prototype-redesign.md](./prototype-redesign.md)

**推奨**: Option B（一次調査優先）
- プロトタイプを一旦凍結
- Googleフォームで即日開始可能
- 抜取検査計画見直しで即効性のある成果（20-30%削減）

**理由**:
1. 一次調査なしでツールを作っても的外れになる可能性大
2. MustのほとんどはGoogleフォーム+スプレッドシートで対応可能
3. データが貯まってからツール設計した方が精度が高い

### 3. 一次調査チェックリスト作成

02_investigation_checklist.mdと特性要因図を統合し、現場用に整理。

**成果**: [field-investigation-checklist.md](./field-investigation-checklist.md)

| 優先度 | 確認項目 |
|--------|---------|
| P0 | 300hの内訳、検査員の人数・体制 |
| P1 | 抜取検査計画の根拠、付帯作業の内容 |
| P2 | 品質実績、サプライヤー管理 |

---

## 作成ファイル

| ファイル | 内容 |
|----------|------|
| [fishbone-diagram.md](./fishbone-diagram.md) | 特性要因図（4M分析） |
| [prototype-redesign.md](./prototype-redesign.md) | プロトタイプ設計見直し |
| [field-investigation-checklist.md](./field-investigation-checklist.md) | 一次調査チェックリスト（現場用） |

---

## 重要な決定

1. **プロトタイプは一旦凍結** — 一次調査の結果を待つ
2. **一次調査を優先** — 特性要因図の仮説を検証してから設計
3. **即効施策を先に** — 抜取検査計画見直し + Googleフォームで即日開始

---

## 確認事項（ユーザーへ）

1. **Option B（一次調査優先）で進めてよいか？**
2. **一次調査のヒアリング対象者は誰か？**
3. **ヒアリングの時期・スケジュール感は？**

---

## 次セッション（Session 41）でやること

1. ユーザー確認事項の回答を受けて方針確定
2. ヒアリングの具体的な準備（日時調整、当日の進め方）
3. Googleフォームの設計（必要に応じて）

---

## 参照

- [m3-research-key-points.md](../session39/m3-research-key-points.md) — 調査資料の重要ポイント
- [review-direction.md](../session39/review-direction.md) — Session 39の方針整理
- [08_lean_improvement_proposals.md](../session39/m3-research-files/08_lean_improvement_proposals.md) — 提案D（推奨）
