# Session 151 計画

**目的**: 定期出力無効化が効いていない理由を調査

**前提**: Session 150で問題の経緯を整理済み

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|--------------------|
| 1 | CFG-VALGETでF9Pの現在設定を読み出す | ubx仕様書 |
| 2 | 定期出力が本当に無効化されているか確認 | - |
| 3 | 「不明データ」の正体を特定 | ログファイル |
| 4 | 単発ポーリングで値が正しく取れるか再確認 | - |
| 5 | 根本原因を特定して修正 | - |

---

## 調査方法

### CFG-VALGET

F9Pの現在設定を読み出すUBXコマンド:
- Class: 0x06 (CFG)
- ID: 0x8B (VALGET)
- 読み出すキー: CFG-MSGOUT-UBX_NAV_PVT_USB等

### ログ分析

「不明データ」のバイト列を詳細分析:
- UBXフレームの一部か？
- 定期出力されているメッセージか？
- 全く別のデータか？

---

## 参照

- [Session 150 comparison-analysis](../session150/comparison-analysis.md)
- [Session 150 summary](../session150/session-summary.md)
- [ADR-012](../../docs/adr/m1/ADR-012-periodic-output-and-unified-api.md)
