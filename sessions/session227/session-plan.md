# Session 227 計画

**目的**: BBR優先順位問題のテスト完了

**前提**: Session 226で対策実装済み、Step 1のみ実施

---

## やること

| # | 作業 | 読むべきファイル | 参照コマンド |
|---|------|-----------------|--------------|
| 1 | Step 2-3 テスト実施 | session226/test-procedure.md | make connect, cfg-valget-bbr, message-scan |
| 2 | テスト結果をtest-procedure.mdに記録 | - | - |
| 3 | ドキュメント更新（テスト成功時） | config-layers-spec.md, 32-cfg-msgout-periodic-output.md | - |

---

## 詳細

### 1. Step 2-3 テスト実施

[test-procedure.md](../session226/test-procedure.md) に従って実施:

**Step 2: 修正後の動作確認**
- USB抜き差し + BE再起動
- connect（通常接続）
- cfg-valget-bbr → BBRの値が変わっていないことを確認
- set-periodic-output-flash
- message-scan

**Step 3: USB抜き差し後の確認**
- USB抜き差し + BE再起動
- connect-raw
- cfg-valget-bbr, cfg-valget-flash
- message-scan → **出力されているか確認**

### 2. テスト結果の記録

test-procedure.mdの「Session 226 テスト結果」セクションに追記。

### 3. ドキュメント更新

テスト成功後、以下を更新:

| ドキュメント | 追記内容 |
|-------------|----------|
| config-layers-spec.md | BBRの「値の存在」について |
| 32-cfg-msgout-periodic-output.md | USB vs UART1の説明 |

---

## 参照

- [session226/session-summary.md](../session226/session-summary.md) - 対策実装の詳細
- [session226/test-procedure.md](../session226/test-procedure.md) - テスト手順書

---

*作成: Session 226 (2026-03-17)*
