# MON-SPAN API 設計

**セッション**: 117
**作成日**: 2026-03-12
**目的**: MON-SPANデータをAPIで取得

---

## 1. 振る舞い

### 目的

MON-SPAN（スペクトラムアナライザ）データをフロントエンドに提供する。

### エンドポイント

```
GET /api/mon-span
```

### 処理フロー

1. 装置接続確認
2. バッファクリア
3. MON-SPAN poll送信（class=0x0A, id=0x31）
4. 応答受信（タイムアウト1秒）
5. パース
6. レスポンス構築

### レスポンス形式

```json
{
  "blocks": [
    {
      "spectrum": [50, 51, 52, ...],  // 256点
      "span": 128000000,              // Hz
      "res": 500000,                  // Hz
      "center": 1575420000,           // Hz
      "pga": 54,                      // dB
      "max_amplitude": 60,            // dB
      "avg_amplitude": 50.5           // dB
    }
  ]
}
```

---

## 2. テストリスト

| ID | シナリオ | 入力 | 期待結果 |
|----|---------|------|----------|
| T1 | 接続済み・正常応答（2ブロック） | 有効なMON-SPAN応答 | 200 + 2ブロック |
| T2 | 未接続 | 未接続状態 | 400 DEVICE_NOT_CONNECTED |
| T3 | タイムアウト | 応答なし | 504 TIMEOUT |
| T4 | パースエラー | 不正UBXデータ | 500 PARSE_ERROR |
| T5 | 空のブロック（nBlocks=0） | 空応答 | 200 + 空配列 |
| T6 | 1ブロックのみ | L1のみ | 200 + 1ブロック |

---

## 3. 実装方針

NAV-SIG APIと同じパターンで実装:
- [nav_sig_api.rs](../../prototype/m1-gnss/backend/src/web/nav_sig_api.rs) をベースにする
- mon_span.rsのパーサーを呼び出す

### ファイル構成

```
backend/src/web/
├── mod.rs          # mon_span_api追加
├── mon_span_api.rs # 新規作成
└── nav_sig_api.rs  # 参考
```

---

## 4. 参照資料

- [mon_span.rs](../../prototype/m1-gnss/backend/src/ubx/mon_span.rs) — パーサー
- [mon-span-parser-spec.md](../session116/mon-span-parser-spec.md) — パーサー仕様

---

*設計作成: 2026-03-12 Session 117*
