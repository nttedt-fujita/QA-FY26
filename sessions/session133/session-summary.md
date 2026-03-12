# Session 133 サマリー

**日時**: 2026-03-12
**目的**: NTRIPクライアント実装

---

## 実施内容

### 1. ntrip-clientクレート調査・不採用決定

- nav-solutions/ntrip-client (v0.1.1) を調査
- **問題発覚**: RTCMパース済みデータ（`rtcm_rs::Message`）を返す
- ZED-F9Pには**生のRTCMバイナリ**を転送する必要があるため不採用
- ADR-011を作成して判断を記録

### 2. NTRIP API独自実装

- `ntrip_api.rs` を新規作成
- TCP + HTTP/1.0ベースのNTRIP Rev1プロトコルを直接実装
- NTRIP仕様書（21-ntrip-protocol-spec.md）に準拠

**実装したエンドポイント**:
| エンドポイント | 機能 |
|---------------|------|
| POST /api/ntrip/connect | NTRIP接続開始 |
| POST /api/ntrip/disconnect | NTRIP切断 |
| GET /api/ntrip/status | 接続状態取得 |

### 3. DeviceManagerにwrite_data追加

- RTCM転送用の`write_data`メソッドを追加
- 既存の`send_ubx`と異なり、書き込んだバイト数を返す

### 4. TDDスキルでレビュー

- 実装後にTDDスキルでレビューを実施
- **指摘事項**:
  - APIハンドラーのテストが欠落
  - テーブルテスト形式を使っていない（rules/06-test-style.md違反）
  - TDD（テスト先行）ではなく実装先行だった

---

## 作成ファイル

| ファイル | 内容 |
|----------|------|
| [ntrip_api.rs](../../prototype/m1-gnss/backend/src/web/ntrip_api.rs) | NTRIP API実装 |
| [ADR-011-ntrip-implementation.md](../../docs/adr/m1/ADR-011-ntrip-implementation.md) | ntrip-client不採用のADR |

---

## ルール遵守状況

### 従えた点
- 推測禁止（NTRIP仕様書を読んでから実装）
- 設計判断はADR作成（ADR-011）
- 探索→計画→実装のサイクル

### 従えなかった点
- テーブルテスト形式を使っていない
- TDD（テスト先行）ではなく実装先行
- ntrip-clientクレートのAPI確認が不十分だった（ソースコード確認前に使おうとした）

---

## 残タスク

1. テストをテーブルテスト形式に書き直す
2. APIハンドラーのテスト追加
3. フロントエンドに接続ボタン追加

---

## 次セッションでやること

- フロントエンドに接続/切断ボタン追加
- テストの改善（テーブルテスト形式への書き直し）

---

*作成: Session 133 終了時*
