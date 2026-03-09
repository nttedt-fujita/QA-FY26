# Session 55 計画

**日時**: 2026-03-XX（予定）
**前提**: Session 54で確認資料作成完了

---

## 目的

M3ネットワークアクセス対応、小板橋さん・末永さんへの確認

---

## タスク

### 0. M3プロトタイプ：別PCからのアクセス対応（優先）

**やりたいこと**: 同じネットワーク内の別PCからM3プロトタイプにアクセスできるようにする

**確認事項**:
- 現在のプロトタイプ構成（フロントエンド/バックエンド）
- ホスト設定（localhost → 0.0.0.0等）
- ファイアウォール設定

---

### 1. 小板橋さんへ確認

**資料**: [session54/koitabashi-confirmation-checklist.md](../session54/koitabashi-confirmation-checklist.md)

**確認項目**:
1. 現在の判定方法（暗黙の基準があるか）
2. C/N0 ≥30 dBHzを基準にしてよいか
3. RTK FIX時間 ≤30-60秒を基準にしてよいか
4. TTFF/PDOPは確認しているか
5. 小峰無線との合意状況

### 2. 末永さんへ相談

**資料**: [session54/suenaga-consultation-checklist.md](../session54/suenaga-consultation-checklist.md)

**確認項目**:
1. 水平精度要件（1.0mで十分か、cm級が必要か）
2. RTK FIX時間要件
3. RTK FIXできない場合のフォールバック動作
4. 最低衛星数

### 3. ツール作成タイミングの議論

**問い**: Phase 1（検証項目の妥当性検証）完了の条件は何か？

**小板橋さんからのコメント（Session 16）**:
> ツール作成の前に、検証項目が適切かどうかの検証をして、その部分を固めるのが重要

**確認すべきこと**:
- 合格基準が決まったらPhase 1完了？
- それとも実際に基準を使って数回検証してから？
- ツール作成の優先度は？（他のミッションとの兼ね合い）

### 4. M4工程不良Excel入手（継続）

**依頼先**: 小笠原さん or 石川さん

---

## 参照資料

- [Session 54サマリー](../session54/session-summary.md)
- [GNSS設計検証基準調査レポート](../../docs/missions/m1-sensor-evaluation/gnss/09-design-verification-criteria.md)
- [受信感度シート分析](../../docs/missions/m1-sensor-evaluation/gnss/02-reception-sensitivity.md)
