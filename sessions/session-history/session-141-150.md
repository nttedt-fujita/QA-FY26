# セッション履歴: Session 141〜150

## Session 141 (2026-03-12)

**概要**: 定期出力概念解説 + TDDスキルでコードレビュー + テスト修正

**実施内容**:
1. **定期出力の概念解説**
   - ポーリング vs 定期出力の違いを図解
   - CFG-MSGOUTの設定方法を説明
2. **TDDスキルでコードレビュー**
   - cfg_valset.rs のテストをレビュー
   - 問題発見: rstest未使用、オフセット直接アクセス、二重保証
3. **テスト修正**
   - rstest形式に統合（20テストケース）
   - UbxFrame ヘルパー構造体でオフセット直接アクセスを改善
4. **06-test-style.md 更新**
   - Rust用のrstest例を追記

**追加発見**:
- プロジェクト全体でrstestが一貫して使われていなかった
- rstest使用: ntrip_api.rs, cfg_valset.rs のみ
- 他は `struct TestCase + for ループ` パターン

**決定事項**:
| 項目 | 決定 |
|------|------|
| Rustテストスタイル | rstest必須 |

**変更ファイル**:
| ファイル | 内容 |
|----------|------|
| [cfg_valset.rs](../../prototype/m1-gnss/backend/src/ubx/cfg_valset.rs) | テストをrstest形式に修正 |
| [06-test-style.md](../../.claude/rules/06-test-style.md) | Rust rstest例を追記 |

**次セッション（Session 142）でやること**:
- 統合API (`GET /api/gnss-state`) 実装（TDDで）
- FE側のポーリング集約

---

## Session 142 (2026-03-12)

**概要**: 統合API仕様設計（TDD Phase 0〜1）

**実施内容**:
1. **TDDスキル適用でPhase 0実施**
   - プロジェクト文脈の相互確認（3ステップ）
   - ADR-012の内容を確認
   - 既存APIコードを調査（nav_sat_api, nav_status_api, nav_sig_api, mon_span_api）
2. **漏れの発見・修正**
   - NAV-PVT: 個別APIが存在しない → 統合APIに含める
   - MON-RF: 個別APIが存在しない → 統合APIに含める
3. **Phase 1: 振る舞いの詳細記述**
   - 6メッセージの全フィールドを明文化
   - 部分失敗時の扱い（partial success + errors配列）

**決定事項**:
| 項目 | 決定 |
|------|------|
| エンドポイント | `GET /api/gnss-state` |
| 対象メッセージ | NAV-PVT, NAV-STATUS, NAV-SAT, NAV-SIG, MON-SPAN, MON-RF |
| 実装方式 | 1回のAPIコール内で6メッセージを順次ポーリング |
| 部分失敗時 | 取得できたデータは返す + `errors`配列 |
| 高度・速度 | 今回は含めない（パーサーに無い、後で追加可） |

**レスポンス構造**:
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

**変更ファイル**: なし（設計のみ）

**次セッション（Session 143）でやること**:
- **REST API vs GraphQL の解説**（ユーザーからの質問）
  - 統合APIが大きく情報の種類が多い → GraphQLの方がいいか？
- TDD Phase 2: テストシナリオリスト作成
- TDD Phase 3-4: テスト・実装
- main.rsにAPI登録

---

## Session 143 (2026-03-12)

**概要**: 統合API実装（BE側完了）

**実施内容**:
1. REST API vs GraphQL 解説（RESTで十分と判断）
2. 統合API (`GET /api/gnss-state`) 実装
   - 6メッセージを順次取得
   - 部分失敗対応（errors配列）
   - マクロ `poll_and_parse!` で重複削減
3. main.rsにAPI登録

**決定事項**:
| 項目 | 決定 |
|------|------|
| API方式 | REST（GraphQL不採用） |

**変更ファイル**:
| ファイル | 内容 |
|----------|------|
| [gnss_state_api.rs](../../prototype/m1-gnss/backend/src/web/gnss_state_api.rs) | 統合API実装 |

**残った課題**:
- FE側が未対応（個別API並行呼び出しでポーリング競合発生）

**次セッション（Session 144）でやること**:
- FE側を統合APIに移行

---
