# Session 139 サマリー

**日付**: 2026-03-12
**目的**: 実機でGGA正式実装・IOエラーの動作確認

---

## 実施内容

### 1. API並行リクエスト問題の調査

藤田さんがログを貼り付けて問題を指摘:
- `/api/nav-status`、`/api/nav-sig`、`/api/nav-sat`、`/api/mon-span` が同時にポーリング
- 結果: 500エラーが頻発、パースエラー多発

### 2. 原因特定

各フロントエンドパネルが**独立したsetInterval**でポーリング:
- `NavStatusPanel`: 1秒ごと
- `NavSigPanel`: 1秒ごと
- `MonSpanPanel`: 5秒ごと

シリアルポートは1つの通信チャネルなので、複数APIが同時にポーリングすると:
- リクエストが混在
- 応答も混在（別のメッセージを受信）
- パースエラー → 500エラー

### 3. 解決策の検討

| 方式 | 概要 |
|------|------|
| A. FE側でシーケンシャル化 | 1つのAPIコールが終わってから次を呼ぶ |
| B. BE側で統合API | 1回のAPIで複数UBXメッセージを取得 |
| C. **Periodic Output（定期出力）** | F9P側が定期的にメッセージを送る（推奨） |

### 4. 結論: Periodic Output が正しいアプローチ

u-blox仕様書から:
- 各メッセージは「Periodic/polled」タイプ
- CFG-MSGOUT設定で「毎ナビゲーションエポックで出力」と設定可能
- F9P側が自動的にメッセージを送ってくれる → ポーリング不要
- メッセージはiTOW（GPSタイムスタンプ）で同期される

**エビデンス**:
- u-blox Interface Description (UBX-22008968) p.150-154: NAV-SAT, NAV-SIG は「Periodic/polled」
- Web検索: "If the rate of this message is set to one (1), it will be output for every navigation epoch"

---

## 決定事項

| 項目 | 決定 |
|------|------|
| API並行問題の解決策 | Periodic Output（定期出力）を採用 |
| 次セッション | エビデンスをきちんと確認してから実装 |

---

## 次セッション（Session 140）でやること

1. u-blox仕様書でPeriodic Outputのエビデンスを詳細確認
   - CFG-MSGOUT設定の仕様確認
   - 設定方法（UBX-CFG-VALSET）の確認
2. Periodic Output実装の設計
3. 実装・動作確認

---

## 参照

- [08-ubx-protocol-index.md](../../docs/missions/m1-sensor-evaluation/gnss/08-ubx-protocol-index.md) - UBXプロトコル索引
- [24-ubx-spec-extract.md](../../docs/missions/m1-sensor-evaluation/gnss/24-ubx-spec-extract.md) - 仕様書抽出
- u-blox Interface Description (UBX-22008968) p.234: CFG-MSGOUT

---

*作成: 2026-03-12 Session 139終了時*
