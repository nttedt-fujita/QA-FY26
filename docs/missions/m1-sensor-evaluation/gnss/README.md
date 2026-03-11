# M1-B GNSS評価 — 小峰無線GPS確認データ分析

**元データ**: 小峰無線GPS確認.xlsx（23MB）+ u-blox F9 HPG 1.32 Interface Description PDF
**評価対象機体**: Ref（RKP001）、No.1、No.2、No.3、No.5 + シルクなし機体（0922, 0909, 0932, 0952, 0754）
**分析期間**: Session 17（テキストデータ）+ Session 18（画像データ）

---

## 重要: 評価の文脈

**IMPORTANT**: この評価は**受入検査ではない**。

| 項目 | 内容 |
|------|------|
| **対象製品** | Holybro H-RTK F9P Ultralightの国内機能コピー製品 |
| **フェーズ** | **原理試作に対する設計検証**（3回目の試作） |
| **目的** | 検証項目が適切かどうかの検証 |
| **現状の合格基準** | **曖昧**（明確な基準がない） |
| **評価手順書** | **ない** |

### 小板橋さんからの重要コメント（Session 16）

> **ツール作成の前に、検証項目が適切かどうかの検証をして、その部分を固めるのが重要**
> これを、**今年中にやるのが望ましい**

> 今は原理試作に対しての設計検証をしているが、**受入検査でも何をするかを整理しておいたほうが良い**かも

詳細: [sessions/session16/gnss-hearing-koitabashi-01.md](../../../../sessions/session16/gnss-hearing-koitabashi-01.md)

---

## ドキュメント一覧

| ファイル | 内容 | 元シート |
|---------|------|---------|
| [01-internal-settings.md](01-internal-settings.md) | 内部設定（FW/PROTVER/パラメータ） | 内部設定 |
| [02-reception-sensitivity.md](02-reception-sensitivity.md) | 受信感度（C/N0）全3回測定データ + u-center画像分析 | 受信感度 / 受信感度_仰角 |
| [03-spectrum-analyze.md](03-spectrum-analyze.md) | スペクトラムアナライザ波形分析 | Spectrum Analyze |
| [04-flight-test.md](04-flight-test.md) | 飛行試験データ + 飛行中ログ時系列分析 | 20260218 |
| [05-mag-check.md](05-mag-check.md) | 磁気コンパスずれ確認 | Mag確認 |
| [06-battery-check.md](06-battery-check.md) | バックアップ電池確認 | 電池確認 |
| [07-cross-sheet-findings.md](07-cross-sheet-findings.md) | 全シート横断の発見事項・合格基準検討・Phase 1の問い | — |
| [08-ubx-protocol-index.md](08-ubx-protocol-index.md) | UBXプロトコル仕様書の索引（Phase 2用） | PDF |
| [09-design-verification-criteria.md](09-design-verification-criteria.md) | 設計検証基準の業界標準調査（Session 53） | — |
| [10-tool-design-notes.md](10-tool-design-notes.md) | ツール設計メモ（Session 56） | Session 16-17統合 |
| [11-px4-uorb-mapping.md](11-px4-uorb-mapping.md) | PX4 uORBとUBXメッセージの対応（Session 57更新） | 調査完了 |
| [12-px4-source-evidence.md](12-px4-source-evidence.md) | PX4ソースコード調査エビデンス（Session 57） | URL+原文抜粋 |
| [13-ubx-parser-test-scenarios.md](13-ubx-parser-test-scenarios.md) | UBXパーサー振る舞い・テストシナリオ（Session 72） | TDD実装 |
| [14-gnss-tool-needs.md](14-gnss-tool-needs.md) | GNSSツール要求（What） | Session 71 |
| [15-gnss-tool-requirements.md](15-gnss-tool-requirements.md) | GNSSツール要件（How） | Session 71 |
| [16-gnss-tool-architecture.md](16-gnss-tool-architecture.md) | GNSSツールアーキテクチャ | Session 71 |
| [17-gnss-tool-implementation-plan.md](17-gnss-tool-implementation-plan.md) | GNSSツール実装計画 | Session 71 |
| [18-cfg-parser-design-decisions.md](18-cfg-parser-design-decisions.md) | CFGパーサー設計判断 | Session 75 |
| [19-gnss-unified-domain-model.md](19-gnss-unified-domain-model.md) | GNSS統合ドメインモデル | Session 109 |
| [20-ntrip-rtk-implementation.md](20-ntrip-rtk-implementation.md) | NTRIP/RTK実装方針 | Session 114 |
| [21-ntrip-protocol-spec.md](21-ntrip-protocol-spec.md) | NTRIPプロトコル仕様抽出 | Session 114 |
| [22-rtk-configuration.md](22-rtk-configuration.md) | ZED-F9P RTK設定 | Session 114 |

### 補足ドキュメント

| ファイル | 内容 |
|---------|------|
| [api-spec.md](api-spec.md) | API仕様 |
| [ubx-nav-messages.md](ubx-nav-messages.md) | UBX NAVメッセージ一覧 |
| [ubx-mon-messages.md](ubx-mon-messages.md) | UBX MONメッセージ一覧 |
| [ubx-signal-identifiers.md](ubx-signal-identifiers.md) | UBX信号識別子 |
| [sources/](sources/) | 1次情報PDF（NTRIP仕様書等） |

---

## Phase 1ロードマップにおける位置づけ

```
Phase 0: 現状把握        ← 完了（Session 15-16）
Phase 1: 検証項目の妥当性検証  ← 現在（Session 17-18でデータ読み込み完了）
Phase 2: ツール設計
Phase 3: ツール実装
Phase 4: 標準化
```

Phase 1の次のステップ: 合格基準の検討 → 末永さんヒアリング

---

## 作成経緯

- Session 17: Excelの全シートテキストデータ + PDFの主要メッセージ定義を分析
- Session 18: Excel内の画像（スクショ）を読み取り、テキストでは得られなかった波形・グラフ・GCS画面を補完
- 本ドキュメント群: 上記2セッションの分析結果を統合し、シートごとに整理
