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
