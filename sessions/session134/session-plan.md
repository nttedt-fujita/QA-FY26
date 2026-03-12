# Session 134 計画

**目的**: NTRIP API テスト改善 + フロントエンド接続ボタン

**前提**: Session 133でNTRIP API実装完了。TDDレビューで改善点が指摘された。

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | テストをテーブルテスト形式に書き直す | ~/.claude/rules/06-test-style.md |
| 2 | 不足テストケース追加 | sessions/session133/session-summary.md（TDDレビュー指摘） |
| 3 | フロントエンドに接続/切断ボタン追加 | prototype/m1-gnss/frontend/src/app/settings/page.tsx |

---

## 詳細

### 1. テーブルテスト形式への書き直し

TDDレビューで指摘された違反を修正:
- `should_succeed` パラメータを含める
- `pytest.mark.parametrize` 相当のRust形式で書く

対象テスト:
- `test_ntrip_manager_*`
- `test_base64_encode`

### 2. 不足テストケース追加

TDDレビューで指摘された不足テスト:
- NtripManager 状態遷移
- Base64 境界値（空文字列、パディングバリエーション）
- APIハンドラー統合テスト（既接続時のConflict等）

### 3. フロントエンド接続ボタン

設定画面に追加:
- 「接続」ボタン → POST /api/ntrip/connect
- 「切断」ボタン → POST /api/ntrip/disconnect
- 接続状態表示 → GET /api/ntrip/status

---

## 確認事項

- GNSSView API使用の経緯を確認（Session 114あたりで議論）
  - 結局どういう実装方針になったか整理

---

## 参照

- [Session 133 summary](../session133/session-summary.md)
- [06-test-style.md](~/.claude/rules/06-test-style.md)
- [ntrip_api.rs](../../prototype/m1-gnss/backend/src/web/ntrip_api.rs)

---

*計画作成: 2026-03-12 Session 133終了時*
