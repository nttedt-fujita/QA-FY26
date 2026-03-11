# Session 86 計画

**目的**: 受入検査ドメインモデリング完了

---

## 背景

Session 85でドメインモデリング未実施のままDB設計に入った問題が発覚。
運用イメージも当初の理解と違っていた。

---

## やること

### 1. ドメインモデリング完了

**運用イメージを反映**:
- 最初の1台が仮の期待値
- 複数台検査で多数派がわかる
- はずれ値も比較対象として保存

**追加で整理**:
- 屋外確認項目の要求整理
- 全結果保存・比較可能な設計

### 2. DB設計見直し

ドメインモデルに基づいて再設計:
- 現在のrepositoryコードは参考として残す
- 必要に応じて修正

### 3. FTDI対応方針（時間があれば）

Session 83で判明した問題:
- 実機はFTDI経由UART接続
- F9P直接のVID/PIDではない

---

## 完了条件

- [ ] ドメインモデルが運用イメージを反映している
- [ ] 屋外確認項目が整理されている
- [ ] DB設計がドメインモデルと整合している

---

## 参照資料

- [session85/incoming-inspection-domain-model.md](../session85/incoming-inspection-domain-model.md) — 途中のドメインモデル
- [session85/session-summary.md](../session85/session-summary.md) — Session 85サマリー
- [14-gnss-tool-needs.md](../../docs/missions/m1-sensor-evaluation/gnss/14-gnss-tool-needs.md) — 要求定義

---

*計画作成: 2026-03-11 Session 85終了時*
