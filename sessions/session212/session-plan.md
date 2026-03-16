# Session 212 計画

**目的**: CFG-CFGによる設定リセットAPIの実装

**前提**: Session 211でu-blox設定管理の仕組みを理解した

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|--------------------|
| 1 | CFG-CFGコマンド実装 | docs/missions/m1-sensor-evaluation/gnss/38-ublox-config-management.md |
| 2 | 設定リセットAPI実装 | prototype/m1-gnss/backend/src/web/device_api.rs |
| 3 | 実機テスト（BBRクリア効果確認） | - |

---

## 詳細

### 1. CFG-CFGコマンド実装

**ファイル**: `prototype/m1-gnss/backend/src/ubx/`

CFG-CFG (0x06 0x09) の送信機能を実装する。

**ペイロード構造**:
```
Offset 0-3: clearMask (X4)
Offset 4-7: saveMask (X4)
Offset 8-11: loadMask (X4)
Offset 12: deviceMask (X1) - オプション
```

**BBRクリアの場合**:
- clearMask = 0xFFFF（全設定）
- saveMask = 0x0000
- loadMask = 0xFFFF（ROM → RAM）
- deviceMask = 0x01（BBR）

### 2. 設定リセットAPI実装

**エンドポイント**: `POST /api/devices/{path}/reset-config`

**動作**:
1. CFG-CFG送信（BBRクリア）
2. ACK確認
3. 結果を返す

### 3. 実機テスト

1. 古い機で定期出力を確認
2. reset-config APIを実行
3. 定期出力が停止することを確認
4. 再起動後も定期出力が停止していることを確認

---

## 代替案

実装が難しい場合の代替案:

1. **全NAVメッセージ一括無効化**
   - CFG-VALSET で全NAV/MONメッセージを0に設定
   - キー数が多いが確実

2. **u-centerで手動リセット**
   - Receiver → Action → Revert Config
   - ツール実装は不要だが手動操作

---

## 参照

- [Session 211 summary](../session211/session-summary.md)
- [38-ublox-config-management.md](../../docs/missions/m1-sensor-evaluation/gnss/38-ublox-config-management.md)
- u-blox F9 HPG 1.32 Interface Description p.64
