# AI検査調査 総括レポート

**作成日**: 2026-03-18（Session 251）
**目的**: AI検査の検討結果を整理し、他ミッションとのトレードオフを明確化

---

## 1. 調査の背景

### きっかけ

- **宇枝部長からの問い合わせ**: AI/クラウドを使った受入検査システムの構想（Session 242）
- **将来の規模拡大**: 1万台製造の話がある

### 調査期間

- Session 242〜250（2026-03-18、同日中に集中的に調査）

---

## 2. 確定事項（結論）

| 項目 | 結論 | 出典 |
|------|------|------|
| **Need（変えられない本質）** | 不良品の市場流出を防ぎ、発生時に追跡できる状態にする | [Session 247: requirements-and-direction.md](../session247/requirements-and-direction.md) |
| **AI検査の位置づけ** | M3/M4の一部として統合（別システムではない） | [Session 247: requirements-and-direction.md](../session247/requirements-and-direction.md) |
| **AI検査対象** | プロポ・ペラの外観検査 | [Session 248: session-summary.md](../session248/session-summary.md) |
| **推奨サービス** | Amazon Bedrock（Claude）— 学習データ不要、低導入ハードル | [Session 246: ai-service-selection-notes.md](../session246/ai-service-selection-notes.md) |
| **クラウド** | 使う方向（1万台規模ではデータ回収・ヒューマンエラー考慮で必須） | [Session 247: requirements-and-direction.md](../session247/requirements-and-direction.md) |

---

## 3. コスト試算結果（客観的事実）

### 3.1 損益分岐点

| 規模 | 損益分岐点 | 出典 |
|------|-----------|------|
| 現状（500台/年） | **13-34年** | [Session 249: ai-inspection-cost-benefit-analysis.md](../session249/ai-inspection-cost-benefit-analysis.md) |
| 1万台/年 | **1年以内** | 同上 |

### 3.2 工数削減効果（推定）

| 規模 | 現状工数 | AI導入後 | 削減 | 削減率 |
|------|---------|----------|------|--------|
| 500台/年 | 9時間/月 | 5.5時間/月 | 3.5時間/月 | 39% |
| 1万台/年 | 185時間/月 | 67時間/月 | 118時間/月 | 64% |

**出典**: [Session 249: ai-inspection-cost-benefit-analysis.md](../session249/ai-inspection-cost-benefit-analysis.md)

### 3.3 初期投資

| 項目 | 概算 |
|------|------|
| 撮影環境構築 | 10-30万円 |
| システム開発（M3/M4統合） | 100-300万円 |
| AI学習・チューニング | 50-100万円 |
| **合計** | **160-430万円** |

**出典**: [Session 249: ai-inspection-cost-benefit-analysis.md](../session249/ai-inspection-cost-benefit-analysis.md)

### 3.4 Bedrock試行コスト

| 項目 | 内容 |
|------|------|
| 環境構築 | 1〜2日 |
| 1枚あたり | 約1.2円 |
| 100枚テスト | 約120円 |

**出典**: [Session 250: session-summary.md](../session250/session-summary.md)

---

## 4. 懸念点・リスク

| 懸念 | 詳細 | 出典 |
|------|------|------|
| **規模感** | 現状500台では効果出しにくい | [Session 250: session-summary.md](../session250/session-summary.md) |
| **判定基準の属人化** | プロポ: 「個人主観」、ペラ: 「都度判断」と記載あり | [Session 249: csv-data-analysis.md](../session249/csv-data-analysis.md) |
| **効果予測困難** | 学習データ量と精度の関係が不確実 | [Session 250: session-summary.md](../session250/session-summary.md) |
| **追加工数** | 撮影工数が新たに発生（1万台で40-80h/月） | [Session 249: ai-inspection-cost-benefit-analysis.md](../session249/ai-inspection-cost-benefit-analysis.md) |
| **作業フロー落とし込み** | 写真を撮るなら人間の目で見たほうがいいケースも | [Session 249: session-summary.md](../session249/session-summary.md) |

---

## 5. 藤田さん担当ミッション全体像

### 5.1 現状（Session 251時点）

| ミッション | 状況 | スケジュール | 出典 |
|-----------|------|-------------|------|
| **M1-A: LiDAR評価** | 調査完了・計画策定待ち | Step1(1Q)→Step2(2Q-3Q)→Step3(3Q-FY27 1Q) | [docs/progress.md](../../docs/progress.md) |
| **M1-B: GNSS評価** | プロトタイプ実装中（アクティブ） | 同上 | [docs/progress.md](../../docs/progress.md) |
| **M2: 点群データ検証** | 方針転換中（放置中） | - | [M2 README](../../docs/missions/m2-pointcloud-verification/README.md) |
| **M3: 受入検査DB** | プロトタイプ完成、⏸️ストップ中 | - | [M3 README](../../docs/missions/m3-incoming-inspection-db/README.md) |
| **M4: 工程不良DB** | 未着手 | - | [M4 README](../../docs/missions/m4-defect-db/README.md) |

### 5.2 優先度（Session 52で確定）

1. **M4（工程不良DB）** — 既に流出問題が発生、最優先
2. **M1-B（GNSS評価）** — 実装中、継続
3. **M3（受入検査DB）** — 一旦ストップ

