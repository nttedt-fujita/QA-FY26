# Linear + Markdown ハイブリッド運用ルール

---
作成: Session 268 (2026-03-19)
適用対象: QA-FY26プロジェクト全体
---

## 目的

Linear（進捗管理・コラボレーション）とMarkdown（技術詳細・履歴管理）を組み合わせたハイブリッド運用で、両方の強みを活かす。

---

## 基本方針

### Linear: 進捗管理・コラボレーション

**用途**:
- タスク進捗管理（Issue単位）
- 優先度・担当者・ステータス管理
- コメント・メンション・通知
- 進捗可視化（ダッシュボード、フィルター）

**記録する情報**:
- タスクの概要（1-2行）
- ステータス（Todo, In Progress, Done）
- 担当者、優先度、ラベル
- 簡単なコメント（進捗報告、ブロッカー報告等）

### Markdown: 技術詳細・履歴管理

**用途**:
- 技術調査・分析の詳細
- 設計判断の記録（ADR）
- セッションごとの詳細な作業記録
- 長期保存が必要な資料

**記録する情報**:
- 調査レポート（出典・原文抜粋付き）
- 設計判断の背景・理由
- session-plan.md、session-summary.md
- 技術資料（図解、コード例等）

---

## 情報配置ルール

### 1. Linearに記録するもの

| 情報種類 | 記録内容 | 例 |
|---------|---------|-----|
| **タスク概要** | 1-2行の要約 | "プレートの暗黙知調査（シリアル突合、印字品質）" |
| **進捗状況** | ステータス、担当者 | "In Progress（藤田）" |
| **簡単なコメント** | 進捗報告、ブロッカー | "田原さんに確認中、明日ヒアリング予定" |
| **リンク** | Markdown資料へのリンク | "詳細: sessions/session268/plate-research.md" |

### 2. Markdownに記録するもの

| 情報種類 | 記録場所 | 例 |
|---------|---------|-----|
| **技術調査** | sessions/sessionXXX/ | linear-research-report.md（出典・原文抜粋付き） |
| **設計判断** | docs/adr/ | ADR-015-cfg-cfg-loadmask.md |
| **セッション記録** | sessions/sessionXXX/ | session-plan.md、session-summary.md |
| **正式ドキュメント** | docs/missions/、docs/tools/ | pricing-and-features.md |
| **プロジェクト設定** | CLAUDE.md、MEMORY.md | ADR一覧、プロジェクト固有ルール |

### 3. 二重管理の禁止

**禁止**: 同じ情報をLinearとMarkdownの両方に詳細記載すること

**正しいパターン**:
- Linear: タスク概要 + Markdown資料へのリンク
- Markdown: 詳細な内容

**例**:
```
【Linear Issue】
タイトル: プレート暗黙知調査
説明: シリアル突合、印字品質、小分け作業の実態調査
詳細: sessions/session267/plate-research.md

【Markdown】
sessions/session267/plate-research.md
（詳細な調査結果、ヒアリング内容、発見事項等を記載）
```

---

## セッション終了時のワークフロー

### 1. Linear操作

```
1. 完了したIssueのステータスを `Done` に変更
2. `Done` になったIssueにコメント追加（完了内容、成果物リンク）
3. 次セッションで取り組むIssueを `Todo` に設定
4. 新しい発見・課題があれば新規Issue作成
5. （オプション）`Done` になったIssueをアーカイブ（アクティブ数削減）
```

### 2. Markdown操作

```
1. session-summary.md を作成（詳細な作業記録）
2. 技術調査を行った場合、調査レポートを作成
3. 重要な決定があれば ADR を作成
4. 次セッションの session-plan.md を作成
5. セッション履歴ファイルに追記（session-history/session-XXX-YYY.md）
```

### 3. 相互リンク

```
1. LinearのIssueコメントにMarkdown資料へのリンクを記載
2. session-summary.md にLinear Issue IDを記載
```

**例**:
```markdown
## session-summary.md

**Linear Issue**: QA-123 プレート暗黙知調査

**実施内容**:
1. 田原さんにヒアリング
2. シリアル突合作業の実態を確認
...
```

---

## Workspace/Project/Epicの構成

### 推奨構成

```
Workspace: QA-FY26
│
├── Project: 受入検査改善（Phase 1-3）
│   ├── Epic: Phase 1 - SIPOC作成
│   │   ├── Issue: 全体フローSIPOC作成
│   │   ├── Issue: 受入検査詳細SIPOC作成
│   │   └── ...
│   ├── Epic: Phase 1 - 暗黙知外部化
│   │   ├── Issue: プレート調査
│   │   ├── Issue: フレーム調査
│   │   └── ...
│   ├── Epic: Phase 1 - 隠れコスト特定
│   │   ├── Issue: 梱包変更作業の工数調査
│   │   └── ...
│   ├── Epic: Phase 2 - VSM作成
│   ├── Epic: Phase 2 - 8つのムダ分析
│   └── Epic: Phase 3 - 解決策策定
│
├── Project: M1 - センサー評価
│   ├── Epic: M1-A Lidar評価
│   └── Epic: M1-B GNSS評価
│       ├── Issue: Phase 3 屋外検査機能
│       └── ...
│
├── Project: M2 - 点群データ検証
├── Project: M3 - 受入検査DB
├── Project: M4 - 工程不良DB
└── Project: その他（FMEA、経年劣化等）
```

### Issueのラベル（推奨）

| ラベル | 用途 |
|--------|------|
| `調査` | 技術調査、ヒアリング |
| `設計` | 設計判断、ADR作成 |
| `実装` | コーディング、プロトタイプ作成 |
| `ドキュメント` | 資料作成、図作成 |
| `ブロッカー` | 進行を妨げる問題 |
| `phase1`, `phase2`, `phase3` | Phase分類 |

