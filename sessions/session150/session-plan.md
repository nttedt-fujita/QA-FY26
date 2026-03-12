# Session 150 計画

**目的**: Layer::Bbr変更後の動作確認

**前提**: Session 149でLayer::Ram → Layer::Bbrに変更済み

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|--------------------|
| 1 | バックエンド起動 + 実機接続 | - |
| 2 | 統合APIの動作確認（タイムアウト減少を確認） | logs/ |
| 3 | 問題解決していればADR-012更新 | docs/adr/m1/ADR-012-periodic-output-and-unified-api.md |
| 4 | 解決していなければSession 130方式を検討 | git diff 4c53ecf HEAD |

---

## 詳細

### 確認ポイント

- 「不明データあり」のログが減少しているか
- 統合APIのタイムアウトが減少しているか

### 問題が解決していない場合の選択肢

1. Session 130方式に戻す（定期出力を制御しない）
2. デバッグログの量を減らす

---

## 参照

- [Session 149 summary](../session149/session-summary.md)
- [Session 148 log-analysis-report](../session148/log-analysis-report.md)
