# Session 226 サマリー

**日時**: 2026-03-17
**目的**: BBR優先順位問題の解決

---

## 実施内容

### 1. 対策の実装

`disable_periodic_output`をRAMのみに変更:

```rust
// 変更前
disable_periodic_output(Layer::RamAndBbr)

// 変更後
disable_periodic_output(Layer::Ram)
```

**変更ファイル**: [device_api.rs:469-472](../../prototype/m1-gnss/backend/src/web/device_api.rs#L469-L472)

**理由**:
- BBRに値が「存在する」とFlashより優先される
- 接続時の一時的な無効化はRAMのみで十分

### 2. 現状確認（Step 1のみ実施）

USB抜き差し + BE再起動後、connect-rawで接続して状態確認:

| レイヤー | NAV_PVT_USB |
|----------|-------------|
| BBR | 0 |
| Flash | 1 |

→ これが「BBRの0がFlashの1より優先される」状態

---

## 作成ファイル

| ファイル | 内容 |
|----------|------|
| [test-procedure.md](test-procedure.md) | テスト手順書（USB抜き差し→BE再起動→状態確認→message-scanをセット） |

---

## 未完了事項（次セッション）

### テスト実施

[test-procedure.md](test-procedure.md) の Step 2-3 を実施:

1. 修正後の動作確認（connectでBBRに書き込まないこと）
2. USB抜き差し後の確認（Flashの設定が有効になること）

### ドキュメント更新

テスト成功後、以下を更新:

| ドキュメント | 追記内容 |
|-------------|----------|
| config-layers-spec.md | BBRの「値の存在」について |
| 32-cfg-msgout-periodic-output.md | USB vs UART1の説明 |

---

## 使用したコマンド

| 用途 | コマンド |
|------|----------|
| 接続（disable_periodic_outputスキップ） | make connect-raw |
| BBR状態確認 | make cfg-valget-bbr |
| Flash状態確認 | make cfg-valget-flash |

---

*作成: Session 226 (2026-03-17)*