---

## アーカイブ運用

### アーカイブの目的

**Freeプランの制約**:
- アクティブissue: 250まで
- アーカイブissue: 無制限

**Basicプランでも推奨**:
- アクティブissueを減らして見通しを良くする
- 完了したissueは随時アーカイブ

### アーカイブのタイミング

| タイミング | 対象 |
|-----------|------|
| **セッション終了時** | `Done` になったissue（オプション） |
| **Phase完了時** | そのPhaseの全issue（必須） |
| **定期メンテナンス** | 1ヶ月以上前の `Done` issue |

### アーカイブ前の確認

- [ ] Issue本文・コメントにMarkdown資料へのリンクが記載されているか
- [ ] session-summary.md にIssue IDが記載されているか
- [ ] 重要な情報がMarkdown側に保存されているか

**理由**: アーカイブ後もIssue情報は参照可能だが、検索性が下がるため、Markdown側に情報を残す

---

## コラボレーション方法

### 1. 藤田さん（メイン）

**役割**:
- Linear Workspace管理者
- Issue作成・更新
- Markdown資料作成
- セッション記録

**作業フロー**:
1. session-plan.md で今日のタスクを確認
2. LinearでIssueを `In Progress` に変更
3. 作業実施、Markdown資料作成
4. LinearにコメントでMarkdown資料リンクを追加
5. session-summary.md 作成、Linear Issueを `Done` に変更

### 2. 田原さん・杉山さん（コラボレーター）

**役割**:
- ヒアリング対応
- Issueへのコメント（進捗報告、質問等）
- Markdown資料の確認（必要に応じて）

**期待する使い方**:
- Linearで進捗を確認
- コメントで質問・フィードバック
- Markdown詳細資料は必要に応じて参照

### 3. いしかわさん・こいたばしさん（オプション）

**役割**:
- 担当ミッション（FMEA、経年劣化等）のIssue管理
- 必要に応じてLinear参加

---

## Claude CodeとLinearの連携

### MCP Serverセットアップ

```bash
# 1. MCP Serverを追加
claude mcp add --transport http linear-server https://mcp.linear.app/mcp

# 2. Claude Codeセッション内で認証
/mcp
# → OAuth認証フローが開始
```

### セッション中の操作例

**Issue作成**:
```
Claude, create a Linear issue for プレート調査:
- Title: プレート暗黙知調査
- Description: シリアル突合、印字品質、小分け作業の実態調査
- Project: 受入検査改善（Phase 1-3）
- Epic: Phase 1 - 暗黙知外部化
- Status: Todo
```

**Issue更新**:
```
Claude, update Linear issue QA-123:
- Status: In Progress
- Comment: Session 268で田原さんにヒアリング実施。詳細: sessions/session268/plate-research.md
```

**Issue検索**:
```
Claude, find Linear issues in project "受入検査改善（Phase 1-3）" with status "In Progress"
```

---

## 禁止事項

### 1. 二重管理

**禁止**: 同じ詳細情報をLinearとMarkdownの両方に記載
**正しい**: Linearは概要、Markdownは詳細

### 2. Linearへの詳細記載

**禁止**: Issueの Description に長文（100行以上）を記載
**正しい**: Markdown資料を作成し、Linearからリンク

### 3. Markdown資料の散逸

**禁止**: 調査資料を sessions/sessionXXX/ に放置
**正しい**: 重要資料は docs/ に正式配置、sessions/ はドラフト扱い

### 4. Linearのみでの完結

**禁止**: 技術調査をLinearのIssueコメントだけで完結
**正しい**: Markdown調査レポートを作成し、出典・原文抜粋を記載

---

## 定期メンテナンス

### 週次（金曜日）

- [ ] 今週完了したIssueを確認
- [ ] `Done` になったIssueをアーカイブ（オプション）
- [ ] 次週のIssueを `Todo` に設定

### 月次（月末）

- [ ] Phase進捗レビュー（ダッシュボード確認）
- [ ] 1ヶ月以上前の `Done` Issueをアーカイブ
- [ ] Markdown資料の整理（sessions/ から docs/ への移動）

### Phase完了時

- [ ] そのPhaseの全Issueをアーカイブ
- [ ] Phase完了レポートを作成（Markdown）
- [ ] 次Phaseの計画をLinearに反映

---

## トラブルシューティング

### 問題: アクティブissueが250に近づいた（Freeプランの場合）

**対策**:
1. 完了したIssueをアーカイブ
2. 古いIssueを削除（重要な情報はMarkdownに保存済みか確認）
3. Basicプランへの移行を検討

### 問題: Markdownとの同期が取れていない

**対策**:
1. session-summary.md にLinear Issue IDを必ず記載
2. LinearのIssueコメントにMarkdown資料リンクを必ず記載
3. 定期的に相互リンクを確認

### 問題: Linearが使いづらい（非技術者）

**対策**:
1. 初回に使い方ガイドを作成
2. 最初は藤田さんがIssue作成、田原さん・杉山さんはコメントのみ
3. 慣れてきたらIssue作成も依頼

---

## 参照

- [Linear公式ドキュメント](https://linear.app/docs)
- [Linear MCP Server](https://linear.app/docs/mcp)
- [pricing-and-features.md](pricing-and-features.md) — Linear価格・機能調査
- [issue-estimate.md](issue-estimate.md) — Issue数見積もり
- [session267/context-and-background.md](../../../sessions/session267/context-and-background.md) — 全体文脈整理

---

**最終更新**: Session 268 (2026-03-19)
**次回レビュー**: Phase 1完了時（運用の振り返り）
