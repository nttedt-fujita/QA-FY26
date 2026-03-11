# Session 107 計画

**目的**: NAV-SIGパーサー実装（TDD）

---

## 背景

Session 106で屋外検査の要求整理が完了。NAV-SIGを使ってL1/L2別の受信感度を取得する方針。

---

## やること

### 1. NAV-SIGパーサー実装（TDD）

**Phase 0-2**: 振る舞い記述・テストシナリオ設計
- NAV-SIGメッセージ構造を確認（ubx-spec-memo.md参照）
- テストケース設計

**Phase 3-5**: 実装
- nav_sig.rsモジュール作成
- パーサー実装
- テスト実行

### 2. ドキュメント整理（直近セッションの資料メンテナンス）

要求整理が形になったので、直近セッション（Session 101-106）で作成したドキュメントを整理:
- 不要な中間ファイルの削除
- 正式配置先への移動（docs/missions/m1-sensor-evaluation/gnss/）
- 重複・上位互換の整理

### 3. 受信感度一覧表示の設計（時間があれば）

- UIモックアップ検討
- API設計

---

## 参照資料

- [Session 106 サマリー](../session106/session-summary.md)
- [ubx-spec-memo.md](../session106/ubx-spec-memo.md) — NAV-SIG仕様
- [outdoor-inspection-needs.md](../session106/outdoor-inspection-needs.md) — 要求整理

---

*計画作成: 2026-03-11 Session 106終了時*
