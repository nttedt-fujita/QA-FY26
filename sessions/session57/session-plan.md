# Session 57 計画

**日時**: 2026-03-XX（予定）
**前提**: Session 56でGNSS資料整理完了、方針転換：先にプロトタイプを作る

---

## 方針転換（Session 56終了時）

> 確認待ちではなく、概ね方向性は見えているので**先にプロトタイプを作った方が早い**
> 聞いた方がいいこともあるが、作りながら確認していく

---

## 目的

GNSS評価ツールのプロトタイプ作成準備

---

## タスク

### 1. PX4ソースコード調査（優先）

**目的**: ツール作成前の事前調査

**確認内容**:
- `src/drivers/gps/devices/ubx/` でUBX→uORB変換を確認
- L1/L2別C/N0がuORB化されているか確認
- どのデータがフライトログから取得可能か明確化

**参照**: [docs/missions/m1-sensor-evaluation/gnss/11-px4-uorb-mapping.md](../../docs/missions/m1-sensor-evaluation/gnss/11-px4-uorb-mapping.md)

### 2. リポジトリ整理

**背景**: 現在の `prototype/` はM3用。GNSS評価ツールと区別が必要

**検討事項**:
- ディレクトリ命名規則（例: `prototype-m3/`, `tools/gnss-eval/` 等）
- 混乱を防ぐための整理

### 3. 技術選定・環境構築

**検討事項**:
- 言語: C++? Python? Rust?
- Linux/Windows両対応の方針
- シリアル通信ライブラリの選定
- 開発環境のセットアップ

### 4. プロトタイプ設計

**最小構成で作る**:
- F9Pとの直接UBX通信
- 受信感度（C/N0）の自動取得
- CSV出力

---

## 残タスク（並行 or 後回し）

- 小板橋さんへ確認（作りながら確認）
- 末永さんへ相談（作りながら確認）
- M4工程不良Excel入手

---

## 参照資料

- [Session 56サマリー](../session56/session-summary.md)
- [docs/missions/m1-sensor-evaluation/gnss/10-tool-design-notes.md](../../docs/missions/m1-sensor-evaluation/gnss/10-tool-design-notes.md) — ツール設計メモ
- [docs/missions/m1-sensor-evaluation/gnss/11-px4-uorb-mapping.md](../../docs/missions/m1-sensor-evaluation/gnss/11-px4-uorb-mapping.md) — PX4 uORBマッピング
