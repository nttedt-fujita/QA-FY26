# M1-GNSS プロトタイプ

## 概要

ZED-F9P GNSS受信機の評価ツール。屋内検査（PGA確認）・屋外検査（衛星捕捉確認）を行う。

## 仕様書参照ルール（必須）

**IMPORTANT**: UBXメッセージの実装・修正時は、以下を必ず確認すること。

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
