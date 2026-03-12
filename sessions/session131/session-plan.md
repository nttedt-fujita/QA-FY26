# Session 131 計画

**目的**: 屋外検査機能の動作確認 + 残タスク整理

**前提**: Session 130でDB保存のコード実装は完了。ただし実動作確認はまだ。

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | 今回作った保存機能のE2E確認 | - |
| 2 | 屋外検査の残タスク確認 | docs/missions/m1-sensor-evaluation/gnss/27-outdoor-inspection-implementation-plan.md |
| 3 | RTK補正サービスの実装状況確認 | Session 125 summary |
| 4 | 残タスク整理・優先順位決め | - |

---

## 詳細

### 1. 保存機能のE2E確認（最優先）

**手順**:
1. バックエンド起動
2. フロントエンド起動
3. 実機接続 or モックで屋外検査実行
4. 「結果を保存」ボタン押下
5. DBに保存されたか確認（sqlite3コマンド等）

**確認ポイント**:
- APIが正常にレスポンスを返すか
- DBにレコードが作成されるか
- 保存後のUI表示（「保存済み」表示）

### 2. 屋外検査の残タスク確認

Session 125で記録された未実装項目を確認:
- RTK補正サービス: ⚠️ 未実装（Session 125時点）
- その他の未実装項目があるか

### 3. 今後の計画

E2E確認と残タスク整理の結果を踏まえて:
- Phase 4（検証・報告）に進めるか判断
- 未実装項目があれば優先順位を決める

---

## 参照

- [Session 130 summary](../session130/session-summary.md)
- [Session 125 summary](../session125/session-summary.md)
- [27-outdoor-inspection-implementation-plan.md](../../docs/missions/m1-sensor-evaluation/gnss/27-outdoor-inspection-implementation-plan.md)

---

*計画作成: 2026-03-12 Session 130終了時*
*修正: 2026-03-12 E2E確認・残タスク確認を優先に変更*
