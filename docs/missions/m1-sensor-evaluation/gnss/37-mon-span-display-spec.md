# MON-SPAN表示仕様

**作成**: Session 190 (2026-03-16)
**出典**: ZED-F9P Integration Manual (UBX-18010802 R16), u-blox F9 HPG 1.32 Interface Description

---

## 概要

MON-SPANメッセージのスペクトラム波形を表示する際の、縦軸（振幅）と横軸（周波数）の変換仕様。

---

## 縦軸（振幅）: dB変換

### 採用した式

```
dB = spectrum × 0.25
```

### 根拠

**出典**: ZED-F9P Integration Manual p.83, Section 3.18 "Spectrum analyzer"

> Each block comprises the following data:
> - **256 spectrum data points (0.25 dB units)**
> - Spectrum span (Hz)
> - Spectrum bin resolution (Hz)
> - Center frequency (Hz)
> - PGA setting (dB)

「0.25 dB units」という記載から、spectrum値は0.25dB単位であることが明確。

### 値の範囲

| spectrum値 | dB値 |
|-----------|------|
| 0 | 0 dB |
| 128 | 32 dB |
| 255 | 63.75 dB |

---

## 横軸（周波数）: 周波数計算

### 採用した式

```
Freq(i) = center + span × (i - 128) / 256
```

- `i`: bin番号（0〜255）
- `center`: 中心周波数 [Hz]（MON-SPANのcenterフィールド）
- `span`: スペクトラム幅 [Hz]（MON-SPANのspanフィールド）

### 根拠

2つの公式ドキュメントに記載があり、微小な差異がある：

| 出典 | 式 | オフセット |
|------|-----|----------|
| **Integration Manual p.83** | `center + span × (i - 128) / 256` | **-128** |
| Interface Description p.134 | `center + span × (i - 127) / 256` | -127 |

### 採用判断

**Integration Manual (R16, 2024年10月) を採用**

理由：
1. Integration Manualの方が新しいリビジョン（R16）
2. 差は 1bin = span/256 Hz（約195kHz @ span=50MHz）で実用上の影響は軽微
3. Integration Manualはアプリケーション向けガイドであり、実装の参照として適切

### 原文

**Integration Manual p.83:**
> "The frequency of each point can be calculated by **Freq(i) = center frequency + spectrum span × (i-128) / 256**, where i=0-255."

**Interface Description p.134:**
> "The center frequency at each bin, assuming a zero-based bin count, can be computed as **f(i) = center + span × (i - 127) / 256**"

---

## 実装上の注意

### 公式の注記

Integration Manual p.83には以下の注記がある：

> "This message is intended for **comparative analysis** of the RF spectrum rather than **absolute and precise measurement**."

スペクトラム表示は相対的な比較用途であり、絶対的・精密な測定用途ではない。

### PGAゲインについて

Interface Description p.134より：

> "Note that the PGA gain is not included in the spectrum data but is available as a separate field."

PGAゲインはspectrum値に含まれていない。必要に応じて別途表示する。

---

## 関連ドキュメント

- [23-mon-span-implementation.md](23-mon-span-implementation.md) — MON-SPAN実装仕様（パーサー）
- [ubx-mon-messages.md](ubx-mon-messages.md) — UBX MONメッセージ仕様
- [ADR-008](../../../adr/m1/ADR-008-outdoor-inspection-criteria.md) — 屋外検査の合格基準

---

## 抽出済みPDF

Session 190で抽出したIntegration Manualの該当ページ：
- [session190/integration-manual-spectrum-analyzer.md](../../../../sessions/session190/integration-manual-spectrum-analyzer.md) — p.83-85
