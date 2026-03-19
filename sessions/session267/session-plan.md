# Session 267 計画

**目的**: 田原さん・杉山さんとのSIPOCワークショップ実施

**前提**:
- Session 266で全体フロー・受入検査詳細のSIPOC作成完了
- クローズドクエスチョン形式のチェックリスト作成完了

---

## やること

| # | 作業 | 読むべきファイル | 参照コマンド |
|---|------|-----------------|--------------|
| 1 | SIPOCワークショップ実施 | sessions/session266/overall-flow-sipoc-template.drawio<br>sessions/session266/iqc-detailed-sipoc-template.drawio<br>sessions/session266/process-checklist.md<br>sessions/session263/sipoc-workshop-execution-plan.md | - |
| 2 | 特に「梱包変更作業」の詳細ヒアリング | sessions/session260/hearing-results.md | - |
| 3 | ワークショップ結果のまとめ・SIPOC修正 | - | - |

---

## 詳細

### 1. SIPOCワークショップ実施

**目的**: 全体フロー・受入検査プロセスの実態確認

**進め方**（Session 263のワークショップ実施計画を参照）:

#### Phase 1: 全体フローの確認（20分）

- SIPOC図（全体フロー）を見せて「こういう流れですか？」と確認
- チェックリスト Part 1 を使って各プロセスを確認
- 抜けているプロセス・順序の違いをヒアリング

**確認資料**:
- [sessions/session266/overall-flow-sipoc-template.drawio](../session266/overall-flow-sipoc-template.drawio)
- [sessions/session266/process-checklist.md](../session266/process-checklist.md) Part 1

#### Phase 2: 受入検査詳細の確認（40分）

- SIPOC図（受入検査詳細）を見せて「こういう流れですか？」と確認
- チェックリスト Part 2 を使って各プロセスを確認
- 各プロセスについて「✅やってる」「❌やってない」「△部分的」で回答

**確認資料**:
- [sessions/session266/iqc-detailed-sipoc-template.drawio](../session266/iqc-detailed-sipoc-template.drawio)
- [sessions/session266/process-checklist.md](../session266/process-checklist.md) Part 2

#### Phase 3: 工数・暗黙知の確認（20分）

- 作業時間の相対的な比較（どの作業に時間がかかるか）
- 暗黙知の可視化（明確なルール vs なんとなく）

**確認資料**:
- [sessions/session266/process-checklist.md](../session266/process-checklist.md) Part 3, Part 4

---

### 2. 特に「梱包変更作業」の詳細ヒアリング

**目的**: Session 260で判明した隠れコストの詳細把握

**確認事項**（チェックリストより）:

| 質問 | 確認内容 |
|------|----------|
| 梱包変更作業は毎回発生しますか？ | 頻度の把握 |
| 梱包変更作業はどのプロセスの後に行いますか？ | タイミングの特定 |
| 梱包変更作業にかかる時間は？（概算） | 工数の概算 |
| 梱包変更作業が必要な理由は？ | 背景の理解 |
| どのサプライヤーの梱包が問題ですか？ | サプライヤー別の問題特定 |

**参照**:
- [sessions/session260/hearing-results.md](../session260/hearing-results.md) — Session 260のヒアリング結果
- `sessions/session260/Photos/` — 現場写真6枚

---

### 3. ワークショップ結果のまとめ・SIPOC修正

**目的**: ヒアリング結果をもとにSIPOC図を修正

**実施内容**:
- チェックリストの回答をもとに、SIPOC図を修正
- 抜けていたプロセスを追加
- 順序の違いを修正
- 梱包変更作業の詳細を反映

**成果物**:
- `sessions/session267/sipoc-workshop-results.md` — ワークショップ結果のまとめ
- `sessions/session267/overall-flow-sipoc-updated.drawio` — 修正後の全体フローSIPOC
- `sessions/session267/iqc-detailed-sipoc-updated.drawio` — 修正後の受入検査詳細SIPOC

---

## 期待される成果物

| ファイル | 内容 |
|----------|------|
| sipoc-workshop-results.md | ワークショップ結果のまとめ（チェックリストの回答、発見事項） |
| overall-flow-sipoc-updated.drawio | 修正後の全体フローSIPOC |
| iqc-detailed-sipoc-updated.drawio | 修正後の受入検査詳細SIPOC |

---

## 参照

- [sessions/session266/session-summary.md](../session266/session-summary.md) — Session 266サマリー
- [sessions/session266/overall-flow-sipoc-template.drawio](../session266/overall-flow-sipoc-template.drawio) — 全体フローSIPOC
- [sessions/session266/iqc-detailed-sipoc-template.drawio](../session266/iqc-detailed-sipoc-template.drawio) — 受入検査詳細SIPOC
- [sessions/session266/process-checklist.md](../session266/process-checklist.md) — チェックリスト
- [sessions/session263/sipoc-workshop-execution-plan.md](../session263/sipoc-workshop-execution-plan.md) — ワークショップ実施計画
- [sessions/session260/hearing-results.md](../session260/hearing-results.md) — Session 260のヒアリング結果

---

*作成: Session 266 (2026-03-19)*
