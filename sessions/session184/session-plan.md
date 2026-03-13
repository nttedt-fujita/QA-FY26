# Session 184 計画

**目的**: GNSSフィルタ連動機能の実装（Phase 1-3〜Phase 2）

**前提**: Session 183でADR-013と実装計画書を作成済み

---

## 作業開始前に必ず読むドキュメント

| ドキュメント | 内容 |
|-------------|------|
| [ADR-013](../../docs/adr/m1/ADR-013-gnss-filter-linkage.md) | GNSSフィルタ連動の設計判断 |
| [実装計画書](../session183/gnss-filter-linkage-plan.md) | Phase別の実装手順 |
| [gnss-constants.ts](../../prototype/m1-gnss/frontend/src/lib/gnss-constants.ts) | 作成済みのGNSS定義（確認用） |
| [GnssFilter.tsx](../../prototype/m1-gnss/frontend/src/components/GnssFilter.tsx) | 作成済みのフィルタコンポーネント（確認用） |

---

## やること

### Phase 1-3〜1-5（フィルタ共通化の残り）

| # | 作業 | 変更ファイル |
|---|------|-------------|
| 1 | SkyPlotPanel.tsx をprops受け取りに変更 | SkyPlotPanel.tsx |
| 2 | NavSigPanel.tsx をprops受け取りに変更 | NavSigPanel.tsx |
| 3 | outdoor/page.tsx でフィルタ状態管理 | outdoor/page.tsx |

### Phase 2（GNSS×周波数帯統計）

| # | 作業 | 変更ファイル |
|---|------|-------------|
| 4 | NavSigPanelにGNSS×周波数帯統計追加 | NavSigPanel.tsx |

---

## 実装手順（計画書より抜粋）

### SkyPlotPanel.tsx の変更

1. `GNSS_LIST`のインポートを`gnss-constants.ts`からに変更
2. `getGnssColor`を`gnss-constants.ts`からインポート
3. ローカルの`visibleGnss`状態を削除
4. `selectedGnss`をpropsで受け取る
5. フィルタUIは`GnssFilter`コンポーネントを使用

### NavSigPanel.tsx の変更

1. `selectedGnss`をpropsで受け取る
2. 信号フィルタリングを`selectedGnss`で実施
3. GNSS×周波数帯統計テーブルを追加

### outdoor/page.tsx の変更

1. `selectedGnss`状態を追加
2. `GnssFilter`コンポーネントを配置
3. 各パネルに`selectedGnss`を渡す

---

## 動作確認項目

- [ ] GNSSフィルタでGPSだけ選択 → SkyPlotでGPSのみ表示
- [ ] GNSSフィルタでGPSだけ選択 → NavSigでGPSのL1/L2のみ表示
- [ ] 全選択/全解除ボタンが動作
- [ ] 選択GNSSのL1/L2統計テーブルが表示

---

## 参照

- [Session 183 サマリー](../session183/session-summary.md)
- [ADR-009: 屋外検査の集計処理をFEで実行](../../docs/adr/m1/ADR-009-outdoor-inspection-fe-aggregation.md)
