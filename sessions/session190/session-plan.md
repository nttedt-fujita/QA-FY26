# Session 190 計画

**前提**: Session 189で比較画面の色分け完了、MON-SPAN dB変換式を調査（式2が有力）

---

## やること

### 1. MON-SPAN dB変換式の確定と実装

**調査結果（Session 189）**:
- 式2: `dB = spectrum / 6.375 + 20` が整合する可能性が高い
- ただし1次情報による根拠は未確認

**実装内容**:
- 縦軸の目盛りをdB表示に変更（20〜60 dB）
- spectrum値をdBに変換して描画
- 注記: 「u-center表示との整合性に基づく推定値」

### 2. 横軸（周波数）の表示改善

**仕様書の計算式**:
```
f(i) = center + span * (i - 127) / 256
```

**u-center表示**:
- 横軸: `-50 -40 -30 -20 -10 +10 +20 +30 +40 +50` MHz（中心からのオフセット）

**実装内容**:
- 横軸を周波数オフセット（MHz）で表示
- または中心周波数からの相対位置で表示

### 3. 動作確認

- 実機データで表示を確認
- u-centerと並べて比較

---

## 読むべきファイル

| ドキュメント | 内容 |
|-------------|------|
| [session189/session-summary.md](../session189/session-summary.md) | 前セッションサマリー |
| [session187/mon-span-spec.md](../session187/mon-span-spec.md) | MON-SPAN仕様抽出 |
| [session187/Qiita-u-center-image.png](../session187/Qiita-u-center-image.png) | u-center参考画像 |

---

## 参照

- [Session 189 サマリー](../session189/session-summary.md)
- [Session 187 サマリー](../session187/session-summary.md)
