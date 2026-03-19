# Session 270 計画

**目的**: 宇枝さん説明資料作成開始（AI導入の現実を伝える）

**前提**: Session 269でLinear導入完了、本来のミッション（説明資料作り）に戻る

---

## やること

| # | 作業 | 読むべきファイル | 参照コマンド |
|---|------|-----------------|--------------|
| 1 | Linear連携動作確認 | [session269/session-summary.md](../session269/session-summary.md) | - |
| 2 | 宇枝さん説明資料の骨子作成 | [session267/context-and-background.md](../session267/context-and-background.md), [session266/process-checklist.md](../session266/process-checklist.md) | - |
| 3 | ヒアリング結果のまとめ | [sessions/session264/](../session264/), [session267/](../session267/) | - |

---

## 詳細

### 1. Linear連携動作確認（5-10分）

**目的**: Claude CodeからLinearを操作できることを確認

**手順**:
1. Claude Codeから「Linearにテストissueを作って」と依頼
2. OAuth認証が必要な場合は完了
3. Issueが作成されたことをLinear UIで確認

**成果物**: 動作確認レポート（簡易）

---

### 2. 宇枝さん説明資料の骨子作成（メイン）

**目的**: AI導入が厳しいこと、手を入れるべき部分が違うことを伝える資料を作成

**伝えるべきポイント**:

#### A. AI導入が厳しい理由

1. **員数確認AI化は効果が薄い**
   - Session 260で判明した隠れコスト（梱包変更作業）の方が大きい
   - AIで自動化しても、本質的な問題は解決しない

2. **外観検査AI化の方が効果大**
   - プロポ・ペラの外観検査の方が手間がかかる
   - ただし、現状の作業フローを可視化してから判断すべき

3. **前提条件の整備が必要**
   - 暗黙知の外部化
   - 作業手順の標準化
   - データの蓄積

#### B. 手を入れるべき部分

1. **プロセスの可視化**（Phase 1: SIPOC）
   - 現状の作業フローを図化
   - ムダと隠れコストの特定
   - Session 266で作成したSIPOC、プロセスチェックリストを活用

2. **暗黙知の外部化**
   - 検査基準の明文化
   - 判断基準の標準化
   - ノウハウの文書化

3. **隠れコストの削減**
   - 梱包変更作業の効率化
   - プレート調査の自動化（AI以前の問題）
   - 手戻りの削減

#### C. ヒアリング結果のまとめ

**小笠原さんとの認識合わせ済み**:
- プレートの情報が不足している
- 梱包変更作業が頻繁に発生
- 検査基準が暗黙知化している

**田原さんとのヒアリング項目**（Session 264で整理済み）:
- 詳細な作業フロー
- 検査基準の確認
- ムダと手戻りの具体例

**成果物**:
- `sessions/session270/ueda-explanation-draft.md` — 説明資料ドラフト
- スライド形式または文書形式（要相談）

---

### 3. ヒアリング結果のまとめ

**目的**: Session 264, 266, 267で収集した情報を統合

**まとめる内容**:
- プレート調査の課題
- 梱包変更作業の頻度と手間
- 検査基準の暗黙知化
- 隠れコスト（Session 260で発見）

**成果物**: `sessions/session270/hearing-summary.md`

---

## 参照

- [session269/session-summary.md](../session269/session-summary.md) — 前セッションサマリー
- [session267/context-and-background.md](../session267/context-and-background.md) — 全体文脈整理
- [session266/iqc-detailed-sipoc-template.drawio](../session266/iqc-detailed-sipoc-template.drawio) — 受入検査詳細SIPOC
- [session266/process-checklist.md](../session266/process-checklist.md) — プロセスチェックリスト
- [session260/](../session260/) — AI化方針転換の経緯
- [session264/](../session264/) — 田原さんヒアリング項目

---

## 制約

- **Linear作業は最小限**（動作確認のみ、Issue登録は後回し）
- **説明資料作成に集中**（本来のミッション）
- **GNSSは次々セッション以降**（まず説明資料を片付ける）

---

**期待成果**: 宇枝さんへの説明資料ドラフトが完成し、AI導入の現実を伝えられる状態
