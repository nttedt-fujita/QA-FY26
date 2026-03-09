# Session 58 計画

**日時**: 2026-03-XX（予定）
**前提**: Session 57でPX4調査完了、ADR-004で方針決定済み

---

## 目的

GNSS評価ツールのプロトタイプ作成

---

## タスク

### 1. 技術選定

**検討事項**:
- 言語: Python（pyserial）が有力
- シリアル通信ライブラリの動作確認
- 出力形式: CSV

**決定すべきこと**:
- Python 3.x のバージョン
- 依存ライブラリ（pyserial等）
- プロジェクト構成（src/, tests/等）

### 2. 環境構築

- `tools/gnss-eval/` に開発環境をセットアップ
- pyproject.toml または requirements.txt 作成
- F9Pとの接続確認（実機があれば）

### 3. プロトタイプ設計

**最小構成**:
1. F9Pとシリアル接続
2. UBX-NAV-SIG取得（L1/L2別C/N0）
3. CSV出力

**参照**:
- [08-ubx-protocol-index.md](../../docs/missions/m1-sensor-evaluation/gnss/08-ubx-protocol-index.md) — UBXプロトコル仕様
- [10-tool-design-notes.md](../../docs/missions/m1-sensor-evaluation/gnss/10-tool-design-notes.md) — 設計メモ

### 4. プロトタイプ実装開始

- UBXメッセージのパース処理
- NAV-SIG → C/N0抽出
- CSV書き出し

---

## 残タスク（並行 or 後回し）

- 小板橋さんへ確認（作りながら確認）
- 末永さんへ相談（作りながら確認）
- M4工程不良Excel入手

---

## 参照資料

- [Session 57サマリー](../session57/session-summary.md)
- [ADR-004](../../prototype/docs/adr/ADR-004-gnss-tool-approach.md) — 方針決定
- [tools/gnss-eval/README.md](../../tools/gnss-eval/README.md) — ツール概要
