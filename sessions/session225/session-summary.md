# Session 225 サマリー

**日時**: 2026-03-17
**目的**: BBR優先順位問題の解決 + 仕様理解の深掘り

---

## 実施内容

### 1. 前回の問題の再現確認

Session 224で特定した状態を確認：

| レイヤー | いじったデバイス | 優先順位 |
|----------|------------------|----------|
| RAM | 0 | 1位（最高） |
| BBR | 0 | 2位 |
| Flash | 1 | 3位 |
| Default | 0 | 4位（最低） |

### 2. いじっていないデバイスとの比較

**重要な発見**: BBRの値の「存在」の違い

| レイヤー | いじったデバイス | いじっていないデバイス |
|----------|------------------|------------------------|
| RAM | 0 | 0 |
| BBR | 0（値あり） | **エラー（値なし）** |
| Flash | 1 | 0 |
| Default | 0 | 0 |

**結論**: `disable_periodic_output`がBBRに0を書き込むことで、BBRに値が「存在する」状態になり、Flashより優先される。

### 3. 定期出力の謎（cfg-valget=0なのに出力あり）

いじっていないデバイスで`cfg-valget NAV_PVT_USB=0`なのにNAV-PVTが出力されていた。

**原因特定**: UART1の設定を確認

| キー | 値 |
|------|-----|
| NAV_PVT_USB (RAM) | 0 |
| NAV_PVT_UART1 (RAM) | **1** |

**結論**: デバイスは**内部的にUART1として接続**されている。USBポートの設定ではなく、UART1の設定が適用される。

---

## 発見事項（重要）

### 1. BBRレイヤーの「値の存在」

- **未設定のデバイス**: BBRに値が存在しない → cfg-valgetでエラー
- **一度でも書き込んだデバイス**: BBRに値が存在する（0でも） → Flashより優先

**出典**: u-blox F9 HPG 1.32 Interface Description p.224
> For each defined item... the receiver software goes through the layers above and stacks all the found items on top. **Some items may not be present in every layer.**

### 2. USB vs UART1の接続形態

組み込みボードでは、PCにはUSB経由で接続するが、**内部的にはUART1として動作**する。

- CFG-MSGOUT-UBX_NAV_PVT_USB (0x20910009) → PC直結USB用
- CFG-MSGOUT-UBX_NAV_PVT_UART1 (0x20910007) → 組み込みボード用

**出典**: 既存コードのコメント（cfg_valset.rs:75-77）
```rust
// 実機はUSBハブ経由でUART1として接続されるため、UART1用キーが必要
```

---

## 変更ファイル

| ファイル | 内容 |
|----------|------|
| [api.mk](../../prototype/m1-gnss/makefiles/api.mk) | cfg-valget-default追加 |
| [cfg_valget_api.rs](../../prototype/m1-gnss/backend/src/web/cfg_valget_api.rs) | NAV_PVT_UART1キー追加 |

---

## 未解決事項（次セッション）

### BBR優先順位問題の解決

いじったデバイスのBBRに0が存在するため、Flashの1が無視される問題。

**対策案**:

| 案 | 内容 | 備考 |
|----|------|------|
| A | disable_periodic_outputをRAMのみに変更 | BBRに書かない |
| B | CFG-VALDELでBBRから値を削除 | 効果要検証 |
| C | 接続時にset-periodic-output実行 | 毎回設定 |

### 仕様書との整合性確認

今回の発見を以下のドキュメントに反映する必要がある：
- [config-layers-spec.md](../session220/config-layers-spec.md) - BBRの「値の存在」について追記
- [32-cfg-msgout-periodic-output.md](../../docs/missions/m1-sensor-evaluation/gnss/32-cfg-msgout-periodic-output.md) - USB vs UART1の説明追加

---

## 使用したコマンド

| 用途 | コマンド |
|------|----------|
| 接続（disable_periodic_outputスキップ） | make connect-raw |
| 各レイヤーの値確認 | make cfg-valget-ram/bbr/flash/default |
| UART1設定確認 | curl ... key=NAV_PVT_UART1 |
| 定期出力確認 | make message-scan |

---

*作成: Session 225 (2026-03-17)*
