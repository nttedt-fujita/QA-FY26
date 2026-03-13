# Session 182 計画

**目的**: 実装確認 + UI改善要望 Phase 3（調査が必要な項目）

**前提**: Session 181でPhase 1-2実装

---

## 最優先：Session 181の実装確認

| 問題 | 確認事項 |
|------|----------|
| RF波形が大きくなっていない | MonSpanPanel.tsxの変更が反映されているか確認 |
| GNSSフィルタがスカイプロットに反映されていない | SkyPlotPanel.tsxのフィルタロジック確認 |
| 仰角の情報が扱われているか | NAV-SATのelevフィールド、スカイプロットでの使用確認 |

---

## やること

### Phase 3: 調査が必要

| # | 要望 | 調査内容 | 読むべきファイル |
|---|------|----------|-----------------|
| 2 | RF波形比較機能 | UI設計、複数データ取得方法 | MonSpanPanel.tsx |
| 4 | L1 CNO MIN/MAX/平均 | ADR-008の根拠再確認、仕様変更可能性 | ADR-008, outdoor-inspection-design.md |
| 5 | GNSS指定受信 | UBX CFG-GNSS設定調査 | u-blox仕様書 |

---

## 参照

- [Session 181 サマリー](../session181/session-summary.md)
- [ADR-008: 屋外検査の合格基準](../../docs/adr/m1/ADR-008-outdoor-inspection-criteria.md)
