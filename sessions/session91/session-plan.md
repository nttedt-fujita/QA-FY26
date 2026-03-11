# Session 91 計画

**目的**: コンポーネント統合（InspectionEngine → Repository）

---

## 背景

Phase 1のコンポーネント（UBXパーサー、DeviceManager、InspectionEngine、Repository）は個別に完成している。これらを統合して、検査結果をDBに保存できるようにする。

---

## やること

### 1. InspectionEngine → Repository統合

- 検査結果（InspectionResult）をRepositoryに保存するフロー実装
- InspectionEngineにRepository参照を追加（またはサービス層を追加）

### 2. 統合フローの実装

```
ロット選択 → 装置検出 → 検査実行 → 結果保存
```

- Lot作成/選択
- Device接続
- InspectionEngine.run_inspection()
- Repository.save_inspection_result()

### 3. 統合テスト作成

- E2Eテスト: モックを使った統合フロー確認
- DBに保存されたデータの検証

---

## 完了条件

- [ ] InspectionEngine → Repository の統合コード
- [ ] 統合テスト（モック使用）
- [ ] 全テストパス

---

## 参照資料

- [Session 90 サマリー](../session90/session-summary.md)
- [統合ドメインモデル](../../docs/missions/m1-sensor-evaluation/gnss/19-gnss-unified-domain-model.md)
- [DB Schema](../../prototype/m1-gnss/db/schema.sql)

---

*計画作成: 2026-03-11 Session 90終了時*
