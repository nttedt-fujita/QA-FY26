# Session 113 計画

**目的**: NAV-SIG フロントエンド実装

---

## 背景

Session 112で `/api/nav-sig` エンドポイントを実装済み。
次はフロントエンドでL1/L2別C/N0一覧とL2受信率を表示する。

---

## やること

### 1. フロントエンド実装

**必須機能**:
- L1/L2別C/N0一覧テーブル
- L2受信率ゲージ（50%基準で色分け）
- 合格/不合格表示

**コンポーネント構成**（案）:
- `NavSigPanel.tsx` — メインコンポーネント
- `SignalTable.tsx` — 信号一覧テーブル
- `ReceptionRateGauge.tsx` — L2受信率ゲージ

### 2. API連携

- `/api/nav-sig` を定期ポーリング（1秒間隔）
- エラーハンドリング（未接続、タイムアウト）

### 3. 実機テスト

- 実機で動作確認
- L2受信状況の表示確認

---

## 参照資料

- [nav-sig-api-design.md](../session112/nav-sig-api-design.md) — API設計
- [ADR-008](../../docs/adr/m1/ADR-008-outdoor-inspection-criteria.md) — 合格基準
- [m1-gnss-milestone.md](../session111/m1-gnss-milestone.md) — マイルストーン

---

## 成果物（予定）

- `NavSigPanel.tsx`
- `SignalTable.tsx`
- `ReceptionRateGauge.tsx`
- 画面モックアップ（必要に応じて）

---

*計画作成: 2026-03-11 Session 112終了時*
