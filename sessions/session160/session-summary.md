# Session 160 サマリー

---
作成: 2026-03-13
---

## 概要

NTRIP + UBXポーリング競合問題の詳細解析とデバッグログ追加

## 実施内容

1. **競合メカニズムの詳細解析**
   - コードリーディングで原因を特定
   - シリアルポートのタイムアウト設定（100ms）を確認
   - RTCM転送直後のUBX送信で20msタイムアウトが発生する問題

2. **デバッグログ追加**
   - gnss_state_api.rs: 各ステップ（drain, send, recv）の所要時間
   - gnss_state_api.rs: ロック取得時間（100ms超で警告）
   - ntrip_api.rs: RTCM書き込み前後の詳細ログ
   - ntrip_api.rs: 50msフラッシュ待機を追加（仮対策）

## 競合メカニズム（解析結果）

| 問題 | 原因 |
|------|------|
| 20msタイムアウト | RTCM転送直後にUBX送信すると、シリアルバッファが処理中 |
| 約50%失敗 | NTRIP転送（1秒間隔）とgnss-state（〜6秒）のタイミング競合 |

## 変更ファイル

| ファイル | 内容 |
|----------|------|
| [gnss_state_api.rs](../../prototype/m1-gnss/backend/src/web/gnss_state_api.rs) | デバッグログ追加 |
| [ntrip_api.rs](../../prototype/m1-gnss/backend/src/web/ntrip_api.rs) | デバッグログ + 50msフラッシュ待機 |

## 正しいMakeコマンド一覧

| 用途 | コマンド |
|------|----------|
| バックエンド起動 | `make dev-backend` |
| フロントエンド起動 | `make dev-frontend` |
| RTKテスト一式 | `make rtk-start` → `make rtk-poll` → `make rtk-stop` |
| ログ監視 | `make rtk-log` |
| 装置接続 | `make connect` |
| NTRIP接続 | `make ntrip-connect`（要 ntrip.conf） |

## 残課題

- 実機テストでデバッグログ確認
- 50msフラッシュ待機の効果検証
- 効果がなければ追加対策の検討

## 次セッション（Session 161）でやること

1. 実機テストで競合の再現・ログ確認
2. 50msフラッシュ待機の効果検証
3. 効果があれば本対策として採用、なければ別アプローチ検討

---

*計画作成: 2026-03-13*
