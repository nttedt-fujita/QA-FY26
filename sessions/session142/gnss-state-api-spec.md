# 統合API (`GET /api/gnss-state`) 仕様

**作成**: Session 142 (2026-03-12)
**出典**: ADR-012, 既存パーサー（nav_pvt.rs, mon_rf.rs等）

---

## 概要

| 項目 | 内容 |
|------|------|
| エンドポイント | `GET /api/gnss-state` |
| 目的 | F9Pの現在状態を1回のAPIで全取得（ポーリング競合防止） |
| 対象メッセージ | NAV-PVT, NAV-STATUS, NAV-SAT, NAV-SIG, MON-SPAN, MON-RF |
| 実装方式 | 1回のAPIコール内で6メッセージを順次ポーリング |
| 部分失敗時 | 取得できたデータは返す + `errors`配列で失敗理由を通知 |

---

## レスポンス構造

```json
{
  "nav_pvt": NavPvtResponse | null,
  "nav_status": NavStatusResponse | null,
  "nav_sat": NavSatResponse | null,
  "nav_sig": NavSigResponse | null,
  "mon_span": MonSpanResponse | null,
  "mon_rf": MonRfResponse | null,
  "errors": string[]
}
```

---

## 各フィールドの詳細

### nav_pvt (NAV-PVT)

出典: [nav_pvt.rs](../../prototype/m1-gnss/backend/src/ubx/nav_pvt.rs)

| フィールド | 型 | 説明 |
|------------|-----|------|
| lat | f64 | 緯度（度） |
| lon | f64 | 経度（度） |
| fix_type | u8 | FIXタイプ（0=No fix, 2=2D, 3=3D） |
| carr_soln | u8 | RTK状態（0=なし, 1=Float, 2=Fixed） |
| num_sv | u8 | 使用衛星数 |
| h_acc | u32 | 水平精度（mm） |
| v_acc | u32 | 垂直精度（mm） |
| is_rtk_fixed | bool | RTK Fixed状態か |
| is_rtk_float | bool | RTK Float状態か |

**除外したフィールド**（パーサーに未実装）:
- height（高度）
- 速度関連

### nav_status (NAV-STATUS)

出典: [nav_status_api.rs](../../prototype/m1-gnss/backend/src/web/nav_status_api.rs)

| フィールド | 型 | 説明 |
|------------|-----|------|
| ttff | u32 | Time to First Fix (ms) |
| msss | u32 | 起動からの経過時間 (ms) |
| gps_fix | u8 | FIXタイプ |
| gps_fix_ok | bool | FIXが有効か |
| is_fix_valid | bool | FIX有効判定（gps_fix >= 2 && gps_fix_ok） |
| carr_soln | u8 | RTK状態 |
| is_rtk_fixed | bool | RTK Fixed状態か |
| is_rtk_float | bool | RTK Float状態か |

### nav_sat (NAV-SAT)

出典: [nav_sat_api.rs](../../prototype/m1-gnss/backend/src/web/nav_sat_api.rs)

| フィールド | 型 | 説明 |
|------------|-----|------|
| satellites | Vec<SatelliteInfoResponse> | 衛星情報リスト |
| stats | SatelliteStatsResponse | 統計情報 |

**SatelliteInfoResponse**:
| フィールド | 型 | 説明 |
|------------|-----|------|
| gnss_id | u8 | GNSS識別子（0=GPS, 2=Galileo, 6=GLONASS等） |
| gnss_name | string | GNSS名 |
| sv_id | u8 | 衛星番号 |
| cno | u8 | C/N0 [dBHz] |
| elev | i8 | 仰角 [deg] |
| azim | i16 | 方位角 [deg] |
| is_used | bool | ナビゲーションに使用中か |
| quality_ind | u8 | 品質指標（0-7） |
| health | u8 | 健全性 |

**SatelliteStatsResponse**:
| フィールド | 型 | 説明 |
|------------|-----|------|
| total_count | usize | 全衛星数 |
| used_count | usize | 使用中衛星数 |
| gnss_counts | GnssCountsResponse | GNSS別衛星数 |

### nav_sig (NAV-SIG)

出典: [nav_sig_api.rs](../../prototype/m1-gnss/backend/src/web/nav_sig_api.rs)

| フィールド | 型 | 説明 |
|------------|-----|------|
| signals | Vec<SignalInfoResponse> | 信号情報リスト |
| stats | SignalStatsResponse | 統計情報 |

