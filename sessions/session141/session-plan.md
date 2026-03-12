# Session 141 計画

**目的**: 定期出力の概念解説 + 統合API実装 + コードレビュー

**前提**: Session 140で定期出力設定（CFG-VALSET）は実装済み

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | 定期出力（Periodic Output）の概念解説 | 32-cfg-msgout-periodic-output.md |
| 2 | TDDスキルで Session 140 コードレビュー | cfg_valset.rs, device_api.rs |
| 3 | 統合API (`GET /api/gnss-state`) 実装 | nav_status_api.rs, nav_sat_api.rs 等 |
| 4 | FE側のポーリング集約 | outdoor/page.tsx |

---

## 詳細

### 1. 定期出力の概念解説

藤田さんリクエスト:
- ポーリング vs 定期出力の違い
- なぜ定期出力が必要か
- CFG-MSGOUTの設定方法

### 2. TDDスキルでコードレビュー

Session 140で追加したコードをレビュー:
- `cfg_valset.rs`: `set_periodic_output()`, テーブルテスト
- `device_api.rs`: `enable_periodic_output()`

チェック観点:
- テーブルテスト形式の準拠
- should_succeed パラメータ
- エラーハンドリング

### 3. 統合API実装

新規エンドポイント `GET /api/gnss-state`:
```json
{
  "nav_status": { ... },
  "nav_sat": { ... },
  "nav_sig": { ... },
  "mon_span": { ... }
}
```

- 内部で順次ポーリング（競合回避）
- 1回のAPIで全データ取得

### 4. FE側のポーリング集約

現状:
- `nav_status_api` を1秒ごとにポーリング
- `nav_sat_api` を5秒ごとにポーリング
- etc...

新:
- `gnss_state_api` を1秒ごとにポーリング
- 1回で全データ取得

---

## 参照

- [Session 140 summary](../session140/session-summary.md)
- [ADR-012](../../docs/adr/m1/ADR-012-periodic-output-and-unified-api.md)
- [32-cfg-msgout-periodic-output.md](../../docs/missions/m1-sensor-evaluation/gnss/32-cfg-msgout-periodic-output.md)

---

*計画作成: 2026-03-12 Session 140終了時*
