# Session 105 計画

**目的**: 実機テスト結果確認 + 屋外検査項目の調査・設計

---

## 背景

Session 104でNMEA ON後のACK待ちを実装。
100回連続テストで0%エラーになることを確認する。

---

## やること

### 1. 実機テスト結果確認

```bash
cd prototype/m1-gnss
make stress-test COUNT=100
```

**期待結果**: Pass=100, Fail=0

### 2. 屋外検査項目の調査・設計

Phase 1実機テスト項目（17-gnss-tool-implementation-plan.md）:
- T1-4: 設定確認（CFG-RATE/CFG-PRTが読める）

現状は屋内検査のみ。次のステップとして:
- 受信感度テスト（屋外）
- スカイプロット表示
- C/N0グラフ表示

**調査内容**:
1. 屋外テストで何を確認するか整理
2. 必要なUBXメッセージ（NAV-SAT, NAV-SIG等）
3. UI要件

---

## 参照資料

- [Session 104 サマリー](../session104/session-summary.md)
- [17-gnss-tool-implementation-plan.md](../../docs/missions/m1-sensor-evaluation/gnss/17-gnss-tool-implementation-plan.md)

---

*計画作成: 2026-03-11 Session 104終了時*
