# Session 166 計画

**目的**: 待機時間の最適化検討

**前提**: Session 165でボーレート自動変更を実装、115200bpsで動作確認済み

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | 現在の待機時間の影響を調査 | gnss_state_api.rs |
| 2 | 待機時間を短縮できるか検討 | - |
| 3 | 恒久対策の検討 | - |

## Session 165の結果

- ボーレート: 115200bpsで接続成功
- ポーリング: 5/5成功
- 応答時間: 約6秒/回（長い）

---

## 詳細

### 1. テスト手順

```bash
# ターミナル1
cd prototype/m1-gnss && make dev-backend

# ターミナル2
cd prototype/m1-gnss && make rtk-debug
```

### 2. 確認ポイント

- [ ] 初回接続: 38400→115200への変更ログが出るか
- [ ] RTKポーリング: 5/5成功するか
- [ ] 再接続: 115200で検出されるか（変更ログが出ない）

---

## 参照

- [Session 165 summary](../session165/session-summary.md)
- [manager.rs](../../prototype/m1-gnss/backend/src/device/manager.rs)

---

*作成: Session 165 (2026-03-13)*
