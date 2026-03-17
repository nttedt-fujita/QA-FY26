# M1-GNSS プロトタイプ

## 概要

ZED-F9P GNSS受信機の評価ツール。屋内検査（PGA確認）・屋外検査（衛星捕捉確認）を行う。

## 仕様書参照ルール（必須）

**IMPORTANT**: UBXメッセージの実装・修正時は、以下を必ず確認すること。

### 0. まず確認

**[gnss/README.md](../../docs/missions/m1-sensor-evaluation/gnss/README.md)** の「○○するときは△△を見る」チェックリストを確認すること。

### 1. 実装前に読むべきファイル

| 対象 | 仕様書 |
|------|--------|
| MON-SPAN | [ubx-mon-messages.md](../../docs/missions/m1-sensor-evaluation/gnss/ubx-mon-messages.md)、[23-mon-span-implementation.md](../../docs/missions/m1-sensor-evaluation/gnss/23-mon-span-implementation.md) |
| MON-RF | [ubx-mon-messages.md](../../docs/missions/m1-sensor-evaluation/gnss/ubx-mon-messages.md) |
| NAV-SIG | [ubx-nav-messages.md](../../docs/missions/m1-sensor-evaluation/gnss/ubx-nav-messages.md)、[25-nav-sig-behavior-spec.md](../../docs/missions/m1-sensor-evaluation/gnss/25-nav-sig-behavior-spec.md) |
| NAV-SAT | [ubx-nav-messages.md](../../docs/missions/m1-sensor-evaluation/gnss/ubx-nav-messages.md) |
| NAV-STATUS/TTFF | [30-ttff-monrf-spec.md](../../docs/missions/m1-sensor-evaluation/gnss/30-ttff-monrf-spec.md) |
| NTRIP/RTK | [20-ntrip-rtk-implementation.md](../../docs/missions/m1-sensor-evaluation/gnss/20-ntrip-rtk-implementation.md) |
| 屋外検査 | [26-outdoor-inspection-domain-model.md](../../docs/missions/m1-sensor-evaluation/gnss/26-outdoor-inspection-domain-model.md) |

### 2. 確認プロセス

1. 仕様書を読む
2. ペイロード構造・フィールドオフセットを確認
3. 既存パーサーとの整合性を確認
4. テストケースが仕様をカバーしていることを確認

### 3. 禁止事項

- 仕様書を読まずに「既存コードと同じパターン」だけで実装すること
- 読んだファイルを明記しないこと

## ディレクトリ構成

```
prototype/m1-gnss/
├── backend/          # Rust (Actix-web)
│   └── src/
│       ├── ubx/      # UBXパーサー（MON-SPAN, MON-RF, NAV-SIG等）
│       └── web/      # APIハンドラー
├── frontend/         # Next.js
│   └── src/
│       └── components/  # UI（NavSigPanel, MonSpanPanel等）
└── docs/             # プロトタイプ固有ドキュメント
```

## 技術スタック

- バックエンド: Rust + Actix-web
- フロントエンド: Next.js + TypeScript
- シリアル通信: serialport crate

## 関連ADR

- [ADR-004](../../docs/adr/m1/ADR-004-gnss-tool-approach.md) - アプローチ選択
- [ADR-005](../../docs/adr/m1/ADR-005-gnss-tool-tech-stack.md) - 技術スタック選定
- [ADR-007](../../docs/adr/m1/ADR-007-baud-rate-detection.md) - ボーレート自動検出
- [ADR-008](../../docs/adr/m1/ADR-008-outdoor-inspection-criteria.md) - 屋外検査合格基準
- [ADR-009](../../docs/adr/m1/ADR-009-outdoor-inspection-fe-aggregation.md) - 屋外検査FE集計

## プロトタイプ固有ドキュメント

- [docs/nav-sig-api-design.md](docs/nav-sig-api-design.md) - NAV-SIG API設計

## コマンドリファレンス

**IMPORTANT**: コマンドを実行する前に、必ず `make help` または api.mk を確認すること。推測でコマンドを作成しない。

詳細: [makefiles/api.mk](makefiles/api.mk)

**デバイス指定**: デフォルトは `/dev/ttyUSB0`。変更する場合は `DEVICE=/dev/ttyUSB1` を付ける。
```bash
make connect DEVICE=/dev/ttyUSB1
make message-scan DEVICE=/dev/ttyUSB1
```

### 基本コマンド

| 用途 | コマンド | 備考 |
|------|----------|------|
| ヘルプ表示 | `make help` | 全コマンド一覧 |
| バックエンド起動 | `make dev-backend` | RUST_LOG=debug付き |
| フロントエンド起動 | `make dev-frontend` | Next.js dev |

### APIデバッグ（makefiles/api.mk）

| 用途 | コマンド | メソッド |
|------|----------|----------|
| 装置一覧 | `make devices` | GET |
| 装置接続 | `make connect` | POST |
| 装置切断 | `make disconnect` | POST |
| GNSS状態 | `make gnss-state` | GET |
| message-scan | `make message-scan` | GET |
| reset-config | `make reset-config` | POST |
| ヘルスチェック | `make health` | GET |

### RTK関連

| 用途 | コマンド | メソッド |
|------|----------|----------|
| RTKデバッグ一括 | `make rtk-debug` | POST+GET |
| NTRIP接続 | `make rtk-connect` | POST |
| NTRIP切断 | `make rtk-disconnect` | POST |
| RTKポーリング | `make rtk-poll` | GET |

### ロット・検査

| 用途 | コマンド | メソッド |
|------|----------|----------|
| ロット一覧 | `make lots` | GET |
| ロット作成 | `make create-lot` | POST |
| 検査実行 | `make inspect LOT_ID=1` | POST |
| 検査履歴 | `make inspections` | GET |
