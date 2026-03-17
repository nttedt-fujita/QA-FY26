# u-blox レイヤー設定 確認コマンド一覧

---
作成: Session 229 (2026-03-17)
---

## レイヤー優先順位

```
起動時の設定読み込み順序（高→低）:
RAM > BBR > Flash > Default
```

**重要**: 上位レイヤーに値があると、下位レイヤーは無視される

---

## 設定値確認コマンド（cfg-valget）

各レイヤーの NAV_PVT_USB 設定値を確認する。

| コマンド | 確認対象 | 用途 |
|----------|----------|------|
| `make cfg-valget-ram` | RAM | 現在有効な設定値 |
| `make cfg-valget-bbr` | BBR | バッテリーバックアップRAM |
| `make cfg-valget-flash` | Flash | 永続ストレージ |
| `make cfg-valget-default` | Default | 工場出荷時初期値 |

### 出力例

**値が存在する場合**:
```json
{
  "key": "NAV_PVT_USB",
  "layer": "bbr",
  "value": 0
}
```

**値が存在しない場合**（エラー）:
```json
{
  "error": "timeout"
}
```

---

## 設定削除コマンド（cfg-valdel）

| コマンド | 削除対象 | 用途 |
|----------|----------|------|
| `make cfg-valdel-bbr` | BBR | BBR優先順位問題の解決 |
| `make cfg-valdel-flash` | Flash | Flash設定のクリア |

### BBR優先順位問題の解決手順

BBRに値（0でも）があると、Flashの値が無視される。

```bash
# 1. 状態確認
make cfg-valget-bbr    # BBR=0 が返る
make cfg-valget-flash  # Flash=1 が返る

# 2. BBR削除
make cfg-valdel-bbr    # ACK

# 3. 確認
make cfg-valget-bbr    # エラー（値なし）

# 4. USB抜き差し + BE再起動
make dev-backend

# 5. 動作確認
make connect-raw       # 定期出力無効化をスキップして接続
make message-scan      # NAV-PVT等が検出されればOK
```

---

## 設定書き込みコマンド

| コマンド | 書き込み先 | 用途 |
|----------|-----------|------|
| `make set-periodic-output` | RAM+BBR+Flash | 定期出力有効化（全レイヤー） |
| `make set-periodic-output-flash` | Flash のみ | Flash永続化テスト |

---

## 接続コマンド

| コマンド | 動作 | 用途 |
|----------|------|------|
| `make connect` | 通常接続（定期出力無効化あり） | 通常使用 |
| `make connect-raw` | 接続（定期出力無効化なし） | Flash設定の動作確認 |

---

## 診断コマンド

| コマンド | 内容 | 用途 |
|----------|------|------|
| `make message-scan` | 定期出力スキャン（12秒） | 定期出力の確認 |
| `make mon-ver` | モジュール情報取得 | ZED-F9P, FWバージョン確認 |
| `make reset-config` | BBR+Flashクリア | 設定全リセット（要注意） |

---

## トラブルシューティング

### 「Flashに書いたのに消える」

**原因**: BBRに0が残っている

**確認**:
```bash
make cfg-valget-bbr    # value=0 なら問題あり
make cfg-valget-flash  # value=1 が期待値
```

**解決**:
```bash
make cfg-valdel-bbr    # BBR削除
```

### 「message-scanで何も検出されない」

**確認**:
```bash
make cfg-valget-ram    # 現在の設定値
```

**対処**:
```bash
make set-periodic-output  # 定期出力を有効化
make message-scan         # 再確認
```

### 「ボーレートが9600bpsに戻った」

**原因**: CFG-CFG の loadMask を使用した

**対処**: ADR-015参照。loadMaskは使用禁止。USB抜き差しで回復。

---

## 参照

- [38-ublox-config-management.md](../../docs/missions/m1-sensor-evaluation/gnss/38-ublox-config-management.md) - 設定管理の詳細
- [api.mk](../../prototype/m1-gnss/makefiles/api.mk) - コマンド定義
