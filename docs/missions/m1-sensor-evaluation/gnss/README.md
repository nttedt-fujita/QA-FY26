# M1-B GNSS評価 — 小峰無線GPS確認データ分析

**元データ**: 小峰無線GPS確認.xlsx（23MB）+ u-blox F9 HPG 1.32 Interface Description PDF
**評価対象機体**: Ref（RKP001）、No.1、No.2、No.3、No.5 + シルクなし機体（0922, 0909, 0932, 0952, 0754）
**分析期間**: Session 17（テキストデータ）+ Session 18（画像データ）

---

## ドキュメント一覧

| ファイル | 内容 | 元シート |
|---------|------|---------|
| [01-internal-settings.md](01-internal-settings.md) | 内部設定（FW/PROTVER/パラメータ） | 内部設定 |
| [02-reception-sensitivity.md](02-reception-sensitivity.md) | 受信感度（C/N0）全3回測定データ + u-center画像分析 | 受信感度 / 受信感度_仰角 |
| [03-spectrum-analyze.md](03-spectrum-analyze.md) | スペクトラムアナライザ波形分析 | Spectrum Analyze |
| [04-flight-test.md](04-flight-test.md) | 飛行試験データ + 飛行中ログ時系列分析 | 20260218 |
| [05-mag-check.md](05-mag-check.md) | 磁気コンパスずれ確認 | Mag確認 |
| [06-battery-check.md](06-battery-check.md) | バックアップ電池確認 | 電池確認 |
| [07-cross-sheet-findings.md](07-cross-sheet-findings.md) | 全シート横断の発見事項・合格基準検討・Phase 1の問い | — |
| [08-ubx-protocol-index.md](08-ubx-protocol-index.md) | UBXプロトコル仕様書の索引（Phase 2用） | PDF |

---

## Phase 1ロードマップにおける位置づけ

```
Phase 0: 現状把握        ← 完了（Session 15-16）
Phase 1: 検証項目の妥当性検証  ← 現在（Session 17-18でデータ読み込み完了）
Phase 2: ツール設計
Phase 3: ツール実装
Phase 4: 標準化
```

Phase 1の次のステップ: 合格基準の検討 → 末永さんヒアリング

---

## 作成経緯

- Session 17: Excelの全シートテキストデータ + PDFの主要メッセージ定義を分析
- Session 18: Excel内の画像（スクショ）を読み取り、テキストでは得られなかった波形・グラフ・GCS画面を補完
- 本ドキュメント群: 上記2セッションの分析結果を統合し、シートごとに整理
