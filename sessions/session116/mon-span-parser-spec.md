# MON-SPANパーサー仕様

**作成日**: 2026-03-12
**セッション**: 116
**実装ファイル**: [prototype/m1-gnss/backend/src/ubx/mon_span.rs](../../prototype/m1-gnss/backend/src/ubx/mon_span.rs)

---

## 振る舞い記述

MON-SPANパーサーは、UBXプロトコルのMON-SPANメッセージ（0x0A 0x31）をパースし、
RFスペクトラムデータを構造体に変換する。

### 主な振る舞い

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

---

## UBX仕様（参照）

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

## テストリスト

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

---

## 構造体

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

## ヘルパーメソッド

| メソッド | 説明 |
|----------|------|
| `frequency_at_bin(i)` | ビンiの周波数（Hz）を計算 |
| `max_amplitude()` | スペクトラムの最大値（dB） |
| `avg_amplitude()` | スペクトラムの平均値（dB） |
| `l1_block()` | L1ブロック（インデックス0）を取得 |
| `l2_block()` | L2ブロック（インデックス1）を取得 |

---

## 次のステップ

MON-SPANを活用するには以下が必要：

1. **MON-SPAN API** — `GET /api/mon-span`
2. **MON-SPAN FE** — スペクトラム波形表示、PGAゲージ

---

## 参照

- [ADR-008](../../docs/adr/m1/ADR-008-outdoor-inspection-criteria.md) — 屋外検査合格基準
- [ubx-mon-messages.md](../../docs/missions/m1-sensor-evaluation/gnss/ubx-mon-messages.md) — MON-SPAN仕様
- [mon_rf.rs](../../prototype/m1-gnss/backend/src/ubx/mon_rf.rs) — 参考にした既存パーサー
