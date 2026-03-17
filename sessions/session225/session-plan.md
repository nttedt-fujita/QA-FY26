# Session 225 計画

**目的**: BBR優先順位問題の解決（set-periodic-outputをRamBbrFlashに変更）

**前提**: Session 224でBBRがFlashより優先される問題を特定

---

## 背景

### 問題

Flashに定期出力設定を保存しても、BBRに0が残っているとFlashが無視される。

### u-blox設定レイヤー優先順位

```
RAM > BBR > Flash > Default
```

### 原因

- 接続時の`disable_periodic_output`がBBR（Layer::RamAndBbr）に0を書き込む
- `set-periodic-output-flash`はFlashのみに1を書く
- 起動時にBBRの0が優先され、Flashの1が無視される

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | set_periodic_output_api.rsでデフォルトをRamBbrFlashに変更 | set_periodic_output_api.rs |
| 2 | ビルド＆テスト | - |
| 3 | 全レイヤーで1になることを確認 | - |
| 4 | USB抜き差し後もmessage-scanで検出されることを確認 | - |

---

## 期待する結果

| 操作 | cfg-valget-ram | cfg-valget-bbr | cfg-valget-flash | message-scan |
|------|----------------|----------------|------------------|--------------|
| set-periodic-output後 | 1 | 1 | 1 | NAV-PVT等検出 |
| USB抜き差し後 | 1 | 1 | 1 | NAV-PVT等検出 |

---

## 参照

- [Session 224 summary](../session224/session-summary.md) - 問題の詳細
- [config-layers-spec.md](../session220/config-layers-spec.md) - u-blox設定レイヤー仕様
