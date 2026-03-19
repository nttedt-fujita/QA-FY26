# 受入検査業務改善の文脈と背景

---
作成: Session 267 (2026-03-19)
出典: Session 241-267の履歴整理
---

## 目的

このドキュメントは、受入検査業務改善の取り組みについて、**全体の文脈と背景を整理**したものです。
AI外観検査の検証から、作業フロー可視化への方向転換、現在の取り組みまでの経緯を記録します。

---

## 1. 要求（What / Need）の変遷

### 1.1 当初の要求（Session 242-247）

**Need（要求）**: **不良品の市場流出を防ぎ、発生時に追跡できる状態にする**

**How（当初のアプローチ）**:
- AI検査システムで工数削減
- 品質向上、属人化解消、スケーラビリティ

**背景**:
- 宇枝部長からの問い合わせ（Session 242）
- Session 25: 宇枝さん「良くなったか見たい」「原因確認したい」
- 問題の本質: ロット概念がない、記録がデータになっていない

**検討内容**:
- M3/M4ドメインモデリングの再確認
- AI検査とM3/M4の統合設計
- 要求・要件・コストの整理

---

### 1.2 方向転換1: 外観検査AI化 → 員数確認AI化（Session 256）

#### Session 248-255: AI外観検査の検証・コスト分析

**対象**: プロポ・ペラの外観検査AI化

**主な発見**:
- 1万台規模でのコスト試算: 約200〜500万円の削減ポテンシャル
- **損益分岐点**: 現状500台で13-34年、1万台で1年以内
- 検査時間: 3分/個（推測）
- **回収年数**: 15-39年（年500台前提で再計算）

**藤田さんの気づき**:
- 規模感的に効果出しにくい可能性
- 写真を撮るなら人間の目で見たほうがいいケースもある
- 1個ずつ撮影 → 人間の目視と工数ほぼ同等
- 複数同時撮影なら効率化効果あり

**成果物**:
- AI検査調査の総括レポート（Session 251）
- Marpスライド（Session 252-255）
- 検査ブース・フロー図（Session 254）

#### Session 256: 小笠原さんからの新情報

**方向転換の理由**:
- 外観検査は旨味少ないと認識済みだった
- **新アプローチ**: 員数確認（ガスケット、グロメット等の小物部品をAIでカウント）

**計算結果**:
- 10回/台・20秒/回の場合: 純削減2.4万円/年、回収66〜178年
- 6回/台に減らしても: 純削減7.5万円/年、回収21〜58年
- **結論**: プロポ・ペラの外観検査AI化は500台/年規模では回収困難

#### Session 257-259: 員数確認AI化の検討

**実施内容**:
- 対象部品リストアップ: 53種類、73件
- 工数上位: Slide Mount(400分)、スライドポスト(240分)、Arm fixed holder(240分)
- 員数確認フロー図・アプリ画面作成（Draw.io）
- Marpスライド作成（16ページ → 8ページに圧縮）

**成果物**:
- [all-parts-list.md](../session257/all-parts-list.md)
- [counting-inspection-flow.drawio](../session258/counting-inspection-flow.drawio)
- [counting-inspection-report.md](../session258/counting-inspection-report.md)

---

### 1.3 方向転換2: AI化 → 作業フロー全体の可視化（Session 260）

#### Session 260: 田原さんヒアリング実施

**主な発見**（現場写真6枚撮影）:
- ✅ **AI化の前提が崩れた**: 員数確認は作業全体の一部でしかない
- ✅ **隠れコストの存在**: 梱包変更作業（開封・小分け・袋詰め）が可視化されていない
- ✅ **現場 vs 管理者のズレ**: 計画立てた人は現場が何やってるか理解していない
- ✅ **本質的な解決策**: 梱包ルール標準化（業者への依頼）、作業フロー全体の可視化

**新方針**:
- 旧方針: AI員数確認でコスト削減
- **新方針**: **現状把握を徹底（作業フロー全体の可視化）→ 根拠に基づく解決策の提案**

**成果物**:
- [hearing-results.md](../session260/hearing-results.md)
- [next-sessions-plan.md](../session260/next-sessions-plan.md)
- 現場写真6枚（梱包状態）

