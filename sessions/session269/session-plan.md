# Session 269 計画

**目的**: ユーザーヒアリング + Linear導入判断

**前提**: Session 268でLinear調査完了、ハイブリッド運用ルール策定完了

---

## やること

| # | 作業 | 読むべきファイル | 参照コマンド |
|---|------|-----------------|--------------|
| 1 | ユーザーヒアリング（「色々言いたいことがある」） | [session268/session-summary.md](../session268/session-summary.md) | - |
| 2 | Linear導入判断（ヒアリング結果を踏まえて） | [docs/tools/linear/pricing-and-features.md](../../docs/tools/linear/pricing-and-features.md), [docs/tools/linear/hybrid-operation-rules.md](../../docs/tools/linear/hybrid-operation-rules.md) | - |
| 3 | Linear MCP Serverセットアップ（導入する場合） | [docs/tools/linear/pricing-and-features.md](../../docs/tools/linear/pricing-and-features.md) | - |
| 4 | phase1-progress.md 作成開始 | [session267/context-and-background.md](../session267/context-and-background.md) | - |

---

## 詳細

### 1. ユーザーヒアリング

**Session 268終了時のコメント**:
> 「次のセッションで思ってることあるから色々いいます」

**ヒアリング項目（想定）**:
- Linear導入への懸念・質問
- 進捗管理の方針
- Phase 1の進め方
- 田原さん・杉山さんとのコラボレーション方法
- その他

**出力**: ヒアリング結果の整理、方針の再確認

---

### 2. Linear導入判断

**ヒアリング結果を踏まえて判断**:

#### 選択肢A: Linear Basicプラン導入

**メリット**:
- アクティブissue無制限
- Claude Code連携（MCP Server）
- 進捗可視化が強力
- コラボレーション機能が充実

**デメリット**:
- コスト: 年$360-600
- 学習コスト（非技術者）

**判断基準**:
- 田原さん・杉山さんとのコラボレーションが必要か
- 年$360-600のコストが許容できるか

#### 選択肢B: Markdownのみで運用

**メリット**:
- コスト: $0
- シンプル（既存フロー）
- Git履歴で管理

**デメリット**:
- コラボレーション弱い
- 進捗可視化が劣る

**判断基準**:
- 藤田さん単独での作業が中心か
- コストを抑えたいか

#### 選択肢C: GitHub Projects（無料）

**メリット**:
- コスト: $0
- Git連携が強い
- 無制限issue

**デメリット**:
- コラボレーション弱い
- 非技術者には難しい

**判断基準**:
- 技術プロトタイプ（M1-GNSS等）のみで使用するか

---

### 3. Linear MCP Serverセットアップ（導入する場合）

**手順**:
```bash
# 1. MCP Serverを追加
claude mcp add --transport http linear-server https://mcp.linear.app/mcp

# 2. Claude Codeセッション内で認証
/mcp
# → OAuth認証フローが開始
```

**確認事項**:
- OAuth認証が成功するか
- Issue作成・検索が動作するか

**出力**: Linear MCP Server動作確認レポート

---

### 4. phase1-progress.md 作成開始

**仕組み決定後に実施**:

#### Linearを使う場合

- Workspace: QA-FY26 作成
- Project: 受入検査改善（Phase 1-3）作成
- Epic: Phase 1 - SIPOC作成、暗黙知外部化、隠れコスト特定
- 初期Issue作成（プレート調査等）

#### Markdownを使う場合

- phase1-progress.md テンプレート作成
- プレートの情報を初期データとして登録

#### ハイブリッドの場合

- 両方実施

**出力**: phase1-progress.md または Linear Project

---

## 参照

- [session268/session-summary.md](../session268/session-summary.md) — 前セッションサマリー
- [docs/tools/linear/pricing-and-features.md](../../docs/tools/linear/pricing-and-features.md) — Linear調査レポート
- [docs/tools/linear/hybrid-operation-rules.md](../../docs/tools/linear/hybrid-operation-rules.md) — ハイブリッド運用ルール
- [session267/context-and-background.md](../session267/context-and-background.md) — 全体文脈整理

---

**期待成果**: Linear導入判断が完了し、Phase 1の進捗管理が開始できる状態
