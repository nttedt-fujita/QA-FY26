# M1-B GNSS評価 — 小峰無線GPS確認データ分析

**元データ**: 小峰無線GPS確認.xlsx（23MB）+ u-blox F9 HPG 1.32 Interface Description PDF
**評価対象機体**: Ref（RKP001）、No.1、No.2、No.3、No.5 + シルクなし機体（0922, 0909, 0932, 0952, 0754）
**分析期間**: Session 17（テキストデータ）+ Session 18（画像データ）

---

## このディレクトリの使い方

### ○○をするときは△△を見る（チェックリスト）

| やること | まず見るもの |
|----------|-------------|
| **屋外検査を使う** | [31-outdoor-inspection-user-guide.md](31-outdoor-inspection-user-guide.md) |
| UBXメッセージを実装する | [08-ubx-protocol-index.md](08-ubx-protocol-index.md) |
| NAV-*を実装する | [ubx-nav-messages.md](ubx-nav-messages.md) |
| MON-*を実装する | [ubx-mon-messages.md](ubx-mon-messages.md)、[23-mon-span-implementation.md](23-mon-span-implementation.md) |
| 合格基準を確認する | [ADR-008](../../../adr/m1/ADR-008-outdoor-inspection-criteria.md) |
| 屋外検査を修正する | [26-outdoor-inspection-domain-model.md](26-outdoor-inspection-domain-model.md) |
| RTK/NTRIPを実装する | [20-ntrip-rtk-implementation.md](20-ntrip-rtk-implementation.md)〜[22-rtk-configuration.md](22-rtk-configuration.md) |
| 集計ロジックを修正する | [ADR-009](../../../adr/m1/ADR-009-outdoor-inspection-fe-aggregation.md) |
| アーキテクチャを確認する | [29-architecture-nextjs-rust.md](29-architecture-nextjs-rust.md) |
| TTFFを実装する | [30-ttff-monrf-spec.md](30-ttff-monrf-spec.md) |

---

## 現在のPhase

**Phase 3: ツール実装**（屋外検査機能）

```
Phase 0: 現状把握        ← 完了（Session 15-16）
Phase 1: 検証項目の妥当性検証  ← 完了（Session 17-56）
Phase 2: ツール設計        ← 完了（Session 57-105）
Phase 3: ツール実装        ← 現在（Session 106〜）
Phase 4: 標準化
```

---

## 重要: 評価の文脈

**IMPORTANT**: この評価は**受入検査ではない**。

| 項目 | 内容 |
|------|------|
| **対象製品** | Holybro H-RTK F9P Ultralightの国内機能コピー製品 |
| **フェーズ** | **原理試作に対する設計検証**（3回目の試作） |
| **目的** | 検証項目が適切かどうかの検証 |
| **現状の合格基準** | **曖昧**（明確な基準がない）→ [ADR-008](../../../adr/m1/ADR-008-outdoor-inspection-criteria.md)で一部定義 |
| **評価手順書** | **ない** |

### 小板橋さんからの重要コメント（Session 16）

> **ツール作成の前に、検証項目が適切かどうかの検証をして、その部分を固めるのが重要**
> これを、**今年中にやるのが望ましい**

> 今は原理試作に対しての設計検証をしているが、**受入検査でも何をするかを整理しておいたほうが良い**かも

詳細: [sessions/session16/gnss-hearing-koitabashi-01.md](../../../../sessions/session16/gnss-hearing-koitabashi-01.md)

---

## ドキュメント一覧

### 基礎調査（01-06）

| # | ファイル | 内容 | 作成Session |
|---|---------|------|------------|
| 01 | [01-internal-settings.md](01-internal-settings.md) | 内部設定（FW/PROTVER/パラメータ） | 17 |
| 02 | [02-reception-sensitivity.md](02-reception-sensitivity.md) | 受信感度（C/N0）全3回測定データ | 17 |
| 03 | [03-spectrum-analyze.md](03-spectrum-analyze.md) | スペクトラムアナライザ波形分析 | 17 |
| 04 | [04-flight-test.md](04-flight-test.md) | 飛行試験データ + ログ時系列分析 | 17 |
| 05 | [05-mag-check.md](05-mag-check.md) | 磁気コンパスずれ確認 | 17 |
| 06 | [06-battery-check.md](06-battery-check.md) | バックアップ電池確認 | 17 |

### 分析・設計（07-19）

| # | ファイル | 内容 | 作成Session |
|---|---------|------|------------|
| 07 | [07-cross-sheet-findings.md](07-cross-sheet-findings.md) | 全シート横断の発見事項・合格基準検討 | 17 |
| 08 | [08-ubx-protocol-index.md](08-ubx-protocol-index.md) | UBXプロトコル仕様書の索引 | 18 |
| 09 | [09-design-verification-criteria.md](09-design-verification-criteria.md) | 設計検証基準の業界標準調査 | 53 |
| 10 | [10-tool-design-notes.md](10-tool-design-notes.md) | ツール設計メモ | 56 |
| 11 | [11-px4-uorb-mapping.md](11-px4-uorb-mapping.md) | PX4 uORBとUBXメッセージの対応 | 57 |
| 12 | [12-px4-source-evidence.md](12-px4-source-evidence.md) | PX4ソースコード調査エビデンス | 57 |
| 13 | [13-ubx-parser-test-scenarios.md](13-ubx-parser-test-scenarios.md) | UBXパーサー振る舞い・テストシナリオ | 72 |
| 14 | [14-gnss-tool-needs.md](14-gnss-tool-needs.md) | GNSSツール要求（What） | 71 |
| 15 | [15-gnss-tool-requirements.md](15-gnss-tool-requirements.md) | GNSSツール要件（How） | 71 |
| 16 | [16-gnss-tool-architecture.md](16-gnss-tool-architecture.md) | GNSSツールアーキテクチャ | 71 |
| 17 | [17-gnss-tool-implementation-plan.md](17-gnss-tool-implementation-plan.md) | GNSSツール実装計画 | 71 |
| 18 | [18-cfg-parser-design-decisions.md](18-cfg-parser-design-decisions.md) | CFGパーサー設計判断 | 75 |
| 19 | [19-gnss-unified-domain-model.md](19-gnss-unified-domain-model.md) | GNSS統合ドメインモデル | 109 |