**教訓**:
- AI化する前に、まず現状を理解する
- 「測定しやすいもの」だけを測定していないか？（測定バイアス）
- 現場の隠れコストを可視化しないと、本質的な解決策は見つからない

---

### 1.4 現在の要求（Session 260-267）

**Need（要求）**: **コスト削減と情報の可視化**

**How（要件）**:
- Phase 1: 現状可視化（SIPOC作成、暗黙知外部化、隠れコスト特定）
- Phase 2: ムダと隠れコストの特定（VSM作成、測定バイアス是正）
- Phase 3: 解決策の策定（未来マップ、ベンダー管理、上層部への説明資料）

**補足**:
- AI化は手段の一つ（作業フロー全体を見てから判断）
- まず現状を理解してから、適切な解決策（AI化 or 標準化 or ベンダー管理等）を選択

---

## 2. Phase構成（Session 261で策定）

### Session 261: ベストプラクティス調査

**実施内容**:
- SIPOC Diagram、Value Stream Mapping の調査
- IQC（受入品質管理）2026年トレンドの調査
- 隠れコスト可視化・リーン製造の8つのムダ（DOWNTIME）の調査
- ベンダー管理・梱包標準化の調査
- データ駆動型意思決定の落とし穴（測定バイアス等）の調査
- 暗黙知の可視化・標準化の調査
- 部分最適 vs 全体最適の調査

**成果物**:
- [best-practices-research.md](../session261/best-practices-research.md)（全8章、出典付き）

**提案されたPhase構成**:

#### Phase 1: 現状可視化 ← **現在ここ**

| タスク | 状態 | セッション |
|--------|------|-----------|
| SIPOC作成（プロセスレベル） | ✅ 完了 | Session 262-266 |
| 暗黙知の外部化（「なんとなく」を標準化） | 🟡 進行中 | Session 267〜 |
| 隠れコストの特定（梱包変更作業等） | 🟡 進行中 | Session 267〜 |

**目標**: 3ヶ月後にPhase 1完了

#### Phase 2: ムダと隠れコストの特定

- VSM作成（詳細なプロセス分析）
- 測定バイアスの是正（測定しやすいものだけ測定する問題）
- リーン製造の8つのムダ（DOWNTIME）の観点で分析

#### Phase 3: 解決策の策定

- 未来マップ（To-Beプロセス）
- ベンダー管理強化（梱包ルール標準化）
- 上層部への説明資料（コスト削減効果の根拠）

---

## 3. Session 262-266: SIPOC作成

### Session 262: SIPOC準備・スキル作成

**実施内容**:
- SIPOC手法の深掘り調査（Step-by-Step、Common Mistakes、SIPOC vs VSM）
- 既存QA知識との整合性確認（ISO 9001、IQC/IPQC/OQC、ロット管理、8D、リーン等）
- SIPOCスキル作成（`~/.claude/skills/sipoc-facilitation/`）
- テンプレート作成（汎用+受入検査用）
- ヒアリング項目準備

**主な発見**:
- SIPOCの正しい作成順序: Process（P）から開始 → Suppliers/Inputs → Outputs → Customers
- SIPOC vs VSMの使い分け: SIPOC（高レベル俯瞰、Define）→ VSM（詳細、Measure）
- QA知識との高い整合性: ISO 9001、IQC/IPQC/OQC、ロット管理、8D、リーン等と統合可能

**成果物**:
- [sipoc-methodology-deep-dive.md](../session262/sipoc-methodology-deep-dive.md)
- `~/.claude/skills/sipoc-facilitation/SKILL.md`
- [sipoc-iqc-template.drawio](../session262/sipoc-iqc-template.drawio)
- [sipoc-workshop-hearing-items.md](../session262/sipoc-workshop-hearing-items.md)

### Session 263: SIPOCワークショップ準備完了

**実施内容**:
- pre-extracted-info-from-excel.md の修正（具体的な根拠（行番号、数値、日付）を追加）
- SIPOCワークショップ実施計画の詳細化（Phase 0-6のタイムライン、確認ポイント、質問例）
- ワークショップ資料の印刷準備チェックリスト作成

