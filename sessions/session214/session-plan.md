# Session 214 計画

**目的**: CFG-CFG仕様の再確認 + 実装修正

**前提**: Session 213でreset-config APIが効かないことが判明

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|------------------|
| 1 | CFG-CFG仕様の再確認 | u-blox F9 HPG 1.32 Interface Description p.64-65 |
| 2 | deviceMaskの正しい使い方を確認 | 同上 |
| 3 | 設定がBBR/Flashどちらに保存されているか確認方法を調査 | CFG-VALGET仕様 |
| 4 | 実装修正 | cfg_cfg.rs |

---

## 確認すべき点

1. **deviceMaskの動作**
   - clearMask + deviceMask=BBR でBBRのみクリアされるか？
   - Flashに保存された設定はdeviceMask=0x02で対象になるか？

2. **loadMaskの動作**
   - loadMask + deviceMask=BBR の場合、BBRが空なら何がロードされる？
   - ROMデフォルトをロードするにはどうすればよい？

3. **設定レイヤーの確認方法**
   - CFG-VALGET でレイヤー指定して読み出せるか？
   - 定期出力設定がBBR/Flashどちらにあるか確認

---

## 参照

- [38-ublox-config-management.md](../../docs/missions/m1-sensor-evaluation/gnss/38-ublox-config-management.md)
- [cfg-cfg-spec.md](../session211/cfg-cfg-spec.md)
- u-blox F9 HPG 1.32 Interface Description p.64-65
