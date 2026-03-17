# ADR-015: CFG-CFGでloadMaskを使わない

## Status

Accepted (2026-03-17)

## Context

reset-config APIでCFG-CFGコマンドを使用してBBR/Flashの設定をクリアする際、loadMaskの使用方法について判断が必要だった。

当初の実装:
- clearMask = ALL（設定クリア）
- loadMask = ALL（ROMデフォルトを即座に適用）

### 発生した問題

Session 215で実機テスト時に、CFG-CFG送信後にACKが受信できず、受信データが文字化け（`1C FC FC 70 80...`）していた。

**原因**: loadMask=ALLにより、ボーレートがROMデフォルト（9600bps）に即座に変更され、115200bpsで受信していたため文字化けが発生。

### 仕様書の記載

u-blox F9 HPG 1.32 Interface Description (UBX-22008968) p.64:

> if any bit is set in the loadMask: The current configuration is discarded and rebuilt from all the lower layers

loadMaskに任意のビットが立っていると、現在の設定が破棄され、下位レイヤー（Flash→BBR→ROM）から再構築される。これにはボーレート設定も含まれる。

## Decision

**loadMask = NONEを使用する。**

```rust
let clear_mask = ConfigMask::ALL.0;  // BBR/Flashの設定をクリア
let save_mask = ConfigMask::NONE.0;
let load_mask = ConfigMask::NONE.0;  // ロードしない（ボーレート維持）
let device_mask = DeviceMask::BbrAndFlash as u8;
```

設定はBBR/Flashからクリアされ、次回電源投入時（USB抜き差し時）にROMデフォルトが適用される。

## Consequences

### 良い点

- ボーレートが変わらないのでACKを正常に受信できる
- 通信が切れないので、リセット後も継続して操作できる

### 悪い点

- 設定は即座には反映されない（次回電源投入時まで）
- ユーザーにUSB抜き差しを案内する必要がある

### 代替案（不採用）

1. **loadMask後にボーレートを再設定**: 複雑になる上、一瞬通信が切れる
2. **loadMask=ALLのまま9600bpsで再接続**: 実装が複雑

## 関連

- Session 215: 問題発見と対策実装
- [cfg_cfg.rs](../../../prototype/m1-gnss/backend/src/ubx/cfg_cfg.rs): 実装
- [38-ublox-config-management.md](../../missions/m1-sensor-evaluation/gnss/38-ublox-config-management.md): 設定レイヤーの解説
