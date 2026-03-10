# Session 80 計画

**目的**: DeviceManager実装（Phase 1 Step 2）

---

## 背景

Session 79でPhase 1 Step 1（UBXパーサー）が完了（7/7メッセージ実装済み）。
Step 2のDeviceManager実装に進む。

---

## やること

### 1. DeviceManager設計

**対象機能**:

| タスク | 詳細 |
|--------|------|
| ポート列挙 | USBシリアルポート一覧取得 |
| 接続管理 | 接続/切断処理 |
| 状態管理 | DeviceStatus管理（Connected/Disconnected/Error） |

**依存クレート**: `serialport`（既にCargo.tomlに追加済み）

### 2. TDD実装

1. **Phase 1**: 振る舞い記述
2. **Phase 2**: テストシナリオリスト
3. **Phase 3**: テストコード作成
4. **Phase 4**: 実装
5. **Phase 5**: リファクタリング

### 3. 実機テスト準備（任意）

DevContainerでのUSBデバイスアクセス設定を確認。

---

## 完了条件

- [ ] ポート列挙機能が動作
- [ ] 接続/切断処理が実装
- [ ] DeviceStatusの状態遷移が正しい
- [ ] 単体テストがパス

---

## 参照資料

- [17-gnss-tool-implementation-plan.md](../../docs/missions/m1-sensor-evaluation/gnss/17-gnss-tool-implementation-plan.md) — 実装計画
- [16-gnss-tool-architecture.md](../../docs/missions/m1-sensor-evaluation/gnss/16-gnss-tool-architecture.md) — アーキテクチャ

---

## 注意事項

- シリアルポートのテストは実機が必要な場合がある
- モック戦略を検討する必要あり（古典派TDD）

---

*計画作成: 2026-03-10 Session 79終了時*
