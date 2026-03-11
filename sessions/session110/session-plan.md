# Session 110 計画

**目的**: NAV-SIGパーサー + signal_stats実装（TDD Phase 3-5）

---

## 背景

Session 109でテストシナリオリストを確定：
- 8.2 L1/L2判定: 17件（バグ修正対象）
- 8.4 signal_stats: 12件（新規実装）

---

## やること

### Phase 3: テストコード作成

**L1/L2判定テスト（既存テストの修正 + 追加）**:
- [ ] 既存テストを確認（バグった仕様で書かれているものを特定）
- [ ] 正しい仕様に基づくテストに修正
- [ ] 不足しているGNSS（SBAS, BeiDou, QZSS）のテスト追加

**signal_statsテスト（新規作成）**:
- [ ] signal_stats.rs 新規作成
- [ ] gps_visible_count テスト4件
- [ ] gps_l2_reception_count テスト4件
- [ ] gps_l2_reception_rate テスト4件

### Phase 4: 実装（Red → Green）

**sigId判定バグ修正**:
- [ ] is_l1(): GPS sigId=3削除、QZSS sigId=3削除、BeiDou sigId=5追加
- [ ] is_l2(): GPS sigId=3追加、QZSS sigId=5追加、BeiDou 2,3追加

**signal_stats実装**:
- [ ] gps_visible_count()
- [ ] gps_l2_reception_count()
- [ ] gps_l2_reception_rate()

### Phase 5: リファクタリング

- [ ] コード整理
- [ ] 全テスト通過確認

---

## 参照資料

- [nav-sig-behavior-spec.md](../session107/nav-sig-behavior-spec.md) — 振る舞い仕様（確定版）
- [ADR-008](../../docs/adr/m1/ADR-008-outdoor-inspection-criteria.md) — 屋外検査の合格基準
- [ubx-signal-identifiers.md](../../docs/missions/m1-sensor-evaluation/gnss/ubx-signal-identifiers.md) — sigId定義

---

*計画作成: 2026-03-11 Session 109終了時*
