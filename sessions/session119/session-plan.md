# Session 119 計画

**目的**: MON-SPAN FE実装

---

## 背景

Session 116-117でMON-SPANパーサー・APIを実装完了。
Session 118でTDDレビューを実施し、テストカバレッジを確認。

次はフロントエンドでスペクトラム波形とPGAゲージを表示する。

---

## やること

### 1. 仕様確認（必須）

**IMPORTANT**: 新ルール `13-spec-first-implementation.md` に従う

- [ubx-mon-messages.md](../../docs/missions/m1-sensor-evaluation/gnss/ubx-mon-messages.md) を読む
- [23-mon-span-implementation.md](../../docs/missions/m1-sensor-evaluation/gnss/23-mon-span-implementation.md) を読む
- [mon_span_api.rs](../../prototype/m1-gnss/backend/src/web/mon_span_api.rs) のレスポンス形式を確認

### 2. 既存パターン確認

- [NavSigPanel.tsx](../../prototype/m1-gnss/frontend/src/components/NavSigPanel.tsx) を参考にする

### 3. 実装

1. **api.ts に getMonSpan() 追加**
   - MonSpanResponse 型定義
   - SpanBlock 型定義

2. **MonSpanPanel.tsx 新規作成**
   - スペクトラム波形表示（256点のグラフ）
   - PGAゲージ（38dB異常 vs 54dB正常）
   - 合格/不合格表示

3. **inspections/page.tsx に統合**

---

## 注意事項

- PGA 38dB → 異常（No.5異常機）
- PGA 54dB → 正常
- スペクトラムは比較分析用（絶対値の精度は高くない）

---

## 参照資料

- [23-mon-span-implementation.md](../../docs/missions/m1-sensor-evaluation/gnss/23-mon-span-implementation.md) — 振る舞い・テストリスト
- [ubx-mon-messages.md](../../docs/missions/m1-sensor-evaluation/gnss/ubx-mon-messages.md) — MON-SPAN仕様
- [NavSigPanel.tsx](../../prototype/m1-gnss/frontend/src/components/NavSigPanel.tsx) — 参考FE

---

*計画作成: 2026-03-12 Session 118終了時*
