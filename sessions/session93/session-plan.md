# Session 93 計画

**目的**: フロントエンド/バックエンド統合開始

---

## 背景

Session 92でボーレート自動検出（ADR-007）を実装し、UIモックアップを作成した。
次は実際のフロントエンド・バックエンド統合を開始する。

---

## やること

### 0. モックアップ色調調整

Session 92で作成したモックアップの色を調整:
- **基調色を1つに統一**（青系）
- トーンの濃淡で画面を区別
- 色数を3色以内に抑える
- 参照: `~/.claude/skills/diagram-design/references/design-principles/06-color-theory.md`

### 1. Next.jsプロジェクト作成

`prototype/m1-gnss/frontend/` に配置:
- App Router構成
- TypeScript
- Tailwind CSS（UIスタイリング）

### 2. Actix-web APIエンドポイント実装

装置接続に必要なAPIから実装:
- `GET /api/devices` - 装置一覧取得
- `POST /api/devices/{path}/connect` - 接続（自動検出）
- `POST /api/devices/{path}/disconnect` - 切断

### 3. 装置接続画面の実装

モックアップ（[ui-mockup-phase1.drawio](../session92/ui-mockup-phase1.drawio)）の②画面を実装:
- スキャンボタン
- 装置カード表示
- 接続/切断ボタン

---

## 完了条件

- [ ] Next.jsプロジェクトが動作する
- [ ] バックエンドAPIが応答する
- [ ] 装置接続画面が表示される（スタブデータでもOK）

---

## 参照資料

- [Session 92 サマリー](../session92/session-summary.md)
- [UIモックアップ](../session92/ui-mockup-phase1.drawio)
- [UI設計書](../session92/ui-design-phase1.md)
- [ADR-005 技術スタック](../../docs/adr/m1/ADR-005-gnss-tool-tech-stack.md)

---

*計画作成: 2026-03-11 Session 92終了時*
