# セッション履歴: Session 241〜250

## Session 241 (2026-03-18)

**概要**: AI調査資料の統合整理

**実施内容**:
1. session236/M3M4tools-AI-research/（10ファイル）を確認
2. `docs/missions/m3-incoming-inspection-db/ai-research/` を新規作成
3. 全ファイルをdocs/へ移動
4. Session 240のギャップ分析を `10_gap_analysis.md` として統合
5. README.mdに「ai-research/」セクションを追加
6. 元ファイル削除（session236/, session240/）

**配置ファイル**:

| 配置先 | ファイル数 |
|--------|-----------|
| [ai-research/](../../docs/missions/m3-incoming-inspection-db/ai-research/) | 11ファイル |

**結論**: M3再開時は `ai-research/` ディレクトリを参照すればよい状態に整理完了

**次セッション**: [session242/session-plan.md](../session242/session-plan.md)

---

## Session 242 (2026-03-18)

**概要**: M3レビュー完了確認 + AI検査システム構想たたき台作成

**実施内容**:
1. M3レビュー完了確認（Session 238計画の最終セッション）
   - CLAUDE.md: 古い記述（kintone比較検討中）を修正
   - README.md: ADRリンク切れ2件を修正
   - ai-research/: 11ファイル全て存在確認
2. AI検査システム構想のたたき台計画書作成
   - 宇枝部長からの問い合わせに対応
   - 要求整理、ハードル、MVPアプローチ、人員の話を整理

**修正ファイル**:

| ファイル | 修正内容 |
|----------|---------|
| [CLAUDE.md](../../CLAUDE.md) | M3方向性を「Go + Next.js採用、M3は⏸️ストップ中」に更新 |
| [README.md](../../docs/missions/m3-incoming-inspection-db/README.md) | ADRリンクを`../../adr/m3/`に修正 |

**作成ファイル**:

| ファイル | 内容 |
|----------|------|
| [ai-inspection-system-draft-plan.md](../session242/ai-inspection-system-draft-plan.md) | AI検査システム構想たたき台計画書 |

**結論**: M3レビュー作業（Session 239-242）完了。次はAI検査システムの要件整理へ

**次セッション**: [session243/session-plan.md](../session243/session-plan.md) — AI検査システム要件・要求の詳細整理

---

## Session 243 (2026-03-18)

**概要**: AI検査システム要件・要求の詳細整理

**実施内容**:
1. 員数確認・外観検査の対象整理
   - ネジは対象外（写真判別困難）
   - 外観検査: シルク印刷のカケ、クラック（難しい）
2. AI化の判断基準を整理
   - 価格高い、影響大きい、まとめて効率化、撮影コスト < AI価値
3. 受入検査CSVデータ確認（Session 235）
   - プロポ: 80分/60個、外装部打痕12個の不良実績
4. QAフレームワークとの整合性確認
   - Session 25の品質管理フレームワーク調査を参照
   - トレーサビリティは必要（ロット＝入荷タイミング）
   - AQLは要相談（部品・文脈依存）

**作成ファイル**:

| ファイル | 内容 |
|----------|------|
| [ai-inspection-requirements-draft.md](../session243/ai-inspection-requirements-draft.md) | AI検査システム要件整理（仮） |
| [qa-framework-considerations.md](../session243/qa-framework-considerations.md) | QAフレームワーク考慮事項（中間） |

**結論**: AI検査システムの要件を仮整理。次はM3/M4ドメインモデリングの過去文脈を再確認

**次セッション**: [session244/session-plan.md](../session244/session-plan.md) — M3/M4ドメインモデリング再確認・AI検査連携検討

---

## Session 244 (2026-03-18)

**概要**: M3/M4ドメインモデリングの再確認（途中）

**実施内容**:
1. M3/M4ドメインモデリングの過去決定事項を確認
   - M3 README、M4 README、セッション履歴（31-50）を確認
   - M3/M4の紐づけ方針（部品、ロット番号、時系列）を再確認
2. ロット概念の設計詳細を確認
   - 方針: 入荷タイミング = ロット
   - schema.sqlで`lots`テーブルに`arrival_date`を確認
3. ドメインモデル関連ファイルの存在確認
   - to-be-model.drawio、as-is-model.drawio、qa-gap-analysis.drawio

**議論**:
- ドメインモデル vs ER図の違いが曖昧 → 次セッションで整理
- created_at / updated_at の追加が必要な箇所を確認 → 次セッション

**作成ファイル**:

