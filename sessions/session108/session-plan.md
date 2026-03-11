# Session 108 計画

**目的**: 屋外検査要求の確定 + NAV-SIGパーサー実装（TDD Phase 2-5）

---

## 背景

Session 107でNAV-SIG仕様を調査し、TDD Phase 1（振る舞い記述）まで完了。
sigId定義を仕様書から正確に抽出し、既存コードにバグがあることを発見。

---

## やること

### 1. 屋外検査要求の確定（優先度: 高）

- [ ] sessions/session106/outdoor-inspection-needs.mdを確認
- [ ] 合格基準を業界標準ベースで確定
  - L1受信感度: ≥30 dBHz（業界標準と一致）
  - L2受信率: GPS 50%以上（要確認）
  - RTK FIX時間: ≤60秒（叩き台）
- [ ] 未確定事項の整理（ヒアリング項目の優先度付け）
- [ ] docs/missions/m1-sensor-evaluation/gnss/に正式配置

### 2. NAV-SIGパーサー実装（TDD Phase 2-5）

**Phase 2**: テストシナリオリスト承認
- [ ] nav-sig-behavior-spec.mdのテストシナリオリストを確認
- [ ] ユーザー承認を得る

**Phase 3-5**: 実装
- [ ] nav_sig.rsのsigId判定バグ修正
- [ ] テストコード作成
- [ ] 実装・テスト実行

### 3. NAV-SATパーサー実装（時間があれば）

- [ ] スカイプロット用の仕様確認
- [ ] TDD Phase 1から開始

---

## 参照資料

- [Session 107 サマリー](../session107/session-summary.md)
- [nav-sig-behavior-spec.md](../session107/nav-sig-behavior-spec.md) — 振る舞い仕様
- [ubx-signal-identifiers.md](../../docs/missions/m1-sensor-evaluation/gnss/ubx-signal-identifiers.md) — sigId定義
- [ubx-nav-messages.md](../../docs/missions/m1-sensor-evaluation/gnss/ubx-nav-messages.md) — NAV-SAT/NAV-SIG仕様
- [outdoor-inspection-needs.md](../session106/outdoor-inspection-needs.md) — 屋外検査要求（ドラフト）

---

*計画作成: 2026-03-11 Session 107終了時*
