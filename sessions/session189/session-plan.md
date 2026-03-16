# Session 189 計画

**前提**: Session 188で比較画面のレイアウト改善完了

---

## やること

### 1. 比較機能の動作確認

- 新レイアウトで正しく表示されるか確認
- L1/L2の色分けが正しいか確認
- 各枠で実線/点線が正しく表示されるか確認

### 2. MON-SPAN spectrum値の1次情報調査（Session 187から継続）

**目的**: spectrum値（0-255）とdB表示の関係を1次情報で確認

**調査対象**:
1. u-blox Interface Description の詳細確認
2. u-blox Application Note の調査
3. u-blox Forum/Portal の調査
4. 学術論文/技術文献

**確認すべき観点**:
- spectrum値（U1: 0-255）は生のADC値か、既にdB変換済みか
- dB表示する際の変換式があるか
- u-centerがどのような処理をしているか

### 3. 調査結果に基づく実装方針決定

調査結果をもとに:
- 縦軸の表示方法を決定
- 必要なら変換式を実装

---

## 読むべきファイル

| ドキュメント | 内容 |
|-------------|------|
| [session188/session-summary.md](../session188/session-summary.md) | 前セッションサマリー |
| [session187/mon-span-spec.md](../session187/mon-span-spec.md) | MON-SPAN仕様抽出 |

---

## 参照

- [Session 188 サマリー](../session188/session-summary.md)
- [Session 187 サマリー](../session187/session-summary.md)
