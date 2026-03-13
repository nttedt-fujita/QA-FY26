# Session 175 計画

**目的**: 生データ保存機能の動作確認

**前提**: Session 174でPhase 2（FE）完了

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | 実機で屋外検査実行 | - |
| 2 | DBにスナップショットが保存されたか確認 | - |
| 3 | GET snapshots APIで取得できるか確認 | - |

---

## 確認コマンド

### BE起動
```bash
cd prototype/m1-gnss && make dev-backend
```

### FE起動
```bash
cd prototype/m1-gnss && make dev-frontend
```

### DB確認
```bash
sqlite3 prototype/m1-gnss/backend/gnss_evaluation.db \
  "SELECT id, inspection_id, timestamp_ms FROM outdoor_inspection_snapshots ORDER BY id DESC LIMIT 10;"
```

### API確認
```bash
curl http://localhost:8080/api/outdoor-inspection-results/1/snapshots | jq
```

---

## 参照

- [session174/session-summary.md](../session174/session-summary.md) - 前セッションサマリー
- [session172/raw-data-storage-plan.md](../session172/raw-data-storage-plan.md) - 設計計画書
