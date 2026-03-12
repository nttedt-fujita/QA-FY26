# RTK実装計画

**作成日**: 2026-03-12 Session 123
**目的**: 屋外検査の集計・判定機能を実装する計画を立てる

---

## 概要

ドメインモデル設計（[outdoor-inspection-domain-model.md](./outdoor-inspection-domain-model.md)）に基づき、
以下の順序で実装を進める。

---

## 改訂版スケジュール（Session 123末時点）

| Session | 内容 | 備考 |
|---------|------|------|
| 124 | Phase 1: 集計ロジック | DB無関係、先に進めてOK |
| 125 | Phase 2: 結果表示UI | DB無関係、先に進めてOK |
| **126** | **全体設計レビュー** | ER図作成、DB設計チェック |
| 127 | Phase 3: DB保存 | 設計レビュー後に実装 |
| 128 | Phase 4: 検証・報告 | - |

**変更理由**: 屋外検査DB設計の前に、全体（屋内/屋外/デバイス/ロット）の整合性チェックが必要

---

## Phase 1: サンプル蓄積・集計（Session 124予定）

### 1.1 Hook作成: `useOutdoorInspection.ts`

**目的**: 検査中のサンプル蓄積と集計処理をカスタムフックに切り出す

**責務**:
- 検査状態管理（開始/停止/実行中）
- サンプル蓄積（NavStatus, NavSig）
- 集計処理
- 判定処理

**インターフェース**:

```typescript
interface UseOutdoorInspectionReturn {
  // 状態
  isInspecting: boolean;
  remainingTime: number;
  samples: OutdoorInspectionSamples | null;
  result: OutdoorInspectionResult | null;

  // アクション
  startInspection: (durationSec: number) => void;
  stopInspection: () => void;
  resetInspection: () => void;
}
```

### 1.2 型定義: `types/outdoor-inspection.ts`

ドメインモデルで設計した型を実装。

### 1.3 集計関数: `lib/outdoor-inspection-calc.ts`

- `calculateRtkFixRate()`
- `calculateRtkFixTime()`
- `calculateL2ReceptionRate()`
- `calculateMinL1Cno()`
- `judgeOutdoorInspection()`

**テスト**: `__tests__/outdoor-inspection-calc.test.ts`

---

## Phase 2: 結果表示UI（Session 125予定）

### 2.1 検査結果パネル: `OutdoorInspectionResultPanel.tsx`

**表示内容**:
- 検査サマリー（検査時間、サンプル数）
- 各指標の値と判定結果
- 総合判定（合格/不合格）
- 不合格理由（ある場合）

**UI設計**:

```
┌────────────────────────────────────────────┐
│ 検査結果                                    │
├────────────────────────────────────────────┤
│ 検査時間: 30秒  サンプル数: 30             │
├────────────────────────────────────────────┤
│                                            │
│  [大きく] ✓ 合格 / ✗ 不合格                │
│                                            │
├────────────────────────────────────────────┤
│ L1受信感度    42 dBHz     ✓ ≥30           │
│ L2受信率      68%         ✓ ≥50%          │
│ RTK FIX時間   8.2秒       ✓ ≤30秒         │
│ RTK FIX率     98.3%       ✓ >95%          │
├────────────────────────────────────────────┤
│ [結果を保存] [再検査]                      │
└────────────────────────────────────────────┘
```

### 2.2 屋外検査ページ改修: `outdoor/page.tsx`

- `useOutdoorInspection` フックの導入
- 検査結果パネルの追加
- 既存のリアルタイム表示との統合

---

## Phase 3: DB保存（Session 126予定）

### 3.1 DBスキーマ追加

```sql
-- マイグレーション
CREATE TABLE outdoor_inspection_results (
  id INTEGER PRIMARY KEY,
  device_id INTEGER REFERENCES devices(id),
  lot_id INTEGER REFERENCES lots(id),
  inspected_at TEXT NOT NULL,
  duration_sec INTEGER NOT NULL,
  sample_count INTEGER NOT NULL,
  rtk_fix_rate REAL NOT NULL,
  rtk_fix_time_ms INTEGER,
  l2_reception_rate REAL NOT NULL,
  l1_min_cno REAL NOT NULL,
  is_pass BOOLEAN NOT NULL,
  l1_cno_pass BOOLEAN NOT NULL,
  l2_rate_pass BOOLEAN NOT NULL,
  rtk_fix_time_pass BOOLEAN NOT NULL,
  rtk_fix_rate_pass BOOLEAN NOT NULL,
  failure_reasons TEXT,
  created_at TEXT DEFAULT CURRENT_TIMESTAMP
);
```

### 3.2 バックエンドAPI

**エンドポイント**: `POST /api/outdoor-inspection-results`

**リクエスト**:
```json
{
  "device_id": 1,
  "lot_id": 1,
  "inspected_at": "2026-03-12T10:30:00Z",
  "duration_sec": 30,
  "sample_count": 30,
  "rtk_fix_rate": 0.983,
  "rtk_fix_time_ms": 8200,
  "l2_reception_rate": 0.68,
  "l1_min_cno": 42.0,
  "is_pass": true,
  "l1_cno_pass": true,
  "l2_rate_pass": true,
  "rtk_fix_time_pass": true,
  "rtk_fix_rate_pass": true,
  "failure_reasons": []
}
```

**レスポンス**: 作成されたレコード

### 3.3 フロントエンド保存機能

- 保存ボタン追加
- 保存成功/失敗のフィードバック

---

## Phase 4: 検証・報告準備（Session 127予定）

### 4.1 u-center照合

**検証項目**:
1. C/N0値の一致確認
2. L2受信率の計算方法確認
3. carrSolnの値確認
4. TTFF/msssの値確認

**手順**:
1. 同一デバイスでu-centerとツールを同時起動
2. 値を比較
3. 差異があれば原因調査

### 4.2 報告資料作成

- 向後さんへの報告用スライド
- デモ用シナリオ

---

## TDD方針

各Phaseで以下のテスト戦略を適用:

| 対象 | テスト種別 | 備考 |
|------|----------|------|
| 集計関数 | ユニットテスト | テーブルテスト形式 |
| 判定関数 | ユニットテスト | 境界値テスト重視 |
| Hook | React Testing Library | 状態遷移テスト |
| API | 統合テスト | リクエスト/レスポンス確認 |

---

## リスク・課題

| リスク | 対策 |
|--------|------|
| RTK FIXしない環境でテストできない | NAV-STATUSのモックデータを用意 |
| u-centerと値が合わない | 仕様書を再確認、パース処理を見直し |
| サンプル数が足りない | サンプリング間隔を500msに短縮可能 |

---

## 参照資料

- [remaining-tasks.md](./remaining-tasks.md) — 残タスク一覧
- [outdoor-inspection-domain-model.md](./outdoor-inspection-domain-model.md) — ドメインモデル
- [ADR-008](../../docs/adr/m1/ADR-008-outdoor-inspection-criteria.md) — 合格基準
