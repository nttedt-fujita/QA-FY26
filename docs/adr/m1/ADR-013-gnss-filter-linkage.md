# ADR-013: GNSSフィルタ連動と周波数帯別統計表示

## ステータス

提案中 (Session 183)

## コンテキスト

### 背景

GNSSモニタ画面には複数のパネルがある：

| パネル | データソース | 表示内容 |
|--------|-------------|---------|
| SkyPlotPanel | NAV-SAT | 衛星位置（スカイプロット）、GNSS別統計 |
| NavSigPanel | NAV-SIG | 衛星×信号別CNO、L2受信率 |
| MonSpanPanel | MON-SPAN | L1/L2帯スペクトラム波形 |

### 課題

1. **パネル間の紐付けがない**
   - スカイプロットでGNSSをフィルタリングしても他パネルに影響しない
   - 「GPSだけ見たい」とき、手動で各パネルを見比べる必要がある

2. **GNSS×周波数帯の関係が不明瞭**
   - 各GNSSがどの周波数帯を使うか画面上でわからない
   - L1/L2帯の状態とGNSSの関係を分析しにくい

3. **ユーザー要求**
   - 「どのGNSSが受信率に影響大きいか」を分析したい
   - フィルタを切り替えて比較できるようにしたい

## 決定

### 1. GNSSフィルタの共通化

**現状**: SkyPlotPanel内でローカル状態として管理

```tsx
// 現状（SkyPlotPanel.tsx内）
const [visibleGnss, setVisibleGnss] = useState<Set<number>>(...);
```

**変更後**: 親コンポーネント（ページ）でフィルタ状態を管理し、各パネルにpropsで渡す

```tsx
// 親コンポーネント
const [selectedGnss, setSelectedGnss] = useState<Set<number>>(...);

// 各パネルに渡す
<SkyPlotPanel selectedGnss={selectedGnss} onGnssChange={setSelectedGnss} />
<NavSigPanel selectedGnss={selectedGnss} />
<MonSpanPanel selectedGnss={selectedGnss} />
```

### 2. GNSS×周波数帯の統計表示

NAV-SIGデータを使い、選択されたGNSSの周波数帯別統計を表示する。

**表示項目**（選択GNSS限定）:

| 項目 | 説明 |
|------|------|
| L1受信衛星数 | L1信号を受信している衛星数 |
| L2受信衛星数 | L2信号を受信している衛星数 |
| L1 CNO平均 | L1信号のCNO平均値 |
| L2 CNO平均 | L2信号のCNO平均値 |
| L2受信率 | L2/L1の比率（既存指標） |

### 3. 周波数帯とGNSSの対応表示

各GNSSが使用する周波数帯の対応を視覚的に表示する。

| GNSS | L1帯 (1575MHz) | L2帯 (1227MHz) |
|------|----------------|----------------|
| GPS | L1 C/A, L1C | L2C |
| Galileo | E1 | (E5a/E5b) |
| BeiDou | B1C | B2a |
| GLONASS | L1 | L2 |
| QZSS | L1 | L2 |

## 実装範囲

### Phase 1: GNSSフィルタ共通化

- GnssFilterContext（またはpropsリレー）でフィルタ状態を共有
- SkyPlotPanelのフィルタUIを抽出して共通化
- 全パネルがフィルタに反応

### Phase 2: GNSS×周波数帯統計

- NavSigPanelを拡張し、選択GNSSのL1/L2統計を表示
- または新しい「統計パネル」を追加

### Phase 3: RF波形との関連付け（優先度低）

- MonSpanPanelで選択GNSSの使用周波数帯をハイライト
- 参考情報として表示（スペクトラム自体は分離不可）

## 根拠: FE計算を採用する理由

ADR-009（屋外検査の集計処理）と同様、統計計算はFEで行う。

| 観点 | 判断 |
|------|------|
| **フィルタ切り替え** | 即座に反映したい → APIコール待ちは不適 |
| **データ所在** | NAV-SIG/NAV-SATは統合APIで既にFEにある |
| **計算量** | 〜50衛星×信号の配列操作は<1ms |
| **BE変更** | 不要（新規API追加なし） |

### BEで計算する必要がないケース

- 大量データ（数千件）のフィルタ → **該当しない**
- 複雑な計算（機械学習等） → **該当しない**
- 複数端末で同じ結果が必要 → **該当しない**

---

## 帰結

### メリット

- GNSS別の分析が容易になる
- 受信状態の比較がしやすくなる
- フィルタ操作が1箇所で済む

### デメリット

- 状態管理が複雑になる（Context or Props drilling）
- UIの変更が必要

### 採用しなかった案

- **案A: パネル間でイベント連携** → 結合度が高くなりすぎる
- **案B: 各パネルに独立フィルタ** → 現状維持で改善にならない

## 変更履歴

| 日付 | 変更内容 | 関連セッション |
|------|----------|---------------|
| 2026-03-13 | 初版作成（提案中） | Session 183 |
