# Session 124 計画

**目的**: Phase 1 — サンプル蓄積・集計の実装

---

## 背景

Session 123で屋外検査のドメインモデルと実装計画を策定した。
Phase 1として、サンプル蓄積と集計ロジックを実装する。

---

## やること（優先順）

### 1. 型定義の作成

**ファイル**: `frontend/src/types/outdoor-inspection.ts`

**内容**:
- `NavStatusSample` — NAV-STATUSのサンプル
- `NavSigSample` — NAV-SIGのサンプル
- `OutdoorInspectionSamples` — 検査中の蓄積データ
- `OutdoorInspectionResult` — 集計結果
- `OutdoorInspectionJudgment` — 判定結果

### 2. 集計関数の実装（TDD）

**ファイル**: `frontend/src/lib/outdoor-inspection-calc.ts`

**関数**:
- `calculateRtkFixRate()` — RTK FIX率
- `calculateRtkFixTime()` — RTK FIX時間
- `calculateL2ReceptionRate()` — L2受信率
- `calculateMinL1Cno()` — L1最小C/N0
- `judgeOutdoorInspection()` — 合否判定

**テスト**: `frontend/src/__tests__/outdoor-inspection-calc.test.ts`

### 3. Hookの作成

**ファイル**: `frontend/src/hooks/useOutdoorInspection.ts`

**責務**:
- 検査状態管理（開始/停止/実行中）
- サンプル蓄積
- 集計処理
- 判定処理

### 4. 屋外検査ページへの統合

**ファイル**: `frontend/src/app/inspections/outdoor/page.tsx`

**変更内容**:
- `useOutdoorInspection` フックの導入
- 既存の検査制御ロジックをフックに移行
- 検査終了時に集計結果を表示（簡易版）

---

## TDD方針

Session 123の計画に従い、テーブルテスト形式で実装する。

**テストケース例**:
```typescript
// RTK FIX率
@pytest.mark.parametrize("samples,expected,should_succeed", [
  // 正常系: 全てFIX
  ([{carr_soln: 2}, {carr_soln: 2}], 1.0, true),
  // 正常系: 半分FIX
  ([{carr_soln: 0}, {carr_soln: 2}], 0.5, true),
  // 正常系: FIXなし
  ([{carr_soln: 0}, {carr_soln: 1}], 0.0, true),
  // 境界値: 空配列
  ([], 0.0, true),
])
```

---

## 参照資料

- [Session 123: ドメインモデル](../session123/outdoor-inspection-domain-model.md)
- [Session 123: 実装計画](../session123/rtk-implementation-plan.md)
- [ADR-008](../../docs/adr/m1/ADR-008-outdoor-inspection-criteria.md)

---

*計画作成: 2026-03-12 Session 123終了時*
