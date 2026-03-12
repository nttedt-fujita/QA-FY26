# NAV-SIG API 設計

**セッション**: 112
**作成日**: 2026-03-11
**目的**: NAV-SIG取得APIの振る舞いとテスト設計

---

## 1. 振る舞い（What）

### エンドポイント

```
GET /api/nav-sig
```

### 前提条件

- 装置が接続済み

### 入力

- なし

### 出力

```json
{
  "signals": [
    {
      "gnss_id": 0,       // 0=GPS, 1=SBAS, 2=Galileo, 3=BeiDou, 5=QZSS, 6=GLONASS
      "sv_id": 1,         // 衛星番号
      "sig_id": 0,        // 信号識別子
      "cno": 40,          // C/N0 [dBHz]
      "quality_ind": 7,   // 品質指標（0-7）
      "is_l1": true,      // L1帯か
      "is_l2": false      // L2帯か
    }
  ],
  "stats": {
    "gps_visible_count": 8,      // GPS L1可視衛星数
    "gps_l2_reception_count": 6, // GPS L2受信中衛星数
    "gps_l2_reception_rate": 0.75 // L2受信率（0.0〜1.0）
  }
}
```

### エラーレスポンス

| 状況 | HTTPステータス | code |
|------|---------------|------|
| 未接続 | 400 | DEVICE_NOT_CONNECTED |
| タイムアウト | 504 | TIMEOUT |
| パースエラー | 500 | PARSE_ERROR |
| 送信エラー | 500 | SEND_ERROR |
| 受信エラー | 500 | RECEIVE_ERROR |

---

## 2. テストリスト

| # | テスト | 入力 | 期待出力 |
|---|--------|------|----------|
| T1 | 接続済み・正常応答 | GPS L1×2 + GPS L2×1 | 200 + signals(3) + stats(visible=2, l2=1, rate=0.5) |
| T2 | 未接続 | - | 400 DEVICE_NOT_CONNECTED |
| T3 | タイムアウト | 応答なし | 504 TIMEOUT |
| T4 | パースエラー | 不正UBX | 500 PARSE_ERROR |
| T5 | 空の信号リスト | 信号0個 | 200 + signals([]) + stats(all 0) |
| T6 | GPS以外混在 | GPS L1×1 + GPS L2×1 + Galileo E1×1 | 200 + statsはGPSのみカウント |

---

## 3. 実装方針

### 処理フロー

1. DeviceManagerのロック取得
2. 接続確認（未接続なら400）
3. バッファクリア（drain_buffer）
4. NAV-SIG poll送信（0x01 0x43）
5. 50ms待機
6. 応答受信（1秒タイムアウト）
7. nav_sig::parse()でパース
8. signal_stats関数で集計
9. レスポンス構築

### 既存コードの活用

- `nav_sig::parse()` — パーサー（Session 110実装済み）
- `nav_sig::gps_visible_count()` — GPS可視衛星数（Session 110実装済み）
- `nav_sig::gps_l2_reception_count()` — L2受信中衛星数（Session 110実装済み）
- `nav_sig::gps_l2_reception_rate()` — L2受信率（Session 110実装済み）

---

## 4. 成果物

- [nav_sig_api.rs](../../prototype/m1-gnss/backend/src/web/nav_sig_api.rs) — APIハンドラー + テスト
- [mod.rs](../../prototype/m1-gnss/backend/src/web/mod.rs) — モジュール追加
- [main.rs](../../prototype/m1-gnss/backend/src/main.rs) — ルート追加

---

## 5. 次セッションでやること

- フロントエンド: L1/L2別C/N0一覧表示
- フロントエンド: L2受信率ゲージ
- 実機テスト

---

## 参照

- [ADR-008](../../docs/adr/m1/ADR-008-outdoor-inspection-criteria.md) — 屋外検査合格基準
- [outdoor-inspection-priority.md](../session111/outdoor-inspection-priority.md) — Phase 1.5優先度
- [m1-gnss-milestone.md](../session111/m1-gnss-milestone.md) — マイルストーン
