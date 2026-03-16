# Session 188 計画

**前提**: Session 187でu-center目盛り調査を実施、1次情報による根拠固めが不足

---

## やること

### 1. MON-SPAN spectrum値の1次情報調査（最優先）

**目的**: spectrum値（0-255）とdB表示の関係を1次情報で確認

**調査対象**:
1. **u-blox Interface Description** の詳細確認
   - MON-SPANセクション以外にも関連記述がないか
   - 「dB」の定義、基準点、変換式

2. **u-blox Application Note** の調査
   - RF/Spectrumに関するアプリケーションノート
   - MON-SPANの使い方ガイド

3. **u-blox Forum/Portal** の調査
   - MON-SPANに関するQ&A
   - spectrum値の解釈に関する公式回答

4. **学術論文/技術文献**
   - GNSSレシーバーのスペクトラム表示に関する文献
   - RF振幅のdB表現に関する標準

**確認すべき観点**:
- spectrum値（U1: 0-255）は生のADC値か、既にdB変換済みか
- dB表示する際の変換式があるか
- u-centerがどのような処理をしているか

### 2. 比較機能のバグ調査

**問題**: Session 187の画像で、比較表示の波形の形がおかしい

**期待する動作**:
- 左データ(ID:39)のL1と右データ(ID:32)のL1を重ねる
- 左データ(ID:39)のL2と右データ(ID:32)のL2を重ねる

**実際の表示**:
- 波形の形が違いすぎる
- 正しく重ねられていない可能性

**調査すべき点**:
- MonSpanComparePanel.tsx のデータ取得ロジック
- 比較ページでのデータの紐付け
- L1/L2の対応が正しいか

### 3. 調査結果に基づく実装方針決定

調査結果をもとに:
- 縦軸の表示方法を決定
- 必要なら変換式を実装
- 比較機能のバグ修正

---

## 読むべきファイル

| ドキュメント | 内容 |
|-------------|------|
| [session187/session-summary.md](../session187/session-summary.md) | 前セッションサマリー |
| [session187/mon-span-spec.md](../session187/mon-span-spec.md) | MON-SPAN仕様抽出 |
| [ubx-mon-messages.md](../../docs/missions/m1-sensor-evaluation/gnss/ubx-mon-messages.md) | 既存の仕様まとめ |

---

## 参照

- [Session 187 サマリー](../session187/session-summary.md)
- [Qiita: u-blox受信機のスペアナ機能](https://qiita.com/XPT60/items/afded5855d2bf2817e4a)
