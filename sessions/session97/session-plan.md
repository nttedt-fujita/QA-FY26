# Session 97 計画

**目的**: フロントエンド（ロット画面）実装

---

## やること

### 1. ロット一覧コンポーネント

- `LotList.tsx` 作成
- ロット一覧の表示
- 選択状態の管理

### 2. ロット詳細/編集フォーム

- `LotForm.tsx` 作成
- 新規作成/編集フォーム
- バリデーション

### 3. ロット画面統合

- `app/lots/page.tsx` 作成
- API連携（GET/POST/PUT）

### 4. タブナビゲーション（必要なら）

- shadcn/ui のTabs導入を検討
- ロット/装置/検査の切り替え

---

## 完了条件

- [ ] ロット一覧が表示される
- [ ] 新規ロット作成ができる
- [ ] ロット選択状態が保持される
- [ ] ブラウザ動作確認済み

---

## 参照資料

- [UI設計書](../session92/ui-design-phase1.md)
- [ロットAPI](../../prototype/m1-gnss/backend/src/web/lot_api.rs)
- [実装ロードマップ](../session95/implementation-roadmap.md)

---

*計画作成: 2026-03-11 Session 96終了時*
