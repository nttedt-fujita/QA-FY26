# M1-B GNSS評価ツール 実装ロードマップ

**作成日**: 2026-03-11 Session 95
**目的**: 残りの実装をセッション単位で計画し、進捗を可視化する

---

## 現状サマリー（Session 94終了時点）

### 完了済み ✅

| 層 | コンポーネント | 備考 |
|---|--------------|------|
| ドメイン | UBXパーサー | 全メッセージタイプ対応 |
| ドメイン | InspectionEngine | 5項目検査ロジック |
| インフラ | DeviceManager | ボーレート自動検出（ADR-007） |
| インフラ | Repository | Lot/Device/Inspection CRUD |
| サービス | InspectionService | Engine→Repository統合 |
| Web API | 装置管理 | GET/POST connect/disconnect |
| フロントエンド | 装置画面 | 基本動作確認済み |

### 未実装 ❌

| 層 | コンポーネント | 優先度 |
|---|--------------|--------|
| Web API | ロット管理 | 高 |
| Web API | 検査実行 | 高 |
| フロントエンド | ロット画面 | 高 |
| フロントエンド | 検査画面 | 高 |
| 統合 | E2Eフロー | 中 |

---

## 実装計画

### Session 95: 計画策定 + ロットAPI実装

**目標**: ロット管理APIを完成させる

| タスク | 成果物 | 見積 |
|--------|--------|------|
| 実装ロードマップ作成 | このドキュメント | 済 |
| ロット管理API実装 | `src/web/lot_api.rs` | 小 |
| APIテスト | 手動テスト（curl） | 小 |

**完了条件**:
- [ ] `GET /api/lots` が動作する
- [ ] `POST /api/lots` が動作する
- [ ] `GET /api/lots/{id}` が動作する
- [ ] `PUT /api/lots/{id}` が動作する

---

### Session 96: 検査API実装

**目標**: 検査実行APIを完成させる

| タスク | 成果物 | 見積 |
|--------|--------|------|
| 検査API実装 | `src/web/inspection_api.rs` | 中 |
| InspectionService統合 | API→Service→Repository | 中 |
| APIテスト | 手動テスト（curl + 実機） | 小 |

**完了条件**:
- [ ] `POST /api/inspections` で検査が実行される
- [ ] 結果がDBに保存される
- [ ] `GET /api/inspections` で履歴が取得できる

---

### Session 97: フロントエンド（ロット画面）

**目標**: ロット管理画面を完成させる

| タスク | 成果物 | 見積 |
|--------|--------|------|
| ロット一覧コンポーネント | `LotList.tsx` | 小 |
| ロット詳細/編集フォーム | `LotForm.tsx` | 中 |
| ロット画面統合 | `app/lots/page.tsx` | 小 |
| API連携テスト | ブラウザ動作確認 | 小 |

**完了条件**:
- [ ] ロット一覧が表示される
- [ ] 新規ロット作成ができる
- [ ] ロット選択状態が保持される

---

### Session 98: フロントエンド（検査画面）+ E2E確認

**目標**: 検査実行画面を完成させ、全フロー動作確認

| タスク | 成果物 | 見積 |
|--------|--------|------|
| 検査画面実装 | `app/inspection/page.tsx` | 中 |
| 検査結果テーブル | `InspectionResult.tsx` | 小 |
| E2Eフロー確認 | 実機で全フロー動作確認 | 中 |
| ドキュメント更新 | README、操作手順 | 小 |

**完了条件**:
- [ ] ロット選択 → 装置接続 → 検査実行 → 結果保存 の全フローが動作
- [ ] 検査結果がテーブルに表示される
- [ ] 操作手順書が作成されている

---

## 依存関係

```
Session 95 (ロットAPI)
    ↓
Session 96 (検査API) ← Repository/InspectionService依存
    ↓
Session 97 (ロット画面) ← ロットAPI依存
    ↓
Session 98 (検査画面 + E2E) ← 全API依存
```

---

## リスク・懸念事項

| リスク | 対策 |
|--------|------|
| 検査APIとInspectionServiceの統合が複雑 | 既存のconverter.rsを活用 |
| フロントエンドのタブナビゲーション | shadcn/uiのTabsを使用 |
| 状態管理（選択中ロット） | Context APIで管理 |

---

## 参照資料

- [UI設計書](../session92/ui-design-phase1.md)
- [ドメインモデル](../session86/gnss-unified-domain-model.md) ※Session 86 summaryで参照
- [ADR-005 技術スタック](../../docs/adr/m1/ADR-005-gnss-tool-tech-stack.md)
- [ADR-007 ボーレート自動検出](../../docs/adr/m1/ADR-007-baud-rate-detection.md)

---

*作成: 2026-03-11 Session 95*
