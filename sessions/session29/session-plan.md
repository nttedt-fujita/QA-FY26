# Session 29 計画

**目的**: データ改善 + スライド用資料作成

---

## 背景

Session 28で以下を完成:
- 新スキル「qa-design-review」
- CLAUDE.mdへの誘導記述
- 品質管理視点のギャップ分析図

未対応のまま残っていたタスク:
- 矢印記号（↓↑）の集計処理（Session 24で発見、未実装）
- ギャップ分析図のスライド組み込み

---

## やること

### 1. 矢印記号（↓↑）の集計処理追加（優先度: 高）

**背景**: Session 24で発見した問題。工数列に「↓」「↑」（上の行と同じの意味）が15件あり、数値として集計されていない。

**対応**:
- `tools/incoming_inspection/data_cleaner.py` に矢印記号の変換ロジックを追加
- 「↓」「↑」を上の行の値で置換
- テストコード追加（TDD）

**参照**:
- [Session 24: session-summary.md](../session24/session-summary.md) — 問題発見
- [Session 6: excel-review.md](../session6/excel-review.md) — 矢印記号の説明

### 2. ギャップ分析図をスライド用MDに変換（優先度: 高）

**背景**: Session 28で作成したギャップ分析図（Draw.io）を、パワポ資料に組み込むためMarkdownに変換。

**対応**:
- Draw.io → SVG/PNG エクスポート
- スライド用Markdownに埋め込み
- 既存のパワポ資料に追加

**参照**:
- [Session 28: qa-gap-analysis.drawio](../session28/qa-gap-analysis.drawio)

### 3. 小笠原さん報告資料の最終確認

- パワポの「次のアクション」セクションを更新（矢印記号の修正ステータス）
- ギャップ分析図の追加

---

## 成果物（予定）

| ファイル | 内容 |
|----------|------|
| `tools/incoming_inspection/data_cleaner.py`（更新） | 矢印記号の変換ロジック追加 |
| `tools/tests/incoming_inspection/test_data_cleaner.py`（更新） | テスト追加 |
| スライド用MD or パワポ（更新） | ギャップ分析図追加 |

---

## 参照資料

- [Session 28: ギャップ分析図](../session28/qa-gap-analysis.drawio)
- [Session 24: 分析レビュー](../session24/session-summary.md)
- [Session 6: Excelレビュー](../session6/excel-review.md)
- [tools/incoming_inspection/data_cleaner.py](../../tools/incoming_inspection/data_cleaner.py)