**SignalInfoResponse**:
| フィールド | 型 | 説明 |
|------------|-----|------|
| gnss_id | u8 | GNSS識別子 |
| sv_id | u8 | 衛星番号 |
| sig_id | u8 | 信号識別子 |
| cno | u8 | C/N0 [dBHz] |
| quality_ind | u8 | 品質指標 |
| is_l1 | bool | L1帯か |
| is_l2 | bool | L2帯か |

**SignalStatsResponse**:
| フィールド | 型 | 説明 |
|------------|-----|------|
| gps_visible_count | usize | GPS L1可視衛星数 |
| gps_l2_reception_count | usize | GPS L2受信中衛星数 |
| gps_l2_reception_rate | f64 | GPS L2受信率（0.0〜1.0） |

### mon_span (MON-SPAN)

出典: [mon_span_api.rs](../../prototype/m1-gnss/backend/src/web/mon_span_api.rs)

| フィールド | 型 | 説明 |
|------------|-----|------|
| blocks | Vec<SpanBlockResponse> | スペクトラムブロックリスト |

**SpanBlockResponse**:
| フィールド | 型 | 説明 |
|------------|-----|------|
| spectrum | Vec<u8> | スペクトラムデータ（256点、dB） |
| span | u32 | 周波数スパン（Hz） |
| res | u32 | 周波数分解能（Hz） |
| center | u32 | 中心周波数（Hz） |
| pga | u8 | PGAゲイン（dB） |
| max_amplitude | u8 | スペクトラム最大値（dB） |
| avg_amplitude | f32 | スペクトラム平均値（dB） |

### mon_rf (MON-RF)

出典: [mon_rf.rs](../../prototype/m1-gnss/backend/src/ubx/mon_rf.rs)

| フィールド | 型 | 説明 |
|------------|-----|------|
| blocks | Vec<RfBlockResponse> | RFブロック情報 |
| has_jamming | bool | ジャミング検出 |
| has_critical_jamming | bool | 危険レベルのジャミング |

**RfBlockResponse**:
| フィールド | 型 | 説明 |
|------------|-----|------|
| block_id | u8 | RFブロックID (0=L1, 1=L2/L5) |
| jamming_state | u8 | ジャミング状態 (0=不明, 1=OK, 2=警告, 3=危険) |
| ant_status | u8 | アンテナ状態 (0=INIT, 1=DONTKNOW, 2=OK, 3=SHORT, 4=OPEN) |
| ant_power | u8 | アンテナ電源状態 (0=OFF, 1=ON, 2=DONTKNOW) |
| noise_per_ms | u16 | ノイズレベル |
| agc_cnt | u16 | AGCモニタ (0-8191) |
| cw_suppression | u8 | CW干渉抑制レベル (0=なし, 255=強) |
| is_jamming_detected | bool | ジャミング検出判定 |
| is_antenna_ok | bool | アンテナ正常判定 |

---

## エラーハンドリング

### 正常系（200 OK）

全メッセージ取得成功:
```json
{
  "nav_pvt": { ... },
  "nav_status": { ... },
  "nav_sat": { ... },
  "nav_sig": { ... },
  "mon_span": { ... },
  "mon_rf": { ... },
  "errors": []
}
```

### 部分成功（200 OK）

一部メッセージ取得失敗:
```json
{
  "nav_pvt": { ... },
  "nav_status": null,
  "nav_sat": { ... },
  "nav_sig": null,
  "mon_span": { ... },
  "mon_rf": { ... },
  "errors": [
    "nav_status: タイムアウト",
    "nav_sig: パースエラー"
  ]
}
```

### エラー系（400 Bad Request）

装置未接続:
```json
{
  "error": "装置が接続されていません",
  "code": "DEVICE_NOT_CONNECTED"
}
```

### エラー系（500 Internal Server Error）

全メッセージ取得失敗:
```json
{
  "error": "全メッセージの取得に失敗",
  "code": "ALL_MESSAGES_FAILED"
}
```

---

## 実装方針

1. **gnss_state_api.rs を新規作成**
2. **既存APIのパターンを踏襲**
   - AppStateからDeviceManager取得
   - drain_buffer → poll送信 → receive_ubx → parse
3. **6メッセージを順次ポーリング**
   - エラーは各メッセージごとに捕捉
   - 成功したデータは返す
4. **main.rsに登録**

---

## 関連ドキュメント

- [ADR-012: 統合API方針](../../docs/adr/m1/ADR-012-periodic-output-and-unified-api.md)
- [TDDスキル](../../.claude/skills/tdd-practice/SKILL.md)
