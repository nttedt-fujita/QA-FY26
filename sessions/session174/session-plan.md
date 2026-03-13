# Session 174 計画

**目的**: 生データ保存機能 Phase 2（FE）または テストコード修正

**前提**: Session 173でPhase 1（BE）完了

---

## やること（選択肢）

### A. Phase 2: FE実装（推奨）

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | useOutdoorInspection拡張（スナップショット蓄積） | frontend/src/hooks/useOutdoorInspection.ts |
| 2 | saveResult拡張（スナップショット送信） | frontend/src/lib/api.ts |
| 3 | 動作確認 | - |

### B. テストコード修正

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | serialportクレートAPI変更への対応 | backend/src/device/manager.rs等（テスト部分） |

---

## 参照

- [session173/session-summary.md](../session173/session-summary.md) - 前セッションサマリー
- [session172/raw-data-storage-plan.md](../session172/raw-data-storage-plan.md) - 設計計画書
