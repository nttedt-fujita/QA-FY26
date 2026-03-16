# MultiDeviceManager 設計書

**作成**: Session 194 (2026-03-16)
**方針**: 案B（MultiDeviceManager新設、DeviceManagerは変更しない）

---

## 概要

Phase 3（複数台同時対応）を実現するため、既存の `DeviceManager` を変更せずに `MultiDeviceManager` を新設する。

### 設計方針

| 観点 | 方針 |
|------|------|
| 既存コード | `DeviceManager` は変更しない（既存テスト影響なし） |
| 並行処理 | 各デバイスを `Arc<Mutex<DeviceManager>>` で独立管理 |
| API | `device_id`（ポートパス）で操作対象を指定 |

---

## 構造

### 現状（Phase 1）

```
AppState
└── device_manager: Mutex<DeviceManager<RealSerialPortProvider>>  // 1台のみ
```

### Phase 3（案B）

```
AppState
└── multi_device_manager: Mutex<MultiDeviceManager>
    └── managers: HashMap<String, Arc<Mutex<DeviceManager<RealSerialPortProvider>>>>
        ├── "/dev/ttyACM0" → DeviceManager (装置1)
        ├── "/dev/ttyACM1" → DeviceManager (装置2)
        └── ...
```

---

## MultiDeviceManager インターフェース

```rust
pub struct MultiDeviceManager {
    managers: HashMap<String, Arc<Mutex<DeviceManager<RealSerialPortProvider>>>>,
    provider: RealSerialPortProvider,
}

impl MultiDeviceManager {
    // 作成
    pub fn new() -> Self;

    // デバイス操作
    pub fn list_devices(&self) -> Result<Vec<DeviceInfo>, Error>;
    pub fn connect(&mut self, path: &str) -> Result<u32, Error>;
    pub fn disconnect(&mut self, path: &str) -> Result<(), Error>;

    // 特定デバイスのマネージャーを取得（並行処理用）
    pub fn get_manager(&self, path: &str) -> Option<Arc<Mutex<DeviceManager<RealSerialPortProvider>>>>;

    // 接続中の全デバイスパスを取得
    pub fn connected_paths(&self) -> Vec<String>;
}
```

---

## API変更

### 現状

| エンドポイント | 説明 |
|---------------|------|
| `GET /api/devices` | 装置一覧 |
| `POST /api/devices/{path}/connect` | 接続 |
| `POST /api/devices/{path}/disconnect` | 切断 |

### Phase 3

| エンドポイント | 説明 |
|---------------|------|
| `GET /api/devices` | 装置一覧（複数表示、各装置の接続状態付き） |
| `POST /api/devices/{path}/connect` | 特定装置に接続（複数OK） |
| `POST /api/devices/{path}/disconnect` | 特定装置を切断 |
| `POST /api/inspections/start-all` | 全接続装置で検査開始（新規） |
| `GET /api/devices/{path}/status` | 特定装置の詳細状態（新規） |

### 変更点

- 既存APIは `device_id`（パス）を明示的に指定する形式に統一
- 複数接続を許可（`MaxDevicesReached` エラーを削除）

---

## 並行検査の実装方針

```rust
impl MultiDeviceManager {
    /// 全接続デバイスで並行検査を開始
    pub async fn start_parallel_inspection(&self) -> HashMap<String, InspectionResult> {
        let handles: Vec<_> = self.managers.iter().map(|(path, manager)| {
            let manager = Arc::clone(manager);
            let path = path.clone();
            tokio::spawn(async move {
                let mut m = manager.lock().await;
                let result = run_inspection(&mut m).await;
                (path, result)
            })
        }).collect();

        let results = futures::future::join_all(handles).await;
        results.into_iter()
            .filter_map(|r| r.ok())
            .collect()
    }
}
```

### ポイント

1. 各 `DeviceManager` は `Arc<Mutex<...>>` で独立管理
2. `tokio::spawn` で各デバイスを並行処理
3. デバイス間でロック競合なし（各デバイスが独立した `Mutex`）

---

## 実装ステップ

| Step | 内容 | 影響範囲 |
|------|------|---------|
| 1 | `MultiDeviceManager` 構造体を追加 | device/multi_manager.rs（新規） |
| 2 | `list_devices()`, `connect()`, `disconnect()` 実装 | 同上 |
| 3 | `AppState` を変更 | device_api.rs |
| 4 | 既存API（list/connect/disconnect）を移行 | device_api.rs |
| 5 | 複数接続テスト追加 | tests/ |
| 6 | 並行検査API追加 | inspection_api.rs |
| 7 | フロントエンド対応 | 複数デバイス選択UI |

---

## テスト方針

### 単体テスト（MultiDeviceManager）

| ケース | 内容 |
|--------|------|
| M1-1 | 0台検出 |
| M1-2 | 複数台検出 |
| M2-1 | 1台接続 |
| M2-2 | 複数台接続 |
| M2-3 | 切断後に再接続 |
| M3-1 | 並行検査（モック） |

### 統合テスト（API）

| ケース | 内容 |
|--------|------|
| API-1 | 複数台が表示される |
| API-2 | 複数台に接続できる |
| API-3 | 全台で検査開始できる |

---

## 参照

- [17-gnss-tool-implementation-plan.md](../../docs/missions/m1-sensor-evaluation/gnss/17-gnss-tool-implementation-plan.md) - Phase 3 計画
- [device/manager.rs](../../prototype/m1-gnss/backend/src/device/manager.rs) - 既存 DeviceManager
