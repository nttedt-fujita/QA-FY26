# Session 262 サマリー

**日付**: 2026-03-19
**目的**: SIPOC作成準備・ヒアリング項目準備

---

## 実施内容

### 1. SIPOC手法の深掘り調査

- Step-by-Stepガイド（7ステップ）の確認
- 一般的な落とし穴（4つ）の整理
- SIPOC vs VSM（詳細プロセスマップ）の使い分け
- 2026年のベストプラクティス調査

**出典**: Web調査（isixsigma.com, Asana, LinkedIn等）

### 2. 既存QA知識との整合性確認

| QA概念 | SIPOCとの関連 |
|--------|-------------|
| ISO 9001 原則4（プロセスアプローチ） | SIPOCはプロセスアプローチの実践ツール |
| ISO 9001 原則6（客観的事実） | Inputs/Outputs を測定可能にする |
| IQC → IPQC → OQC フロー | SIPOCのProcessに対応 |
| ロット管理・トレーサビリティ | Suppliers → Inputs → Outputs → Customers の流れ |
| 8D問題解決（D2: 問題定義） | SIPOCで問題のスコープを明確化 |
| リーン・8つのムダ | SIPOCでムダの仮説を立てる → VSMで定量化 |

**結論**: SIPOCは既存QA知識と高い整合性があり、QAフレームワークの実践ツールとして位置づけられる。

### 3. SIPOCスキル作成

**スキル名**: `sipoc-facilitation`
**配置場所**: `~/.claude/skills/sipoc-facilitation/`

**構成**:
```
sipoc-facilitation/
├── SKILL.md              ← メインスキル（500行以下）
└── templates/
    ├── sipoc-template.drawio       ← 汎用テンプレート
    └── （プロジェクト固有は sessions/ に配置）
```

**含まれる内容**:
- Step-by-Stepガイド（7ステップ）
- 一般的な落とし穴（4つ）と対策
- SIPOC vs VSMの使い分け
- QA知識との接続（ISO 9001、IQC/IPQC/OQC、ロット管理等）
- ワークショップ実施チェックリスト

### 4. テンプレート作成

#### 汎用テンプレート

**ファイル**: `~/.claude/skills/sipoc-facilitation/templates/sipoc-template.drawio`

**特徴**:
- 5要素のカラムレイアウト（Suppliers, Inputs, Process, Outputs, Customers）
- 使い方ガイド付き
- 矢印で流れを示す

#### 受入検査用テンプレート

**ファイル**: [sipoc-iqc-template.drawio](sipoc-iqc-template.drawio)

**特徴**:
- Session 260の知見を反映（中国サプライヤ、梱包変更作業等）
- 「ヒアリング時に追加」枠を設置
- ワークショップでの確認ポイントをメモ欄に記載

### 5. ヒアリング項目準備

**ファイル**: [sipoc-workshop-hearing-items.md](sipoc-workshop-hearing-items.md)

**構成**:
- ワークショップの流れ（事前説明、SIPOC作成、追加項目洗い出し）
- 初期ヒアリング項目（Suppliers, Inputs, Process, Outputs, Customers別）
- SIPOC作成中にメモすべきこと（追加項目、認識のズレ、曖昧な返答）
- ワークショップ後のアクション

**重要な認識**:
- **これは完全なリストではない**
- **SIPOC作成中に追加ヒアリング項目が見えてくる**
- SIPOC作成の目的は「完璧な図を描く」ことではなく、「分からないことを洗い出す」こと

---

## 主な発見

### 1. SIPOCの正しい作成順序

❌ **誤り**: Suppliers（S）から順に埋める
✅ **正解**: Process（P）から開始 → Suppliers/Inputs → Outputs → Customers

**理由**: プロセスの枠組みを決めると、残りの要素が明確になる

### 2. SIPOC vs VSMの使い分け

| 観点 | SIPOC | VSM |
|------|-------|-----|
| **レベル** | 高レベル（30,000フィート） | 詳細レベル（地上） |
| **目的** | 全体俯瞰、スコープ定義 | ムダ・ボトルネック特定 |
| **使用タイミング** | Define フェーズ（最初） | Measure フェーズ（次） |
| **ステップ数** | 4-7ステップ | 数十ステップ（詳細） |
| **見えるもの** | 何を・誰に・何から | 待機時間、ムダ、リードタイム |

**プロセス改善の流れ**: SIPOC作成 → 問題領域特定 → VSMで深掘り → ムダ排除 → 未来マップ作成

### 3. 一般的な落とし穴（Common Mistakes）

| 落とし穴 | 問題 | 対策 |
|---------|------|------|
| 詳細レベルの誤り | 詳細不足 or 詳細過剰 | 4-7ステップに抑える |
| 古いデータの使用 | 最新の状態を反映していない | 現場に行って確認（Gemba Walk） |
| コラボレーション不足 | 一人で作成してしまう | 部門横断チームで作成 |
| 過度な単純化 | 複雑なプロセスを過度に単純化 | SIPOC → VSM の順で作成 |

### 4. QA知識との高い整合性

