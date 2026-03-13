# Session 178 サマリー

**目的**: FE/BEポーリング処理の図解作成と対策実装

---

## 実施内容

### 1. 概念説明図作成（polling-architecture.drawio）

FE/BEのポーリング構造を可視化:
- FE（ブラウザ）: 30秒タイマー、1秒ポーリング
- BE（Actix-web）: Mutex保護されたAPIハンドラ
- リクエストキュー: 最大25-29個が滞留
- Mutex競合: gnss_state_api と ntrip_api の取り合い

**問題の構造**:
- FEが30秒で30回リクエスト
- BEは6秒/回でしか処理できない
- キューに滞留 → FE終了後も平均70秒処理継続

### 2. シーケンス図作成（request-lifecycle.drawio）

リクエストのライフサイクルを時系列で可視化:
- Phase 1（検査中 30秒）: FEポーリング → キュー蓄積 → Mutex待機
- Phase 2（FE終了後）: キューに残った25-29個を処理継続
- 約100秒でようやくキュー消化完了

### 3. 対策案A実装（ポーリング間隔6秒化）

**変更ファイル**: [outdoor/page.tsx](../../prototype/m1-gnss/frontend/src/app/inspections/outdoor/page.tsx)

```diff
- pollIntervalMs: 1000,
+ pollIntervalMs: 6000,
```

**期待効果**:
- 30秒で5回のみリクエスト（従来30回）
- キュー滞留なし
- FE終了後のBE処理時間を大幅短縮

---

## 作成ファイル

| ファイル | 内容 |
|----------|------|
| [polling-architecture.drawio](./polling-architecture.drawio) | ポーリング構造概要図 |
| [request-lifecycle.drawio](./request-lifecycle.drawio) | リクエストライフサイクル図 |
| [session-summary.md](./session-summary.md) | このファイル |

---

## 変更ファイル

| ファイル | 変更内容 |
|----------|----------|
| `frontend/src/app/inspections/outdoor/page.tsx` | pollIntervalMs: 1000 → 6000 |

---

## 動作確認結果

6秒ポーリングで検査実行:

| 項目 | 以前（1秒ポーリング） | 今回（6秒ポーリング） |
|------|---------------------|---------------------|
| gnss-state呼び出し回数 | 30回 | **6回** ✅ |
| POST後のBE処理時間 | 60-80秒 | **約5秒** ✅ |
| ロック待機警告 | 100件 | **0件** ✅ |

**残課題**: POST後に1件分（約5秒）の処理が残る。この結果をFEに表示すべき（タイミング調整）

---

## 次セッションでやること

1. **FE表示タイミング調整**: POST後のBE処理完了を待ってから結果表示
2. **スナップショット可視化**: DBから取得したデータでスカイプロット再表示

---

## 参照

- [session177/log-analysis-results.md](../session177/log-analysis-results.md) - ログ分析結果
- [ADR-012](../../docs/adr/m1/ADR-012-periodic-output-and-unified-api.md) - 統合API採用経緯

---

*作成: Session 178 (2026-03-13)*
