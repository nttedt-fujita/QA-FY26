# Session 95 計画

**目的**: 過去の成果物を整理し、実装状況を明確化

---

## 背景

Session 94でロット管理画面の実装を開始したが、以下の懸念が生じた:
- ドメインモデリングをやったのに、なぜバックエンド実装が完全でないのか
- 過去の要求整理と現在の実装状況に乖離がある

---

## やること

### 1. 過去の成果物を確認

以下のドキュメントを読み直す:
- [gnss-unified-domain-model.md](../session86/gnss-unified-domain-model.md) — ドメインモデル
- [UI設計書](../session92/ui-design-phase1.md) — API設計含む
- [リポジトリ実装](../../prototype/m1-gnss/backend/src/repository/) — 現在のDB層

### 2. 実装状況を可視化

| 層 | コンポーネント | 状態 | 備考 |
|---|---|---|---|
| ドメイン | UBXパーサー | ✅ | |
| ドメイン | InspectionEngine | ✅ | |
| インフラ | DeviceManager | ✅ | |
| インフラ | Repository | ✅ | Lot/Device/Inspection CRUD |
| Web API | 装置管理 | ✅ | |
| Web API | ロット管理 | ❌ | |
| Web API | 検査実行 | ❌ | |
| フロントエンド | 装置画面 | △ | スタブ |
| フロントエンド | ロット画面 | ❌ | |
| フロントエンド | 検査画面 | ❌ | |

### 3. 次のステップを決定

整理結果に基づいて、残りの実装順序を決める。

---

## 完了条件

- [ ] 過去の成果物を読み直し、現状を把握した
- [ ] 実装状況マトリクスを更新した
- [ ] 残りの実装計画を明確化した

---

## 参照資料

- [Session 94 サマリー](../session94/session-summary.md)
- [ドメインモデル](../session86/gnss-unified-domain-model.md)
- [UI設計書](../session92/ui-design-phase1.md)

---

*計画作成: 2026-03-11 Session 94終了時*
