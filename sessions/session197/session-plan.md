# Session 197 計画

**前提**: Session 196 で MultiDeviceManager を API に統合完了。既存 API は後方互換性を維持。

---

## やること

### 選択肢

Session 196 で Phase 3 の BE 統合が完了。次の方針を選択：

| # | 作業 | 内容 |
|---|------|------|
| A | FE 複数台対応 | デバイス選択 UI、比較画面でのデバイス指定 |
| B | 比較 API 追加 | 2 台同時データ取得の API を追加 |
| C | 実機テスト | 2 台の F9P を接続して動作確認 |

---

## 参照

| ドキュメント | 内容 |
|-------------|------|
| [session196/session-summary.md](../session196/session-summary.md) | 前セッションの成果（API 統合） |
| [ADR-014](../../docs/adr/m1/ADR-014-multi-device-manager.md) | 複数装置同時対応の実装方式 |
