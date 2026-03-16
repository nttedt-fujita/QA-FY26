# Session 191 計画

**前提**: Session 190でdB表示を実装済み。横軸（周波数）の実装が残っている。

---

## やること

### 1. 横軸（周波数）の表示実装

**仕様** (37-mon-span-display-spec.md より):
```
Freq(i) = center + span × (i - 128) / 256
```

- `i`: bin番号（0〜255）
- `center`: 中心周波数 [Hz]
- `span`: スペクトラム幅 [Hz]

**実装内容**:
- SpectrumChartPropsにcenter/span（オプション）を追加
- 横軸ラベルを周波数オフセット（MHz）で表示
- u-center風に `-50 -40 -30 -20 -10 0 +10 +20 +30 +40 +50` MHz

### 2. 動作確認

- 実機データで表示を確認
- u-centerと並べて比較

---

## 読むべきファイル

| ドキュメント | 内容 |
|-------------|------|
| [37-mon-span-display-spec.md](../../docs/missions/m1-sensor-evaluation/gnss/37-mon-span-display-spec.md) | MON-SPAN表示仕様 |
| [session190/session-summary.md](../session190/session-summary.md) | 前セッションサマリー |
