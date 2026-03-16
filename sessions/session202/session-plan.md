# Session 202 計画

**目的**: 残作業（シリアル表示修正 + 古い機テスト）

**前提**: Session 201でNAV-TIMEQZSS実装完了、試作機2台テスト済み

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | 装置画面のシリアル表示修正 | `prototype/m1-gnss/frontend/src/components/DeviceCard.tsx` |
| 2 | 古い機2台のテスト | - |

---

## 詳細

### 1. 装置画面のシリアル表示問題

**現状**:
- DeviceCard.tsxで`serial_number`（FTDIのシリアル）を表示
- F9Pのシリアル（`f9p_serial`）と異なる値が出て混乱の原因

**対策**:
- `f9p_serial`を優先表示、なければ`serial_number`をフォールバック

### 2. 古い機テスト

Session 201で確認済み:
- 検証用: D30I4QFD / 9543F2097F
- 試作機1: D30I4QFD / A5400AEB1F
- 試作機2: D30I4QFD / A44052ED9D

未確認:
- 古い機2台

---

## 参照

- [Session 201 summary](../session201/session-summary.md)
