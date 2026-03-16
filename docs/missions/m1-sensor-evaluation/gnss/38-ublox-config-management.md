# u-blox 設定管理の仕組み

---
作成: Session 211 (2026-03-16)
出典: u-blox F9 HPG 1.32 Interface Description (UBX-22008968), u-center User Guide (UBX-13005250)
---

## 概要

u-blox Generation 9 レシーバー（ZED-F9P等）の設定管理の仕組みを解説する。

---

## 1. 設定レイヤー

レシーバーには4つの設定レイヤーがある。

| レイヤー | 永続性 | 説明 |
|---------|--------|------|
| **ROM** | 永続（読取専用） | 工場出荷時のデフォルト値 |
| **Flash** | 永続 | ユーザーが書き込んだ永続設定 |
| **BBR** | バッテリー持続中 | Battery-Backed RAM（バッテリー切れで消える） |
| **RAM** | 電源ON中のみ | 現在動作中の設定値 |

### レイヤーの優先順位

起動時、設定は以下の優先順位で読み込まれる：

```
Flash（あれば） > BBR（あれば） > ROM（デフォルト）
                                    ↓
                                   RAM（現在値）
```

---

## 2. 設定関連UBXコマンド

### 2.1 CFG-VALGET (0x06 0x8B) — 設定値の読み出し

| 項目 | 内容 |
|------|------|
| 役割 | 指定したKey IDの現在値を取得 |
| 方向 | PC → レシーバー（リクエスト） → PC（レスポンス） |
| 用途 | 設定のバックアップ、現在値の確認 |

**例**: CFG-MSGOUT-UBX_NAV_PVT_USB の値を取得

### 2.2 CFG-VALSET (0x06 0x8A) — 設定値の書き込み

| 項目 | 内容 |
|------|------|
| 役割 | 指定したKey IDに値を書き込む |
| 方向 | PC → レシーバー |
| 用途 | 設定の変更（定期出力の有効化/無効化など） |

**例**: CFG-MSGOUT-UBX_NAV_PVT_USB = 0 で定期出力を無効化

### 2.3 CFG-VALDEL (0x06 0x8C) — 設定値の削除

| 項目 | 内容 |
|------|------|
| 役割 | 指定したKey IDをBBR/Flashから削除 |
| 方向 | PC → レシーバー |
| 用途 | 個別設定の削除（デフォルトに戻す） |

### 2.4 CFG-CFG (0x06 0x09) — レイヤー操作

| 項目 | 内容 |
|------|------|
| 役割 | レイヤー全体のクリア/保存/読み込み |
| 方向 | PC → レシーバー |
| 用途 | 設定の一括保存/クリア/デフォルト復元 |

**出典**: u-blox F9 HPG 1.32 Interface Description p.64

#### ペイロード構造

| Offset | Type | Name | Description |
|--------|------|------|-------------|
| 0 | X4 | clearMask | クリアする設定のマスク |
| 4 | X4 | saveMask | 保存する設定のマスク |
| 8 | X4 | loadMask | ロードする設定のマスク |
| 12 | X1 | deviceMask | 対象メモリ（オプション） |

#### deviceMask のビット

| bit | Name | Description |
|-----|------|-------------|
| 0 | devBBR | Battery-backed RAM |
| 1 | devFlash | Flash |

#### 動作（プロトコル23.01以降）

> - if any bit is set in the clearMask: **all configuration in the selected non-volatile memory is deleted**
> - if any bit is set in the saveMask: all current configuration is stored (copied) to the selected layers
> - if any bit is set in the loadMask: The current configuration is discarded and rebuilt from all the lower layers

---

## 3. u-center メニューとの対応

**出典**: u-center User Guide (UBX-13005250) p.24, p.64-65, p.67-68

### Receiver → Action メニュー

| メニュー | 対応コマンド | 動作 |
|----------|-------------|------|
| Save Config | CFG-CFG (saveMask) | RAM → BBR/Flash に保存 |
| Load Config | CFG-CFG (loadMask) | BBR/Flash → RAM に読み込み |
| Revert Config | CFG-CFG (clearMask + loadMask) | BBR/Flashクリア → デフォルト復元 |

