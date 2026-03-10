# DeviceManager 振る舞い記述（TDD Phase 1）

**作成日**: 2026-03-10 Session 80
**ステータス**: 承認済み

---

## 設計判断

| 項目 | 決定 | 理由 |
|------|------|------|
| トレイト | `SerialPortProvider` で外部依存を抽象化 | 単体テストでモック差し替え可能 |
| ホットプラグ | 1Hz（1秒間隔）ポーリング | シンプルさ優先 |
| Inspecting状態 | DeviceManagerが管理 | 振る舞い視点で「装置の状態」に含まれるのが自然 |
| 複数装置管理 | Phase 1は1台制限、データ構造は複数対応で準備 | 段階的拡張 |

---

## 振る舞い記述

### 1. ポート列挙

| ID | 振る舞い | 入力 | 期待される出力 |
|----|----------|------|---------------|
| L1 | USBシリアルポートの一覧を取得できる | なし | ポート情報のリスト（パス、VID、PID、シリアル番号） |
| L2 | ポートがない場合は空リストを返す | なし | 空のリスト |
| L3 | u-blox F9P装置だけをフィルタできる | なし | VID=0x1546, PID=0x01A9 のポートのみ |

### 2. 接続管理

| ID | 振る舞い | 入力 | 期待される出力 |
|----|----------|------|---------------|
| C1 | 指定したポートに接続できる | ポートパス, ボーレート（デフォルト115200） | 接続成功（DeviceHandle） |
| C2 | 存在しないポートに接続するとエラー | 無効なパス | エラー（PortNotFound） |
| C3 | 既に開かれているポートに接続するとエラー | 使用中のパス | エラー（PortBusy） |
| C4 | 接続中の装置を切断できる | DeviceHandle | 切断成功 |
| C5 | 切断後に再接続できる | ポートパス | 接続成功 |
| C6 | 接続後に応答確認ができる | DeviceHandle | 1秒以内に応答あり → 成功 |
| C7 | Phase 1では1台のみ接続可能 | 2台目の接続試行 | エラー（MaxDevicesReached） |

### 3. 状態管理

| ID | 振る舞い | 入力 | 期待される出力 |
|----|----------|------|---------------|
| S1 | 検出後の状態はDetectedになる | ポート列挙後 | DeviceStatus::Detected |
| S2 | 接続試行中の状態はConnectingになる | 接続開始 | DeviceStatus::Connecting |
| S3 | 接続後の状態はConnectedになる | 接続成功後 | DeviceStatus::Connected |
| S4 | 検査中の状態はInspectingになる | 検査開始 | DeviceStatus::Inspecting |
| S5 | 切断後の状態はDisconnectedになる | 切断後 | DeviceStatus::Disconnected |
| S6 | 通信エラー時の状態はErrorになる | 通信エラー発生 | DeviceStatus::Error(reason) |
| S7 | 現在の接続状態を取得できる | DeviceHandle | DeviceStatus |

### 4. UBXメッセージ送受信

| ID | 振る舞い | 入力 | 期待される出力 |
|----|----------|------|---------------|
| U1 | UBXメッセージを送信できる | DeviceHandle, UBXメッセージ | 送信成功 |
| U2 | UBXメッセージを受信できる | DeviceHandle | 受信データ |
| U3 | 接続していない状態で送信するとエラー | UBXメッセージ | エラー（NotConnected） |
| U4 | タイムアウトした場合はエラー | DeviceHandle | エラー（Timeout） |

### 5. ポート監視（ポーリング）

| ID | 振る舞い | 入力 | 期待される出力 |
|----|----------|------|---------------|
| W1 | 1秒間隔でポート一覧を更新できる | なし | 更新されたポート情報 |
| W2 | 新しいポートが検出されたら通知 | USBケーブル挿入 | 新しいポート情報 |
| W3 | ポートが消えたら通知 | USBケーブル抜去 | 消えたポート情報 |

### 6. 境界値・異常系

| ID | 振る舞い | 入力 | 期待される出力 |
|----|----------|------|---------------|
| E1 | USBケーブルが抜かれた場合 | 通信中に物理切断 | DeviceStatus::Disconnected |
| E2 | ボーレート不一致の場合 | 接続時 | 通信エラー |
| E3 | 応答タイムアウト（1秒超過） | 応答確認 | エラー（Timeout） |

---

## テスト戦略（古典派TDD）

### 単体テスト対象（モック不要）

| 対象 | 理由 |
|------|------|
| DeviceStatus 状態遷移ロジック | 純粋なロジック |
| ポートフィルタリング（VID/PID） | 純粋なロジック |
| UBXメッセージのフレーミング | 純粋なロジック |

### 単体テスト対象（モック使用）

| 対象 | モック対象 | 理由 |
|------|-----------|------|
| DeviceManager.list_devices() | SerialPortProvider | 外部依存（serialportクレート）の分離 |
| DeviceManager.connect() | SerialPortProvider | 外部依存の分離 |

### 統合テスト対象（実機必要）

| 対象 | 理由 |
|------|------|
| 実際のポート列挙 | 実機でしか検証できない |
| 実際のUBX送受信 | 実機でしか検証できない |

---

## 参照

- [16-gnss-tool-architecture.md](../../docs/missions/m1-sensor-evaluation/gnss/16-gnss-tool-architecture.md) — アーキテクチャ設計
- [15-gnss-tool-requirements.md](../../docs/missions/m1-sensor-evaluation/gnss/15-gnss-tool-requirements.md) — 要件定義

---

*作成: 2026-03-10 Session 80*
