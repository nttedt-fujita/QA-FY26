# セッション履歴: Session 111〜120

## Session 111 (2026-03-11)

**概要**: 屋外検査の残作業整理（TTFF、MON-RF、全体タスク）

**実施内容**:
1. **TTFF測定の調査**
   - NAV-STATUS.ttff（ミリ秒）で直接取得可能
   - Cold/Warm/Hot Startの再現手順が課題
2. **MON-RF（ジャミング検出）の調査**
   - jammingState（0-3）で検出
   - 優先度: 低（補助情報）
3. **Phase 1.5 優先度整理**
   - Step 1: NAV-SIG API/FE連携（最優先）
   - Step 2: RTK FIX率測定
   - Step 3: MON-SPANパーサー
4. **M1-GNSS全体タスク整理**
   - Phase 1.5完了後にPhase 2/3を検討

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [session111/ttff-monrf-spec.md](../session111/ttff-monrf-spec.md) | TTFF・MON-RF仕様調査 |
| [session111/outdoor-inspection-priority.md](../session111/outdoor-inspection-priority.md) | Phase 1.5優先度整理 |
| [session111/m1-gnss-all-tasks.md](../session111/m1-gnss-all-tasks.md) | M1-GNSS全体タスク |
| [session111/session-summary.md](../session111/session-summary.md) | セッションサマリー |
| [session112/session-plan.md](../session112/session-plan.md) | 次セッション計画 |

**決定事項**:
- Phase 2/3はPhase 1.5完了後に検討
- 進捗管理はoutdoor-inspection-priority.mdを都度更新

**次セッション（Session 112）でやること**:
- NAV-SIG API/FE連携の設計・実装開始

---

## Session 112 (2026-03-11)

**概要**: NAV-SIG API設計・実装（TDD）

**実施内容**:
1. **既存API構造の確認**
   - フレームワーク: Actix-web
   - 計画にAxumと誤記があった（hooks観察に記録）
2. **NAV-SIG API設計（TDD）**
   - 振る舞い記述
   - テストリスト作成（6項目）
3. **NAV-SIG API実装**
   - `GET /api/nav-sig` エンドポイント
   - 既存signal_stats関数を活用
   - 6テスト全てパス

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [session112/nav-sig-api-design.md](../session112/nav-sig-api-design.md) | 振る舞い・テストリスト |
| [session112/session-summary.md](../session112/session-summary.md) | セッションサマリー |
| [session113/session-plan.md](../session113/session-plan.md) | 次セッション計画 |
| nav_sig_api.rs | APIハンドラー + テスト |

**決定事項**:
- NAV-SIG APIは既存のsignal_stats関数を呼び出す設計

**次セッション（Session 113）でやること**:
- フロントエンド: L1/L2別C/N0一覧表示
- フロントエンド: L2受信率ゲージ
- 実機テスト

---

## Session 113 (2026-03-11)

**概要**: NAV-SIG フロントエンド実装

**実施内容**:
1. **api.ts にNAV-SIG API追加**
   - NavSignal, SignalStats, NavSigResponse 型定義
   - getNavSig() 関数
2. **NavSigPanel.tsx 新規作成**
   - L1/L2別C/N0一覧テーブル
   - L2受信率ゲージ（50%基準）
   - 合格/不合格表示
   - 1秒ポーリング
3. **inspections/page.tsx に統合**
4. **実機テスト**
   - GPS L1: 3衛星、L2: 0衛星（0%不合格）確認

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [NavSigPanel.tsx](../../prototype/m1-gnss/frontend/src/components/NavSigPanel.tsx) | 新規作成 |
| [session113/session-summary.md](../session113/session-summary.md) | セッションサマリー |
| [session114/session-plan.md](../session114/session-plan.md) | 次セッション計画 |

**確認事項**:
- NAV-SIGは検査ロジックに未組込（リアルタイムモニタリング専用）
- ユーザー要望: 屋外/屋内検査はページを分けたい

**次セッション（Session 114）でやること**:
- 屋内/屋外検査ページ分離

---

## Session 114 (2026-03-11)

**概要**: ネットワークRTK（NTRIP）調査

**実施内容**:
1. **NTRIPプロトコル調査（1次情報）**
   - ESA公式ドキュメント（NtripDocumentation.pdf）を取得・分析
   - NTRIP Version 1.0 仕様を抽出
   - Rev1 vs Rev2 の違いを整理
