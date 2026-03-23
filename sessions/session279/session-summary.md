# Session 279 サマリー

**日付**: 2026-03-23（月曜日）

---

## 実施内容

### 1. プロジェクト管理手法のWeb調査

以下のトピックについて調査:
- SSOT（Single Source of Truth）の原則
- Issue Tracker + Documentation の使い分け
- Living Documentation / Docs as Code
- コンテキストスイッチングの軽減

### 2. 既存ルールとの整合性確認

- `14-document-management.md`, `07-information-routing.md` を確認
- Living Documentation の考え方は既に採用済みと確認
- 不足: SSOTの明確化、progress.mdの更新ルール

### 3. 改善方針の策定

- progress.md を「ハブ」として再定義
- docs/index.md は廃止
- セッション終了時にprogress.md更新を必須化

### 4. 作業計画の作成

Session 280〜284 の5セッションでdocs/断捨離を実施する計画を作成。

---

## 成果物

| ファイル | 内容 |
|----------|------|
| [progress-management-research.md](progress-management-research.md) | 調査結果まとめ |
| [progress-management-policy.md](../../docs/progress-management-policy.md) | 進捗管理方針（正式ドキュメント） |
| [docs-cleanup-plan.md](docs-cleanup-plan.md) | 断捨離作業計画（マイルストーン付き） |

---

## 主な決定事項

| 決定 | 内容 |
|------|------|
| SSOTの明確化 | Linear=タスク状態、session-history=記録、progress.md=ハブ |
| progress.md | ハブ形式に書き換え（リンクのみ、説明文なし） |
| docs/index.md | 廃止 |

---

## 次セッション（280）でやること

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | 各ミッションの最新状況を確認 | session-history, 各README.md |
| 2 | Linear issueのリンクを収集 | Linear API |
| 3 | progress.md を書き換え | docs/progress.md |
| 4 | docs/index.md を削除 | - |

---

## 参照

- [docs-cleanup-plan.md](docs-cleanup-plan.md) — 作業計画（全体）
- [progress-management-policy.md](../../docs/progress-management-policy.md) — 方針
