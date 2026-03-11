# Session 90 計画

**目的**: ボーレート自動判定調査 + コンポーネント統合

---

## 背景

Session 89でFTDI対応とボーレート設定機能を実装した。次はボーレート自動判定の可能性を調査し、InspectionEngine → Repository統合を進める。

---

## やること

### 1. ZED-F9P Integration ManualのPDF読み込み

- Pythonスクリプトで目次を抽出
- ボーレート関連セクションを特定（Auto-baud, UART設定など）
- 該当ページを抽出・分析

### 2. ボーレート対応方針の決定

調査結果に基づいて方針を決定：
- A: 自動判定機能あり → DeviceManagerに実装
- B: 自動判定機能なし → 複数ボーレート試行方式
- C: 手動指定のまま → 現場運用として問題なし

ADR候補として記録。

### 3. InspectionEngine → Repository統合（時間があれば）

- 検査結果をDBに保存するフロー
- ロット選択 → 装置検出 → 検査実行 → 結果保存

---

## 完了条件

- [ ] Integration Manualの目次を抽出
- [ ] ボーレート関連セクションを特定・分析
- [ ] ボーレート対応方針を決定（ADR候補）
- [ ] （任意）InspectionEngine → Repository統合

---

## 参照資料

- [Session 89 調査計画](../session89/baud-rate-investigation-plan.md)
- [Session 78 CFG-PRT抽出結果](../session78/cfg-rate-prt-raw.md)
- ZED-F9P Integration Manual（ユーザーPDF取得済み）

---

*計画作成: 2026-03-11 Session 89終了時*
