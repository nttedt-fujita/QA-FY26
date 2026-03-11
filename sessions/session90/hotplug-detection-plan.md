# ホットプラグ検出機能の計画

**作成日**: 2026-03-11
**Session**: 90
**目的**: USBデバイスの挿抜を検出し、UIに即座に反映する

---

## 背景

受入検査では、F9Pを1台ずつUSB/UARTで接続して検査する。検査完了後にケーブルを抜いて次のF9Pを接続する流れ。

**現状の問題**:
- ケーブル抜去を検出するには `list_devices()` を定期的に呼ぶ必要がある（ポーリング）
- 「抜けた」瞬間にUIに反映されない

**目標**:
- USBケーブルを抜いたら、即座に「切断されました」とUIに表示
- 新しいデバイスを挿したら、即座に「検出しました」とUIに表示

---

## 方式比較

### 方式A: ポーリング（現在の実装）

```rust
loop {
    let devices = manager.list_devices();
    // 差分を検出してUIに通知
    sleep(Duration::from_secs(1));
}
```

| 観点 | 評価 |
|------|------|
| 実装難易度 | ◎ 簡単 |
| 検出速度 | △ ポーリング間隔依存（1秒程度） |
| CPU使用率 | △ 常にCPU使用 |
| クロスプラットフォーム | ◎ どこでも動く |

**適用場面**: プロトタイプ、シンプルな要件

---

### 方式B: イベント駆動（udev / WMI）

```rust
// Linux (udev)
let mut monitor = udev::MonitorBuilder::new()?.listen()?;
for event in monitor.iter() {
    match event.event_type() {
        EventType::Add => on_device_added(event),
        EventType::Remove => on_device_removed(event),
        _ => {}
    }
}
```

| 観点 | 評価 |
|------|------|
| 実装難易度 | △ プラットフォーム別対応が必要 |
| 検出速度 | ◎ 即座（ミリ秒単位） |
| CPU使用率 | ◎ イベントがないときは0% |
| クロスプラットフォーム | × Linux/Windowsで別実装 |

**適用場面**: 本番環境、リアルタイム性が重要

---

### 方式C: ハイブリッド（推奨）

**Phase 1**: ポーリング方式で実装（シンプル）
**Phase 2**: イベント駆動に移行（オプション、必要に応じて）

```rust
pub trait DeviceEventSource {
    /// デバイスイベントを待機
    async fn next_event(&mut self) -> DeviceEvent;
}

pub enum DeviceEvent {
    Added(PortInfo),
    Removed(String), // path
}

// Phase 1: ポーリング実装
pub struct PollingEventSource { ... }

// Phase 2: udev実装（Linuxのみ）
pub struct UdevEventSource { ... }
```

---

## Phase 1 実装計画

### 目標

- デバイス挿抜をUIに反映できる状態にする
- ポーリング方式で十分な応答性を確保（500ms間隔）

### 実装内容

1. **DeviceWatcher構造体の追加**
   - バックグラウンドスレッドで `list_devices()` を定期実行
   - 差分を検出してコールバック呼び出し

2. **コールバックインターフェース**
   ```rust
   pub trait DeviceWatcherCallback {
       fn on_device_added(&self, device: &Device);
       fn on_device_removed(&self, path: &str);
   }
   ```

3. **DeviceManagerへの統合**
   - `start_watching()` / `stop_watching()` メソッド追加

### ファイル構成

```
src/device/
├── mod.rs
├── status.rs
├── filter.rs
├── manager.rs
└── watcher.rs  ← 新規追加
```

---

## Phase 2 実装計画（将来）

### 目標

- イベント駆動で即座に検出
- CPU使用率を最小化

### 実装内容

1. **udevEventSource（Linux）**
   - `udev` クレートを使用
   - `/dev/ttyACM*` の追加/削除を監視

2. **WmiEventSource（Windows）**
   - `windows` クレートを使用
   - WMIでデバイス変更イベントを購読

3. **抽象化レイヤー**
   - `DeviceEventSource` トレイトでプラットフォーム差を吸収

---

## 現在の進捗

| Phase | 内容 | 状態 |
|-------|------|------|
| Phase 1 Step 1 | UBXパーサー | ✅ 完了 |
| Phase 1 Step 2 | DeviceManager | ✅ 完了 |
| Phase 1 Step 3 | InspectionEngine | ✅ 完了 |
| Phase 1 Step 4 | DB Repository | ✅ 完了 |
| Phase 1 Step 5 | FTDI対応 | ✅ 完了 |
| Phase 1 Step 6 | ボーレート自動検出 | 📝 ADR作成済み（実装は後） |
| Phase 1 Step 7 | コンポーネント統合 | ⏳ 未着手 |
| Phase 1 Step 8 | 実機テスト | ⏳ 未着手 |
| **追加** | ホットプラグ検出（ポーリング） | 📝 計画作成 |

---

## 次のアクション

1. **Step 7（コンポーネント統合）を先に完了** — InspectionEngine → Repository
2. **Step 8（実機テスト）を実施**
3. **ホットプラグ検出は実機テスト後に検討** — 必要性を実際の運用で確認

---

## 参照

- [DeviceManager実装](../../prototype/m1-gnss/backend/src/device/manager.rs)
- [状態遷移定義](../../prototype/m1-gnss/backend/src/device/status.rs)
- [ADR-007 ボーレート自動検出](../../docs/adr/m1/ADR-007-baud-rate-detection.md)
