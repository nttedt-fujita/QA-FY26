# Session 35 サマリー

**日付**: 2026-03-06
**目的**: プロトタイプ環境構築（Go + Next.js）

---

## 実施内容

| 項目 | 状態 | 詳細 |
|------|------|------|
| AWSコスト試算確認 | ✅ | プロトタイプ: Lightsail $5/月、本番: ~$25/月 |
| prototypeディレクトリ作成 | ✅ | backend/, frontend/, db/ |
| DB設計（叩き台） | ✅ | schema.sql 作成 |
| Goバックエンド初期化 | ✅ | go mod init, 最小HTTPサーバー |
| Next.jsフロントエンド初期化 | ✅ | create-next-app（TypeScript, Tailwind） |

---

## 作成ファイル

| ファイル | 内容 |
|----------|------|
| [prototype/db/schema.sql](../../prototype/db/schema.sql) | DB設計（To-Beモデル準拠） |
| [prototype/backend/go.mod](../../prototype/backend/go.mod) | Goモジュール定義 |
| [prototype/backend/cmd/api/main.go](../../prototype/backend/cmd/api/main.go) | 最小HTTPサーバー |
| [prototype/frontend/](../../prototype/frontend/) | Next.jsプロジェクト |

---

## 残課題

- **テストコードなし**: TDD Phase 0まで確認したが、実装は次セッションへ
- **「3倍」記述の根拠問題**: スライドに書いた「西村→杉山で工数3倍」は分析不十分。ユーザー側で削除対応

---

## 次セッション（Session 36）でやること

1. TDD Phase 0-2: テストシナリオ設計（ロットCRUD）
2. TDD Phase 3-4: テスト → 実装（Red → Green）
3. SQLite接続実装
