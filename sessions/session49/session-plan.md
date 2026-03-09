# Session 49 計画

**日時**: 2026-03-09（予定）
**前提**: Session 48で方針決定完了

---

## 目的

1. ロット一覧画面の追加
2. アーキテクチャドキュメント作成

---

## タスク

### 1. ロット一覧画面の追加（優先）

**作業内容**:
| 作業 | 詳細 |
|------|------|
| `/lots` ページ作成 | 既存の `/records` をベースにコピー＆修正 |
| API呼び出し | `GET /api/v1/lots` を使用（実装済み） |
| テーブル表示 | ロットID、部品名、サプライヤー、数量、入荷日 |
| ナビゲーション更新 | ヘッダーに「ロット一覧」リンク追加 |
| （オプション）フィルター | 部品名、サプライヤーで絞り込み |

**見積もり**: 1セッション内で完結可能

### 2. アーキテクチャドキュメント作成

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| `prototype/docs/ARCHITECTURE.md` | 全体アーキテクチャ |

**含めるべき内容**:
- 技術スタック（Go, Next.js, PostgreSQL, Docker）
- コンポーネント構成図
- ディレクトリ構造
- 技術選定の経緯（Session 34の決定を参照）
- データフロー
- 開発環境・本番環境の違い
- 今後の拡張方針

### 3. demo-guide.md のリンク修正

**修正箇所**:
- `hearing-items.md` のパス修正
  - 誤: `../../docs/missions/m3-incoming-inspection-db/hearing-items.md`
  - 正: `../../docs/missions/m3-incoming-inspection-db/hearing/hearing-items.md`

---

## 優先順位

1. ロット一覧画面（具体的な成果物）
2. アーキテクチャドキュメント（引き継ぎ準備）
3. demo-guide.md修正（軽微）

---

## 参照資料

- [Session 48 サマリー](../session48/session-summary.md)
- [records/page.tsx](../../prototype/frontend/src/app/records/page.tsx) — コピー元
- [Session 34 技術選定](../session-history/session-031-040.md#session-34) — Go採用の経緯
- [architecture-concerns.md](../../prototype/docs/architecture-concerns.md) — 現在の懸念点
