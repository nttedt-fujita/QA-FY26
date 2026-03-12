# Session 133 計画

**目的**: NTRIPクライアント実装

**前提**: Session 132でNTRIP認証設定画面を実装完了。設定はローカルストレージに保存済み。

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | ntrip-clientクレート調査 | docs/missions/m1-sensor-evaluation/gnss/20-ntrip-rtk-implementation.md |
| 2 | バックエンドにクレート追加 | prototype/m1-gnss/backend/Cargo.toml |
| 3 | NTRIP接続API設計 | prototype/m1-gnss/backend/src/api/ |
| 4 | NTRIP接続API実装 | - |
| 5 | フロントエンドから接続開始 | prototype/m1-gnss/frontend/src/app/settings/page.tsx |

---

## 詳細

### 1. ntrip-clientクレート調査

Session 114で調査済み:
- nav-solutions/ntrip-client (0.1.1)
- 依存: tokio, reqwest, rtcm-rs

### 2. バックエンドにクレート追加

```toml
ntrip-client = "0.1"
```

### 3. NTRIP接続API設計

**エンドポイント案**:
- `POST /api/ntrip/connect` — 接続開始
- `POST /api/ntrip/disconnect` — 切断
- `GET /api/ntrip/status` — 状態取得

**リクエスト**:
```json
{
  "caster_url": "ntrip.jenoba.jp",
  "port": 2101,
  "mountpoint": "TOKYO_RTCM3",
  "username": "user123",
  "password": "********"
}
```

### 4. RTCMデータ転送

受信したRTCMバイナリをそのままZED-F9Pのシリアルポートに書き込む:
```rust
serial_port.write(&rtcm_data)?;
```

### 5. フロントエンド接続ボタン

設定画面に「接続」「切断」ボタンを追加。

---

## 参照

- [Session 132 summary](../session132/session-summary.md)
- [20-ntrip-rtk-implementation.md](../../docs/missions/m1-sensor-evaluation/gnss/20-ntrip-rtk-implementation.md)

---

*計画作成: 2026-03-12 Session 132終了時*
