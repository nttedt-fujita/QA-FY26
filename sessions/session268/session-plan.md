# Session 268 計画

**目的**: Linear調査・進捗管理の仕組み決定

**前提**: Session 267で全体の文脈整理完了、進捗管理の必要性を確認

---

## やること

| # | 作業 | 読むべきファイル | 参照コマンド |
|---|------|-----------------|--------------|
| 1 | Linear調査（コスト、機能、運用） | - | - |
| 2 | Claude CodeとLinearの連携確認 | - | - |
| 3 | 進捗管理の仕組み決定 | [context-and-background.md](../session267/context-and-background.md) | - |
| 4 | phase1-progress.md 作成開始 | - | - |

---

## 詳細

### 1. Linear調査

**調査項目**:

#### コスト
- 無料枠の内容（ユーザー数、Issue数、機能制限）
- 有料プランの価格（個人、チーム）
- QA-FY26プロジェクトでの想定コスト

#### 機能
- コラボレーション機能（コメント、メンション、通知）
- 進捗可視化（進捗率、ダッシュボード、フィルター）
- API連携（REST/GraphQL、Webhook）
- エクスポート機能（CSV、PDF、Markdown）

#### 運用
- Workspace/Project/Epicの構成
- Issue作成・更新のワークフロー
- 他のツールとの連携（GitHub、Slack等）

**出力**: Linear調査レポート（コスト、機能、運用方法）

---

### 2. Claude CodeとLinearの連携確認

**確認項目**:

#### Linear CLI
- インストール方法
- 認証設定
- 基本コマンド（issue create, update, comment等）

#### 自動化フロー（テスト）
```bash
# Issue作成
linear issue create --title "テスト" --description "自動化テスト"

# コメント追加
linear comment create ISSUE-ID --body "Session 268でテスト"
```

**出力**: Claude Code + Linear 連携ガイド

---

### 3. 進捗管理の仕組み決定

**選択肢の再確認**:
- A. Linearのみ
- B. Markdownのみ
- C. Linear + Markdown ハイブリッド（推奨）

**決定事項**:
- どの方法を採用するか
- 運用フロー（セッション終了時に何をするか）
- コラボレーション方法（誰と共有するか）

**出力**: 進捗管理運用ルール

---

### 4. phase1-progress.md 作成開始

**仕組み決定後に実施**:

#### Linearを使う場合
- Workspace/Project作成
- Epic作成（Phase 1, 2, 3）
- 初期Issue作成（プレート調査等）

#### Markdownを使う場合
- phase1-progress.md テンプレート作成
- プレートの情報を初期データとして登録

#### ハイブリッドの場合
- 両方実施

**出力**: phase1-progress.md または Linear Project

---

## 参照

- [session267/session-summary.md](../session267/session-summary.md) — 前セッションサマリー
- [session267/context-and-background.md](../session267/context-and-background.md) — 全体文脈整理

---

**期待成果**: 進捗管理の仕組みが決まり、Phase 1の進捗トラッキングが開始できる状態
