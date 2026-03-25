# Session 307 確認ファイル一覧

**作成**: Session 307 (2026-03-25)

---

## 1. セッション管理関連

| ファイル | 内容 |
|----------|------|
| `sessions/session-history/session-301-310.md` | Session 301-306の履歴。301でdrawio更新、302-304でスライド作成、305でLinear更新、306でv4作成 |
| `sessions/session306/session-summary.md` | 前セッションのサマリー。v4作成（8枚、抽象的）、担当者責任を避ける前提、kintoneはスライドに入れない |
| `sessions/session307/session-plan.md` | 今セッションの計画。状況把握・整理、方向性の再確認が目的 |

---

## 2. Linear API結果

**確認方法**: `curl -X POST https://api.linear.app/graphql`

| issue | タイトル | 状態 |
|-------|---------|------|
| QA-6 | プレート調査（暗黙知外部化） | Done |
| QA-8 | 社長説明スライド作成（3/25） | Done |
| QA-10 | GENメーカー問い合わせ（API連携可否） | Done |
| QA-7 | 梱包変更作業・隠れコストの定量調査 | In Progress |
| QA-9 | 宮崎さん打ち合わせ（kintone調査） | Todo |
| QA-11 | 棚卸参加（3月末）— 現場作業フロー理解 | Todo |
| QA-5 | SIPOC作成と現場レビュー準備 | Backlog |
| QA-12 | PSI集計自動化の設計 | Backlog |
| QA-13 | GENデータの扱い確認（エクスポート機能検証） | Backlog |

---

## 3. M1 GNSS関連

### 確認したファイル

| ファイル | 内容 |
|----------|------|
| `prototype/m1-gnss/README.md` | 技術スタック（Rust + Actix-web）、起動方法、APIエンドポイント一覧 |
| `sessions/session-history/*.md` | M1-GNSS関連のセッション履歴（最終作業: Session 228） |

### Exploreエージェントが確認した主要ファイル

**バックエンド（prototype/m1-gnss/backend/）**:
- `src/ubx/*.rs` — UBXメッセージパーサー（12種類: NAV-PVT, NAV-SAT, NAV-SIG, MON-SPAN, CFG-VALSET等）
- `src/device/*.rs` — シリアル通信、複数装置同時対応（multi_manager.rs）
- `src/web/*.rs` — REST APIハンドラー（統合状態、NTRIP、検査API等）
- `src/inspection/*.rs` — 検査ロジック、合格判定
- `src/repository/*.rs` — SQLite実装

**フロントエンド（prototype/m1-gnss/frontend/）**:
- `app/page.tsx` — トップページ
- `app/devices/page.tsx` — 装置一覧
- `app/inspections/indoor/page.tsx` — 屋内検査
- `app/inspections/outdoor/page.tsx` — 屋外検査
- `components/*.tsx` — SkyPlotPanel, NavSigPanel, MonSpanPanel, GnssFilter等

### M1 GNSS実装状況サマリー

- **バックエンド**: 21,310行、ほぼ完成
- **フロントエンド**: 4,863行、ほぼ完成
- **最終作業**: Session 228（2026-03-17）CFG-VALDEL実装
- **残タスク**: 実機テスト再開、検査サマリー画面、CSV出力拡充

---

## 4. スライド関連

| ファイル | 内容 |
|----------|------|
| `sessions/session306/ceo-presentation-v4.md` | スライドv4（8枚）。起承転結構成、フローチャートのみ、抽象的な内容 |

### v4の構成

1. タイトル
2. 起: 2つの課題（104h/年、委託先在庫）
3. 承: 現状フロー（図のみ）
4. 転: 理想像（図のみ）
5. 結: 効果と進め方
6. 補足: 3月の調査活動
7. 補足: SIPOC
8. 質疑応答

---

*作成: Session 307 (2026-03-25)*
