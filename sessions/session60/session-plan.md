# Session 60 計画

**日時**: 2026-03-XX（予定）
**前提**: Session 59で技術選定比較ドキュメント作成完了

---

## 目的

GNSS評価ツールの技術選定最終決定と実装開始

---

## タスク

### 1. 技術選定の最終決定

**確認事項**:
- 案A（Rust + Actix-web + 自前UBXパース）で進めるか
- 小板橋さんへの確認（実機借用タイミング等）

**成果物**:
- ADR-005（GNSS評価ツール技術選定）

### 2. Rust + Actix-web環境構築

- `tools/gnss-eval/` にRustプロジェクト作成
- Cargo.toml設定（actix-web, serialport, serde等）
- 基本的なHTTPサーバー動作確認

### 3. UBXパース設計

**必要なメッセージ**:
| メッセージ | Class | ID | 用途 |
|-----------|-------|-----|------|
| NAV-SIG | 0x01 | 0x43 | L1/L2別C/N0 |
| NAV-SAT | 0x01 | 0x35 | 衛星方位角・仰角 |
| NAV-PVT | 0x01 | 0x07 | 位置・RTK状態 |
| NAV-STATUS | 0x01 | 0x03 | TTFF |
| MON-SPAN | 0x0A | 0x31 | スペクトラム |

**参照**:
- [08-ubx-protocol-index.md](../../docs/missions/m1-sensor-evaluation/gnss/08-ubx-protocol-index.md)

### 4. モック実装開始

- 実機なしで開発できる部分から開始
- UBXメッセージのパース処理（テストデータで検証）
- HTTPエンドポイント設計

---

## 参照資料

- [Session 59サマリー](../session59/session-summary.md)
- [技術選定比較](../session59/gnss-tool-tech-comparison.md)
- [ADR-004](../../docs/adr/ADR-004-gnss-tool-approach.md) — 方針決定（直接UBX通信）
