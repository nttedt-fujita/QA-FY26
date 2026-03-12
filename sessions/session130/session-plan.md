# Session 130 計画

**目的**: M1-GNSS実装の続き — 屋外検査Phase 3（DB保存）

**前提**: Session 129でドキュメント整理ルール化完了

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | DB保存機能の実装 | docs/missions/m1-sensor-evaluation/gnss/26-outdoor-inspection-domain-model.md |
| 2 | APIエンドポイント追加 | docs/missions/m1-sensor-evaluation/gnss/api-spec.md |
| 3 | テスト追加 | - |

---

## 詳細

### 1. DB保存機能の実装

Session 123で設計した `outdoor_inspection_results` テーブルを実装:
- マイグレーション作成
- Rust側のモデル・リポジトリ実装

### 2. APIエンドポイント追加

- `POST /api/outdoor-inspection-results` — 結果保存
- `GET /api/outdoor-inspection-results/{device_serial}` — 結果取得

---

## 参照

- [Session 129 summary](../session129/session-summary.md)
- [26-outdoor-inspection-domain-model.md](../../docs/missions/m1-sensor-evaluation/gnss/26-outdoor-inspection-domain-model.md)
- [27-outdoor-inspection-implementation-plan.md](../../docs/missions/m1-sensor-evaluation/gnss/27-outdoor-inspection-implementation-plan.md)

---

*計画作成: 2026-03-12 Session 129終了時*
