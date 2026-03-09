# GNSS評価ツール

**ステータス**: 設計中
**関連ミッション**: M1-B（GNSS設計検証）

---

## 概要

F9P GNSSモジュールと直接UBX通信し、設計検証・受入検査に必要なデータを取得するツール。

---

## 背景

PX4フライトログ（ULog）からは以下のデータが取得**できない**ことが判明した（Session 57調査）：

| 項目 | 理由 |
|------|------|
| L1/L2別C/N0 | PX4がUBX-NAV-SIGを処理していない |
| TTFF（初期FIX時間） | PX4がttffを抽出していない |
| スペアナデータ | UBX-MON-SPANがuORB化されていない |

詳細: [ADR-004](../../prototype/docs/adr/ADR-004-gnss-tool-approach.md)

---

## 取得予定データ

| UBXメッセージ | 取得データ | 用途 |
|--------------|-----------|------|
| UBX-NAV-SIG | L1/L2別C/N0 | 受信感度評価 |
| UBX-NAV-STATUS | ttff | 初期FIX時間計測 |
| UBX-NAV-PVT | RTK状態、精度、衛星数 | 基本性能 |
| UBX-MON-SPAN | スペクトラムデータ | 干渉検出 |
| UBX-MON-RF | ジャミング状態 | 干渉検出 |
| UBX-MON-VER | FW/PROTVER | 内部設定確認 |

---

## 技術選定（未決定）

| 項目 | 候補 | 備考 |
|------|------|------|
| 言語 | Python / C++ / Rust | Pythonが最有力（開発速度） |
| シリアル通信 | pyserial / serialport | — |
| 出力形式 | CSV | 既存Excelワークフローとの互換 |
| 対応OS | Linux / Windows | 両対応が望ましい |

---

## 参照資料

- [08-ubx-protocol-index.md](../../docs/missions/m1-sensor-evaluation/gnss/08-ubx-protocol-index.md) — UBXプロトコル仕様索引
- [10-tool-design-notes.md](../../docs/missions/m1-sensor-evaluation/gnss/10-tool-design-notes.md) — ツール設計メモ
- [11-px4-uorb-mapping.md](../../docs/missions/m1-sensor-evaluation/gnss/11-px4-uorb-mapping.md) — PX4調査結果
- [12-px4-source-evidence.md](../../docs/missions/m1-sensor-evaluation/gnss/12-px4-source-evidence.md) — 調査エビデンス
