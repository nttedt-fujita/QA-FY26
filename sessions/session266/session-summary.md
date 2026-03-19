# Session 266 サマリー

**日付**: 2026-03-19
**目的**: 全体フロー確認用SIPOC作成・受入検査プロセス詳細化

---

## 実施内容

1. **全体フロー確認用SIPOC作成**
   - 発注 → 出荷までの全体フロー（10プロセス）を可視化
   - 受入検査を全体フローの中に位置づけ
   - 成果物: [overall-flow-sipoc-template.drawio](overall-flow-sipoc-template.drawio)

2. **受入検査プロセスの詳細化**
   - Session 262の粗い粒度（5プロセス）を詳細化（11プロセス）
   - クローズドクエスチョンができるレベルまで分解
   - 梱包変更作業（隠れコスト）を明示化
   - 成果物: [iqc-detailed-sipoc-template.drawio](iqc-detailed-sipoc-template.drawio)

3. **プロセス確認用チェックリスト作成**
   - 田原さん・杉山さんに「✅やってる」「❌やってない」「△部分的」で答えてもらえる形式
   - Part 1: 全体フローの確認（10プロセス）
   - Part 2: 受入検査詳細の確認（11プロセス + 追加質問）
   - Part 3: 工数・時間に関する確認（相対的な比較）
   - Part 4: 暗黙知の可視化
   - 成果物: [process-checklist.md](process-checklist.md)

---

## 成果物

| ファイル | 内容 |
|----------|------|
| [overall-flow-sipoc-template.drawio](overall-flow-sipoc-template.drawio) | 全体フロー確認用SIPOC（発注 → 出荷、10プロセス） |
| [iqc-detailed-sipoc-template.drawio](iqc-detailed-sipoc-template.drawio) | 受入検査プロセス詳細化SIPOC（11プロセス） |
| [process-checklist.md](process-checklist.md) | プロセス確認用チェックリスト（クローズドクエスチョン形式） |

---

## 主な改善点

### 1. 全体フローの可視化

**目的**: 受入検査だけでなく、前後の流れを把握

**プロセス**（10ステップ）:
1. 部品発注
2. 入庫予定決定
3. 納品（受け取り）
4. **受入検査**（詳細は受入検査SIPOCで展開）
5. 入庫
6. 出庫（組立へ）
7. 組立
8. 出荷検査
9. 倉庫保管
10. 出荷

**効果**:
- 部門横断的な理解の促進
- 隠れコストの可視化（どこで時間がかかっているか）
- 責任の所在・引き継ぎポイントの明確化

### 2. 受入検査プロセスの詳細化

**改善前（Session 262、粗い粒度）**:
1. 部品入荷
2. 検査準備（梱包確認）← 抽象的すぎる
3. 検査実施（外観・員数）
4. 記録作成（Excel入力）
5. 入庫/不合格処理

**問題点**:
- 「検査準備（梱包確認）」だけでは、クローズドクエスチョンができない
- 「これでいいか？」と聞けない
- 梱包変更作業（隠れコスト）がどこに含まれるか不明

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

**効果**:
- 田原さんに「✅これやってる」「❌これやってない」と答えてもらえる粒度
- 梱包変更作業（隠れコスト）を明示化
- 各工程の所要時間を測定できる

### 3. クローズドクエスチョン形式のチェックリスト

**目的**: Session 265の発見「プロセスの粒度が粗すぎてクローズドクエスチョンができない」を解決

**構成**:
- **Part 1**: 全体フロー（10プロセス）の確認
- **Part 2**: 受入検査詳細（11プロセス）の確認
  - 梱包変更作業について（最重要）
  - 員数確認について
  - 外観検査について
  - 測定（ノギス等）について
  - Excel記録作成について
  - 抜けている作業はないか？
- **Part 3**: 工数・時間に関する確認（相対的な比較）
- **Part 4**: 暗黙知の可視化

**効果**:
- 田原さん・杉山さんが答えやすい形式
- 「✅やってる」「❌やってない」「△部分的」の3択
- △や❌の場合は追加でヒアリング

