# Session 181 サマリー

**日付**: 2026-03-13

**概要**: UI改善要望の優先順位整理・Phase 1-2実装

---

## 実施内容

### Phase 1: 確実にできる（完了）

| # | 要望 | 実装内容 |
|---|------|----------|
| 6 | RF波形表示サイズ拡大 | h-16→h-32、viewBox height 60→100 |
| 1 | シリアル番号表示 | BE APIにserial_number追加、履歴画面に表示 |

### Phase 2: 軽い確認→実装（完了）

| # | 要望 | 結果 |
|---|------|------|
| 3 | RF波形サンプルごと確認 | 既にスナップショットにMON-SPAN含まれており実装済み |
| 7 | GNSS切り分け | SkyPlotPanelにフィルタ機能+GNSS別統計テーブル追加 |

---

## #7 GNSS切り分け機能詳細

- **凡例クリックでフィルタ**: GPS/Galileo/BeiDou等を個別ON/OFF
- **全表示/全非表示ボタン**: 一括切り替え
- **GNSS別統計テーブル**: 衛星数、使用中、CNO平均/最小/最大

---

## 残った課題

### Phase 3: 調査が必要（次セッション）

| # | 要望 | 調査内容 |
|---|------|----------|
| 2 | RF波形比較機能 | UI設計、複数データ取得方法 |
| 4 | L1 CNO MIN/MAX/平均 | ADR-008の根拠再確認、仕様変更可能性 |
| 5 | GNSS指定受信 | UBX CFG-GNSS設定調査 |

---

## 変更ファイル

### バックエンド

| ファイル | 変更内容 |
|----------|----------|
| `backend/src/web/outdoor_inspection_api.rs` | OutdoorResultResponseにserial_number追加 |

### フロントエンド

| ファイル | 変更内容 |
|----------|----------|
| `frontend/src/components/MonSpanPanel.tsx` | 波形サイズ拡大（h-32, height=100） |
| `frontend/src/components/SkyPlotPanel.tsx` | GNSSフィルタ機能+統計テーブル追加 |
| `frontend/src/app/inspections/history/page.tsx` | シリアル番号表示追加 |
| `frontend/src/lib/api.ts` | OutdoorInspectionResultにserial_number追加 |

---

## 次セッションでやること

[session182/session-plan.md](../session182/session-plan.md) 参照
