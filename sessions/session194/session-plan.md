# Session 194 計画

**前提**: Session 193でドキュメント整備完了。Phase 3（複数台同時対応）が未着手であることを確認。

---

## やること

### Phase 3（複数台同時対応）の要件整理

Session 186-187で計画されたが未着手の作業を開始。

**タスク**:
1. 現在のDeviceManager実装を確認
2. 複数装置接続時の挙動を整理
3. 実装方針を検討

**Phase 3の目標**（[17-gnss-tool-implementation-plan.md](../../docs/missions/m1-sensor-evaluation/gnss/17-gnss-tool-implementation-plan.md)より）:
| 優先度 | 機能 | 要件 | 備考 |
|--------|------|------|------|
| P3-1 | 複数装置同時接続 | FR7 | 2〜5台 |
| P3-2 | 状態一覧表示 | FR8 | 認知負荷軽減 |
| P3-3 | 並行検査 | FR7 | 同時に検査実行 |

---

## 読むべきファイル

| ドキュメント | 内容 |
|-------------|------|
| [17-gnss-tool-implementation-plan.md](../../docs/missions/m1-sensor-evaluation/gnss/17-gnss-tool-implementation-plan.md) | Phase 3計画（P3-1〜P3-3） |
| [session186/session-summary.md](../session186/session-summary.md) | 並列検査計画の経緯 |
| backend/src/device_manager.rs | 現在のDeviceManager実装 |