SIPOCは既存QA知識（ISO 9001、IQC/IPQC/OQC、ロット管理、8D、リーン等）と高い整合性があり、QAフレームワークの実践ツールとして位置づけられる。

---

## 成果物

| ファイル | 内容 |
|----------|------|
| [sipoc-methodology-deep-dive.md](sipoc-methodology-deep-dive.md) | SIPOC手法の深掘り調査レポート |
| `~/.claude/skills/sipoc-facilitation/SKILL.md` | SIPOCファシリテーションスキル |
| `~/.claude/skills/sipoc-facilitation/templates/sipoc-template.drawio` | 汎用SIPOCテンプレート |
| [sipoc-iqc-template.drawio](sipoc-iqc-template.drawio) | 受入検査用SIPOCテンプレート |
| [sipoc-workshop-hearing-items.md](sipoc-workshop-hearing-items.md) | SIPOCワークショップ用ヒアリング項目 |
| [pre-extracted-info-from-excel.md](pre-extracted-info-from-excel.md) | Excelから事前抽出した情報（⚠️要修正: 具体的根拠不足） |

---

## 残った課題

### pre-extracted-info-from-excel.md の修正が必要

**問題**:
- サプライヤー名、部品名等を記載しているが、Excelの具体的な根拠（行番号、列、数値、日付）が示されていない
- 抽象的すぎて、ワークショップで使いづらい

**修正方針（Session 263）**:
- Session 6のexcel-review.mdから具体例を引用
- 例: 「行27: 品番EX000934-02, 吐出口, 金森産業, 不良数=1」のように具体的に記載
- 検査工数の具体例を示す（「行XX: 検査工数15分」等）

### 今後の整理が必要な観点

**現状認識**:
- 課題解決対象の確認・状況把握フェーズ
- AI活用のアプローチをどこに適用するか、まだ整理が必要
- 分析ツール化の可能性（Excelデータ分析、工数内訳可視化等）

**次ステップ**:
1. SIPOCワークショップで現状把握を完了
2. VSM作成で詳細な工数内訳を測定
3. AI適用箇所の特定（員数確認、梱包変更作業、その他）
4. 分析ツール化の検討（Excelデータ可視化、工数分析等）

---

## 次セッションへの引き継ぎ

### Session 263: 田原さん・杉山さんとのSIPOC作成ワークショップ実施

**準備完了**:
- ✅ SIPOC手法の深掘り理解
- ✅ SIPOCスキル作成
- ✅ テンプレート準備（汎用+受入検査用）
- ✅ 初期ヒアリング項目リスト

**ワークショップで実施すること**:
1. SIPOCとは何か（簡単な説明）
2. Process（P）から開始してSIPOCを作成
3. **追加ヒアリング項目をメモ**（重要！）
4. 現場と管理者の認識のズレを発見

**ワークショップ後のアクション**:
1. SIPOCのデジタル化（Draw.io）
2. 追加ヒアリング項目の整理・優先順位付け
3. VSM作成の計画

---

## 学び・気づき

### 1. スキル作成の要否判断基準

**スキル化すべき理由**:
1. 繰り返し使用する可能性が高い
2. 正しい手順の遵守が重要
3. QA知識との統合が必要
4. 他のプロセスマッピング手法との使い分けが必要

### 2. SIPOCの本質

- ❌ 完璧な図を描くこと
- ✅ **現場と一緒に描きながら、分からないことを洗い出すこと**

### 3. プロセス改善の全体像

```
SIPOC（高レベル俯瞰）
  ↓
問題領域を特定
  ↓
VSM（詳細マッピング）
  ↓
ムダを特定・排除
  ↓
未来マップ作成
```

---

## 参照資料

### Web調査（出典）

- [SIPOC Diagram: Definition & How to Create One Fast [2026] • Asana](https://asana.com/resources/sipoc-diagram)
- [How to Create a SIPOC Diagram: Step-by-Step Guide - isixsigma.com](https://www.isixsigma.com/sipoc-copis/how-to-create-a-sipoc-diagram-step-by-step-guide/)
- [Common Mistakes When Using SIPOC - isixsigma.com](https://www.isixsigma.com/sipoc-copis/common-mistakes-when-using-sipoc/)
- [SIPOC and Process Maps: Why do you need both? | LinkedIn](https://www.linkedin.com/pulse/20141017124808-6000961-sipoc-and-process-maps-why-do-you-need-both)
- [A comprehensive guide to creating your first SIPOC diagram | Nulab](https://nulab.com/learn/project-management/comprehensive-guide-creating-first-sipoc-diagram/)

### プロジェクト内資料

- [Session 260: 田原さんヒアリング結果](../session260/hearing-results.md)
- [Session 261: ベストプラクティス調査](../session261/best-practices-research.md)
- [docs/qa-knowledge/qa-fundamentals.md](../../docs/qa-knowledge/qa-fundamentals.md)
- [docs/qa-knowledge/company-qa-qc.md](../../docs/qa-knowledge/company-qa-qc.md)
- [sessions/session25/quality-framework-research.md](../session25/quality-framework-research.md)

---

*作成: Session 262 (2026-03-19)*
