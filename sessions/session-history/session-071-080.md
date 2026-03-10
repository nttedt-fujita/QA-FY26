# セッション履歴: Session 71〜80

## Session 71 (2026-03-10)

**概要**: AS-DT1質問リストの最終レビュー・背景補強

**実施内容**:
1. **質問リストv6のレビュー** — 背景・理由が不十分であることを指摘
2. **質問リストv7作成** — 農薬散布用途・みかん畑環境を明記、仕様書の「避けるべき環境」を引用、エビデンスURL追加
3. **正式配置先への反映**

**重要な改訂**:
- 想定用途（農薬散布、みかん畑）を概要に明記
- Q01/Q02を統合、電線・みかんの木の枝など具体例を追加
- Q3/Q4に仕様書の「ほこりの多い所は避ける」「激しく振動する所は避ける」を引用
- 質問番号を連番化（11件→10件）

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [session71/as-dt1-spec-questions-v7.md](../session71/as-dt1-spec-questions-v7.md) | 質問リストv7 |
| [session71/session-summary.md](../session71/session-summary.md) | セッションサマリー |
| [session72/session-plan.md](../session72/session-plan.md) | 次セッション計画 |

**反省点**:
- session-managementスキルを使ったが、前セッションの資料をきちんと読んでいなかった
- 放熱板の仕様の出典（Session 69のスクリーンショット）を見落とした

**次セッション（Session 72）でやること**:
- GNSS評価ツール: UBXパーサー実装（NAV-STATUS/NAV-DOP/MON-RF）— TDD
- DevContainer内でのテスト実行確認

---

## Session 72 (2026-03-10)

**概要**: GNSS評価ツール UBXパーサー実装（TDD）

**実施内容**:
1. **DevContainer環境整備** — Dockerfile.devのRustバージョンを`latest`に更新
2. **NAV-STATUSパーサー実装** — TTFF、RTK状態、FIX有効性判定
3. **NAV-DOPパーサー実装** — 精度劣化係数（スケール0.01変換）
4. **MON-RFパーサー実装** — ジャミング状態監視、可変長ブロック対応

**テスト結果**: 15テスト全パス

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [nav_status.rs](../../prototype/m1-gnss/backend/src/ubx/nav_status.rs) | NAV-STATUSパーサー |
| [nav_dop.rs](../../prototype/m1-gnss/backend/src/ubx/nav_dop.rs) | NAV-DOPパーサー |
| [mon_rf.rs](../../prototype/m1-gnss/backend/src/ubx/mon_rf.rs) | MON-RFパーサー |
| [session72/ubx-parser-test-scenarios.md](../session72/ubx-parser-test-scenarios.md) | テストシナリオ文書 |
| [session72/session-summary.md](../session72/session-summary.md) | セッションサマリー |

**Hooks観察**:
- TDD Phase 1（振る舞い記述）で仕様書フィールドのヌケモレ発生
- ユーザー指摘で発覚、`~/.claude/hooks-observations.md`に記録

---

## Session 73 (2026-03-10)

**概要**: GNSS評価ツールの要求整理・要求定義作成

**実施内容**:
1. **ドキュメント配置** — Session 72のテストシナリオを正式配置先に移動
2. **過去の要求確認** — Session 16（小板橋さんヒアリング）、Session 56-57の内容確認
3. **ヒアリング** — 藤田さんの理想像を整理（自動検査、複数台同時接続、SQLite保存）
4. **要求定義作成** — EARSパターンで10件の要求を定義

**重要な決定**:
| 項目 | 決定 |
|------|------|
| 用途 | 受入検査（屋内で確認できる範囲）|
| RTK確認 | 一旦保留（受入検査ではやらない）|
| 同時接続台数 | 2〜5台程度（認知負荷に収まる範囲）|
| データ保存 | SQLite |
| レポート出力 | PDF + CSV |

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [session73/gnss-tool-needs.md](../session73/gnss-tool-needs.md) | 要求定義（EARS記法）|
| [session73/session-summary.md](../session73/session-summary.md) | セッションサマリー |
| [session74/session-plan.md](../session74/session-plan.md) | 次セッション計画 |
| [docs/.../13-ubx-parser-test-scenarios.md](../../docs/missions/m1-sensor-evaluation/gnss/13-ubx-parser-test-scenarios.md) | テストシナリオ正式配置 |

---
