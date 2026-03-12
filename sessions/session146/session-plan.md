# Session 146 計画

**目的**: 定期出力無効化後の動作確認

**前提**: Session 145で定期出力を無効化（ポーリング専用に変更）

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | **実行時ログをファイルに出力する設定** | backend/src/main.rs |
| 2 | バックエンド再起動後の動作確認 | - |
| 3 | 問題があれば修正 | 該当ファイル |
| 4 | ADR-012を更新 | docs/adr/m1/ADR-012-periodic-output-and-unified-api.md |

---

## 詳細

### 1. 実行時ログをファイルに出力

状況把握のため、バックエンドの実行時ログをファイルに残す設定を追加。
- env_loggerまたはtracing-subscriberでファイル出力設定
- ログローテーションも検討

### 2. 動作確認

- 屋外検査画面で統合APIが正しく動作するか確認
- 各パネル（NAV-STATUS, SkyPlot, NAV-SIG, MON-SPAN）が表示されるか
- エラーが発生しないか

### 3. ADR-012を更新

Session 145で決定した内容を追記:
- 定期出力を無効化した理由
- 受信スレッド未実装のため、ポーリング専用で運用

---

## 参照

- [Session 145 サマリー](../session145/session-summary.md)
- [ADR-012](../../docs/adr/m1/ADR-012-periodic-output-and-unified-api.md)