**出典**: [M3 README](../../docs/missions/m3-incoming-inspection-db/README.md)（Session 52の小笠原さんフィードバック）

---

## 6. AI検査作業のトレードオフ

### 6.1 AI検査作業に必要な工数（概算）

| フェーズ | 作業内容 | 工数目安 |
|---------|---------|---------|
| **PoC準備** | AWS環境構築、撮影環境準備 | 1-2日 |
| **PoC実施** | 画像収集、テスト、チューニング | 1-2週間 |
| **システム開発** | M3/M4統合、UI開発 | 2-4週間 |
| **運用準備** | 手順書作成、現場展開 | 1週間 |
| **合計** | | **1-2ヶ月** |

### 6.2 AI検査に1-2ヶ月割いた場合の影響

| ミッション | 現状 | 影響 |
|-----------|------|------|
| **M1-A: LiDAR評価** | 放置中 | 継続放置（評価計画策定がさらに遅延） |
| **M1-B: GNSS評価** | 実装中 | **中断** → 再開時に文脈再構築が必要 |
| **M2: 点群検証** | 放置中 | 継続放置 |
| **M4: 工程不良DB** | 未着手（最優先のはず） | **着手遅延** → 流出問題対応が遅れる |

### 6.3 トレードオフの選択肢

| 選択肢 | AI検査 | 他ミッション | リスク |
|--------|--------|-------------|--------|
| **A: AI検査優先** | PoC実施 | 全て遅延 | M4の流出問題対応が後回し |
| **B: M4優先+最小AI** | 調査結果のみ報告 | M4着手 | AI検査は「見積もり済み」で保留 |
| **C: 並行（マンパワー）** | PoC実施 | M4も並行 | **工数不足で両方中途半端になるリスク** |

---

## 7. 「マンパワーで成立させろ」と言われた場合の回答材料

### 7.1 現実的な制約

| 制約 | 説明 |
|------|------|
| **担当者1名** | 藤田さんのみが全ミッションを担当 |
| **既存ミッション4つ** | M1-A/B, M2, M3, M4（うちM4が最優先） |
| **FY26残り** | 2026年3月時点で残りわずか |

### 7.2 不整合が発生するシナリオ

**シナリオ: AI検査PoCに1-2ヶ月集中した場合**

```
現在（2026-03-18） ← FY25終わり、FY26（2026年4月〜2027年3月）開始直前
    │
    ├── AI検査PoC（1-2ヶ月）
    │       ├── AWS環境構築
    │       ├── 画像収集・テスト
    │       └── チューニング
    │
    ▼
2026年5月頃
    │
    └── 結果:
        - M4: 未着手のまま（流出問題未対応）→ FY26 1Qが終わる
        - M1-B: 中断 → Step2(2Q-3Q)に間に合わない可能性
        - AI検査: PoCのみ完了（本番導入はさらに先）
```

**補足**: FY26 = 2026年4月〜2027年3月。M1のスケジュールはStep1(1Q)→Step2(2Q-3Q)→Step3(3Q-FY27 1Q)。
どちらにしても、AI検査に1-2ヶ月割くと他ミッションのスケジュールが圧迫される。

### 7.3 推奨対応

| 優先度 | 対応 | 根拠 |
|--------|------|------|
| **高** | AI検査は「調査完了・見積もり済み」として一旦区切り | 損益分岐点が1万台前提のため、現状規模では投資対効果が低い |
| **高** | M4着手 | 既に流出問題が発生している（Session 52 小笠原さんフィードバック） |
| **中** | M1-B継続 | 実装中のため、中断すると再開コストが高い |
| **低** | AI検査PoCは1万台規模が確定してから | 現状規模では効果が出にくい |

---

## 8. 参照資料一覧

### AI検査調査資料（Session 242-250）

| Session | 資料 | 内容 |
|---------|------|------|
| 242 | [ai-inspection-system-draft-plan.md](../session242/ai-inspection-system-draft-plan.md) | 構想たたき台 |
| 243 | [ai-inspection-requirements-draft.md](../session243/ai-inspection-requirements-draft.md) | 要件整理（仮） |
| 246 | [ai-service-selection-notes.md](../session246/ai-service-selection-notes.md) | AWSサービス比較 |
| 247 | [requirements-and-direction.md](../session247/requirements-and-direction.md) | 要求整理・方針確定 |
| 249 | [ai-inspection-cost-benefit-analysis.md](../session249/ai-inspection-cost-benefit-analysis.md) | コスト・ベネフィット分析 |
| 249 | [csv-data-analysis.md](../session249/csv-data-analysis.md) | 不良パターン分析（CSVより） |
| 250 | [session-summary.md](../session250/session-summary.md) | 状況整理・懸念点 |

### 過去の重要決定

| Session | 決定 | 出典 |
|---------|------|------|
| 52 | M4優先、M3一旦ストップ | [M3 README](../../docs/missions/m3-incoming-inspection-db/README.md) |
| 34 | バックエンドをGoに変更 | [CLAUDE.md](../../CLAUDE.md) |

---

*作成: Session 251 (2026-03-18)*