2. **ZED-F9P RTK設定調査**
   - Integration Manual からRTK configuration (Page 16-24) を抽出
   - RTCMデータの送信方法確認（シリアルに流すだけでOK）
3. **Rustライブラリ調査・身元確認**
   - nav-solutions/ntrip-client を推奨ライブラリとして選定
   - 組織の信頼性確認（GNSS専門、27リポジトリ、活発）

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [20-ntrip-rtk-implementation.md](../../docs/missions/m1-sensor-evaluation/gnss/20-ntrip-rtk-implementation.md) | 調査結果まとめ（Session 115で正式配置） |
| [21-ntrip-protocol-spec.md](../../docs/missions/m1-sensor-evaluation/gnss/21-ntrip-protocol-spec.md) | NTRIP仕様抽出（Session 115で正式配置） |
| [22-rtk-configuration.md](../../docs/missions/m1-sensor-evaluation/gnss/22-rtk-configuration.md) | ZED-F9P RTK設定（Session 115で正式配置） |
| [session114/session-summary.md](../session114/session-summary.md) | セッションサマリー |

**決定事項**:
- NTRIPクライアント機能をアプリに追加する方針
- nav-solutions/ntrip-client を使用
- NTRIP設定UI（5項目）が必要

**次セッション（Session 115）でやること**:
- NTRIP機能実装開始（クレート追加、API設計、UI）

---

## Session 115 (2026-03-11)

**概要**: RTK関連ドキュメント正式配置

**実施内容**:
1. **全体進捗おさらい**
   - M1-GNSS全体タスクを確認
   - RTK実装は優先度が高くないことを確認
2. **RTK関連ドキュメント正式配置**
   - Session 114のドラフトを`docs/missions/m1-sensor-evaluation/gnss/`に移動
   - 20-ntrip-rtk-implementation.md（実装方針）
   - 21-ntrip-protocol-spec.md（仕様抽出）
   - 22-rtk-configuration.md（ZED-F9P設定）
   - sources/NtripDocumentation.pdf（原本）
3. **クリーンアップ**
   - session114からドラフト削除
   - README.md更新

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [20-ntrip-rtk-implementation.md](../../docs/missions/m1-sensor-evaluation/gnss/20-ntrip-rtk-implementation.md) | NTRIP/RTK実装方針 |
| [21-ntrip-protocol-spec.md](../../docs/missions/m1-sensor-evaluation/gnss/21-ntrip-protocol-spec.md) | NTRIP仕様抽出 |
| [22-rtk-configuration.md](../../docs/missions/m1-sensor-evaluation/gnss/22-rtk-configuration.md) | ZED-F9P RTK設定 |
| [session115/session-summary.md](../session115/session-summary.md) | セッションサマリー |
| [session116/session-plan.md](../session116/session-plan.md) | 次セッション計画 |

**決定事項**:
- RTK実装は優先度が高くない
- 次の優先: MON-SPAN or 屋内/屋外ページ分離

**次セッション（Session 116）でやること**:
- MON-SPANパーサー実装、または屋内/屋外検査ページ分離

---

## Session 116 (2026-03-12)

**概要**: MON-SPANパーサー実装

**実施内容**:
1. **MON-SPANパーサー実装**
   - `mon_span.rs` 新規作成
   - mon_rf.rsと同じパターンで実装
   - 8テスト全パス
2. **TDDレビューによるヌケ補完**
   - span/res/center抽出の検証追加
   - avg_amplitude()ヘルパーのテスト追加
3. **ドキュメント化**
   - 振る舞い・テストリストを仕様書にまとめた

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [mon_span.rs](../../prototype/m1-gnss/backend/src/ubx/mon_span.rs) | MON-SPANパーサー |
| [session116/mon-span-parser-spec.md](../session116/mon-span-parser-spec.md) | パーサー仕様書 |
| [session116/session-summary.md](../session116/session-summary.md) | セッションサマリー |
| [session117/session-plan.md](../session117/session-plan.md) | 次セッション計画 |

**決定事項**:
- MON-SPANパーサーはmon_rf.rsと同じパターンで実装
- テストはテーブルテスト形式で統一

**次セッション（Session 117）でやること**:
- MON-SPAN API/FE連携、または屋内/屋外検査ページ分離

---

## Session 117 (2026-03-12)

**概要**: MON-SPAN API実装 + 仕様確認プロセス問題発覚

**実施内容**:
1. **MON-SPAN API実装**
   - `GET /api/mon-span` エンドポイント追加
   - NAV-SIG APIと同じパターンで実装
   - 6テスト全パス、全体186テストパス
