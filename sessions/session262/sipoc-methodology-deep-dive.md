# SIPOC手法の深掘り調査

**作成日**: 2026-03-19 (Session 262)
**目的**: SIPOC作成の具体的なプロセス、ベストプラクティス、QA知識との整合性を整理

---

## 1. SIPOC手法の概要

### 1.1 定義（再確認）

**SIPOC** = **S**uppliers（供給者）、**I**nputs（入力）、**P**rocess（プロセス）、**O**utputs（出力）、**C**ustomers（顧客）の5要素をマッピングするプロセス可視化ツール

**起源**: Six Sigma DMAIC の「Define（定義）」フェーズで使用される高レベルプロセスマップ

**位置づけ**: 詳細なプロセスマップ（VSM、swimlane diagram等）を作成する前の「30,000フィートからの俯瞰図」

**出典**:
- [SIPOC Diagram: Definition & How to Create One Fast [2026] • Asana](https://asana.com/resources/sipoc-diagram)
- [What is SIPOC in Six Sigma? Everything You Need to Know - SixSigma.us](https://www.6sigma.us/process-mapping/sipoc-six-sigma/)

---

## 2. SIPOC作成の正式な手順

### 2.1 Step-by-Step Guide

#### **Step 1: プロセスの定義**
- 対象プロセスの範囲・境界を明確にする
- プロセスの開始点と終了点を決定
- スコープを絞り込む（「受入検査全体」ではなく「部品Aの受入検査」など）

#### **Step 2: ステークホルダーの招集**
- 内部・外部のステークホルダーを招集
- **必須参加者**: 現場オペレーター、保守担当者（第一線知識）
- 部門横断チームの構成（Six Sigma の原則）

#### **Step 3: プロセス（P）から開始**
- **重要**: 「Suppliers（S）」が最初だが、実際には「Process（P）」から始めるのがベストプラクティス
- プロセスを4-7ステップ程度の高レベル工程に分解
- 詳細な作業指示書レベルは含めない（高レベルを維持）

**例（受入検査）**:
1. 部品の入荷
2. 検査準備（梱包確認）
3. 検査実施（外観・員数・機能）
4. 記録作成
5. 入庫/不合格処理

#### **Step 4: 供給者（S）と入力（I）の特定**
- 各プロセスに何を提供するか（Inputs）を洗い出す
- その提供者（Suppliers）を特定
- 供給者の信頼性・一貫性を評価

**例（受入検査）**:
- Suppliers: 中国サプライヤA、国内サプライヤB、物流業者
- Inputs: 部品、梱包材、納品書、検査指示書

#### **Step 5: 出力（O）の定義**
- プロセスが生成する全ての出力を特定
- **可能な限り測定可能・観察可能**にする
- 明確な定義により「成功とは何か」を全員が理解

**例（受入検査）**:
- Outputs: 検査記録、合格ロット、不合格報告書、入庫指示

#### **Step 6: 顧客（C）の特定**
- 出力を受け取る全ての顧客・部署を特定
- 内部顧客（組立担当、倉庫担当等）と外部顧客を区別

**例（受入検査）**:
- Customers: 組立担当、倉庫担当、品質管理担当、経営層（レポート）

#### **Step 7: レビューと検証**
- ドラフト完成後、プロセスステークホルダー全員で検証
- 正確性・完全性を確認

**出典**:
- [How to Create a SIPOC Diagram: Step-by-Step Guide - isixsigma.com](https://www.isixsigma.com/sipoc-copis/how-to-create-a-sipoc-diagram-step-by-step-guide/)
- [A comprehensive guide to creating your first SIPOC diagram | Nulab](https://nulab.com/learn/project-management/comprehensive-guide-creating-first-sipoc-diagram/)

---

### 2.2 重要なプリンシプル

#### **High-Level Perspective（高レベル視点）**
- 細部に飛び込む前に全体像を把握
- SIPOCの力はシンプルさにある → 細かい詳細を含める衝動に抵抗する
- 4-7ステップ程度のプロセスに抑える

#### **Sticky Notes Before Digital（デジタル化の前に付箋）**
- 最初はホワイトボードに付箋で作成
- 配置変更・修正が簡単
- 議論を活性化

#### **Iterative Process（反復プロセス）**
- 一度で完成しない
- ステークホルダーとの議論を通じて改善

**出典**: [What is a SIPOC Diagram? Step-by-Step Guide with Templates | Creately](https://creately.com/guides/sipoc-diagram/)

---

## 3. 一般的な落とし穴（Common Mistakes）

### 3.1 詳細レベルの誤り

**問題**: 詳細が不足 or 詳細すぎ
- 不足: 高レベルすぎて意味がない
- 過剰: 作業指示書レベルまで書く

**適切なバランス**:
- プロセス: 4-7ステップ程度
- 高レベル概要として機能する範囲

**出典**: [Common Mistakes When Using SIPOC - isixsigma.com](https://www.isixsigma.com/sipoc-copis/common-mistakes-when-using-sipoc/)

---

### 3.2 古いデータの使用

**問題**: 最新の状態を反映していない
- 古いメトリクス、誤ったデータ
- 現状を反映していない情報

**対策**:
- 現場に行って確認（Gemba Walk）
- 直近のデータを使用
- ステークホルダーと最新状態を確認

---

### 3.3 コラボレーション不足

**問題**: 一人で作成してしまう
- SIPOCはグループ活動であり、ソロ活動ではない
- コラボレーションにより全員が貢献し、実際のプロセスを正確に反映

**対策**:
- 部門横断チームで作成
- オペレーターと保守担当者を必ず含める
- ワークショップ形式で一緒に描く

---

### 3.4 複雑なプロセスの過度な単純化

**問題**: 複雑なプロセスを過度に単純化し、誤解や重要な詳細の見落とし
- 線形シーケンスでの描写は、反復的・循環的な性質を反映しない

**対策**:
- SIPOCは高レベルツール → 複雑な詳細が必要な場合は詳細プロセスマップ（VSM等）を使用
- SIPOC → VSM の順で作成

**出典**: [Common Mistakes When Using SIPOC - isixsigma.com](https://www.isixsigma.com/sipoc-copis/common-mistakes-when-using-sipoc/)

---

## 4. SIPOC vs 詳細プロセスマップ

### 4.1 役割の違い

| 観点 | SIPOC | 詳細プロセスマップ（VSM等） |
|------|-------|---------------------------|
| **レベル** | 高レベル（30,000フィート） | 詳細レベル（地上） |
| **目的** | 全体俯瞰、スコープ定義 | ムダ・ボトルネック特定 |
| **使用タイミング** | Define フェーズ（最初） | Measure フェーズ（次） |
| **ステップ数** | 4-7ステップ | 数十ステップ（詳細） |
| **見えるもの** | 何を・誰に・何から | 待機時間、ムダ、リードタイム |

**出典**:
- [SIPOC and Process Maps: Why do you need both? | LinkedIn](https://www.linkedin.com/pulse/20141017124808-6000961-sipoc-and-process-maps-why-do-you-need-both)
- [SIPOC: Why do I need a SIPOC and a Process Map? - 100% Effective](https://www.100pceffective.com/need-sipoc-process-map/)

---

### 4.2 両方が必要な理由

**SIPOC**:
- プロセスが何であるか、いつ始まりいつ終わるか（スコープ）
- 誰が影響を受けるか（ステークホルダー）
- 何を測定すべきか（Inputs/Outputs）
- プロジェクトで何をするか（コミュニケーション）

**詳細プロセスマップ（VSM等）**:
- どこで欠陥が発生しているか
- どこで遅延が起きているか
- どこでリワークループが発生しているか
- 根本原因の追跡

**プロセス改善の流れ**:
1. SIPOC作成 → 全体像把握
2. 問題領域を特定
3. その領域を詳細にマッピング（VSM等）
4. ムダを特定・排除
5. 未来マップ作成

**出典**: [Guide to High-Level Process Mapping (SIPOC) | Juran Institute](https://www.juran.com/blog/guide-to-high-level-process-mapping-sipoc/)

---

## 5. SIPOCとQA知識（既存ドキュメント）の整合性

### 5.1 QA基礎知識との接続

#### **ISO 9001原則4: プロセスアプローチ**（[qa-fundamentals.md](../../docs/qa-knowledge/qa-fundamentals.md) より）
- 業務をプロセス単位で管理・改善
- **→ SIPOCはプロセスアプローチの実践ツール**

#### **ISO 9001原則6: 客観的事実に基づく意思決定**
- データと情報に基づいて判断
- **→ SIPOCの Inputs/Outputs は測定可能にする**

#### **QA vs QC**
- QA（品質保証）= ユーザー視点・全ライフサイクル
- QC（品質管理）= 現場視点・工程ごと
- **→ SIPOCはQA視点（全体俯瞰）、VSMはQC視点（工程詳細）**

---

### 5.2 品質管理フローとの接続

#### **IQC → IPQC → OQC のフロー**（[quality-framework-research.md](../../sessions/session25/quality-framework-research.md) より）

```
サプライヤ → [IQC] → 製造工程 → [IPQC/PQC] → [FQC] → [OQC] → 顧客
              ↑          ↑              ↑          ↑
           受入検査    工程内検査      最終検査    出荷検査
             M3          M4
```

**SIPOCとの接続**:
- **Suppliers**: サプライヤ（部品供給者）
- **Inputs**: 部品、梱包材、納品書
- **Process**: IQC（受入検査）→ IPQC（工程検査）→ OQC（出荷検査）
- **Outputs**: 検査記録、合格/不合格報告
- **Customers**: 次工程（組立）、経営層（レポート）

---

### 5.3 ロット管理・トレーサビリティとの接続

#### **ロット管理の目的**（[quality-framework-research.md](../../sessions/session25/quality-framework-research.md) より）
1. 在庫の適正化
2. **トレーサビリティ確保** ← SIPOCの S → I → P → O → C が追跡可能性を明示
3. 品質不良の影響最小化

**SIPOCとの関連**:
- **Suppliers**: どのサプライヤから来たか（トレースバック）
- **Inputs**: ロット番号、発注番号（識別情報）
- **Outputs**: 検査記録にロット番号を含める（トレースフォワード）

---

### 5.4 8Dフレームワーク（問題解決）との接続

**8D問題解決手法**（[qa-fundamentals.md](../../docs/qa-knowledge/qa-fundamentals.md) より）:
- D2: 問題定義
- D4: 根本原因分析

**SIPOCとの関連**:
- **D2（問題定義）**: SIPOC作成で問題のスコープを明確にする
- **D4（根本原因分析）**: SIPOC → VSM で根本原因を追跡

---

### 5.5 リーン製造・8つのムダ（DOWNTIME）との接続

**8つのムダ**（[best-practices-research.md](../session261/best-practices-research.md) Section 3.1より）:
- Defects（不良品）、Overproduction（過剰生産）、Waiting（待機）、Non-Utilized Talent（未活用人材）、Transportation（運搬）、Inventory（在庫）、Motion（動作）、Excess Processing（過剰加工）

**SIPOCとVSMの役割分担**:
- **SIPOC**: 8つのムダを発見する「起点」（どこにムダがあるか仮説）
- **VSM**: 8つのムダを「定量化」（待機時間、運搬距離等を測定）

---

## 6. SIPOCスキル作成の要否判断

### 6.1 スキル化すべき理由

#### **理由1: 繰り返し使う可能性が高い**
- M3（受入検査）だけでなく、M4（工程不良）、その他の業務プロセスでも使用可能
- プロセス改善のたびにSIPOCを作成

#### **理由2: 正しい手順の遵守が重要**
- 一般的な落とし穴（古いデータ、コラボレーション不足等）を避ける必要
- 高レベルと詳細のバランスが難しい

#### **理由3: QA知識との統合**
- SIPOCを単独ツールとして使うのではなく、QAフレームワーク（ISO 9001、8D、リーン等）と統合して使用
- QA担当者としての視点を保持

#### **理由4: 他のプロセスマッピング手法との使い分け**
- SIPOC、VSM、Swimlane diagram、Turtle diagram の使い分けが必要
- 「いつSIPOCを使い、いつVSMを使うか」の判断基準が必要

---

### 6.2 スキルに含めるべき内容

#### **必須要素**:
1. **Step-by-Stepガイド**（Section 2.1）
2. **一般的な落とし穴と対策**（Section 3）
3. **SIPOC vs VSMの使い分け**（Section 4）
4. **QA知識との接続**（Section 5）
5. **チェックリスト**（実施前・実施中・実施後）
6. **テンプレート参照**（Excel、Draw.io）

#### **スキル構成案**:
```
~/.claude/skills/sipoc-facilitation/
├── SKILL.md              ← メインスキル
├── references/
│   ├── step-by-step.md   ← 詳細手順
│   ├── common-mistakes.md ← 落とし穴
│   ├── sipoc-vs-vsm.md   ← 使い分けガイド
│   └── qa-integration.md ← QA知識統合
└── templates/
    ├── sipoc-template.xlsx
    └── sipoc-template.drawio
```

---

### 6.3 スキル名の提案

**候補1**: `sipoc-facilitation`
- SIPOC作成のファシリテーションに焦点
- ワークショップ運営の観点

**候補2**: `process-mapping`
- SIPOC、VSM、Swimlane等を包括
- より広いスコープ

**候補3**: `qa-process-analysis`
- QA視点でのプロセス分析全般
- SIPOCはその一部

**推奨**: **`sipoc-facilitation`**
- 今回の用途（田原さん・杉山さんとのワークショップ）に最も適合
- 将来、他のプロセスマッピング手法を追加する場合は `process-mapping` スキルを別途作成

---

## 7. まとめ

### 7.1 SIPOC手法の特徴

| 観点 | 内容 |
|------|------|
| **目的** | プロセスの高レベル俯瞰、スコープ定義、ステークホルダー特定 |
| **使用タイミング** | プロセス改善の最初（Define フェーズ） |
| **ステップ数** | 4-7ステップ（高レベル） |
| **作成方法** | 部門横断チームで、Process（P）から開始 |
| **次のステップ** | 詳細プロセスマップ（VSM等）で深掘り |

---

### 7.2 QA知識との整合性

| QA概念 | SIPOCとの関連 |
|--------|-------------|
| ISO 9001 原則4（プロセスアプローチ） | SIPOCはプロセスアプローチの実践ツール |
| ISO 9001 原則6（客観的事実） | Inputs/Outputs を測定可能にする |
| IQC → IPQC → OQC フロー | SIPOCのProcessに対応 |
| ロット管理・トレーサビリティ | Suppliers → Inputs → Outputs → Customers の流れ |
| 8D問題解決（D2: 問題定義） | SIPOCで問題のスコープを明確化 |
| リーン・8つのムダ | SIPOCでムダの仮説を立てる → VSMで定量化 |

**結論**: SIPOCは既存QA知識と高い整合性があり、QAフレームワークの実践ツールとして位置づけられる。

---

### 7.3 スキル作成の結論

**結論**: **スキル作成を推奨**

**理由**:
1. 繰り返し使用する可能性が高い
2. 正しい手順の遵守が重要
3. QA知識との統合が必要
4. 他のプロセスマッピング手法との使い分けが必要

**スキル名**: `sipoc-facilitation`

**次ステップ**: Session 262 でスキル作成を実施

---

**出典まとめ**:
- [SIPOC Diagram: Definition & How to Create One Fast [2026] • Asana](https://asana.com/resources/sipoc-diagram)
- [How to Create a SIPOC Diagram: Step-by-Step Guide - isixsigma.com](https://www.isixsigma.com/sipoc-copis/how-to-create-a-sipoc-diagram-step-by-step-guide/)
- [Common Mistakes When Using SIPOC - isixsigma.com](https://www.isixsigma.com/sipoc-copis/common-mistakes-when-using-sipoc/)
- [SIPOC and Process Maps: Why do you need both? | LinkedIn](https://www.linkedin.com/pulse/20141017124808-6000961-sipoc-and-process-maps-why-do-you-need-both)
- [A comprehensive guide to creating your first SIPOC diagram | Nulab](https://nulab.com/learn/project-management/comprehensive-guide-creating-first-sipoc-diagram/)
- [What is a SIPOC Diagram? Step-by-Step Guide with Templates | Creately](https://creately.com/guides/sipoc-diagram/)

---

*作成: Session 262 (2026-03-19)*
