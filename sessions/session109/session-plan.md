# Session 109 計画

**目的**: NAV-SIG振る舞い仕様更新 + 実装（TDD Phase 3-5）

---

## 背景

Session 108で屋外検査の合格基準（ADR-008）を確定し、以下を決定：
- qualityInd ≥ 5 を「L2受信中」の定義
- 集計ロジックはフリー関数（signal_stats）
- 閾値判定は検査ロジックの責任

---

## やること

### 1. NAV-SIG振る舞い仕様の更新

- [ ] sessions/session107/nav-sig-behavior-spec.md を更新
- [ ] ADR-008の決定を反映
  - qualityInd判定の振る舞い追加
  - signal_statsの振る舞い追加
- [ ] テストシナリオリストの更新

### 2. NAV-SIGパーサー実装（TDD Phase 3-5）

**Phase 3**: テストコード作成
- [ ] nav_sig.rsのテスト作成
- [ ] sigId判定バグ修正のテスト

**Phase 4**: 実装（Red → Green）
- [ ] sigId判定バグ修正
- [ ] パース機能実装

**Phase 5**: リファクタリング
- [ ] コード整理

### 3. signal_statsモジュール実装

- [ ] gps_visible_count()
- [ ] gps_l2_reception_count()
- [ ] gps_l2_reception_rate()
- [ ] テスト作成

---

## 参照資料

- [ADR-008](../../docs/adr/m1/ADR-008-outdoor-inspection-criteria.md) — 屋外検査の合格基準
- [Session 108 サマリー](../session108/session-summary.md)
- [nav-sig-behavior-spec.md](../session107/nav-sig-behavior-spec.md) — 振る舞い仕様（更新対象）
- [ubx-signal-identifiers.md](../../docs/missions/m1-sensor-evaluation/gnss/ubx-signal-identifiers.md) — sigId定義

---

*計画作成: 2026-03-11 Session 108終了時*
