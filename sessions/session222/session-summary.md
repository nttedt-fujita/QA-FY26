# Session 222 サマリー

**日付**: 2026-03-17
**目的**: Flash/BBR仕様調査 + 実機確認

---

## 実施内容

### 1. 仕様調査（公式ドキュメントから事実確認）

**Interface Description p.224**（出典確認済み）:

| レイヤー | 仕様 |
|---------|------|
| RAM | volatile RAM。電源断で消える |
| BBR | "preserved as long as a battery backup supply is provided during off periods" |
| Flash | "only available if there is a usable external flash memory" |

**Integration Manual p.88**（出典確認済み）:
> "If V_BCKP is not provided, the module performs a cold start at power up."

**Integration Manual p.104**:
> "For achieving a minimal TTFF after a power down, make sure to connect a backup battery to V_BCKP."

### 2. Holybro H-RTK F9P仕様確認

- [公式仕様ページ](https://docs.holybro.com/gps-and-rtk-system/zed-f9p-h-rtk-series/specification): V_BCKP、バックアップバッテリー、外部Flashの**記載なし**
- 回路図は公開されていない（3D CADモデルのみ）

### 3. 実機テスト

| 操作 | 結果 |
|------|------|
| set-periodic-output-flash（layers=0x04） | **ACK** |
| USB抜き差し後 message-scan | **0件** |

**観察**: Flashレイヤーへの保存はACKを返すが、USB抜き差し後に設定は維持されない。

---

## 結論（仮説）

| 項目 | 状態 | 根拠 |
|------|------|------|
| V_BCKPバッテリー | **なし** | USB抜き差し後にBBRが消える（Session 221） |
| 外部Flash | **不明** | ACKは返るが設定は維持されない。CFG-VALGETで確認が必要 |

---

## 残課題

CFG-VALGET APIを作成し、Flashレイヤーから値を読み取れるか確認する。
- 読み取れる → Flash搭載だが書き込みに問題あり
- 読み取れない → Flash非搭載

---

## 作成・変更ファイル

| ファイル | 内容 |
|----------|------|
| set_periodic_output_api.rs | layerクエリパラメータ追加 |
| api.mk | set-periodic-output-flashコマンド追加 |

---

## 使用したコマンド

| 用途 | コマンド |
|------|----------|
| 装置接続 | make connect |
| Flash保存テスト | make set-periodic-output-flash |
| メッセージスキャン | make message-scan |

---

## 次セッション

[session223/session-plan.md](../session223/session-plan.md)