**成果物**:
- [sipoc-workshop-execution-plan.md](../session263/sipoc-workshop-execution-plan.md)
- [print-preparation-checklist.md](../session263/print-preparation-checklist.md)

### Session 264: ドメインモデルと課題の整合性確認

**実施内容**:
- 現在のM3ドメインモデル確認（schema.sql、Goコード）
- Excelの課題とドメインモデルの対応確認（9つの課題カテゴリを検証）
- ドメインモデル修正の必要性判断（優先度A/B/Cに分類）

**主な発見**:
- ドメインモデルは基本的に課題に対応している
- 優先度Aの必須修正は1件: **サプライヤーロット番号の欠落** → ワークショップで実態確認後に判断

**成果物**:
- [domain-model-review.md](../session264/domain-model-review.md)

### Session 265: 田原さんヒアリング準備・プロセス確認項目の整理

**実施内容**:
- Session 264の計画見直し（サプライヤーロット番号フィールド追加の判断を見直し）
- AI導入方針の整理（員数確認AI化 → 梱包変更作業でのコスト削減）
- 田原さんヒアリング項目の作成（プロセスの正確性確認に焦点）
- プロセス粒度の問題発見（クローズドクエスチョンができない）
- 全体フローの重要性（発注 → 出荷までの把握が必要）

**主な発見**:
- サプライヤーロット番号の実態: ロット管理自体がされていない、中国製部品はサプライヤーロットなし → 追加は保留
- 工数可視化の問題: 田原さんに聞いても正確な数字が出せない、この「分からない」状態自体が問題
- プロセスの粒度が粗すぎる: 「検査準備（梱包確認）」では抽象的すぎて、クローズドクエスチョンができない

**成果物**:
- [tahara-hearing-items.md](../session265/tahara-hearing-items.md)

### Session 266: 全体フロー確認用SIPOC作成・受入検査プロセス詳細化

**実施内容**:
- 全体フロー確認用SIPOC作成（発注 → 出荷、10プロセス）
- 受入検査プロセスの詳細化（5プロセス → 11プロセスに分解）
- プロセス確認用チェックリスト作成（クローズドクエスチョン形式）

**主な改善点**:

#### プロセス粒度の詳細化

**改善前（Session 262、粗い粒度）**:
1. 部品入荷
2. 検査準備（梱包確認）← 抽象的すぎる
3. 検査実施（外観・員数）
4. 記録作成（Excel入力）
5. 入庫/不合格処理

**改善後（Session 266、詳細化）**:
1. 部品入荷（納品受け取り）
2. 作業場所への移動
3. 検査段取り（場所・人員確保）
4. 開梱作業
5. **梱包変更作業**（Session 260の隠れコスト）
6. 員数確認
7. 外観検査
8. 測定（ノギス等）
9. 記録作成（Excel入力）
10. 入庫のための梱包作業
11. 入庫

#### クローズドクエスチョン形式のチェックリスト

**構成**:
- Part 1: 全体フロー（10プロセス）の確認
- Part 2: 受入検査詳細（11プロセス）の確認
  - 梱包変更作業について（最重要）
  - 員数確認、外観検査、測定、Excel記録
  - 抜けている作業の確認
- Part 3: 工数・時間に関する確認（相対的な比較）
- Part 4: 暗黙知の可視化

**効果**:
- 田原さん・杉山さんが「✅やってる」「❌やってない」「△部分的」で答えられる
- △や❌の場合は追加でヒアリング

**成果物**:
- [overall-flow-sipoc-template.drawio](../session266/overall-flow-sipoc-template.drawio)
- [iqc-detailed-sipoc-template.drawio](../session266/iqc-detailed-sipoc-template.drawio)
- [process-checklist.md](../session266/process-checklist.md)

---

## 4. Session 267: 現在地（今日）

### ヒアリング試行

**状況**:
- 田原さんが忙しくて詳細ヒアリングは無理だった
- 軽く聞いた範囲での発見

### 発見内容: プレートの例

#### 暗黙知（やっている作業）
1. **シリアル番号突合作業**
   - 袋の表記「XX番～YY番」と実物の印字を照合
   - 手作業で全数確認

