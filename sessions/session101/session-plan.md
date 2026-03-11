# Session 101 計画

**目的**: UBX通信タイミング問題の対策実装

---

## 背景

Session 100でタイミング問題の根本原因を特定。対策方針（案A + B）を決定済み。

---

## やること

### 1. 案A: receive_ubx で `B5 62` 同期

**ファイル**: `backend/src/device/manager.rs`

**変更内容**:
- 受信データから `B5 62` を探す
- UBXフレームのみ抽出して返す

**テスト追加**:
- NMEAが先に来てもUBXフレームを読める
- タイムアウト内にB5 62が見つからない場合はエラー

### 2. 実機テストで効果確認

```bash
cd prototype/m1-gnss
make dev-backend    # ターミナル1
make connect        # ターミナル2
make inspect LOT_ID=1
```

### 3. 案B: 検査中のNMEA OFF/ON（案Aで不十分な場合）

**新規ファイル**: `backend/src/ubx/cfg_valset.rs`
**変更ファイル**: `backend/src/inspection/engine.rs`

---

## 参照資料

- [Session 100 サマリー](../session100/session-summary.md)
- [タイミング問題対策計画](../session100/nmea-timing-fix-plan.md)
- [manager.rs](../../prototype/m1-gnss/backend/src/device/manager.rs)
- [engine.rs](../../prototype/m1-gnss/backend/src/inspection/engine.rs)

---

*計画作成: 2026-03-11 Session 100終了時*
