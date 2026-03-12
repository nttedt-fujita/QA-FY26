# ADR-012: 定期出力(Periodic Output)と統合API採用

**ステータス**: 承認済み
**日付**: 2026-03-12
**関連セッション**: Session 139, 140

---

## コンテキスト

Session 139で、複数のAPIが同時にF9Pをポーリングすることで発生する問題が特定された:

1. 各FEパネルが独立したsetIntervalでポーリング（1秒〜5秒ごと）
2. シリアルポートは1チャネル → 複数ポーリングで応答が混在
3. パースエラー多発 → 500エラー

## 検討した選択肢

### 1. FE側でシーケンシャル化

FEでポーリングを順次実行する。

- メリット: BE変更なし
- デメリット: FE側の複雑化、状態管理が分散

### 2. 受信スレッド + キャッシュ

F9Pの定期出力をバックグラウンドスレッドで受信し、最新状態をキャッシュ。

- メリット: 理想的なアーキテクチャ
- デメリット:
  - シリアルポートの排他制御が複雑（Mutex設計）
  - NTRIP RTCMライターとの共存問題
  - 読み書き分離の必要性（serialportクレートは非対応）

### 3. 統合API（採用）

全データを1回のAPIで順次取得。

- メリット:
  - 既存のポーリングコードを流用可能
  - 順次実行するので競合なし
  - 実装が簡単
- デメリット:
  - 1回のAPIで複数メッセージをポーリング → 応答時間増加

## 決定

**統合API方式を採用する。**

1. 接続時にCFG-VALSETで定期出力を有効化（実装済み）
2. `GET /api/gnss-state` を新設し、全データを1回で取得
3. FE側のポーリングを統合APIに集約

## 実装内容（Session 140）

### 完了

- CFG-VALSET生成関数 (`set_periodic_output`)
- 接続時に定期出力を有効化 (`enable_periodic_output`)
- CFG-MSGOUTキーID定義

### 次回（Session 141）

- 統合API (`GET /api/gnss-state`) 実装
- FE側のポーリング集約
- TDDスキルでコードレビュー

## 技術詳細

### CFG-MSGOUT設定キー

| メッセージ | キー名 | Key ID |
|-----------|--------|--------|
| NAV-PVT | CFG-MSGOUT-UBX_NAV_PVT_USB | 0x20910009 |
| NAV-STATUS | CFG-MSGOUT-UBX_NAV_STATUS_USB | 0x2091001d |
| NAV-SAT | CFG-MSGOUT-UBX_NAV_SAT_USB | 0x20910018 |
| NAV-SIG | CFG-MSGOUT-UBX_NAV_SIG_USB | 0x20910348 |
| MON-SPAN | CFG-MSGOUT-UBX_MON_SPAN_USB | 0x2091038e |
| MON-RF | CFG-MSGOUT-UBX_MON_RF_USB | 0x2091035c |

出典: u-blox F9 HPG 1.32 Interface Description (UBX-22008968) p.234-251

## 変更履歴

| 日付 | 変更内容 | 関連セッション |
|------|----------|---------------|
| 2026-03-12 | 初版作成 | Session 140 |
