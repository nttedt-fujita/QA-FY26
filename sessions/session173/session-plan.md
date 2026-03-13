# Session 173 計画

**目的**: 生データ保存機能の計画レビュー・実装開始

**前提**: Session 172で設計計画書を作成済み

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | 計画書のレビュー | sessions/session172/raw-data-storage-plan.md |
| 2 | 確認事項の決定 | - |
| 3 | 実装開始（Phase 1: BE） | prototype/m1-gnss/db/schema.sql |

---

## 確認事項（計画書セクション7より）

1. **オプションC（ハイブリッド）で進めてよいか？**
2. **スナップショットの保存間隔**: 1秒でよいか？
3. **データ保持期間**: 削除ポリシーは必要か？
4. **再生UI**: スライダー？自動再生？
5. **Phase分割**: Phase 1-2を先に、Phase 3は後でもよいか？

---

## 実装Phase 1（BE）

| # | タスク | 成果物 |
|---|--------|--------|
| 1-1 | DBマイグレーション | outdoor_inspection_snapshots テーブル |
| 1-2 | リポジトリ実装 | insert/get_snapshots |
| 1-3 | POST API拡張 | snapshots受け取り・保存 |
| 1-4 | GET API追加 | snapshots取得 |

---

## 参照

- [Session 172 計画書](../session172/raw-data-storage-plan.md)
- [Session 172 サマリー](../session172/session-summary.md)
