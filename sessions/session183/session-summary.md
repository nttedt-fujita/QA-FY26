# Session 183 サマリー

**日付**: 2026-03-13

**概要**: GNSSフィルタ連動機能の設計・計画

---

## 実施内容

### 1. 紐付け機能の要件整理

ユーザー要望:
- 「どのGNSSが受信率に影響大きいか」を分析したい
- フィルタを切り替えて比較できるようにしたい

### 2. ADR-013作成

[ADR-013: GNSSフィルタ連動と周波数帯別統計表示](../../docs/adr/m1/ADR-013-gnss-filter-linkage.md)

| 決定 | 内容 |
|------|------|
| フィルタ共通化 | 親コンポーネントで状態管理、propsで各パネルに渡す |
| 統計計算 | FEで実行（ADR-009と同じ判断） |
| 実装範囲 | Phase 1-2-3の段階的実装 |

### 3. 実装計画書作成

[gnss-filter-linkage-plan.md](gnss-filter-linkage-plan.md)

### 4. Phase 1-1, 1-2 実装開始

| ファイル | 内容 |
|----------|------|
| `lib/gnss-constants.ts` | GNSS定義の共通モジュール（新規） |
| `components/GnssFilter.tsx` | GNSSフィルタコンポーネント（新規） |

---

## 作成・変更ファイル

| ファイル | 変更内容 |
|----------|----------|
| `docs/adr/m1/ADR-013-gnss-filter-linkage.md` | 新規作成 |
| `sessions/session183/gnss-filter-linkage-plan.md` | 新規作成 |
| `prototype/m1-gnss/frontend/src/lib/gnss-constants.ts` | 新規作成 |
| `prototype/m1-gnss/frontend/src/components/GnssFilter.tsx` | 新規作成 |
| `CLAUDE.md` | ADR一覧にADR-013追加 |

---

## 残った作業（Phase 1-3以降）

| Phase | 作業 | 状態 |
|-------|------|------|
| 1-3 | SkyPlotPanel.tsx をprops受け取りに変更 | 未着手 |
| 1-4 | NavSigPanel.tsx をprops受け取りに変更 | 未着手 |
| 1-5 | outdoor/page.tsx でフィルタ状態管理 | 未着手 |
| 2 | NavSigPanelにGNSS×周波数帯統計追加 | 未着手 |
| 3 | MonSpanとの関連付け（優先度低） | 未着手 |

---

## 次セッションでやること

[session184/session-plan.md](../session184/session-plan.md) 参照
