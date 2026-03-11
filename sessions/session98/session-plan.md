# Session 98 計画

**目的**: フロントエンド（検査画面）実装

---

## やること

### 1. 検査用API関数追加

- `api.ts` に検査操作関数を追加
- runInspection, listInspections

### 2. 検査画面コンポーネント

- `app/inspections/page.tsx` 作成
- ロット選択表示
- 接続中装置表示
- 検査開始ボタン

### 3. 検査結果テーブル

- 検査項目ごとの結果表示
- 期待値 / 実測値 / 判定
- 総合判定表示

### 4. API連携

- `POST /api/inspections` - 検査実行
- `GET /api/inspections` - 検査履歴

---

## 完了条件

- [ ] 検査画面が表示される
- [ ] ロット・装置の選択状態が表示される
- [ ] 検査開始→結果表示が動作する
- [ ] ブラウザ動作確認済み

---

## 参照資料

- [UI設計書](../session92/ui-design-phase1.md)
- [検査API](../../prototype/m1-gnss/backend/src/web/inspection_api.rs)
- [Session 97 サマリー](../session97/session-summary.md)

---

*計画作成: 2026-03-11 Session 97終了時*