| ファイル | 内容 |
|----------|------|
| [domain-model-review-notes.md](../session244/domain-model-review-notes.md) | 中間ドキュメント（次セッション継続用） |

**結論**: ドメインモデリングの過去決定は確認完了。図の詳細確認・整理は次セッションで継続

**次セッション**: [session245/session-plan.md](../session245/session-plan.md) — to-be-model.drawio確認・ドメインモデル/ER図整理

---

## Session 245 (2026-03-18)

**概要**: ドメインモデル vs ER図の整理 + AI検査連携設計たたき台

**実施内容**:
1. to-be-model.drawioの確認
   - 実態は「ER図相当」（PK/FK、カラム詳細を含む）
   - 純粋なドメインモデル（概念図）ではない
2. schema.sqlとの整合性確認
   - 8テーブル全て一致、created_at/updated_at全テーブルに存在
3. M3/M4図の状況確認
   - M3: 3つの図あり、M4: ドメインモデル図なし
4. 解説ドキュメント作成（Living Documentation観点含む）
5. AI検査連携設計のたたき台作成（3案提示、案A推奨）

**作成ファイル**:

| ファイル | 内容 |
|----------|------|
| [domain-model-vs-er-diagram.md](../session245/domain-model-vs-er-diagram.md) | ドメインモデル vs ER図の解説、整理方針 |
| [ai-inspection-m3-integration-draft.md](../session245/ai-inspection-m3-integration-draft.md) | AI検査とM3連携設計のたたき台 |

**主な発見**:
- to-be-model.drawio = ER図（名前と実態の不一致）
- ドメインモデル（概念図）がM3/M4とも欠如
- 致命的な整合性問題はなし

**次セッション**: [session246/session-plan.md](../session246/session-plan.md) — M3/M4ドメインモデル（概念図）の新規作成

---

## Session 246 (2026-03-18)

**概要**: M3/M4ドメインモデル新規作成 + AI検査サービス選定議論

**実施内容**:
1. M3ドメインモデル（概念図）を新規作成
   - エンティティ: サプライヤ、部品、ロット、検査記録、不良レポート、不問判定
   - ビジネスルール4項目を図中に記載
2. M4ドメインモデル（概念図）を新規作成
   - 工程不良記録、不良コード体系（3階層）、原因コード体系（4M1E）
3. to-be-model.drawio → er-diagram.drawio リネーム
4. AI検査サービス選定の議論
   - AWS各サービスの出力仕様を調査（Rekognition Custom Labels、SageMaker、Bedrock）
   - 新たな状況: 「1万台生産を見据えたスケーラビリティ」が必要
   - 方針: 要求・要件・コストを整理してからサービス選定する

**作成/変更ファイル**:

| ファイル | 内容 |
|----------|------|
| [domain/domain-model.drawio](../../docs/missions/m3-incoming-inspection-db/domain/domain-model.drawio) | M3ドメインモデル（新規） |
| [domain/domain-model.drawio](../../docs/missions/m4-defect-db/domain/domain-model.drawio) | M4ドメインモデル（新規） |
| [to-be/er-diagram.drawio](../../docs/missions/m3-incoming-inspection-db/to-be/er-diagram.drawio) | リネーム（旧: to-be-model.drawio） |
| [ai-service-selection-notes.md](../session246/ai-service-selection-notes.md) | AI検査サービス選定中間メモ |

**主な発見**:
- Bedrockは学習データ不要でスケーラブル（MVP向き）
- Rekognition Custom LabelsはBoundingBox取得可能
- 1万台規模では要求・要件・コストを先に整理する必要あり

**次セッション**: [session247/session-plan.md](../session247/session-plan.md) — AI検査サービス選定（要求・要件・コスト整理）

---

## Session 247 (2026-03-18)

**概要**: AI検査・M3/M4システムの要求整理と方針確定

**実施内容**:
1. もともとの要求を過去セッションから確認
   - Session 25: 宇枝さん「良くなったか見たい」「原因確認したい」
   - 問題の本質: 不良品の市場流出、ロット概念がない、記録がデータになっていない
2. 方針の確定
   - AI検査 = M3/M4の一部（統合）
   - クラウドは使う方向（1万台規模では必須）
   - ランニングコストは抑えたい
3. 比較材料の整理フレームワーク作成
   - 軸1: クラウド構成（A〜D）
   - 軸2: 規模（現状 vs 1万台）
