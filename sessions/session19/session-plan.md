# Session 19 計画

**目的**: 合格基準のエビデンス収集（Web調査）+ PDF詳細分析

---

## 背景（Session 18の結論）

- Excel全シートのデータ分析が完了（テキスト + 画像）
- 統合ドキュメントを `docs/missions/m1-sensor-evaluation/gnss/` に整理済み
- **合格基準の叩き台は作ったが、根拠となるエビデンスがない**
- PDF（UBX仕様書）は主要メッセージ部分のみ読了（305ページ中の一部）

---

## やること

### 1. PDF（UBX仕様書）の詳細分析

Session 17で未読の範囲を確認し、Phase 2（ツール設計）に必要な情報を整理する。

- p.1-131: Configuration、プロトコル概要等
- p.159-305: RTCM、SPARTN、その他メッセージ
- 特に注目: RTK関連の設定パラメータ、推奨構成

### 2. Web調査 — 合格基準のエビデンス収集

07-cross-sheet-findings.md の叩き台に根拠を与えるための調査。

| 調査項目 | 探すべきエビデンス |
|---------|----------------|
| GNSS受信感度の合格基準 | u-blox公式のアプリケーションノート、推奨C/N0閾値 |
| RTK FIX時間の業界標準 | 農業ドローン・測量分野での許容時間 |
| RTK FIXに必要な衛星数 | RTK演算に最低限必要な衛星数の技術文献 |
| L2受信の要否と閾値 | 二周波RTKの要件（論文・メーカー資料） |
| PGA値の正常範囲 | u-blox F9Pのアンテナ設計ガイドライン |
| MAGキャリブレーション基準 | ArduPilot/PX4の推奨値 |

### 3. 末永さんヒアリング準備

Web調査の結果を踏まえて、ヒアリング項目を更新する。
「業界ではこう言われているが、当社の運用ではどう考えるか？」という形で質問を構成。

---

## 参考資料

- [Session 18 統合ドキュメント](../../docs/missions/m1-sensor-evaluation/gnss/README.md)
- [合格基準叩き台・ヒアリング項目](../../docs/missions/m1-sensor-evaluation/gnss/07-cross-sheet-findings.md)
- [UBX仕様書索引](../../docs/missions/m1-sensor-evaluation/gnss/08-ubx-protocol-index.md)
- [Session 17 PDF読込範囲](../session17/files-reviewed.md)
