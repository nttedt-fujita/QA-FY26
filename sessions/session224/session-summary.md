# Session 224 サマリー

**日時**: 2026-03-17
**目的**: CFG-VALGET実機テスト + Flash搭載確認 + 仮説検証

---

## 実施内容

### 1. CFG-VALGET実機テスト

| テスト | 結果 |
|--------|------|
| set-periodic-output-flash | ACK成功 |
| cfg-valget-flash | value=1（Flashに値がある） |
| USB抜き差し後 cfg-valget-flash | value=1（維持されている） |
| USB抜き差し後 message-scan | 0件（出力されていない） |

**結論**: Flashには値が保存されているが、起動時にRAMに適用されていない

### 2. 仮説検証ツール作成

| ツール | 用途 |
|--------|------|
| `make cfg-valget-ram` | RAMレイヤーの値を確認 |
| `make cfg-valget-bbr` | BBRレイヤーの値を確認 |
| `make connect-raw` | 接続時の定期出力無効化をスキップ |

### 3. 仮説検証テスト

**初期仮説**: 接続時にRAMを上書きしているため、FlashからRAMへのコピーが見えない

**テスト結果**:

| テスト | cfg-valget-ram | 結果 |
|--------|----------------|------|
| 通常接続後 | 0 | 予想通り |
| connect-raw後 | 0 | **仮説と異なる** |

### 4. 真の原因特定

**全レイヤーの値を確認**:

| レイヤー | 値 | 優先順位 |
|----------|-----|----------|
| RAM | 0 | 1位（最高） |
| BBR | **0** | **2位** |
| Flash | 1 | 3位 |
| Default | 0 | 4位（最低） |

**u-blox仕様**（Interface Description p.224）:
> 起動時に全レイヤーを読み込み、優先順位の高いものを採用

**原因**: 以前の`connect`時に`send_disable_periodic_output`がBBR（Layer::RamAndBbr）に0を書き込んだ。その後Flashにだけ1を書いても、**BBRの0がFlashの1より優先される**。

---

## 発見事項（重要）

### u-blox設定レイヤーの優先順位

```
RAM > BBR > Flash > Default
```

**落とし穴**: BBRに値が残っていると、Flashに書いた値が無視される

### 時系列で見た問題

| 時点 | 操作 | RAM | BBR | Flash |
|------|------|-----|-----|-------|
| 初期 | - | 0 | 0 | 0 |
| connect | disable→RamAndBbr | 0 | **0** | - |
| set-periodic-output-flash | 書込→Flash | - | - | **1** |
| USB抜き差し（再起動） | 復元 | **0**（BBRから） | 0 | 1 |

---

## 対策案（次セッション）

| 案 | 内容 | 採用 |
|----|------|------|
| A | set-periodic-outputでBBR+Flashに書く | **採用予定** |
| B | CFG-VALDELでBBRから値を削除 | - |
| C | 現状維持（毎回接続時に設定） | - |

**案A**: `layer=all`（RAM+BBR+Flash）で書けば、BBRもFlashも1になり、再起動後も維持される

---

## 変更ファイル

| ファイル | 内容 |
|----------|------|
| [device_api.rs](../../prototype/m1-gnss/backend/src/web/device_api.rs) | ConnectQuery追加、skip_disableパラメータ対応 |
| [api.mk](../../prototype/m1-gnss/makefiles/api.mk) | cfg-valget-ram, cfg-valget-bbr, connect-raw追加 |

---

## 次セッションでやること

1. **案Aの実装**: set-periodic-outputのデフォルトをRamBbrFlashに変更
2. **再テスト**: USB抜き差し後もmessage-scanで定期出力が検出されることを確認

---

## 参照資料

- [config-layers-spec.md](../session220/config-layers-spec.md) - u-blox設定レイヤー仕様（IF p.223-225）
- [session-plan.md](session-plan.md) - 本セッションの計画

---

*作成: Session 224 (2026-03-17)*
