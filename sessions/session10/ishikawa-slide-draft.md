---
marp: true
theme: default
size: 16:9
paginate: true
header: "M3/M4 技術方針ドラフト — 石川さん認識合わせ"
footer: "品質保証グループ ふじた | 2026-03-05"
style: |
  section {
    font-family: 'Yu Gothic', 'Hiragino Sans', 'Noto Sans JP', 'Meiryo', sans-serif;
    font-size: 20px;
    line-height: 1.4;
    padding: 40px;
  }
  h1 {
    font-size: 28px;
    color: #1a5276;
    border-bottom: 3px solid #2980b9;
    padding-bottom: 8px;
    text-align: center;
  }
  h2 {
    font-size: 22px;
    color: #2c3e50;
  }
  table {
    font-size: 17px;
  }
  .draft-warning {
    background: #fee2e2;
    padding: 15px;
    border-radius: 8px;
    border-left: 5px solid #ef4444;
    font-weight: bold;
    text-align: center;
  }
  .highlight {
    background: #fef3c7;
    padding: 15px;
    border-radius: 8px;
    border-left: 4px solid #f59e0b;
  }
  .decision-box {
    background: #dbeafe;
    padding: 15px;
    border-radius: 8px;
    border-left: 5px solid #2563eb;
  }
  .question-box {
    background: #ecfdf5;
    padding: 15px;
    border-radius: 8px;
    border-left: 5px solid #10b981;
  }
  .two-column {
    display: flex;
    gap: 20px;
  }
  .content-box {
    flex: 1;
    background: #f8fafc;
    padding: 15px;
    border-radius: 8px;
  }
  .pro {
    color: #059669;
  }
  .con {
    color: #dc2626;
  }
---

# M3/M4 技術方針ドラフト
## 石川さん認識合わせ

**品質保証グループ ふじた**

<div class="draft-warning">

⚠️ **これは仮の計画です** — ヒアリング結果次第で変わります

</div>

---

# この資料の目的

<div class="two-column">
<div class="content-box">

## 共有したいこと

1. M3/M4の技術的な選択肢
2. 調査した結果の要約
3. 現時点での仮の方針

</div>
<div class="content-box">

## 確認したいこと

1. 方針の認識に齟齬がないか
2. 懸念点・追加の視点
3. 次のステップ（ヒアリング）の進め方

</div>
</div>

<div class="highlight">

**重要**: 最終判断は**ヒアリング→プロトタイプ→評価**の後に行います

</div>

---

# 技術選択肢: 3つの案

| 案 | 概要 | 特徴 |
|----|------|------|
| **A. kintone単体** | 既存kintoneでDB化 | 最も簡単・ただし分析に限界 |
| **B. kintone + 外部分析** | kintone + Python/QuickSight | 入力と分析が分離 |
| **C. AWS自前開発** | PWA + APIサーバ + DB | 自由度高い・保守は自分 |

---

# 案A: kintone単体

<div class="two-column">
<div class="content-box">

## できること

- ✅ データ入力（フォーム）
- ✅ 一覧表示・検索
- ✅ 基本グラフ（棒・円・折れ線）
- ✅ タブレット対応

</div>
<div class="content-box">

## できないこと

- ❌ **SPC（管理図）**
- ❌ **パレート図**
- ❌ 高度な統計分析
- ❌ 最小値・最大値の集計

</div>
</div>

<div class="highlight">

**結論**: 記録・閲覧だけなら十分。**分析が必要なら不十分**

</div>

---

# 案B: kintone + 外部分析

```
┌─────────────┐      API      ┌─────────────────┐
│   kintone   │ ───────────→  │  Python/QuickSight │
│  （入力）    │  データ取得   │    （分析）        │
└─────────────┘              └─────────────────┘
       ↑                           ↓
    ユーザー                   グラフ・レポート
    （入力）                   （別画面で見る）
```

<div class="two-column">
<div class="content-box">

## <span class="pro">メリット</span>

- kintone部分はサイボウズ任せ
- 分析は自由に組める

</div>
<div class="content-box">

## <span class="con">デメリット</span>

- 入力画面と分析画面が**分離**
- 結局Python部分は自分で保守
- API制限（1日10,000リクエスト）

</div>
</div>

---

# 案C: AWS自前開発

```
┌────────────────────────────────────────┐
│           PWAアプリ（モジュラーモノリス）  │
│  ┌──────────┐    ┌──────────────────┐ │
│  │  入力     │    │  分析ダッシュボード  │ │
│  │  フォーム  │    │  ・管理図          │ │
│  │          │    │  ・パレート図       │ │
│  └──────────┘    └──────────────────┘ │
│           ↓              ↑             │
│         ┌─────────────────┐           │
│         │      DB         │           │
│         └─────────────────┘           │
└────────────────────────────────────────┘
```

<div class="two-column">
<div class="content-box">

## <span class="pro">メリット</span>

- 入力と分析が**統合**
- 分析ロジックを自由に組み込める
- 将来の拡張も自由

</div>
<div class="content-box">

## <span class="con">デメリット</span>

- 初期開発コストが高い
- **全部自分で保守**
- 一人運用リスク

</div>
</div>

---

# 比較表

| 観点 | A. kintone単体 | B. kintone + 分析 | C. AWS自前開発 |
|------|---------------|------------------|---------------|
| **初期開発** | ◎ 短い | ○ 中程度 | △ 長い |
| **分析機能** | △ 基本のみ | ○ Python次第 | ◎ 何でも可 |
| **UI統合** | ◎ | △ 分離 | ◎ 統合 |
| **保守範囲** | ◎ サイボウズ任せ | △ 分析は自分 | △ 全部自分 |
| **月額コスト** | 1,800円/人 | 同左 + α | ~3,750円 |
| **カスタマイズ** | △ 限定的 | △ 限定的 | ◎ 自由 |

