# Session 131 計画

**目的**: M1-GNSS Phase 4 — 検証・報告準備

**前提**: Session 130でPhase 3（DB保存）完了、屋外検査機能は一通り実装済み

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | 実機でのE2Eテスト | - |
| 2 | u-center照合（C/N0、L2受信率、carrSoln確認） | docs/missions/m1-sensor-evaluation/gnss/27-outdoor-inspection-implementation-plan.md（Phase 4セクション） |
| 3 | 差異があれば原因調査・修正 | - |
| 4 | 報告資料作成（向後さん向け） | - |

---

## 詳細

### 1. 実機でのE2Eテスト

- バックエンド起動
- フロントエンド起動
- 実機接続→屋外検査実行→保存→DB確認

### 2. u-center照合

**検証項目**:
1. C/N0値の一致確認
2. L2受信率の計算方法確認
3. carrSolnの値確認（RTK FIX判定）
4. TTFF/msssの値確認

**手順**:
1. 同一デバイスでu-centerとツールを同時起動
2. 値を比較
3. 差異があれば原因調査

### 3. 報告資料

- デモ用シナリオ
- スクリーンショット

---

## 参照

- [Session 130 summary](../session130/session-summary.md)
- [27-outdoor-inspection-implementation-plan.md](../../docs/missions/m1-sensor-evaluation/gnss/27-outdoor-inspection-implementation-plan.md)

---

*計画作成: 2026-03-12 Session 130終了時*