---

## ワークショップでの使い方

### 準備する資料

1. **SIPOC図2枚**（印刷またはPC画面）:
   - [overall-flow-sipoc-template.drawio](overall-flow-sipoc-template.drawio)
   - [iqc-detailed-sipoc-template.drawio](iqc-detailed-sipoc-template.drawio)

2. **チェックリスト**（印刷）:
   - [process-checklist.md](process-checklist.md)

3. **参考資料**:
   - Session 260の現場写真（`sessions/session260/Photos/`）
   - Session 262の事前抽出情報（Excel行番号・品番入り）

### 進め方

1. **全体フローの確認**（20分）
   - SIPOC図を見せて「こういう流れですか？」と確認
   - チェックリスト Part 1 を使って各プロセスを確認
   - 抜けているプロセス・順序の違いをヒアリング

2. **受入検査詳細の確認**（40分）
   - SIPOC図を見せて「こういう流れですか？」と確認
   - チェックリスト Part 2 を使って各プロセスを確認
   - 特に「梱包変更作業」について詳細ヒアリング（Session 260の隠れコスト）

3. **工数・暗黙知の確認**（20分）
   - チェックリスト Part 3（工数・時間）
   - チェックリスト Part 4（暗黙知）

---

## Session 265からの改善

### Session 265での発見

**問題点**:
1. プロセスの粒度が粗すぎる（「検査準備」など抽象的）
2. クローズドクエスチョンができない
3. 全体フローの把握が必要（受入検査だけでは不十分）

**改善方針**:
1. 全体フロー確認用SIPOC作成
2. 受入検査プロセスの詳細化（10プロセス程度に分解）
3. クローズドクエスチョン形式への変換

### Session 266での実施

**達成できたこと**:
- ✅ 全体フロー確認用SIPOC作成（10プロセス）
- ✅ 受入検査プロセスの詳細化（11プロセス）
- ✅ クローズドクエスチョン形式のチェックリスト作成

**効果**:
- 田原さん・杉山さんに「これでいいか？」と聞けるレベルまで分解
- 梱包変更作業（隠れコスト）を明示化
- 各工程の所要時間を測定できる粒度

---

## 次セッションへの引き継ぎ

### Session 267: 田原さん・杉山さんとのSIPOCワークショップ実施

**前提**:
- Session 266で全体フロー・受入検査詳細のSIPOC作成完了
- クローズドクエスチョン形式のチェックリスト作成完了

**実施すること**:
1. SIPOCワークショップ実施（田原さん・杉山さん）
2. チェックリストを使ってプロセスの実態確認
3. 特に「梱包変更作業」の詳細ヒアリング
4. 抜けているプロセス・暗黙知の洗い出し
5. ワークショップ結果のまとめ・SIPOC修正

**読むべきファイル**:
- [sessions/session266/overall-flow-sipoc-template.drawio](overall-flow-sipoc-template.drawio)（全体フロー）
- [sessions/session266/iqc-detailed-sipoc-template.drawio](iqc-detailed-sipoc-template.drawio)（受入検査詳細）
- [sessions/session266/process-checklist.md](process-checklist.md)（チェックリスト）
- [sessions/session263/sipoc-workshop-execution-plan.md](../session263/sipoc-workshop-execution-plan.md)（実施計画）

---

## 参照

### プロジェクト内資料

- [sessions/session265/session-summary.md](../session265/session-summary.md) — Session 265（プロセス粒度の問題発見）
- [sessions/session262/sipoc-iqc-template.drawio](../session262/sipoc-iqc-template.drawio) — Session 262のSIPOC（粗い粒度）
- [sessions/session260/hearing-results.md](../session260/hearing-results.md) — Session 260（隠れコスト発見）
- [sessions/session263/sipoc-workshop-execution-plan.md](../session263/sipoc-workshop-execution-plan.md) — ワークショップ実施計画

---

*作成: Session 266 (2026-03-19)*
*出典: Session 265の発見事項、Session 260のヒアリング結果*
