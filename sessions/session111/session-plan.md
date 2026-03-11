# Session 111 計画

**目的**: 屋外検査の残作業整理（TTFF、MON-RF等の調査）

---

## 背景

Session 110でNAV-SIGパーサー + signal_statsを実装完了。
屋外検査の残作業が複数あり、優先度・仕様を整理する必要がある。

ユーザーの関心：
- TTFF測定（重要度高そう）
- MON-RF（よくわからないが重要そう）
- 全体の進捗把握

---

## やること

### 1. TTFF測定の調査

- TTFFとは何か（Cold/Warm/Hot Start）
- 測定手順の確認（NAV-STATUSから取得可能？）
- 合格基準の確認（業界標準は？）
- 実装難易度の見積もり

### 2. MON-RFの調査

- MON-RFメッセージの仕様確認
- ジャミング検出の仕組み
- 屋外検査での用途
- 実装難易度の見積もり

### 3. 残作業の優先度整理

現在把握している残作業：
- NAV-SIG → APIエンドポイント → FE連携
- MON-SPANパーサー（RFスペクトラム）
- NAV-SATパーサー（スカイプロット）
- RTK FIX時間測定
- TTFF測定
- MON-RF（ジャミング）
- 複数台同時確認

→ 優先度を再整理

### 4. （任意）進捗管理方法の決定

- CLAUDE.mdに軽量な進捗セクションを追加するか
- 現状維持（セッション履歴で管理）か

---

## 参照資料

- [outdoor-inspection-needs.md](../session106/outdoor-inspection-needs.md) — 要求整理
- [ADR-008](../../docs/adr/m1/ADR-008-outdoor-inspection-criteria.md) — 屋外検査の合格基準
- [ubx-spec-memo.md](../session106/ubx-spec-memo.md) — UBXメッセージ仕様

---

*計画作成: 2026-03-11 Session 110終了時*
