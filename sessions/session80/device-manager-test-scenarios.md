# DeviceManager テストシナリオリスト（TDD Phase 2）

**作成日**: 2026-03-10 Session 80
**ステータス**: 承認待ち（次セッションで承認後Phase 3へ）

---

## テスト分類

| 分類 | 対象 | モック |
|------|------|--------|
| **A. 状態遷移テスト** | DeviceStatus | 不要（純粋ロジック） |
| **B. フィルタリングテスト** | ポートフィルタ | 不要（純粋ロジック） |
| **C. DeviceManagerテスト** | list_devices, connect等 | SerialPortProvider |

---

## A. 状態遷移テスト（単体・モック不要）

| ID | シナリオ | 入力 | 期待出力 |
|----|---------|------|---------|
| A1 | 初期状態はDetected | 新規Device作成 | Detected |
| A2 | Detected → Connecting | connect開始 | Connecting |
| A3 | Connecting → Connected | connect成功 | Connected |
| A4 | Connecting → Error | connect失敗 | Error("接続失敗") |
| A5 | Connected → Inspecting | 検査開始 | Inspecting |
| A6 | Inspecting → Connected | 検査完了 | Connected |
| A7 | Inspecting → Error | 検査中エラー | Error("通信エラー") |
| A8 | Connected → Disconnected | disconnect呼び出し | Disconnected |
| A9 | Connected → Disconnected | USBケーブル抜去 | Disconnected |
| A10 | Disconnected → Connecting | 再接続開始 | Connecting |
| A11 | 不正な遷移は拒否される | Detected → Inspecting | エラー（IllegalTransition） |

---

## B. フィルタリングテスト（単体・モック不要）

| ID | シナリオ | 入力 | 期待出力 |
|----|---------|------|---------|
| B1 | F9Pはフィルタを通過 | VID=0x1546, PID=0x01A9 | true |
| B2 | VIDが異なればフィルタで除外 | VID=0x1234, PID=0x01A9 | false |
| B3 | PIDが異なればフィルタで除外 | VID=0x1546, PID=0x0000 | false |
| B4 | VID/PIDがNoneならフィルタで除外 | VID=None, PID=None | false |
| B5 | 複数ポートからF9Pのみ抽出 | [F9P, 他社, F9P] | [F9P, F9P] |

---

## C. DeviceManagerテスト（単体・モック使用）

### C1. ポート列挙

| ID | シナリオ | モックの振る舞い | 期待出力 |
|----|---------|----------------|---------|
| C1-1 | ポートがない場合 | available_ports → [] | 空リスト |
| C1-2 | F9Pが1台接続 | available_ports → [F9P] | [Device{Detected}] |
| C1-3 | F9Pが複数台接続 | available_ports → [F9P, F9P] | [Device, Device] |
| C1-4 | F9P以外は除外される | available_ports → [F9P, 他社] | [Device] |

### C2. 接続管理

| ID | シナリオ | モックの振る舞い | 期待出力 |
|----|---------|----------------|---------|
| C2-1 | 正常に接続できる | open → Ok(SerialPort) | Ok(DeviceHandle) |
| C2-2 | 存在しないポート | open → Err(NotFound) | Err(PortNotFound) |
| C2-3 | ポートが使用中 | open → Err(Busy) | Err(PortBusy) |
| C2-4 | 接続後に状態がConnectedになる | open → Ok | status == Connected |
| C2-5 | 切断できる | - | Ok(()) |
| C2-6 | 切断後に状態がDisconnectedになる | - | status == Disconnected |
| C2-7 | 切断後に再接続できる | open → Ok | Ok(DeviceHandle) |
| C2-8 | Phase 1: 2台目接続はエラー | 既に1台接続中 | Err(MaxDevicesReached) |

### C3. UBX送受信

| ID | シナリオ | モックの振る舞い | 期待出力 |
|----|---------|----------------|---------|
| C3-1 | UBXメッセージ送信成功 | write → Ok | Ok(()) |
| C3-2 | UBXメッセージ受信成功 | read → Ok(data) | Ok(UbxMessage) |
| C3-3 | 未接続で送信 | - | Err(NotConnected) |
| C3-4 | タイムアウト | read → timeout | Err(Timeout) |

### C4. 応答確認

| ID | シナリオ | モックの振る舞い | 期待出力 |
|----|---------|----------------|---------|
| C4-1 | 1秒以内に応答あり | read → Ok (500ms) | Ok(()) |
| C4-2 | 1秒超過でタイムアウト | read → timeout | Err(Timeout) |

---

## テスト優先度

| 優先度 | 対象 | 理由 |
|--------|------|------|
| **P1** | A1-A11（状態遷移） | ビジネスルールの不変条件 |
| **P1** | B1-B5（フィルタ） | 装置検出の正確性 |
| **P2** | C1-1〜C1-4（ポート列挙） | 基本機能 |
| **P2** | C2-1〜C2-8（接続管理） | 基本機能 |
| **P3** | C3, C4（UBX送受信） | InspectionEngineでも検証可能 |

---

## 振る舞い記述との対応

| 振る舞いID | テストシナリオ | カバー |
|-----------|--------------|--------|
| L1-L3 | B1-B5, C1-1〜C1-4 | ✅ |
| C1-C7 | C2-1〜C2-8 | ✅ |
| S1-S7 | A1-A11 | ✅ |
| U1-U4 | C3-1〜C3-4 | ✅ |
| W1-W3 | （ポーリングは統合テスト） | ⚠️ 単体テスト対象外 |
| E1-E3 | A9, C4-2 | ✅ |

---

## 次セッションでやること

1. このテストシナリオリストの承認
2. Phase 3: テストコード作成
3. Phase 4: 実装（Red → Green）

---

*作成: 2026-03-10 Session 80*
