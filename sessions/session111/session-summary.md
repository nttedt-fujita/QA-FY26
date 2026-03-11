# Session 111 サマリー

**日付**: 2026-03-11
**目的**: 屋外検査の残作業整理（TTFF、MON-RF等の調査）

---

## 実施内容

### 1. TTFF測定の調査

- **NAV-STATUS.ttff**（ミリ秒）で直接取得可能
- Cold/Warm/Hot Startの再現手順が課題
- 優先度: 中（手順確立が必要）

### 2. MON-RF（ジャミング検出）の調査

- **jammingState**（0-3）でジャミング状態を検出
  - 0: unknown, 1: ok, 2: warning, 3: critical
- L1/L2それぞれのブロックで確認可能
- 優先度: 低（補助情報）

### 3. 残作業の優先度整理

**Phase 1.5（屋外検査）の実装順序を確定**:

| Step | 作業 | 優先度 |
|------|------|--------|
| 1 | NAV-SIG API/FE連携 | **最優先** |
| 2 | RTK FIX率測定 | 高 |
| 3 | MON-SPANパーサー | 高 |
| 4 | NAV-SATパーサー | 中 |
| 5 | TTFF測定 | 中 |
| 6 | MON-RF | 低 |

### 4. M1-GNSS全体の整理

**フェーズ構成**:
- Phase 1: 屋内検査5項目 ← **完了済み**
- Phase 1.5: 屋外検査機能 ← **今ここ**
- Phase 2: レポート・履歴 ← Phase 1.5完了後に検討
- Phase 3: 複数台同時対応 ← Phase 1.5完了後に検討

### 5. 進捗管理方針

- **outdoor-inspection-priority.md** を都度更新する
- **m1-gnss-all-tasks.md** で全体を管理

---

## 作成ファイル

| ファイル | 内容 |
|----------|------|
| [ttff-monrf-spec.md](ttff-monrf-spec.md) | TTFF・MON-RF仕様調査結果 |
| [outdoor-inspection-priority.md](outdoor-inspection-priority.md) | Phase 1.5優先度整理 |
| [m1-gnss-all-tasks.md](m1-gnss-all-tasks.md) | M1-GNSS全体タスク整理 |

---

## 決定事項

| 項目 | 決定 |
|------|------|
| 次の作業 | NAV-SIG API/FE連携（Phase 1.5 Step 1） |
| Phase 2/3の扱い | Phase 1.5完了後に検討 |
| 進捗管理 | outdoor-inspection-priority.mdを都度更新 |

---

## 次セッション（Session 112）でやること

**NAV-SIG API/FE連携の設計・実装開始**:

1. バックエンド
   - `/api/nav-sig` エンドポイント追加
   - 検査エンジンでNAV-SIG定期取得

2. 検査ロジック
   - OutdoorInspector追加（閾値判定）

3. フロントエンド
   - L1/L2別C/N0一覧テーブル
   - L2受信率ゲージ
