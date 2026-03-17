# Session 228 計画

**目的**: CFG-VALDEL実装 + BBR削除テスト

**前提**: Session 227で仕様書を確認済み。BBRを削除すればFlashが参照されることを確認。

---

## やること

| # | 作業 | 読むべきファイル | 参照コマンド |
|---|------|-----------------|--------------|
| 1 | CFG-VALDELメッセージ実装 | session227/cfg-valdel-spec.md | - |
| 2 | CFG-VALDEL API実装 | cfg_valset.rs（参考） | - |
| 3 | Makeコマンド追加 | api.mk | - |
| 4 | BBR削除テスト | - | make cfg-valdel-bbr |
| 5 | Flashの値が使われるか確認 | - | USB抜き差し → connect-raw → message-scan |

---

## 詳細

### 1. CFG-VALDELメッセージ実装

**仕様**（session227/cfg-valdel-spec.md）:
- Class: 0x06, ID: 0x8C
- ペイロード:
  - version: U1 (0x00)
  - layers: X1 (bit 1: BBR, bit 2: Flash)
  - reserved0: U1[2]
  - keys: U4[] (削除するKey ID)

### 2. CFG-VALDEL API実装

`/api/cfg-valdel` エンドポイント:
- クエリパラメータ: `layer=bbr|flash`
- 対象Key: NAV_PVT_USB (0x20910009)

### 3. Makeコマンド追加

```makefile
cfg-valdel-bbr:
    curl -X DELETE $(API_URL)/api/cfg-valdel?layer=bbr
```

### 4-5. テスト

1. make cfg-valdel-bbr → BBRからNAV_PVT_USBを削除
2. make cfg-valget-bbr → エラー（値なし）を確認
3. USB抜き差し + BE再起動
4. make connect-raw
5. make message-scan → **出力されるはず**

---

## 参照

- [session227/session-summary.md](../session227/session-summary.md) - 仕様確認結果
- [session227/cfg-valdel-spec.md](../session227/cfg-valdel-spec.md) - CFG-VALDEL仕様抽出
- [38-ublox-config-management.md](../../docs/missions/m1-sensor-evaluation/gnss/38-ublox-config-management.md) - 設定管理仕様

---

*作成: Session 227 (2026-03-17)*
