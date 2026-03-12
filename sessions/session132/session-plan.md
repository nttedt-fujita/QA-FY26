# Session 132 計画

**目的**: NTRIP認証設定画面の実装

**前提**: Session 131で残タスク整理完了。NTRIP実装が最優先。

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | NTRIP仕様の確認 | docs/missions/m1-sensor-evaluation/gnss/20-ntrip-rtk-implementation.md, 21-ntrip-protocol-spec.md |
| 2 | 既存の設定画面の確認 | prototype/m1-gnss/frontend/src/app/ 配下 |
| 3 | 設定画面の設計 | - |
| 4 | 設定画面の実装 | - |

---

## 詳細

### 1. NTRIP仕様の確認

以下のドキュメントを読んで、設定画面に必要な項目を洗い出す:

- [20-ntrip-rtk-implementation.md](../../docs/missions/m1-sensor-evaluation/gnss/20-ntrip-rtk-implementation.md) — 実装方針
- [21-ntrip-protocol-spec.md](../../docs/missions/m1-sensor-evaluation/gnss/21-ntrip-protocol-spec.md) — プロトコル仕様
- [22-rtk-configuration.md](../../docs/missions/m1-sensor-evaluation/gnss/22-rtk-configuration.md) — ZED-F9P RTK設定

**確認すべき項目**:
- NTRIPサーバーURL
- マウントポイント
- ユーザー名/パスワード
- その他必要なパラメータ

### 2. 既存の設定画面の確認

既存のUI実装パターンを確認:

- `/devices` 画面のフォーム
- `/inspections/indoor` 画面のフォーム

### 3. 設定画面の設計

- 入力項目の決定
- UIモックアップ（テキストベース）
- バリデーションルール

### 4. 設定画面の実装

- `/settings/ntrip` ページ作成
- フォームコンポーネント
- ローカルストレージ or DB保存

---

## 残タスク一覧（Session 131で整理）

| # | タスク | 優先度 | 状態 |
|---|--------|--------|------|
| 1 | NTRIP認証設定画面 | 高 | ← 今回 |
| 2 | NTRIPクライアント実装 | 高 | 次回以降 |
| 3 | device_id紐付け実装 | 中 | 後回し |
| 4 | 自動保存に変更 | 中 | 後回し |
| 5 | u-center照合 | 低 | 後回し |

---

## 参照

- [Session 131 summary](../session131/session-summary.md)
- [gnss/README.md チェックリスト](../../docs/missions/m1-sensor-evaluation/gnss/README.md)

---

*計画作成: 2026-03-12 Session 131終了時*