### RTK/NTRIP（20-22）

| # | ファイル | 内容 | 作成Session |
|---|---------|------|------------|
| 20 | [20-ntrip-rtk-implementation.md](20-ntrip-rtk-implementation.md) | NTRIP/RTK実装方針 | 114 |
| 21 | [21-ntrip-protocol-spec.md](21-ntrip-protocol-spec.md) | NTRIPプロトコル仕様抽出 | 114 |
| 22 | [22-rtk-configuration.md](22-rtk-configuration.md) | ZED-F9P RTK設定 | 114 |

### 実装仕様（23-30）

| # | ファイル | 内容 | 作成Session |
|---|---------|------|------------|
| 23 | [23-mon-span-implementation.md](23-mon-span-implementation.md) | MON-SPAN実装仕様 | 117 |
| 24 | [24-ubx-spec-extract.md](24-ubx-spec-extract.md) | PDF抽出：UBXプロトコル仕様 | 107 |
| 25 | [25-nav-sig-behavior-spec.md](25-nav-sig-behavior-spec.md) | NAV-SIGパーサー振る舞い仕様 | 107/109 |
| 26 | [26-outdoor-inspection-domain-model.md](26-outdoor-inspection-domain-model.md) | 屋外検査ドメインモデル設計 | 123 |
| 27 | [27-outdoor-inspection-implementation-plan.md](27-outdoor-inspection-implementation-plan.md) | 屋外検査実装計画 | 123 |
| 28 | [28-tdd-review-result.md](28-tdd-review-result.md) | TDDレビュー結果 | 122 |
| 29 | [29-architecture-nextjs-rust.md](29-architecture-nextjs-rust.md) | Next.js/Rustアーキテクチャ解説 | 122 |
| 30 | [30-ttff-monrf-spec.md](30-ttff-monrf-spec.md) | TTFF・MON-RF仕様調査 | 111 |
| 31 | [31-outdoor-inspection-user-guide.md](31-outdoor-inspection-user-guide.md) | 屋外検査ユーザーガイド | 131 |

### PDF仕様抽出・調査（33-38）

| # | ファイル | 内容 | 作成Session |
|---|---------|------|------------|
| 33 | [33-toc-ublox-f9-interface-description.md](33-toc-ublox-f9-interface-description.md) | u-blox F9 HPG 1.32 Interface Description 目次 | 171 |
| 34 | [34-ubx-mon-comms.md](34-ubx-mon-comms.md) | UBX-MON-COMMS仕様（p.126-130） | 156 |
| 35 | [35-ubx-uart-config.md](35-ubx-uart-config.md) | CFG-UART1/2仕様（p.270-274） | 157 |
| 36 | [36-ntrip-rtk-findings.md](36-ntrip-rtk-findings.md) | NTRIP/RTK仕様調査まとめ | 156 |
| 37 | [37-mon-span-display-spec.md](37-mon-span-display-spec.md) | MON-SPAN表示仕様（dB変換・周波数計算） | 190 |
| 38 | [38-ublox-config-management.md](38-ublox-config-management.md) | u-blox設定管理の仕組み（CFG-CFG/VALGET/VALSET） | 211 |

### 図表

| ファイル | 内容 |
|---------|------|
| [gnss-er-diagram.drawio](gnss-er-diagram.drawio) | ER図（実装完了後に更新予定） |

### 補足ドキュメント（番号なし）

| ファイル | 内容 |
|---------|------|
| [api-spec.md](api-spec.md) | API仕様 |
| [ubx-nav-messages.md](ubx-nav-messages.md) | UBX NAVメッセージ仕様（NAV-SAT, NAV-SIG等） |
| [ubx-mon-messages.md](ubx-mon-messages.md) | UBX MONメッセージ仕様（MON-SPAN等） |
| [ubx-signal-identifiers.md](ubx-signal-identifiers.md) | UBX信号識別子 |
| [sources/](sources/) | 1次情報PDF（NTRIP仕様書等） |

---

## 作成経緯

- Session 17: Excelの全シートテキストデータ + PDFの主要メッセージ定義を分析
- Session 18: Excel内の画像（スクショ）を読み取り、テキストでは得られなかった波形・グラフ・GCS画面を補完
- Session 106〜: ツール実装（屋外検査機能）
- Session 127: ドキュメント整理・番号振り直し
- Session 171: PDF仕様抽出ドキュメント整理（33-36）
