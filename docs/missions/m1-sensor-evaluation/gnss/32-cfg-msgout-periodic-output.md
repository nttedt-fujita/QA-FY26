# CFG-MSGOUT: 定期出力（Periodic Output）設定

**作成**: Session 140 (2026-03-12)
**出典**: u-blox F9 HPG 1.32 Interface Description (UBX-22008968) p.234-251

---

## 概要

UBXメッセージには2種類の出力モードがある:
- **Polled**: リクエスト（Poll）に対して応答を返す
- **Periodic**: 設定したレートで自動的に出力される

CFG-MSGOUTを使うと、各メッセージの定期出力レートを設定できる。

---

## 設定方法

### UBX-CFG-VALSET

設定変更には `UBX-CFG-VALSET` (Class: 0x06, ID: 0x8A) を使用。

```
ペイロード構成:
[0]     version: 0x00
[1]     layers: 0x01 (RAM), 0x02 (BBR), 0x04 (Flash)
[2-3]   reserved: 0x00 0x00
[4+]    cfgData: キー(4bytes, LE) + 値(1byte)
```

### 出力レートの値

| 値 | 意味 |
|----|------|
| 0 | 出力しない（Pollのみ） |
| 1 | 毎エポック出力 |
| N | Nエポックごとに出力 |

---

## USB用設定キー一覧

| メッセージ | キー名 | Key ID | 用途 |
|-----------|--------|--------|------|
| NAV-PVT | CFG-MSGOUT-UBX_NAV_PVT_USB | 0x20910009 | 位置・速度・時刻 |
| NAV-STATUS | CFG-MSGOUT-UBX_NAV_STATUS_USB | 0x2091001d | FIX状態・TTFF |
| NAV-SAT | CFG-MSGOUT-UBX_NAV_SAT_USB | 0x20910018 | 衛星情報 |
| NAV-SIG | CFG-MSGOUT-UBX_NAV_SIG_USB | 0x20910348 | 信号情報 |
| MON-SPAN | CFG-MSGOUT-UBX_MON_SPAN_USB | 0x2091038e | スペクトラム |
| MON-RF | CFG-MSGOUT-UBX_MON_RF_USB | 0x2091035c | RF状態 |

---

## 推奨設定

屋外検査ツール用の推奨設定:

| メッセージ | レート | 理由 |
|-----------|--------|------|
| NAV-PVT | 1 | 位置・RTK状態は毎秒更新 |
| NAV-STATUS | 1 | TTFF監視のため毎秒 |
| NAV-SAT | 5 | 衛星情報は5秒ごとで十分 |
| NAV-SIG | 5 | 信号情報も5秒ごと |
| MON-SPAN | 10 | スペクトラムは10秒ごと |
| MON-RF | 10 | RF状態も10秒ごと |

---

## 実装コード

```rust
// prototype/m1-gnss/backend/src/ubx/cfg_valset.rs

pub const CFG_MSGOUT_NAV_PVT_USB: u32 = 0x20910009;
pub const CFG_MSGOUT_NAV_STATUS_USB: u32 = 0x2091001d;
// ... 他のキー

pub fn set_periodic_output(config: &PeriodicOutputConfig, layer: Layer) -> Vec<u8> {
    // CFG-VALSETメッセージを生成
}
```

---

## 関連ドキュメント

- [08-ubx-protocol-index.md](08-ubx-protocol-index.md) - UBXプロトコル索引
- [ADR-012](../../../adr/m1/ADR-012-periodic-output-and-unified-api.md) - 定期出力採用の決定
