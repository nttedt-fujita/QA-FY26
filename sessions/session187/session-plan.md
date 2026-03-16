# Session 187 計画

**前提**: Session 186でRF波形比較機能を実装済み

---

## やること

### 1. u-center目盛り調査（優先）

**背景**: 現在の目盛り（0-255）がu-centerと異なる可能性

**タスク**:
1. u-centerの説明書/マニュアルをWeb検索
2. MON-SPAN仕様書の再確認（Interface Description PDF）
3. u-centerのSpectrum画面の縦軸表示を確認
4. 必要に応じて目盛り修正

**調査観点**:
- u-centerの縦軸は何dB〜何dBか
- 0-255の値がそのままdB値なのか、変換が必要か

### 2. RF波形比較機能の動作確認（実機）

**タスク**:
- 比較ページ（/inspections/compare）の動作確認
- 2つのデータを選んで重ね表示できるか確認

### 3. 並列検査機能の要件整理

**背景**: Phase 3で計画されている複数台同時検査

**タスク**:
- 現在のDeviceManager実装を確認
- 複数装置接続時の挙動を整理
- 実装方針を検討

---

## 読むべきファイル

| ドキュメント | 内容 |
|-------------|------|
| [23-mon-span-implementation.md](../../docs/missions/m1-sensor-evaluation/gnss/23-mon-span-implementation.md) | MON-SPAN仕様 |
| [ubx-mon-messages.md](../../docs/missions/m1-sensor-evaluation/gnss/ubx-mon-messages.md) | UBX仕様 |
| [17-gnss-tool-implementation-plan.md](../../docs/missions/m1-sensor-evaluation/gnss/17-gnss-tool-implementation-plan.md) | Phase 3並列検査計画 |

---

## 参照

- [Session 186 サマリー](../session186/session-summary.md)
