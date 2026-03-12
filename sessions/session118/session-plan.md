# Session 118 計画

**目的**: MON-SPAN API TDDレビュー + FE実装

---

## 背景

Session 117でMON-SPAN API（`GET /api/mon-span`）を実装完了。
ただし仕様書確認プロセスに問題があったため、TDDスキルでヌケモレチェックを行う。

---

## 最優先: TDDレビュー

Session 117で実装した範囲に対してTDDスキルを使ってレビュー:

1. **仕様書確認**
   - ubx-mon-messages.md を読む
   - mon_span_api.rs のテストリストと照合

2. **TDDスキルでヌケモレチェック**
   - 振る舞い記述の網羅性
   - テストケースの漏れ
   - エッジケース・異常系

3. **必要に応じてテスト追加**

4. **仕様書参照ルールの強制化**
   - CLAUDE.mdに仕様書ファイルパスを明記するルール追加
   - 実装前に仕様書を読むことを強制

---

## その後のやること候補

### 案A: MON-SPAN FE実装

1. **仕様確認**（必須）
   - ubx-mon-messages.md を読む
   - mon_span_api.rs のレスポンス形式を確認

2. **既存パターン確認**
   - NavSigPanel.tsx を参考にする

3. **実装**
   - api.ts に getMonSpan() 追加
   - MonSpanPanel.tsx 新規作成
     - スペクトラム波形表示（256点のグラフ）
     - PGAゲージ（38dB異常 vs 54dB正常）
   - inspections/page.tsx に統合

### 案B: 屋内/屋外検査ページ分離

- Session 113で出たユーザー要望
- 現在の `/inspections` を分割

---

## 注意事項

**IMPORTANT**: Session 117で発覚した問題の再発防止

- 実装前に仕様書（ubx-mon-messages.md等）を必ず読む
- 読んだファイルを明記する
- 「既存コードと同じパターンで」だけで進めない

---

## 参照資料

- [ubx-mon-messages.md](../../docs/missions/m1-sensor-evaluation/gnss/ubx-mon-messages.md) — MON-SPAN仕様
- [mon_span_api.rs](../../prototype/m1-gnss/backend/src/web/mon_span_api.rs) — API実装
- [NavSigPanel.tsx](../../prototype/m1-gnss/frontend/src/components/NavSigPanel.tsx) — 参考FE

---

*計画作成: 2026-03-12 Session 117終了時*
