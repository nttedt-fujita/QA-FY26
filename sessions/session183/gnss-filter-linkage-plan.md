# GNSSフィルタ連動機能 実装計画

**関連ADR**: [ADR-013: GNSSフィルタ連動と周波数帯別統計表示](../../docs/adr/m1/ADR-013-gnss-filter-linkage.md)

**作成日**: 2026-03-13 (Session 183)

---

## 概要

GNSSモニタ画面の各パネル（SkyPlot、NavSig、MonSpan）間でGNSSフィルタを連動させる。
選択したGNSSの周波数帯別統計も表示できるようにする。

---

## Phase 1: GNSSフィルタ共通化

### 変更ファイル

| ファイル | 変更内容 |
|----------|----------|
| `app/inspections/outdoor/page.tsx` | フィルタ状態を親で管理 |
| `components/SkyPlotPanel.tsx` | フィルタ状態をpropsで受け取る |
| `components/NavSigPanel.tsx` | フィルタ状態をpropsで受け取る |
| `components/MonSpanPanel.tsx` | （Phase 3で対応） |

### 設計

#### 1. GNSS定義の共通化

現在`SkyPlotPanel.tsx`にある`GNSS_LIST`を共通モジュールに移動。

```typescript
// lib/gnss-constants.ts
export const GNSS_LIST = [
  { id: 0, name: "GPS", color: "#3b82f6" },
  { id: 2, name: "Galileo", color: "#f59e0b" },
  { id: 3, name: "BeiDou", color: "#ef4444" },
  { id: 6, name: "GLONASS", color: "#06b6d4" },
  { id: 5, name: "QZSS", color: "#8b5cf6" },
  { id: 1, name: "SBAS", color: "#22c55e" },
] as const;

export type GnssId = typeof GNSS_LIST[number]["id"];
```

#### 2. GNSSフィルタコンポーネント抽出

`SkyPlotPanel`内のフィルタUIを独立コンポーネントに。

```typescript
// components/GnssFilter.tsx
interface GnssFilterProps {
  selectedGnss: Set<number>;
  onGnssChange: (gnss: Set<number>) => void;
  availableGnss?: Set<number>;  // データがあるGNSS
}

export function GnssFilter({ selectedGnss, onGnssChange, availableGnss }: GnssFilterProps) {
  // 凡例ボタン（クリックでトグル）
  // 全選択/全解除ボタン
}
```

#### 3. 親コンポーネントでの状態管理

```typescript
// app/inspections/outdoor/page.tsx
const [selectedGnss, setSelectedGnss] = useState<Set<number>>(
  new Set(GNSS_LIST.map((g) => g.id))
);

return (
  <>
    <GnssFilter
      selectedGnss={selectedGnss}
      onGnssChange={setSelectedGnss}
    />
    <SkyPlotPanel selectedGnss={selectedGnss} />
    <NavSigPanel selectedGnss={selectedGnss} />
    <MonSpanPanel />  {/* Phase 3で対応 */}
  </>
);
```

---

## Phase 2: GNSS×周波数帯統計

### 変更ファイル

| ファイル | 変更内容 |
|----------|----------|
| `components/NavSigPanel.tsx` | 選択GNSSの統計表示を追加 |
| `lib/api.ts` | 型定義の確認（変更不要の可能性） |

### 設計

#### 1. 統計計算の拡張

NAV-SIGデータから選択GNSSの統計を計算。

```typescript
interface GnssFreqStats {
  gnssId: number;
  gnssName: string;
  l1Count: number;
  l2Count: number;
  l1AvgCno: number | null;
  l2AvgCno: number | null;
  l2Rate: number;  // l2Count / l1Count
}

function calculateGnssFreqStats(
  signals: NavSignal[],
  selectedGnss: Set<number>
): GnssFreqStats[] {
  // 選択されたGNSSのみで統計
}
```

#### 2. 統計テーブル表示

```
選択GNSS: GPS, Galileo
┌──────────┬────────┬────────┬──────────┬──────────┬─────────┐
│ GNSS     │ L1衛星 │ L2衛星 │ L1 CNO   │ L2 CNO   │ L2受信率│
├──────────┼────────┼────────┼──────────┼──────────┼─────────┤
│ GPS      │ 8      │ 6      │ 38.2     │ 32.5     │ 75%     │
│ Galileo  │ 5      │ -      │ 35.1     │ -        │ -       │
├──────────┼────────┼────────┼──────────┼──────────┼─────────┤
│ 合計     │ 13     │ 6      │ 37.0     │ 32.5     │ 46%     │
└──────────┴────────┴────────┴──────────┴──────────┴─────────┘
```

---

## Phase 3: RF波形との関連付け（優先度低）

### 変更ファイル

| ファイル | 変更内容 |
|----------|----------|
| `components/MonSpanPanel.tsx` | 選択GNSSの周波数帯情報を表示 |

### 設計

- スペクトラム自体は分離不可（周波数帯全体のデータ）
- 選択GNSSが使う周波数帯を「テキスト注釈」で表示
- 例: 「選択中: GPS → L1/L2帯に関連」

---

## 実装順序

1. **Phase 1-1**: `lib/gnss-constants.ts` を作成
2. **Phase 1-2**: `components/GnssFilter.tsx` を作成
3. **Phase 1-3**: `SkyPlotPanel.tsx` をprops受け取りに変更
4. **Phase 1-4**: `NavSigPanel.tsx` をprops受け取りに変更
5. **Phase 1-5**: `outdoor/page.tsx` でフィルタ状態を管理
6. **Phase 2-1**: NavSigPanelにGNSS×周波数帯統計を追加
7. **Phase 3**: （別セッションで検討）

---

## 動作確認項目

### Phase 1 完了時

- [ ] GNSSフィルタでGPSだけ選択 → SkyPlotでGPSのみ表示
- [ ] GNSSフィルタでGPSだけ選択 → NavSigでGPSのL1/L2のみ表示
- [ ] 全選択/全解除ボタンが動作

### Phase 2 完了時

- [ ] 選択GNSSのL1/L2統計テーブルが表示
- [ ] 複数GNSS選択時は合計行も表示
- [ ] L2受信率が正しく計算される

---

## リスク

| リスク | 対策 |
|--------|------|
| propsが深くなりすぎる | 必要ならContextに移行 |
| NAV-SIGにGNSSフィルタがない | APIレベルでは全取得、FEでフィルタ |
| パフォーマンス低下 | useMemoで再計算を抑制 |
