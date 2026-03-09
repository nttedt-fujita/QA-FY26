# Session 57 サマリー

**日時**: 2026-03-09
**目的**: GNSS評価ツール プロトタイプ作成準備

---

## 実施内容

### 1. PX4ソースコード調査

PX4-GPSDriversのソースコードをGitHub経由で調査し、以下を確認：

| 確認項目 | 結果 |
|----------|------|
| UBX-NAV-SIG対応 | **未実装** |
| L1/L2別C/N0のULog取得 | **不可** |
| TTFF（UBX-NAV-STATUS.ttff） | **未実装**（構造体にはあるが抽出していない） |

**結論**: L1/L2別C/N0、TTFF、スペアナデータはフライトログから取得できない。F9Pとの直接UBX通信が必須。

### 2. ADR-004作成

方針決定を記録：
- **採用**: 直接UBX通信ツール
- **却下**: PX4ドライバ改造（将来の選択肢として保留）
- **理由**: 地上での設計検証・受入検査が目的のため、PX4連携は不要

### 3. エビデンスドキュメント作成

URL + 原文抜粋を含む調査エビデンスを作成：
- PX4-GPSDrivers ubx.h / ubx.cpp の関連コード
- PX4 SatelliteInfo.msg / SensorGps.msg の構造

### 4. リポジトリ整理

- `tools/gnss-eval/` ディレクトリを作成
- READMEに技術選定候補・取得予定データを記載
- tools/README.mdを更新

---

## 作成ファイル

| ファイル | 内容 |
|----------|------|
| [prototype/docs/adr/ADR-004-gnss-tool-approach.md](../../prototype/docs/adr/ADR-004-gnss-tool-approach.md) | 方針決定ADR |
| [docs/missions/m1-sensor-evaluation/gnss/12-px4-source-evidence.md](../../docs/missions/m1-sensor-evaluation/gnss/12-px4-source-evidence.md) | PX4調査エビデンス |
| [tools/gnss-eval/README.md](../../tools/gnss-eval/README.md) | GNSS評価ツール設計メモ |

---

## 更新ファイル

| ファイル | 内容 |
|----------|------|
| [docs/missions/m1-sensor-evaluation/gnss/11-px4-uorb-mapping.md](../../docs/missions/m1-sensor-evaluation/gnss/11-px4-uorb-mapping.md) | 調査結果反映、TTFF追加 |
| [docs/missions/m1-sensor-evaluation/gnss/README.md](../../docs/missions/m1-sensor-evaluation/gnss/README.md) | ファイル一覧更新 |
| [tools/README.md](../../tools/README.md) | gnss-eval追加 |
| [CLAUDE.md](../../CLAUDE.md) | ADR-004追加 |

---

## 重要な発見

1. **PX4はUBX-NAV-SIGを処理していない** — L1/L2区別なし
2. **TTFFもuORB化されていない** — 構造体にはあるが抽出していない
3. **PX4改造は可能だが後回し** — 地上評価ツールを先に作る方針

---

## 次セッションでやること

→ [session58/session-plan.md](../session58/session-plan.md)

- 技術選定（Python + pyserial が有力）
- 環境構築
- プロトタイプ設計・実装開始