### Tools → Receiver Configuration... / GNSS Configuration...

| 機能 | 対応コマンド | 動作 |
|------|-------------|------|
| Transfer GNSS > File | CFG-VALGET | 全設定値を読み出してファイル保存 |
| Transfer File > GNSS | CFG-VALSET | ファイルから設定を書き込み |

---

## 4. 定期出力（Periodic Output）の仕組み

### 設定キー

CFG-MSGOUT グループで、各メッセージの定期出力を制御する。

**形式**: `CFG-MSGOUT-UBX_{CLASS}_{ID}_{PORT}`

| キー例 | 意味 |
|--------|------|
| CFG-MSGOUT-UBX_NAV_PVT_USB | NAV-PVTをUSBに出力 |
| CFG-MSGOUT-UBX_NAV_SAT_UART1 | NAV-SATをUART1に出力 |

### 値の意味

| 値 | 意味 |
|----|------|
| 0 | 出力しない |
| 1 | 1Hz（毎秒） |
| 2 | 0.5Hz（2秒に1回） |
| n | 1/n Hz |

---

## 5. 古い機の定期出力問題

### 状況

RTK基準局として使用されていた機体には、BBRに大量の定期出力設定が保存されている。

```
起動 → BBRから設定読み込み → 大量のメッセージが定期出力される
```

### 対処方法の比較

| 方法 | コマンド | メリット | デメリット |
|------|----------|---------|-----------|
| 個別無効化 | CFG-VALSET | 既存設定を維持 | キー数が膨大 |
| BBRクリア | CFG-CFG (clearMask) | 一発で解決 | 他の設定も消える |
| 全NAV無効化 | CFG-VALSET（ループ） | NAVのみ対象 | 実装が必要 |

### これまでの経緯

| Session | 対応キー数 | 結果 |
|---------|-----------|------|
| 199-201 | 24キー | 不十分 |
| 209 | 34キー | 不十分 |
| 210 | 50キー | まだ3メッセージ出力 |

→ 根本対処としてCFG-CFGによるBBRクリアを検討中

---

## 6. 参照ドキュメント

| ドキュメント | 場所 | 内容 |
|-------------|------|------|
| u-blox F9 HPG 1.32 Interface Description | [sources/](sources/) | UBXプロトコル仕様 |
| u-center User Guide | [sources/](sources/) | u-centerの使い方 |
| [33-toc-ublox-f9-interface-description.md](33-toc-ublox-f9-interface-description.md) | 同ディレクトリ | PDF目次（どのページに何があるか） |

### 関連ページ（u-blox F9 HPG 1.32 Interface Description）

| 内容 | ページ |
|------|--------|
| CFG-CFG | p.64 |
| CFG-VALGET | p.96 |
| CFG-VALSET | p.96 |
| CFG-MSGOUT | p.234 |
| Configuration layers | p.223 |

---

## 7. 図解

### 設定レイヤーとコマンドの関係

```
┌─────────────────────────────────────────────────────────┐
│                   レシーバー内部                         │
├─────────────┬─────────────┬─────────────┬───────────────┤
│     ROM     │    Flash    │     BBR     │      RAM      │
│ (デフォルト) │   (永続)    │ (バッテリ)  │   (現在値)    │
└─────────────┴─────────────┴─────────────┴───────────────┘
                     ↑              ↑              ↑
                     │              │              │
CFG-VALGET: ─────────┼──────────────┼──────────────┘ 読み出し
                     │              │
CFG-VALSET: ─────────┼──────────────┼──────────────→ 書き込み
                     │              │              （対象レイヤー指定可）
                     │              │
CFG-CFG:             │              │
  clearMask ─────────┴──────────────┘ クリア
  saveMask  ←───────────────────────────────────── RAMから保存
  loadMask  ─────────────────────────────────────→ RAMへ読み込み
```