4. ヒアリング確認項目の整理
   - そもそも何にAIを使うのが効果的か → 未検証
   - AI使用候補の洗い出し・現状工数・1万台規模での変化・削減見積もり

**作成ファイル**:

| ファイル | 内容 |
|----------|------|
| [requirements-and-direction.md](../session247/requirements-and-direction.md) | 要求整理・方針・ヒアリング確認項目 |

**確定事項**:
- Need: 不良品の市場流出を防ぎ、発生時に追跡できる状態にする
- AI検査の主目的: 工数削減（品質向上・属人化解消・スケーラビリティも副次効果）

**残課題**:
- AI使用候補の効果検証（ヒアリング必要）
- 1万台規模での検査工数確認

**次セッション**: [session248/session-plan.md](../session248/session-plan.md) — ヒアリング実施・AI使用候補の優先順位決定

---

## Session 248 (2026-03-18)

**概要**: AI検査対象の絞り込み・1万台規模でのコスト試算

**実施内容**:
1. AI検査対象をプロポ・ペラに絞り込み
   - 外観検査でAI化の効果が見込める2部品に特化
   - 小笠原さん不在のためヒアリングは次回以降
2. CSVデータから工数を確認
   - プロポ: 80分/60個（1.3分/個）
   - ペラ: 120-200分/100個（1.2-2分/個）
3. 1万台規模でのコスト試算（概算）
   - 前提: 年間500台 → 1万台
   - プロポ+ペラ外観検査: 約9h/月 → 約185h/月
   - AI化で年間200〜500万円の削減ポテンシャル

**確定事項**:
- AI検査対象: プロポ・ペラの外観検査
- 現状規模: 年間約500台

**追加で整理が必要な項目（藤田さん指摘）**:
- AI検査に伴う追加工数（写真撮影、品質ジャッジ）
- お金以外のメリット（品質安定、属人化解消）

**次セッション**: [session249/session-plan.md](../session249/session-plan.md) — ヒアリング実施・お金以外のメリット整理

---

## Session 249 (2026-03-18)

**概要**: AI検査コスト・ベネフィット分析・CSVデータ分析

**実施内容**:
1. AI検査のコスト・ベネフィット分析
   - 追加工数（写真撮影、品質ジャッジ）を整理
   - お金以外のメリット（品質安定、属人化解消）を整理
   - 損益分岐点: 現状規模13-34年、1万台なら1年以内
2. CSVデータ分析（Session 235データ活用）
   - プロポ: 20%不良率、外装部打痕が主（「個人主観」と記載）
   - ペラ: 0-14%不良率、キズ・汚れ・曇りキズ（「都度判断依頼」と記載）

**作成ファイル**:

| ファイル | 内容 |
|----------|------|
| [ai-inspection-cost-benefit-analysis.md](../session249/ai-inspection-cost-benefit-analysis.md) | コスト・ベネフィット分析 |
| [csv-data-analysis.md](../session249/csv-data-analysis.md) | 不良パターン分析 |

**重要な指摘（藤田さん）**:
- 写真を撮るなら人間の目で見たほうがいいケースもある
- M3プロトタイプの「記録効率化」アプローチが効く可能性
- AI可能と書いたものも精査が必要（ヒアリングで確認）

**次セッション**: [session250/session-plan.md](../session250/session-plan.md) — ヒアリング実施・AI検査対象の精査

---

## Session 250 (2026-03-18)

**概要**: AI検査対象の精査・状況整理・方針確認

**実施内容**:
1. AI検査の効率化条件を整理
   - 複数同時撮影 → 効率化効果あり
   - 1個ずつ撮影 → 人間の目視と工数ほぼ同等
2. 客観的事実ベースで情報整理
   - 過去セッション（242-249）の調査結果をまとめた
3. Bedrock環境構築コスト調査
   - 画像トークン計算: `(幅px × 高さpx) / 750`
   - 1枚あたり約1.2円、環境構築1-2日

**作成ファイル**:

| ファイル | 内容 |
|----------|------|
| [session-summary.md](../session250/session-summary.md) | セッションサマリー |

**重要な気づき（藤田さん）**:
- 規模感的に効果出しにくい可能性
- 作業フローへの落とし込みが難しい
- 学習データ量と精度の関係が予測困難（効果見積もりの困難さ）
- 1万台規模で機能する設計が前提、効果が期待できるならやる
- 事前に小規模で検証（PoC）すべき

**次セッション**: [session251/session-plan.md](../session251/session-plan.md) — AI検査一次報告資料の作成（Marpスライド）

---
