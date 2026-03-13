# Mutex競合問題の分析

**目的**: gnss_state_apiとntrip_apiの競合問題を整理し、解決策を検討する

---

## 現状の処理フロー

### gnss_state_api（読み込み側）

```
GET /api/gnss-state
│
├── 1. ロック取得 device_manager.lock()
│
├── 2. バッファドレイン
│
├── 3. バッファ空待ち（最大500ms）
│
├── 4. NMEA OFF送信
│
├── 5. 6メッセージ順次取得（各メッセージごとに:）
│      ├── ドレイン
│      ├── poll送信
│      ├── バッファ空待ち（最大10ms）
│      ├── 受信（タイムアウト2秒）
│      └── パース
│
└── 6. ロック解放
```

**ロック保持時間（理論値）**:
- 最良ケース: 6メッセージ × ~50ms = 約300ms
- 最悪ケース: 6メッセージ × 2000ms = 約12秒
- 実測値（Session 175）: 16秒（タイムアウト連発時）

### ntrip_api（書き込み側）

```
RTCMデータ受信ループ
│
├── 1. ネットワークからRTCMデータ受信
│
├── 2. ロック取得 device_manager.lock()
│
├── 3. RTCMデータ書き込み
│
├── 4. フラッシュ待機（50ms）
│
└── 5. ロック解放
```

**ロック保持時間（理論値）**:
- 通常: 書き込み + 50ms = 約60-100ms
- RTCMデータは約1秒間隔で到着

---

## 競合のメカニズム

```
時間軸
─────────────────────────────────────────────────────────────→

gnss_state_api:
  [========= ロック保持 =========]  待ち  [=====]
       6メッセージ取得（最大12秒）        ↑ロック待ち

ntrip_api:
  待ち [===] 待ち [===] 待ち [===] 待ち [===]
    ↑     ↑     ↑     ↑
    ロック待ち（gnss_stateが保持中）
```

**問題点**:
1. gnss_state_apiが長時間ロックを保持
2. その間NTRIPのRTCMデータ転送が滞る
3. RTCMが滞るとRTK Fixが維持できない
4. RTK Fixが外れるとgnss_state_apiの応答も遅くなる（悪循環）

---

## 案D（タイムアウト短縮）の検討

### 変更内容
```rust
// 現状
let raw = match $manager.receive_ubx(std::time::Duration::from_millis(2000)) {

// 案D
let raw = match $manager.receive_ubx(std::time::Duration::from_millis(500)) {
```

### 時間の計算

| 項目 | 現状（2秒） | 案D（500ms） |
|------|------------|--------------|
| 1メッセージ最悪ケース | 2秒 | 500ms |
| 6メッセージ最悪ケース | 12秒 | 3秒 |
| ロック保持時間（最悪） | 12秒+ | 3秒+ |

### **結論: 案Dでは解決できない**

**理由**:
1. **ロック競合の構造は変わらない**
   - タイムアウトを短縮しても、gnss_state_apiとntrip_apiが同じMutexを取り合う構造は同じ
   - 3秒間ロックを保持 → その間NTRIPは停止

2. **NTRIP転送頻度との不整合**
   - RTCMデータは約1秒間隔で到着
   - gnss_stateが3秒ロック保持 → 2-3回分のRTCMが滞る
   - RTK Fixの維持に影響

3. **タイムアウト短縮のデメリット**
   - F9Pの応答が遅い場合（正常な遅延）でもタイムアウトになる
   - 信頼性が下がる

---

## 案A（try_clone読み書き分離）の検討

### 構成

```
現状:
DeviceManager (Mutex) ──→ シリアルポート
    ↑                        ↑
gnss_state_api          ntrip_api
  (読み込み)            (書き込み)
    └───── 競合 ─────────┘

案A:
DeviceManager ──→ シリアルポート（読み込み用）
    ↑
gnss_state_api
  (読み込み)

NtripWriter ──→ シリアルポート（書き込み用）← try_clone()
    ↑
ntrip_api
  (書き込み)

    └───── 競合なし ─────┘
```

### try_clone()の挙動

serialport v4のドキュメントより:
> Creates a shallow clone of the port. This will allow concurrent reads and writes to the port from multiple threads at a time.

**ポイント**:
- 読み込みと書き込みは独立して動作可能
- 同じ物理ポートを共有
- スレッドセーフ

### 実装の検討事項

1. **DeviceManagerの変更**
   - `try_clone()`を呼び出す箇所の特定
   - 書き込み専用のハンドルをどこで保持するか

2. **ntrip_apiの変更**
   - DeviceManagerではなく、書き込み専用ハンドルを使用
   - ロック競合を解消

3. **課題**
   - 同時読み書きで問題が起きないか（F9P側の挙動）
   - 既存コードへの影響範囲

### **案Aの結論: 検討の価値あり**

ただし、以下を確認する必要がある:
1. try_clone()の実装がserialport v4で確認できるか
2. F9Pが同時読み書きに対応しているか（仕様確認）
3. 影響範囲（DeviceManager, ntrip_api）

---

## 次のステップ

1. **案Aの実現可能性を詳細検討**
   - try_clone()のAPIを確認
   - DeviceManagerの変更範囲を特定
   - ntrip_apiの変更範囲を特定

2. **プロトタイプ実装**
   - 最小限の変更で試す

---

*作成: Session 176 (2026-03-13)*
