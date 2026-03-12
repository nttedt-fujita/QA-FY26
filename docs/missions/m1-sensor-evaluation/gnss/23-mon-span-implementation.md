# MON-SPAN 実装仕様

**作成日**: 2026-03-12
**関連セッション**: 116, 117, 118

---

## 概要

MON-SPAN（UBX 0x0A 0x31）はRFスペクトラムアナライザ機能を提供するメッセージ。
PGAゲインによる異常検出（No.5異常機の識別）に使用する。

---

## 1. 振る舞い記述

### 1.1 パーサー（mon_span.rs）

MON-SPANパーサーは、UBXプロトコルのMON-SPANメッセージをパースし、
RFスペクトラムデータを構造体に変換する。

**主な振る舞い**:

1. **バイト列→MonSpan構造体への変換**
   - UBXフレーム（ヘッダー、チェックサム含む）を検証
   - ペイロードから各フィールドを抽出

2. **各RFブロックのスペクトラムデータ取得**
   - 256点のスペクトラム振幅（dB）
   - 周波数パラメータ（span, res, center）

3. **PGAゲイン取得（異常検出用）**
   - No.5異常機: 38dB（低い）
   - 正常機: 54dB程度

4. **周波数計算ヘルパー**
   - `f(i) = center + span × (i - 127) / 256`
   - ビンインデックスから実周波数（Hz）を計算

### 1.2 API（mon_span_api.rs）

MON-SPANデータをフロントエンドに提供する。

**エンドポイント**: `GET /api/mon-span`

**処理フロー**:
1. 装置接続確認
2. バッファクリア
3. MON-SPAN poll送信（class=0x0A, id=0x31）
4. 応答受信（タイムアウト1秒）
5. パース
6. レスポンス構築

**レスポンス形式**:
```json
{
  "blocks": [
    {
      "spectrum": [50, 51, 52, ...],
      "span": 128000000,
      "res": 500000,
      "center": 1575420000,
      "pga": 54,
      "max_amplitude": 60,
      "avg_amplitude": 50.5
    }
  ]
}
```

---

## 2. UBX仕様

参照: [ubx-mon-messages.md](ubx-mon-messages.md)

| 項目 | 値 |
|------|-----|
| Class | 0x0A |
| ID | 0x31 |
| ペイロード長 | 4 + numRfBlocks × 272 バイト |

### ブロック構造（272バイト）

| Offset | Type | Name | Unit | 説明 |
|--------|------|------|------|------|
| 0-255 | U1[256] | spectrum | dB | スペクトラムデータ |
| 256-259 | U4 | span | Hz | 周波数スパン |
| 260-263 | U4 | res | Hz | 周波数分解能 |
| 264-267 | U4 | center | Hz | 中心周波数 |
| 268 | U1 | pga | dB | PGAゲイン |
| 269-271 | U1[3] | reserved1 | - | 予約 |

---

## 3. テストリスト

### 3.1 パーサーテスト（mon_span.rs）

| # | テスト名 | カテゴリ | 検証内容 |
|---|----------|----------|----------|
| 1 | test_parse_success_cases | 正常系 | 1ブロック、2ブロック、0ブロックのパース |
| 2 | test_spectrum_extraction | 正常系 | スペクトラム値の抽出、max_amplitude() |
| 3 | test_pga_extraction | 正常系 | PGAゲインの抽出（54dB正常、38dB異常） |
| 4 | test_frequency_calculation | 正常系 | 周波数計算（bin 0, 127, 255） |
| 5 | test_span_res_center_extraction | 正常系 | span/res/centerフィールドの抽出 |
| 6 | test_avg_amplitude | 正常系 | avg_amplitude()ヘルパー |
| 7 | test_parse_error_cases | 異常系 | ヘッダー不正、ID不一致、チェックサム、長さ不足 |
| 8 | test_block_access_helpers | ヘルパー | l1_block(), l2_block() |

### 3.2 APIテスト（mon_span_api.rs）

| ID | シナリオ | 入力 | 期待結果 |
|----|---------|------|----------|
| T1 | 接続済み・正常応答（2ブロック） | 有効なMON-SPAN応答 | 200 + 2ブロック |
| T2 | 未接続 | 未接続状態 | 400 DEVICE_NOT_CONNECTED |
| T3 | タイムアウト | 応答なし | 504 TIMEOUT |
| T4 | パースエラー | 不正UBXデータ | 500 PARSE_ERROR |
| T5 | 空のブロック（nBlocks=0） | 空応答 | 200 + 空配列 |
| T6 | 1ブロックのみ | L1のみ | 200 + 1ブロック |

---

## 4. TDDレビュー結果（Session 118）

### 4.1 仕様書カバレッジ

| 仕様項目 | パーサーテスト | APIテスト |
|----------|---------------|-----------|
| Class/ID (0x0a 0x31) | test_parse_error_cases | - |
| ヘッダー (version, numRfBlocks) | test_parse_success_cases | - |
| ブロック構造 | test_span_res_center_extraction | - |
| spectrum (256点) | test_spectrum_extraction | - |
| span/res/center (U4) | test_span_res_center_extraction | T1 |
| pga (U1) | test_pga_extraction | T1 |
| 周波数計算式 | test_frequency_calculation | - |
| max_amplitude() | test_spectrum_extraction | T1 |
| avg_amplitude() | test_avg_amplitude | - |
| L1/L2アクセス | test_block_access_helpers | - |

### 4.2 結論

**テストは仕様をカバーしている。追加テストは不要。**

---

## 5. 構造体

```rust
pub struct SpanBlock {
    pub spectrum: [u8; 256],  // スペクトラム振幅（dB）
    pub span: u32,            // 周波数スパン（Hz）
    pub res: u32,             // 分解能（Hz）
    pub center: u32,          // 中心周波数（Hz）
    pub pga: u8,              // PGAゲイン（dB）
}

pub struct MonSpan {
    pub version: u8,
    pub n_blocks: u8,
    pub blocks: Vec<SpanBlock>,
}
```

---

## 6. ヘルパーメソッド

| メソッド | 説明 |
|----------|------|
| `frequency_at_bin(i)` | ビンiの周波数（Hz）を計算 |
| `max_amplitude()` | スペクトラムの最大値（dB） |
| `avg_amplitude()` | スペクトラムの平均値（dB） |
| `l1_block()` | L1ブロック（インデックス0）を取得 |
| `l2_block()` | L2ブロック（インデックス1）を取得 |

---

## 7. 実装ファイル

| ファイル | 内容 |
|----------|------|
| [mon_span.rs](../../../../prototype/m1-gnss/backend/src/ubx/mon_span.rs) | パーサー |
| [mon_span_api.rs](../../../../prototype/m1-gnss/backend/src/web/mon_span_api.rs) | API |

---

## 8. 参照

- [ubx-mon-messages.md](ubx-mon-messages.md) — MON-SPAN仕様
- [ADR-008](../../../adr/m1/ADR-008-outdoor-inspection-criteria.md) — 屋外検査合格基準
- [mon_rf.rs](../../../../prototype/m1-gnss/backend/src/ubx/mon_rf.rs) — 参考にした既存パーサー
