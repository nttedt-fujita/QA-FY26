# Session 183 計画

**目的**: UI改善Phase 3（調査が必要な項目）+ 紐付け機能

**前提**: Session 182でRF波形改善完了

---

## やること

### 優先度高

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | スカイプロットと衛星信号の紐付け方針決定 | - |
| 2 | 紐付け機能実装（案A/B/Cから選択後） | SkyPlotPanel.tsx, MonSpanPanel.tsx |

### 優先度中

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 3 | L1 CNO MIN/MAX/平均の根拠再確認 | ADR-008, outdoor-inspection-design.md |
| 4 | RF波形比較機能の設計 | MonSpanPanel.tsx |

### 優先度低（調査）

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 5 | GNSS指定受信（UBX CFG-GNSS） | u-blox仕様書 |

---

## 紐付け機能の案（Session 182で提示、回答待ち）

- **案A**: GNSSフィルタを共通化（GPSだけ表示→両パネルで連動）
- **案B**: L1/L2帯とその帯域のGNSS統計を並べて表示
- **案C**: 衛星をクリック→その衛星が使う周波数帯をハイライト

---

## 参照

- [Session 182 サマリー](../session182/session-summary.md)
- [ADR-008: 屋外検査の合格基準](../../docs/adr/m1/ADR-008-outdoor-inspection-criteria.md)
