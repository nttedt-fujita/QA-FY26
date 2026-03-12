# 統合APIログ分析レポート

**作成**: Session 148 (2026-03-12)
**対象ログ**: `gnss-backend.log.2026-03-12` (08:04:48〜08:05:45)

---

## 概要

統合API (`/api/gnss-state`) で発生しているタイムアウト・エラーの原因を特定するため、デバッグログにClass/IDを追加して分析を実施した。

---

## 分析結果

### 1. 統合APIで取得できているメッセージ

| メッセージ | Class | ID | サイズ | 結果 |
|-----------|-------|-----|--------|------|
| NAV-PVT | 0x01 | 0x07 | 100バイト | ✅ 毎回成功 |
| NAV-STATUS | 0x01 | 0x03 | - | ❌ タイムアウト |
| NAV-SAT | 0x01 | 0x35 | 16-28バイト | ✅ 成功 |
| NAV-SIG | 0x01 | 0x43 | - | ❌ タイムアウト |
| MON-SPAN | 0x0A | 0x31 | - | ❌ タイムアウト |
| MON-RF | 0x0A | 0x38 | 60バイト | ✅ 毎回成功 |

### 2. 「不明データ」の正体

ログに繰り返し出現するパターン:
```
UBX前に522バイトの不明データあり: 39 3F 32 45 30 3C 38 42...
UBX前に474バイトの不明データあり: 7B 7E 7E 84 83 82 86 86...
UBX前に504バイトの不明データあり: 51 45 4E 51 53 52 5B 5C...
```

**特徴**:
- サイズ: 約400〜550バイト（MON-SPANのスペクトラムデータ 256バイト × 2ブロック相当）
- 値の範囲: 0x20〜0x9A（C/N0値として妥当）
- UBXヘッダー(B5 62)がない → フレーム途中から読み始めている

**結論**: MON-SPAN（0x0A, 0x31）の定期出力が有効なまま

### 3. 根本原因

`cfg_valset.rs` で定期出力を無効化する際、**USB用のキー**を使用している:

```rust
pub const CFG_MSGOUT_NAV_PVT_USB: u32 = 0x20910009;
pub const CFG_MSGOUT_MON_SPAN_USB: u32 = 0x2091038e;
// ...etc
```

しかし、実際の接続は `/dev/ttyUSB0`（FTDI経由）であり、F9Pから見ると**UART1経由**の通信。

**USB用キーでは無効化されない**ため、UART1用キーを使う必要がある。

---

## 対策（次セッションで実施）

### 対策案A: UART1用キーを追加して無効化（推奨）

`cfg_valset.rs` に UART1用キーを追加:
```rust
// UART1用キー
pub const CFG_MSGOUT_NAV_PVT_UART1: u32 = 0x20910007;
pub const CFG_MSGOUT_MON_SPAN_UART1: u32 = 0x2091038c;
// ...etc
```

### 対策案B: USB/UART1両方を無効化

接続タイプを問わず両方のキーを設定する。

---

## 参照

- [cfg_valset.rs](../../prototype/m1-gnss/backend/src/ubx/cfg_valset.rs) - 現在の実装
- [u-blox F9 HPG Interface Description](https://www.u-blox.com/) - キーID一覧（p.234-251）
