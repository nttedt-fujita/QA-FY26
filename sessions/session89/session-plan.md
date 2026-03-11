# Session 89 計画

**目的**: FTDI対応 + InspectionEngine/Repository統合

---

## 背景

Session 81-87でGNSS評価ツールのPhase 1 Step 1-4（UBXパーサー、DeviceManager、InspectionEngine、DB Repository）を実装した。Session 88でドキュメント整理を実施。

次はFTDI対応とコンポーネント統合を行い、実機テストに向けた準備を進める。

---

## やること

### 1. FTDI対応

- Session 83で判明: F9P実機はFTDI経由UART接続（VID=0x0403, PID=0x6015）
- フィルタリング: FTDI対応追加 or 手動ポート指定
- ボーレート: 設定可能にする（デフォルト115200、38400も対応）

### 2. InspectionEngineとRepositoryの統合

- InspectionEngineの検査結果をRepositoryに保存
- ロット選択 → 装置検出 → 検査実行 → 結果保存のフロー

### 3. 実機テスト準備

- 統合テスト計画の確認
- テスト項目T1-1〜T1-7の準備

---

## 完了条件

- [ ] FTDI経由での接続が可能
- [ ] ボーレート設定が可能
- [ ] InspectionEngine → Repository の保存フローが動作
- [ ] 次のアクション（実機テスト）が明確

---

## 参照資料

- [Session 83 F9P実機テスト結果](../session83/session-summary.md)
- [Session 87 DB Repository実装](../session87/session-summary.md)
- [17-gnss-tool-implementation-plan.md](../../docs/missions/m1-sensor-evaluation/gnss/17-gnss-tool-implementation-plan.md)

---

*計画作成: 2026-03-11 Session 88終了時*
