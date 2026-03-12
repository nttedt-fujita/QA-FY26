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

## Session 144 (2026-03-12)

**概要**: FE側を統合API (`/api/gnss-state`) に移行

**実施内容**:
1. **統合API用の型定義・関数追加** (`lib/api.ts`)
   - `GnssStateResponse`, `NavPvtResponse`, `MonRfResponse` 等
   - `getGnssState()` 関数
2. **統合API用hook作成** (`hooks/useGnssState.ts`)
   - ポーリングロジック集約
3. **各パネルのprops変更**
   - 自前fetch削除 → `data` props受け取り形式
   - NavStatusPanel, SkyPlotPanel, MonSpanPanel, NavSigPanel
4. **屋外検査画面の統合API対応**
   - `useGnssState` hookでポーリング
   - 各パネルにデータを渡す

**決定事項**:
| 項目 | 決定 |
|------|------|
| パネルの責務 | 表示専用（fetchしない） |
| ポーリング | 親コンポーネントで統合API 1回 |

**変更ファイル**:
| ファイル | 内容 |
|----------|------|
| [api.ts](../../prototype/m1-gnss/frontend/src/lib/api.ts) | 統合API型定義・関数追加 |
| [useGnssState.ts](../../prototype/m1-gnss/frontend/src/hooks/useGnssState.ts) | 新規hook |
| [NavStatusPanel.tsx](../../prototype/m1-gnss/frontend/src/components/NavStatusPanel.tsx) | data props形式に変更 |
| [SkyPlotPanel.tsx](../../prototype/m1-gnss/frontend/src/components/SkyPlotPanel.tsx) | data props形式に変更 |
| [MonSpanPanel.tsx](../../prototype/m1-gnss/frontend/src/components/MonSpanPanel.tsx) | data props形式に変更 |
| [NavSigPanel.tsx](../../prototype/m1-gnss/frontend/src/components/NavSigPanel.tsx) | data props形式に変更 |
| [outdoor/page.tsx](../../prototype/m1-gnss/frontend/src/app/inspections/outdoor/page.tsx) | 統合API対応 |

**次セッション（Session 145）でやること**:
- 実機接続での動作確認
- 問題があれば修正

---

## Session 145 (2026-03-12)

**概要**: 統合API動作確認 → 定期出力とポーリング混在問題を修正

**実施内容**:
1. **統合API動作確認で問題発見**
   - 「データなし」「全メッセージの取得に失敗」エラー
   - 一部メッセージでタイムアウト/パースエラー
2. **原因特定**
   - Session 139-144の設計を再確認
   - 問題: 定期出力が有効 + 統合APIはポーリング方式 → 混在
   - 定期出力メッセージがバッファに溜まり、ポーリング応答と混在
3. **修正**
   - 定期出力を無効化（`enable_periodic_output` → `send_disable_periodic_output`）
   - 統合APIはポーリング専用で動作

**決定事項**:
| 項目 | 決定 |
|------|------|
| 定期出力 | 無効化（受信スレッド未実装のため） |

**変更ファイル**:
| ファイル | 内容 |
|----------|------|
| [device_api.rs](../../prototype/m1-gnss/backend/src/web/device_api.rs) | 定期出力を無効化に変更 |

**次セッション（Session 146）でやること**:
- バックエンド再起動後の動作確認
- ADR-012を更新（定期出力無効化の経緯を追記）

---

## Session 146 (2026-03-12)

**概要**: ログファイル出力設定 + NMEA出力無効化

**実施内容**:
1. **ログファイル出力を設定**
   - `env_logger` → `tracing` + `tracing-subscriber` + `tracing-appender`
   - 日次ローテーション、コンソール＋ファイル両方出力
2. **動作確認で問題発見**
   - 屋内検査後の屋外検査で統合APIが断続的に500エラー
   - 原因: NMEAデータがUBXポーリングを妨害
   - 屋内検査終了時にNMEA ONに戻していた
3. **修正**
   - デバイス接続時にNMEA出力もOFFにする
   - `send_disable_nmea_output`関数を追加

**変更ファイル**:
| ファイル | 内容 |
|----------|------|
| [Cargo.toml](../../prototype/m1-gnss/backend/Cargo.toml) | tracing関連クレート追加 |
| [main.rs](../../prototype/m1-gnss/backend/src/main.rs) | ログ初期化をtracing-subscriberに変更 |
| [device_api.rs](../../prototype/m1-gnss/backend/src/web/device_api.rs) | NMEA出力無効化を追加 |
| [manager.rs](../../prototype/m1-gnss/backend/src/device/manager.rs) | log→tracing置き換え |
| [engine.rs](../../prototype/m1-gnss/backend/src/inspection/engine.rs) | log→tracing置き換え |

**次セッション（Session 147）でやること**:
- NMEA OFF修正後の動作確認
- ADR-012を更新

---

## Session 147 (2026-03-12)

**概要**: 統合APIのNMEA問題を修正

**実施内容**:
1. **ログ分析で問題特定**
   - 屋内検査後に統合APIで500エラー・タイムアウト頻発
   - 原因: 屋内検査終了時にNMEA ONに戻していた
2. **修正1: 統合APIでNMEA OFFを送信**
   - `send_disable_nmea_output`をpubに変更
   - `gnss_state_api.rs`でメッセージ取得前にNMEA OFF送信
3. **修正2: 屋内検査終了時のNMEA ON戻しを削除**
   - `engine.rs`のNMEA ON送信処理を削除
   - テスト`test_h3_nmea_off_only`に更新

**決定事項**:
| 項目 | 決定 |
|------|------|
| NMEA制御 | 屋内検査終了時にONに戻さない |

**変更ファイル**:
| ファイル | 内容 |
|----------|------|
| [device_api.rs](../../prototype/m1-gnss/backend/src/web/device_api.rs) | send_disable_nmea_outputをpubに変更 |
| [gnss_state_api.rs](../../prototype/m1-gnss/backend/src/web/gnss_state_api.rs) | NMEA OFF送信を追加 |
| [engine.rs](../../prototype/m1-gnss/backend/src/inspection/engine.rs) | NMEA ON戻し処理を削除 |

**残った課題**:
- 統合APIでタイムアウトが多い（屋内環境の影響？タイミング問題？）
- ADR-012の更新が未実施

**次セッション（Session 148）でやること**:
- 屋外環境での統合API動作確認
- タイムアウト問題が継続する場合はタイミング調整
- ADR-012を更新

---
