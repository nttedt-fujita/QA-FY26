# Session 149 サマリー

**実施日**: 2026-03-12
**目的**: 定期出力の無効化をUART1用キーで修正

---

## 実施内容

1. **UART1用キーを追加**
   - cfg_valset.rsに6つのUART1用キーを追加
   - CFG-MSGOUT-UBX_NAV_PVT_UART1等

2. **disable_periodic_outputを修正**
   - USB用6キー + UART1用6キー = 12キーを設定するように変更
   - テストも12キー対応に更新

3. **動作確認で問題発覚**
   - UART1用キー追加後も「不明データあり」が続く
   - Session 130との差分を確認 → Session 130では定期出力を制御していなかった

4. **Layer::Bbrに変更**
   - Layer::Ram → Layer::Bbr に変更
   - BBR（バッテリバックアップRAM）に書き込むことで、電源を切っても設定が残る
   - 次回起動時に確認が必要

---

## 決定事項

| 項目 | 決定 |
|------|------|
| 定期出力無効化のレイヤー | Layer::Bbr（不揮発性） |

---

## 変更ファイル

| ファイル | 内容 |
|----------|------|
| cfg_valset.rs | UART1用キー追加、disable_periodic_output修正 |
| device_api.rs | Layer::Ram → Layer::Bbr |

---

## 残った課題

- Layer::Bbr変更後の動作確認（次セッションで実施）
- デバッグログの量が多いかどうかの確認
- ADR-012の更新

---

## 次セッション（Session 150）でやること

1. Layer::Bbr変更後の動作確認
2. 問題が解決していれば、ADR-012を更新
3. 解決していなければ、Session 130方式（定期出力を制御しない）を検討
