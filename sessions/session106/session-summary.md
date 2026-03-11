# Session 106 サマリー

**日付**: 2026-03-11
**目的**: MON-SPAN仕様確認 + 屋外検査要求整理

---

## 実施内容

### 1. UBX仕様書からの情報抽出

Pythonスクリプト（PyMuPDF）でPDFから目次抽出→該当ページ読み取りを実施。

**抽出したメッセージ**:
- MON-SPAN (0x0a 0x31) — p.134
- NAV-SAT (0x01 0x35) — p.150-151
- NAV-SIG (0x01 0x43) — p.152-154

### 2. MON-SPAN仕様の確認

| 項目 | 値 |
|------|-----|
| Type | Periodic/polled |
| スペクトラムデータ | 256点 (U1[256]) |
| 単位 | dB |
| 中心周波数計算 | f(i) = center + span × (i - 127) / 256 |
| 128MHz/256点の分解能 | 500kHz |

### 3. NAV-SAT vs NAV-SIG の比較

**結論**: 屋外検査では**NAV-SIG**を使用（L1/L2別のC/N0が必要）

| 項目 | NAV-SAT | NAV-SIG |
|------|---------|---------|
| 粒度 | 衛星単位 | 信号単位（L1/L2別） |
| C/N0 | 衛星代表値 | 信号ごとの値 |
| 用途 | スカイプロット | 受信感度評価 |

### 4. 屋外検査の要求整理

優先度の高い要求（What）:
1. L1/L2別の受信感度（C/N0）確認 — NAV-SIG
2. 衛星ごとの仰角・方位角一覧 — NAV-SAT
3. L1/L2波の受信状態一覧化 — NAV-SIG
4. RTK FIX時間測定 — NAV-PVT（既存）

---

## 作成ファイル

| ファイル | 内容 |
|----------|------|
| [ubx-spec-memo.md](ubx-spec-memo.md) | MON-SPAN, NAV-SAT, NAV-SIG仕様メモ |
| [outdoor-inspection-needs.md](outdoor-inspection-needs.md) | 屋外検査の要求整理（What） |
| extracted/*.txt | PDF抽出結果 |

---

## hooks観察

**観察1**: PDF抽出スクリプトを毎回新規作成している
- 既存のtools/pdf_page_extractor.pyを使うべきだった
- 改善案: tools/にPDF用共通ツールを整備

---

## 次セッション（Session 107）でやること

1. NAV-SIGパーサー実装（TDD）
2. 受信感度一覧表示の設計

---

*作成: 2026-03-11 Session 106終了時*
