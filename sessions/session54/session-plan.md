# Session 54 計画

**日時**: 2026-03-XX（予定）
**前提**: Session 53でGNSS設計検証基準の業界標準調査完了

---

## 目的

小板橋さんへの認識確認、末永さんへの相談内容精査

---

## タスク

### 0. 製品名・メーカーの裏取り（優先）

**問題**: 「HORIBAウルトラライト」と記載しているが、正しくは「**ホリブロウルトラライト**」の可能性。

**やること**:
1. ウェブ調査で正式な製品名・メーカーを確認
2. 関連ドキュメントの製品名を修正
   - docs/missions/m1-sensor-evaluation/gnss/README.md
   - docs/missions/m1-sensor-evaluation/gnss/09-design-verification-criteria.md
   - sessions/session16/gnss-hearing-koitabashi-01.md

---

### 1. 小板橋さんへの認識確認（優先）

**参照**: [docs/missions/m1-sensor-evaluation/gnss/07-cross-sheet-findings.md](../../docs/missions/m1-sensor-evaluation/gnss/07-cross-sheet-findings.md)

**確認項目**:
1. No.5の13 dBHzはどの衛星・どの条件で測定されたか？
2. 現在の設計検証で使っている判定基準は？
3. TTFF（Cold/Warm/Hot Start）は確認しているか？
4. PDOPの閾値は設定しているか？
5. 小峰無線との内部設定合意は完了したか？

### 2. 末永さんへの相談内容精査

**参照**: 同上

**確認項目**:
1. AirGrowの飛行制御でGNSSに要求される精度は？
2. RTK FIX時間の要件は？
3. 再捕捉時間（Reacquisition Time）の要件は？

### 3. M4工程不良Excel入手（継続）

**依頼先**: 小笠原さん or 石川さん

---

## 優先順位

1. 小板橋さんへの認識確認
2. 末永さんへの相談内容精査
3. M4工程不良Excel入手

---

## 参照資料

- [Session 53サマリー](../session53/session-summary.md)
- [GNSS設計検証基準調査レポート](../../docs/missions/m1-sensor-evaluation/gnss/09-design-verification-criteria.md)
- [全シート横断の発見事項](../../docs/missions/m1-sensor-evaluation/gnss/07-cross-sheet-findings.md)
