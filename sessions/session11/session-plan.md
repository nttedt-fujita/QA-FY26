# Session 11 計画

**作成日**: 2026-03-05
**前セッション**: Session 10（モジュラーモノリス調査・スライド作成・ヒアリング準備）

---

## 今回やること

### 1. 石川さんとのヒアリング実施

- [hearing-sheet-p0.md](../session10/hearing-sheet-p0.md) を使用
- P0（大枠の認識合わせ）を中心に

**確認項目**:
- M3/M4の目的・ゴールの認識合わせ
- kintone vs AWS の方向性
- 今期中の最低限達成したい状態
- 一人運用についてのチーム認識
- データ量の感覚（月間入荷回数、組立台数等）
- 分析要件（SPC・パレート図の必要性）

### 2. ヒアリング結果の整理

- 石川さんの回答を [session7/ears-requirements-hypotheses.md](../session7/ears-requirements-hypotheses.md) の仮説と照合
- 仮説の検証結果を記録
- 新たに判明した要求を追加

### 3. 技術選定の判断（ヒアリング結果を踏まえて）

| 分岐 | 判断 |
|------|------|
| 「SPCは不要」と判明 | kintone単体も選択肢に |
| 「分析が重要」と判明 | AWS自前開発を進める |
| 「どちらでもよい」 | AWS自前開発（藤田さんのスキルアップも考慮） |

---

## 参照資料

### Session 10で作成した資料

| ファイル | 内容 |
|----------|------|
| [session10/ishikawa-slide-draft.md](../session10/ishikawa-slide-draft.md) | 石川さん向けスライド |
| [session10/hearing-sheet-p0.md](../session10/hearing-sheet-p0.md) | ヒアリングシート（印刷用） |
| [session10/modular-monolith-report.md](../session10/modular-monolith-report.md) | モジュラーモノリス調査 |

### その他

| ファイル | 内容 |
|----------|------|
| [session8/hearing-items-integrated.md](../session8/hearing-items-integrated.md) | ヒアリング項目統合版（全量） |
| [session7/ears-requirements-hypotheses.md](../session7/ears-requirements-hypotheses.md) | EARS形式要求仮説 |

---

## 背景

Session 10で以下を完了:
- モジュラーモノリス調査 → M3/M4に適切
- 石川さん向けスライド作成 → 16ページ、3案比較
- ヒアリングシート作成 → P0中心、印刷用

**次のステップ**: 実際にヒアリングを実施し、技術選定を決定する
