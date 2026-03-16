# Session 191 サマリー

**日付**: 2026-03-16
**目的**: MON-SPAN横軸（周波数オフセット）表示の実装

---

## 実施内容

### 1. 横軸の周波数オフセット表示を実装

- `SpectrumChartProps` に `centerHz` / `spanHz` を追加
- 横軸ラベルを周波数オフセット（MHz）で表示
- 計算式: `Freq(i) = center + span × (i - 128) / 256`（Integration Manual p.83準拠）

### 2. u-center風の10MHz刻み目盛りを追加

Session 187で保存したQiita画像を参考に実装:
- **メイン目盛り**: 10MHz刻み（ラベル付き）
- **サブ目盛り**: 5MHz刻み（グリッド線のみ）
- **細分目盛り**: 2.5MHz刻み（拡大時のみ）

### 表示例

| span | 目盛り |
|------|--------|
| 50MHz | `-20`, `-10`, `0`, `+10`, `+20` |
| 128MHz | `-60`, `-50`, ... `0` ... `+50`, `+60` |

---

## 変更ファイル

| ファイル | 内容 |
|----------|------|
| `components/MonSpanPanel.tsx` | 周波数オフセット表示、10MHz刻み目盛り |
| `components/MonSpanComparePanel.tsx` | 比較画面にも centerHz/spanHz を渡す |

---

## 残った作業

→ [session192/session-plan.md](../session192/session-plan.md) 参照

次セッションでは元々の計画を再確認してから進める。