2. **問題発覚：仕様書を読まずに実装**
   - PDFから抽出した仕様書（ubx-mon-messages.md）を確認せずに実装
   - 既存コードのパターンを真似ただけだった
   - Session 116も同様のアプローチだった可能性
3. **仕様書との照合（事後確認）**
   - ブロック構造、周波数計算式: 一致
   - 結果的に正しかったが、プロセスに問題あり

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [mon_span_api.rs](../../prototype/m1-gnss/backend/src/web/mon_span_api.rs) | MON-SPAN API |
| [session117/mon-span-api-design.md](../session117/mon-span-api-design.md) | API設計 |
| [session117/session-summary.md](../session117/session-summary.md) | セッションサマリー |
| [session118/session-plan.md](../session118/session-plan.md) | 次セッション計画 |

**問題と対処**:
- 「推測で進めない」ルール違反
- 今後: 実装前に仕様書を必ず読む、読んだファイルを記録する

**次セッション（Session 118）でやること**:
- MON-SPAN FE実装、または屋内/屋外検査ページ分離

---

## Session 118 (2026-03-12)

**概要**: MON-SPAN TDDレビュー + 仕様書参照ルール強制化

**実施内容**:
1. **TDDレビュー（MON-SPAN）**
   - 仕様書（ubx-mon-messages.md）とテストを照合
   - パーサーテスト: 8項目すべて仕様をカバー
   - APIテスト: 6項目すべて適切
   - 結論: 追加テスト不要
2. **仕様書参照ルールの強制化**
   - グローバルルール: `~/.claude/rules/13-spec-first-implementation.md`
   - M1-GNSS固有: `prototype/m1-gnss/CLAUDE.md` 新規作成
3. **ドキュメント正式配置**
   - Session 116/117のドラフトを統合
   - `23-mon-span-implementation.md` 作成
   - ドラフト削除

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| `~/.claude/rules/13-spec-first-implementation.md` | 仕様書参照ルール（グローバル） |
| [prototype/m1-gnss/CLAUDE.md](../../prototype/m1-gnss/CLAUDE.md) | M1-GNSS固有ルール |
| [23-mon-span-implementation.md](../../docs/missions/m1-sensor-evaluation/gnss/23-mon-span-implementation.md) | MON-SPAN実装仕様（正式版） |
| [session118/session-summary.md](../session118/session-summary.md) | セッションサマリー |
| [session119/session-plan.md](../session119/session-plan.md) | 次セッション計画 |

**決定事項**:
- 仕様書参照ルールをグローバルルールとして追加
- プロトタイプごとにCLAUDE.mdで具体的な仕様書パスを記載
- MON-SPANテストは追加不要

**次セッション（Session 119）でやること**:
- MON-SPAN FE実装（スペクトラム波形、PGAゲージ）

---

## Session 119 (2026-03-12)

**概要**: MON-SPAN FE実装

**実施内容**:
1. **仕様確認**（新ルール準拠）
   - 23-mon-span-implementation.md確認
   - mon_span_api.rsレスポンス形式確認
2. **api.ts に MON-SPAN API追加**
   - SpanBlock, MonSpanResponse型定義
   - getMonSpan()関数
3. **MonSpanPanel.tsx 新規作成**
   - スペクトラム波形表示（SVG 256点）
   - PGAゲージ（45dB基準）
   - L1/L2ブロック別表示
4. **実機テスト・調整**
   - タイムアウト延長（1秒→3秒）
   - ポーリング間隔延長（2秒→5秒）

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [MonSpanPanel.tsx](../../prototype/m1-gnss/frontend/src/components/MonSpanPanel.tsx) | 新規作成 |
| [api.ts](../../prototype/m1-gnss/frontend/src/lib/api.ts) | getMonSpan()追加 |
| [session119/session-summary.md](../session119/session-summary.md) | セッションサマリー |
| [session120/session-plan.md](../session120/session-plan.md) | 次セッション計画 |

**確認結果**:
- FEビルド: ✅ 成功
- BEテスト: ✅ 186テスト全パス
- 実機（室内）: ✅ 動作確認

**Phase 1.5進捗**:
- Step 1 NAV-SIG: ✅ 完了
- Step 2 RTK FIX率: ❌ 未着手
- Step 3 MON-SPAN: ✅ 完了

**次セッション（Session 120）でやること**:
- 屋外動作確認
- TTFF測定・スカイプロット（優先）

---