2. **印字品質確認**
   - 印字の色合いが見づらくないかチェック

3. **小分け作業の実態**
   - 納品 → 入庫の間に小分け
   - 出庫しやすいように調整

#### 隠れコスト（無駄な作業）
1. **小分け再作業**
   - 問題: 出庫先によって小分けの単位が違う
   - 結果: 再度小分けし直し（無駄な再作業）
   - 根本原因: 一貫性がない（標準化されていない）

2. **サイレントチェンジ対応**
   - 問題: 印字色が勝手に変わる（サプライヤーが指定なしで変更）
   - 結果: 都度確認が必要、基準が不明瞭
   - 根本原因: サプライヤー管理の弱さ（印字仕様を指定・固定化していない）

### 位置づけ

**これはPhase 1の「暗黙知の外部化」と「隠れコスト特定」に該当**

---

## 5. 時間軸の整理

| 期間 | フェーズ | 目標 |
|------|---------|------|
| **3ヶ月** | Phase 1: 現状可視化 | SIPOC + 暗黙知外部化 + 隠れコスト特定 |
| **Phase 1完了後** | Phase 2: ムダ特定 | VSM作成、8つのムダ分析 |
| **Phase 2完了後** | Phase 3: 解決策策定 | 未来マップ、ベンダー管理、上層部資料 |
| **1年後くらい** | - | コスト削減と情報の可視化を達成 |

---

## 6. 関連資料

### AI検査関連
- [ai-inspection-summary-with-tradeoffs.md](../session251/ai-inspection-summary-with-tradeoffs.md)
- [ai-inspection-report.md](../session252/ai-inspection-report.md)（外観検査スライド）
- [counting-inspection-report.md](../session258/counting-inspection-report.md)（員数確認スライド）

### Session 260の方向転換
- [hearing-results.md](../session260/hearing-results.md)
- [next-sessions-plan.md](../session260/next-sessions-plan.md)

### ベストプラクティス調査
- [best-practices-research.md](../session261/best-practices-research.md)

### SIPOC関連
- [sipoc-methodology-deep-dive.md](../session262/sipoc-methodology-deep-dive.md)
- [overall-flow-sipoc-template.drawio](../session266/overall-flow-sipoc-template.drawio)
- [iqc-detailed-sipoc-template.drawio](../session266/iqc-detailed-sipoc-template.drawio)
- [process-checklist.md](../session266/process-checklist.md)

### セッション履歴
- [session-history/session-241-250.md](../session-history/session-241-250.md)
- [session-history/session-251-260.md](../session-history/session-251-260.md)
- [session-history/session-261-270.md](../session-history/session-261-270.md)

---

## 7. 重要な教訓

### AI化の前提を疑う
- AI化が効果的かどうかは、**作業全体を見てから判断する**
- 一部の作業だけを切り出してAI化しても、全体の工数削減にならない可能性

### 隠れコストを可視化する
- 測定しやすいものだけ測定していないか？（測定バイアス）
- 現場で「なんとなく」やっている作業を可視化する

### 現場と管理者のズレ
- 計画を立てる人は、現場が何をやっているか理解していない
- 現場ヒアリングを徹底し、実態を把握する

### 部分最適 vs 全体最適
- 個別の作業を最適化しても、全体としての無駄が残る可能性
- システム思考で全体最適を考える

### 標準化とベンダー管理
- AI化よりも、標準化やベンダー管理の方が根本解決になる場合がある
- まず現状を理解してから、適切な解決策を選択

---

## 8. 次のステップ（Session 267以降）

### Phase 1の残りタスク
- ✅ SIPOC作成（Session 266完了）
- 🟡 **暗黙知の外部化** ← プレートの作業を記録する方法を決める
- 🟡 **隠れコストの特定** ← 小分け再作業等を記録する方法を決める

### 進捗管理の仕組み
- 部品別の調査状況を管理するファイル作成（検討中）
- 3ヶ月の長期管理に適した方法を採用

### ヒアリング戦略
- 田原さんが忙しい中で、少しずつ情報を集める
- 写真撮影、作業観察、短時間ヒアリングの組み合わせ

---

**最終更新**: Session 267 (2026-03-19)
