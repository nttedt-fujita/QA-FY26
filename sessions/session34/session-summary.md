# Session 34 サマリー

**日付**: 2026-03-06
**目的**: ダッシュボード確認・技術選定見直し

---

## 実施内容

### 1. ダッシュボードのエラー修正

Phase 1プロトタイプ（Streamlitダッシュボード）にエラーがあったため修正:

- **原因**: CSVのカラム名（`品名`、`検査内容`）とコードの参照名（`品名_正規化`、`検査内容_正規化`）が不一致
- **修正**: dashboard.pyの参照カラム名を実際のCSVに合わせて修正

### 2. ダッシュボード動作確認

http://localhost:8501 で正常に起動・表示を確認。

### 3. 技術選定の見直し

Session 8の別セッション資料（05_techstack_plan.md）で「TypeScript統一」が選定されていたが、再検討。

**議論のポイント**:
- TypeScript統一の理由が「認知負荷最小化」だけでは弱い
- GoはパフォーマンスMS依存関係・脆弱性リスクで有利
- M3/M4の規模では「パフォーマンス差は無意味」は事実だが、Goを選ばない積極的な理由にはならない

**決定**: バックエンドを**Goに変更**

### 4. プロトタイプのスコープ確認

Session 32で確定済みのスコープを再確認:

| Phase | 内容 | 状態 |
|-------|------|------|
| Phase 1 | 分析ダッシュボード | ✅完成 |
| Phase 2 | 入力フォーム | 次にやる |
| Phase 3 | M3/M4統合 | 協定書締結後 |

---

## 成果物

| ファイル | 内容 |
|----------|------|
| [dashboard.py](../../tools/incoming_inspection/dashboard.py) | カラム名修正 |

---

## 重要な決定

### 技術スタック変更

| 項目 | 変更前 | 変更後 |
|------|--------|--------|
| バックエンド | TypeScript (Lambda) | **Go** |
| フロントエンド | Next.js + TypeScript | 変更なし |
| DB | RDS PostgreSQL | 変更なし |
| IaC | AWS CDK (TypeScript) | 変更なし |

**変更理由**:
- パフォーマンス: Goが圧倒的に有利
- 依存関係: Goは標準ライブラリ充実、npm依存地獄を回避
- 脆弱性リスク: Goの方が低い
- デプロイ: 単一バイナリでシンプル

---

## 次セッションでやること

1. **プロジェクト構成の確定** — prototype/ ディレクトリの構成を決定
2. **環境構築** — Go バックエンド + Next.js フロントエンドの初期セットアップ
3. **To-Beドメインモデルの確認** — Session 33で作成したモデルをベースにDB設計

---

## 参照資料

- [05_techstack_plan.md](../session8/another-session-Need-requirements-files/05_techstack_plan.md) — 元の技術選定
- [typescript-vs-go-report.md](../session9/typescript-vs-go-report.md) — 言語比較調査
- [prototype-approach.md](../../docs/missions/m3-incoming-inspection-db/to-be/prototype-approach.md) — プロトタイプスコープ
