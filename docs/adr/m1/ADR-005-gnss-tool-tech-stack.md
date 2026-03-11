# ADR-005: GNSS評価ツール技術スタック選定

**ステータス**: 承認済み
**日付**: 2026-03-09
**最終更新**: 2026-03-10（Session 75でフロントエンド選定を更新）
**関連ADR**: [ADR-004](ADR-004-gnss-tool-approach.md)（直接UBX通信アプローチ採用）

---

## 変更履歴

| 日付 | 変更内容 | 関連セッション |
|------|----------|---------------|
| 2026-03-09 | 初版作成（静的HTML + JavaScript） | Session 59 |
| 2026-03-10 | フロントエンドをNext.js（React）に変更 | Session 61, 75 |

---

## コンテキスト

ADR-004で「直接UBX通信ツール」を採用することが決定した。
本ADRでは、そのツールの技術スタック（言語、フレームワーク、開発環境）を決定する。

### 要求

| # | 要求 | 備考 |
|---|------|------|
| R1 | Webアプリ的なUI | ブラウザで操作可能 |
| R2 | F9Pとシリアル通信 | UBXプロトコル |
| R3 | 衛星スカイプロット表示 | 方位角・仰角をプロット |
| R4 | C/N0（受信感度）表示 | L1/L2別、衛星別 |
| R5 | CSV出力 | 既存Excelワークフローとの互換 |
| R6 | 高いパフォーマンス | リアルタイム更新 |
| R7 | 環境の再現性 | チームメンバーが同じ環境で開発可能 |

---

## 決定

**Rust + Actix-web + 自前UBXパース + Docker（Dev Container）** を採用する。

### 技術スタック

| 項目 | 選択 | 理由 |
|------|------|------|
| 言語 | Rust | パフォーマンス最高、メモリ安全性 |
| Webフレームワーク | Actix-web | 19-20k req/s、成熟 |
| シリアル通信 | serialport crate | 成熟、クロスプラットフォーム |
| UBXパース | 自前実装 | 必要な5-6メッセージのみ、仕様書ベース |
| 開発環境 | Docker + Dev Container | 環境再現性、Rustインストール不要 |
| フロントエンド | Next.js（React） | 表現力が高い、エコシステムが豊富（Session 61で変更）|
| データベース | SQLite | 軽量、単一ファイル、セットアップ不要 |

### ディレクトリ構成

```
prototype/m1-gnss/
├── backend/                # Rustバックエンド
│   ├── .devcontainer/
│   │   └── devcontainer.json
│   ├── Dockerfile.dev
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       ├── ubx/           # UBXパース
│       ├── serial/        # シリアル通信
│       └── web/           # HTTPサーバー
└── frontend/              # Next.js フロントエンド
    ├── package.json
    ├── app/               # App Router
    └── components/
```

---

## 代替案

### 案B: Rust + Actix-web + ubxlib（FFI）

- **メリット**: u-blox公式ライブラリで信頼性最高
- **却下理由**: FFI（C言語連携）が面倒、開発コスト増

### 案C: Go + Gin + 自前UBXパース

- **メリット**: 開発速度が速い、単一バイナリ配布
- **却下理由**: パフォーマンスがRustより劣る

### 案D: Python + FastAPI + pyubx2

- **メリット**: 開発最速、pyubx2が成熟
- **却下理由**: パフォーマンス低、配布が面倒（Python環境必須）

### 案E: C++ + ubxlib + Web

- **メリット**: パフォーマンス最高、公式ライブラリ使用可能
- **却下理由**: 開発コスト非常に高い

---

## 結果

- 高パフォーマンス（19-20k req/s）
- 単一バイナリ配布が可能
- Docker環境で開発環境の再現性を確保
- Rust学習コストは受け入れる（長期的なメリット優先）

---

## 参照

- [Session 59 技術選定比較](../../../sessions/session59/gnss-tool-tech-comparison.md)
- [Session 61 フロントエンド選定](../../../sessions/session61/session-summary.md) — Next.js採用の決定
- [ADR-004 GNSS評価ツールのアプローチ選択](ADR-004-gnss-tool-approach.md)
