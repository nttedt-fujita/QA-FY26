# Session 142 サマリー

**日付**: 2026-03-12
**概要**: 統合API仕様設計（TDD Phase 0〜1）

---

## 実施内容

1. **TDDスキル適用でPhase 0実施**
   - プロジェクト文脈の相互確認（3ステップ）
   - ADR-012の内容を確認
   - 既存APIコードを調査

2. **漏れの発見・修正**
   - NAV-PVT: 個別APIが存在しない → 統合APIに含める
   - MON-RF: 個別APIが存在しない → 統合APIに含める

3. **Phase 1: 振る舞いの詳細記述**
   - 6メッセージの全フィールドを明文化
   - 部分失敗時の扱い（partial success + errors配列）

---

## 決定事項

| 項目 | 決定 |
|------|------|
| エンドポイント | `GET /api/gnss-state` |
| 対象メッセージ | NAV-PVT, NAV-STATUS, NAV-SAT, NAV-SIG, MON-SPAN, MON-RF |
| 実装方式 | 1回のAPIコール内で6メッセージを順次ポーリング |
| 部分失敗時 | 取得できたデータは返す + `errors`配列 |
| 高度・速度 | 今回は含めない（パーサーに無い、後で追加可） |

---

## レスポンス構造

```json
{
  "nav_pvt": { "lat", "lon", "fix_type", "carr_soln", "num_sv", "h_acc", "v_acc", "is_rtk_fixed", "is_rtk_float" },
  "nav_status": { "ttff", "msss", "gps_fix", "gps_fix_ok", "is_fix_valid", "carr_soln", "is_rtk_fixed", "is_rtk_float" },
  "nav_sat": { "satellites", "stats" },
  "nav_sig": { "signals", "stats" },
  "mon_span": { "blocks" },
  "mon_rf": { "blocks", "has_jamming", "has_critical_jamming" },
  "errors": []
}
```

---

## 確認したファイル

| ファイル | 内容 |
|----------|------|
| [ADR-012](../../docs/adr/m1/ADR-012-periodic-output-and-unified-api.md) | 統合API方針 |
| [nav_sat_api.rs](../../prototype/m1-gnss/backend/src/web/nav_sat_api.rs) | 既存API参考 |
| [nav_status_api.rs](../../prototype/m1-gnss/backend/src/web/nav_status_api.rs) | 既存API参考 |
| [nav_sig_api.rs](../../prototype/m1-gnss/backend/src/web/nav_sig_api.rs) | 既存API参考 |
| [mon_span_api.rs](../../prototype/m1-gnss/backend/src/web/mon_span_api.rs) | 既存API参考 |
| [nav_pvt.rs](../../prototype/m1-gnss/backend/src/ubx/nav_pvt.rs) | NAV-PVTパーサー |
| [mon_rf.rs](../../prototype/m1-gnss/backend/src/ubx/mon_rf.rs) | MON-RFパーサー |

---

## 成果物

| ファイル | 内容 |
|----------|------|
| [gnss-state-api-spec.md](gnss-state-api-spec.md) | 統合API仕様書（フィールド詳細、エラーハンドリング） |

---

## 残った課題

- TDD Phase 2〜4（テスト・実装）は次セッションで実施

---

## 次セッションでやること

→ [session143/session-plan.md](../session143/session-plan.md)
