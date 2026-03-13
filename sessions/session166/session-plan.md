# Session 166 計画

**目的**: ボーレート自動変更の実機テスト

**前提**: Session 165でボーレート自動変更を実装済み

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | バックエンド起動 | - |
| 2 | `make rtk-debug`で接続テスト | - |
| 3 | ボーレート変更ログ確認 | - |
| 4 | 再接続テスト（BBR永続化確認） | - |

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
