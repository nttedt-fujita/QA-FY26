# Session 140 サマリー

**日付**: 2026-03-12
**目的**: Periodic Output（定期出力）のエビデンス確認と設計・実装

---

## 実施内容

### 1. u-blox仕様書のエビデンス確認

- Pythonスクリプトで PDF から CFG-MSGOUT, CFG-VALSET の仕様を抽出
- 各メッセージの USB用設定キーを特定

### 2. 設計ドキュメント作成

- [periodic-output-design.md](periodic-output-design.md) を作成
- 統合APIと受信スレッドの選択肢を比較

### 3. 実装

- `cfg_valset.rs` に CFG-MSGOUT キーID と `set_periodic_output()` を追加
- `device_api.rs` に接続時の `enable_periodic_output()` を追加
- テスト4件追加、全10件パス

### 4. 方針決定

- **統合API方式を採用**（受信スレッドは複雑なため見送り）
- ADR-012 を作成して決定を記録

---

## 主な発見

### 受信スレッド実装の複雑さ

1. シリアルポートの排他制御（Mutex設計）
2. NTRIP RTCMライターとの共存問題
3. 読み書き分離（serialportクレートは非対応）

→ 統合API方式でシンプルに解決

---

## 成果物

| ファイル | 内容 |
|----------|------|
| [periodic-output-design.md](periodic-output-design.md) | 設計ドキュメント |
| [cfg_valset.rs](../../prototype/m1-gnss/backend/src/ubx/cfg_valset.rs) | CFG-VALSET生成（定期出力設定追加） |
| [device_api.rs](../../prototype/m1-gnss/backend/src/web/device_api.rs) | 接続時に定期出力有効化 |
| [ADR-012](../../docs/adr/m1/ADR-012-periodic-output-and-unified-api.md) | 定期出力と統合API採用の決定 |
| [32-cfg-msgout-periodic-output.md](../../docs/missions/m1-sensor-evaluation/gnss/32-cfg-msgout-periodic-output.md) | CFG-MSGOUT仕様ドキュメント |

---

## 残タスク

1. 統合API (`GET /api/gnss-state`) 実装
2. FE側のポーリング集約
3. TDDスキルでコードレビュー
4. 定期出力の概念解説（藤田さんリクエスト）

---

## 次セッション（Session 141）でやること

1. 定期出力の概念解説
2. 統合API実装
3. TDDスキルでコードレビュー
