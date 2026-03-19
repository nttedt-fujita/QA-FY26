# Session 266 計画

**目的**: 全体フロー確認用SIPOC作成・受入検査プロセス詳細化

**前提**:
- Session 265で田原さんヒアリング項目を作成
- プロセスの粒度が粗すぎることが判明（クローズドクエスチョンができない）
- 全体フロー（発注 → 出荷）の把握が必要

---

## やること

| # | 作業 | 読むべきファイル | 参照コマンド |
|---|------|-----------------|--------------|
| 1 | 全体フロー確認用SIPOC作成 | sessions/session265/session-summary.md<br>sessions/session260/hearing-results.md | - |
| 2 | 受入検査プロセスの詳細化 | sessions/session262/sipoc-iqc-template.drawio<br>sessions/session265/tahara-hearing-items.md | - |
| 3 | クローズドクエスチョン形式への変換 | sessions/session265/session-summary.md | - |

---

## 詳細

### 1. 全体フロー確認用SIPOC作成

**目的**: 発注 → 出荷までの全体フローを可視化

**想定プロセス**:
1. 部品発注
2. 入庫予定決定
3. 納品（受け取り）
4. **受入検査**（詳細化対象）
5. 入庫
6. 出庫（組立へ）
7. 組立
8. 出荷検査
9. 倉庫保管
10. 出荷

**成果物**:
- `sessions/session266/overall-flow-sipoc-template.drawio`

---

### 2. 受入検査プロセスの詳細化

**現在のSIPOC**（粗い）:
1. 部品入荷
2. 検査準備（梱包確認）
3. 検査実施（外観・員数）
4. 記録作成（Excel入力）
5. 入庫/不合格処理

**詳細化後**（細かい）:
1. 部品入荷（納品受け取り）
2. 作業場所への移動
3. 検査段取り（場所・人員確保）
4. 開梱作業
5. **梱包変更作業**（Session 260で判明した隠れコスト）
6. 員数確認
7. 外観検査
8. 測定（ノギス等）
9. 入庫のための梱包作業
10. 入庫

**成果物**:
- `sessions/session266/iqc-detailed-sipoc-template.drawio`

---

### 3. クローズドクエスチョン形式への変換

**目的**: 田原さんに「✅これやってる」「❌これやってない」と答えてもらえる粒度にする

**作成内容**:
- 各プロセスのチェックリスト形式
- 具体的な作業内容を列挙

**成果物**:
- `sessions/session266/process-checklist.md`

---

## 期待される成果物

| ファイル | 内容 |
|----------|------|
| overall-flow-sipoc-template.drawio | 全体フロー確認用SIPOC（発注 → 出荷） |
| iqc-detailed-sipoc-template.drawio | 受入検査プロセス詳細化SIPOC（10プロセス） |
| process-checklist.md | プロセス確認用チェックリスト（クローズドクエスチョン形式） |

---

## 参照

- [sessions/session265/session-summary.md](../session265/session-summary.md) — Session 265サマリー
- [sessions/session265/tahara-hearing-items.md](../session265/tahara-hearing-items.md) — 田原さんヒアリング項目
- [sessions/session262/sipoc-iqc-template.drawio](../session262/sipoc-iqc-template.drawio) — 現在のSIPOC
- [sessions/session260/hearing-results.md](../session260/hearing-results.md) — Session 260のヒアリング結果

---

*作成: Session 265 (2026-03-19)*
