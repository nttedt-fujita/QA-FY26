# Session 53 サマリー

**日時**: 2026-03-09
**目的**: M1-B GNSS設計検証基準の業界標準調査

---

## 実施内容

### 1. GNSS設計検証基準の調査

業界標準・学術的根拠・メーカー仕様を調査し、Session 17-18で作成した叩き台の妥当性を検証。

#### 主な発見

| 項目 | 叩き台 | 業界標準 | 判定 |
|------|--------|---------|------|
| L1受信感度 | ≥25 dBHz | **≥30 dBHz** | 叩き台は甘すぎる |
| RTK FIX時間 | ≤3分 | **≤30秒** | 叩き台は緩すぎる |
| 飛行中水平精度 | ≤1.0m | cm級 | 用途次第で妥当 |
| 飛行中衛星数 | ≥20 | 6-8 | 叩き台は厳しめ（良い方向） |

#### 叩き台に含まれていない項目

- TTFF（Cold/Warm/Hot Start）
- PDOP/HDOP
- 再捕捉時間
- ジャミング耐性

### 2. 文脈の整理

**重要な気づき**: 元の評価は「受入検査」ではなく「**原理試作に対する設計検証**」だった。
Session 16で小板橋さんから聞いていたが、READMEに記載されていなかった。

### 3. ドキュメント整理

- README.mdに評価の文脈を追記
- 調査資料を然るべき場所（docs/missions/m1-sensor-evaluation/gnss/）に配置
- 07-cross-sheet-findings.md を業界標準調査結果で更新

---

## 作成・更新ファイル

| ファイル | 内容 |
|----------|------|
| [docs/missions/m1-sensor-evaluation/gnss/README.md](../../docs/missions/m1-sensor-evaluation/gnss/README.md) | 評価の文脈（原理試作/設計検証）を追記 |
| [docs/missions/m1-sensor-evaluation/gnss/07-cross-sheet-findings.md](../../docs/missions/m1-sensor-evaluation/gnss/07-cross-sheet-findings.md) | 業界標準調査結果で更新、質問リスト整理 |
| [docs/missions/m1-sensor-evaluation/gnss/09-design-verification-criteria.md](../../docs/missions/m1-sensor-evaluation/gnss/09-design-verification-criteria.md) | **新規作成** 設計検証基準の業界標準調査レポート |
| [session53/gnss-acceptance-criteria-research.md](gnss-acceptance-criteria-research.md) | 調査作業ファイル（正式版は09に配置） |

---

## 重要な決定・学び

1. **評価の文脈は最初にREADMEに書く** — 何のための評価かを明記しておかないと、後から混同する
2. **業界標準は設計検証でも参考になる** — 閾値（C/N0 30dBHz等）は汎用的

---

## 次セッション（Session 54）でやること

1. **小板橋さんに認識確認**
   - 07-cross-sheet-findings.mdの質問リスト参照
   - 特に: 現在の判定基準、TTFF/PDOPの確認有無

2. **末永さんへの相談内容精査**（時間があれば）
   - AirGrowの飛行制御でGNSSに要求される精度
   - RTK FIX時間の要件

3. **M4工程不良Excel入手**（継続）

---

## 未解決事項

- 小板橋さん・末永さんへの確認は未実施
- M2確認リストの回答収集は未着手
