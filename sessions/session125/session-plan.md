# Session 125 計画

**目的**: 全体状況のおさらい、整合性確認、ドキュメント整理

---

## 背景

Session 124でPhase 1（集計ロジック）とPhase 2（結果表示UI）の枠組みが完了。
ただし、以下の課題がある:

1. **RTK補正サービス接続が未実装** — FIX率・FIX時間は画面だけ作った状態
2. **合格判定の定義の再確認が必要** — ADR-008の基準と実装の整合性
3. **ドキュメントの正式配置が必要** — sessions/からdocs/への移動
4. **全体の整合性確認が必要** — 実装内容が正しいか不安

---

## やること（優先順）

### 0. 全体状況のおさらい（最優先）

- これまでの実装内容の棚卸し
- 仕様書との照合（RTK設定、合格基準）
- 何ができていて何ができていないかの明確化

### 1. ER図の作成

現在のDB構造（屋内検査、ロット、デバイス）と、追加予定の屋外検査テーブルを含めたER図を作成。

**対象テーブル**:
- `lots` — ロット
- `devices` — デバイス
- `inspection_results` — 屋内検査結果
- `outdoor_inspection_results` — 屋外検査結果（新規）

**確認ポイント**:
- 外部キー制約の整合性
- 屋内/屋外検査の共通性（device_id, lot_id）
- 将来の拡張性（検査種別の追加）

### 2. DB設計の最終確認

Session 123で設計した `outdoor_inspection_results` スキーマを確認:
- カラムの過不足
- 型の妥当性
- インデックス設計

### 3. 余裕があれば: Phase 3（DB保存）着手

- マイグレーション作成
- API実装（`POST /api/outdoor-inspection-results`）

---

## 参照資料

- [Session 123: ドメインモデル](../session123/outdoor-inspection-domain-model.md)
- [Session 123: 実装計画](../session123/rtk-implementation-plan.md)
- [ADR-008](../../docs/adr/m1/ADR-008-outdoor-inspection-criteria.md) — 合格基準
- [ADR-009](../../docs/adr/m1/ADR-009-outdoor-inspection-fe-aggregation.md) — FE集計

---

*計画作成: 2026-03-12 Session 124終了時*
