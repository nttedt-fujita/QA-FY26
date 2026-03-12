# Session 148 サマリー

**日付**: 2026-03-12
**目的**: 統合APIの問題原因特定（デバッグログ追加）

---

## 実施内容

### 1. 仮説の整理

前回（Session 147）の修正後もタイムアウト/エラーが発生していた。
仮説を立ててデバッグログで検証する方針を決定。

### 2. デバッグログ追加

`receive_ubx` でUBXフレーム抽出成功時にClass/IDを出力するよう修正:
```rust
// 変更前
debug!("receive_ubx: UBXフレーム抽出成功（{}バイト）", ubx_frame.len());

// 変更後
let class = ubx_frame.get(2).copied().unwrap_or(0);
let id = ubx_frame.get(3).copied().unwrap_or(0);
debug!("receive_ubx: UBXフレーム抽出成功（{}バイト, Class=0x{:02X}, ID=0x{:02X}）", ubx_frame.len(), class, id);
```

### 3. ログ分析で原因特定

**発見した問題**:
- 統合APIで NAV-STATUS, NAV-SIG, MON-SPAN がタイムアウトしている
- 「不明データ」として500バイト前後のデータが混入（MON-SPANのスペクトラムデータ）

**根本原因**:
- `disable_periodic_output` で **USB用キー** を使用している
- 実際の接続は FTDI経由（UART1）なので、**UART1用キー** を使う必要がある

---

## 発見事項

| 項目 | 内容 |
|------|------|
| MON-SPAN定期出力 | USB用キーでは無効化されていない |
| 対策 | UART1用キーを追加して無効化する |

---

## 変更ファイル

| ファイル | 内容 |
|----------|------|
| [manager.rs](../../prototype/m1-gnss/backend/src/device/manager.rs) | Class/IDログ追加 |

---

## 成果物

| ファイル | 内容 |
|----------|------|
| [log-analysis-report.md](log-analysis-report.md) | ログ分析レポート |

---

## 次セッション（Session 149）でやること

1. UART1用キーを `cfg_valset.rs` に追加
2. `disable_periodic_output` でUART1用キーを使用するよう修正
3. 動作確認
4. ADR-012を更新

---

## 参照

- [Session 148 plan](session-plan.md)
- [Session 147 summary](../session147/session-summary.md)