---

# 調査で分かったこと

## 1. 技術的なポイント

| 調査項目 | 結果 |
|---------|------|
| **kintoneの分析機能** | SPC・管理図・パレート図は標準では不可 |
| **kintoneのAPI** | 1日10,000リクエスト、1回500件取得が上限 |
| **AWS QuickSight** | 計算フィールド + 参照線で管理図も実装可能 |
| **モジュラーモノリス** | M3/M4の規模には十分。マイクロサービスは過剰 |

## 2. 言語選択

| 調査項目 | 結果 |
|---------|------|
| **TypeScript vs Go** | 言語選択より**アーキテクチャ選択が先** |
| **TypeScript 7.0** | コンパイラがGoで書き直されただけ。実行時性能は同じ |

---

# 現時点の仮の方針

<div class="decision-box">

**分析が肝なら案C（AWS自前開発）が適切**

</div>

## 理由

1. **M3/M4の目的は「分析による品質改善」** — 記録だけなら今のExcelで十分
2. **入力と分析が統合**されていないと使いづらい
3. **kintone + Pythonでも結局Python部分は自分で保守** — 保守コスト削減効果が薄い
4. **AWS経験は個人のスキルアップにもなる**（クラウドの実務経験）

---

# ただし「案Cありき」ではない

<div class="highlight">

**最終判断はヒアリング後**に行います

</div>

## ヒアリングで確認すること

| # | 確認項目 | 判断への影響 |
|---|----------|------------|
| 1 | **SPC・管理図は本当に必要？** | 不要なら案Aで済む可能性 |
| 2 | 現場でどんな分析をしたい？ | 分析要件の具体化 |
| 3 | タブレット入力の頻度・場所は？ | オフライン対応の要否 |
| 4 | 誰が使う？頻度は？ | ユーザー数・負荷の見積もり |

---

# 懸念点と対策

<div class="two-column">
<div class="content-box">

## 懸念: 一人運用リスク

**対策**:
- ドキュメント整備（ADR、設計書）
- シンプルな構成を維持
- 外注可能なスコープを明確化
- 保守計画を事前に合意

</div>
<div class="content-box">

## 懸念: 開発期間

**対策**:
- **段階的リリース**: まずMVP（最小機能）
- プロトタイプでフィードバック
- 完璧を目指さない

</div>
</div>

<div class="two-column">
<div class="content-box">

## 懸念: 藤田さんが異動したら？

**対策**:
- 「作った人が一生保守」を避ける設計
- 外注に出せるドキュメント整備
- 保守計画を組織決定として記録

</div>
<div class="content-box">

## 懸念: kintoneで良いのでは？

**回答**:
- 記録だけならkintoneで十分
- **分析が必要ならkintoneでは不十分**
- ヒアリングで要件を明確にしてから判断

</div>
</div>

---

# 保守計画（案）

<div class="decision-box">

**保守を「誰かの善意」に依存しない仕組みを作る**

</div>

## 事前合意が必要な項目

| # | 合意項目 | 内容 |
|---|----------|------|
| 1 | **保守責任者** | 品質G全体 or 藤田さん固定？ |
| 2 | **保守工数枠** | 年間どのくらいの工数を見込む？ |
| 3 | **外注予算** | 外部に出す場合の予算は？ |
| 4 | **引き継ぎ条件** | 藤田さん異動時の引き継ぎ方法 |
| 5 | **廃止条件** | 使われなくなったら誰が廃止判断？ |

---

# 次のステップ

<div class="question-box">

## 石川さんに確認したいこと

1. この方針（ヒアリング → プロトタイプ → 最終判断）でOKか？
2. ヒアリング対象は誰？（現場で入力している人）
3. 懸念点・追加の視点はあるか？
4. 保守計画は一緒に決めていきたい

</div>

## 直近のアクション

| # | アクション | 担当 |
|---|-----------|------|
| 1 | **この方針について認識合わせ**（今回） | 藤田・石川 |
| 2 | 現行Excel運用ヒアリング | 藤田 |
| 3 | 要求仮説の検証・整理 | 藤田 |
| 4 | プロトタイプ開発（案C前提で着手） | 藤田 |

---

# 補足: アーキテクチャ用語

## モジュラーモノリスとは？

| 特徴 | 説明 |
|------|------|
| **単一デプロイ** | 全機能を1つのアプリとしてデプロイ |
| **モジュール分離** | 内部は機能ごとに分離（受入検査 / 工程不良 / 分析） |
| **マイクロサービスとの違い** | 別々のサーバに分けない → 運用がシンプル |
| **M3/M4に適する理由** | 規模が小さい、一人運用、コスト重視 |

<div class="highlight">

**結論**: M3/M4にはモジュラーモノリスが適切。マイクロサービスは過剰。

</div>

---

# まとめ

<div class="two-column">
<div class="content-box">

## 技術方針（仮）

- **案C: AWS自前開発**を軸に検討
- ただし最終判断は**ヒアリング後**
- **分析が不要**と判明したら案Aに変更も

</div>
<div class="content-box">

## 次のステップ

1. この認識合わせ
2. 現場ヒアリング
3. プロトタイプ開発
4. 最終判断

</div>
</div>

<div class="draft-warning">

⚠️ **これは仮の計画です** — ヒアリング結果次第で変わります

</div>
