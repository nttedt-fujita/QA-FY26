# Session 221 サマリー

**日付**: 2026-03-17
**目的**: reset-configテスト再実行（Flash対応版）

---

## 実施内容

1. **Phase 1テスト開始**
   - set-periodic-output実行 → ACK（成功）
   - message-scan → NAV-PVT(14), NAV-STATUS(2), MON-RF(1), NAV-SIG(1) 検出
   - USB抜き差し → **0件**（定期出力消失）

2. **問題調査**
   - Flashレイヤーの仕様確認（Session 220で抽出済み）
   - MON-VER API作成してモジュール情報取得

3. **MON-VER API作成**
   - `GET /api/devices/{path}/mon-ver`
   - モジュール情報: ZED-F9P, HPG 1.32, PROTVER 27.31

---

## 発見した問題

**事実**:
- set-periodic-output（layers=0x07）実行 → ACK返却
- USB抜き差し後 → 定期出力0件

**仕様確認**（出典: IF p.224）:
- Flashレイヤーは「外部Flashメモリがある場合のみ」使用可能
- BBRは「バッテリーバックアップがないと電源断で消える」

**仮説**（未確認）:
- 評価ボードに外部Flashメモリが搭載されていない
- バッテリーバックアップも接続されていない
- そのため、layers=0x07を指定しても実際はRAMのみに保存された

---

## 作成ファイル

| ファイル | 内容 |
|----------|------|
| [mon_ver_api.rs](../../prototype/m1-gnss/backend/src/web/mon_ver_api.rs) | MON-VER API（新規） |

## 変更ファイル

| ファイル | 内容 |
|----------|------|
| web/mod.rs | mon_ver_api追加 |
| web/device_api.rs | ルーティング追加 |
| api.mk | `make mon-ver` コマンド追加 |

---

## 使用したコマンド

| 用途 | コマンド |
|------|----------|
| 装置接続 | make connect |
| 定期出力有効化 | make set-periodic-output |
| メッセージスキャン | make message-scan |
| モジュール情報取得 | make mon-ver |

---

## 残タスク

1. **評価ボードのFlash有無を確認**
   - 評価ボードの種類を確認（SparkFun, ArduSimple等）
   - 外部Flashメモリの有無を確認

2. **対応策の検討**
   - Flashがない場合の代替案
   - 毎回接続時にset-periodic-outputを実行する等

---

## 次セッション

[session222/session-plan.md](../session222/session-plan.md)
