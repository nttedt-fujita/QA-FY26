# Session 115 計画

**目的**: NTRIP機能実装開始

---

## 背景

Session 114でネットワークRTK（NTRIP）の調査完了:
- プロトコル仕様を把握
- Rustライブラリ（ntrip-client）を選定
- 実装方針を確定

---

## やること

### 1. ntrip-client クレート追加

```toml
[dependencies]
ntrip-client = "0.1"
```

### 2. NTRIP接続設定API設計

必要な設定項目:
- キャスターURL
- ポート
- マウントポイント
- ユーザーID
- パスワード

### 3. RTCMストリーム転送実装

- NTRIPキャスターからRTCM受信
- ZED-F9Pにシリアル転送

### 4. フロントエンドUI

- NTRIP設定画面の追加

---

## 参照資料

- [20-ntrip-rtk-implementation.md](../../docs/missions/m1-sensor-evaluation/gnss/20-ntrip-rtk-implementation.md) — 調査結果まとめ
- [21-ntrip-protocol-spec.md](../../docs/missions/m1-sensor-evaluation/gnss/21-ntrip-protocol-spec.md) — NTRIP仕様
- [22-rtk-configuration.md](../../docs/missions/m1-sensor-evaluation/gnss/22-rtk-configuration.md) — ZED-F9P RTK設定

**注**: 実際のセッションではRTK実装ではなく、ドキュメント正式配置を実施。詳細はsession-summary.md参照。

---

*計画作成: 2026-03-11 Session 114終了時*
