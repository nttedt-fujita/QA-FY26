# Session 162 サマリー

---
作成: 2026-03-13
---

## 概要

NTRIP+UBX競合問題の対策検証と、Makeコマンド整理

## 実施内容

1. **ログ分析**
   - 前セッション(161)で追加した50ms安定化待機の効果を検証
   - 結果: 5回中2回失敗（40%失敗率）→ **50msでは不十分**

2. **安定化待機を100msに延長**
   - `gnss_state_api.rs` の `stabilize_delay_ms` を50→100に変更
   - 次セッションで効果検証予定

3. **Makeコマンド整理**
   - `rtk-debug` 新設: デバイス接続 + NTRIP接続 + ポーリングを1コマンドで実行
   - `rtk-connect` 新設: デバイス接続 + NTRIP接続
   - `rtk-disconnect` 新設: NTRIP切断
   - `rtk-status` 新設: NTRIP状態確認
   - 古い `ntrip-xxx` コマンドを削除

## 変更ファイル

| ファイル | 内容 |
|----------|------|
| [gnss_state_api.rs](../../prototype/m1-gnss/backend/src/web/gnss_state_api.rs) | 安定化待機50ms→100ms |
| [api.mk](../../prototype/m1-gnss/makefiles/api.mk) | RTKコマンド整理 |

## 新しいワークフロー

```bash
# ターミナル1: バックエンド起動（ログ表示）
make dev-backend

# ターミナル2: RTKデバッグ一括実行
make rtk-debug

# または個別操作
make rtk-connect    # デバイス接続 + NTRIP接続
make rtk-poll       # ポーリング5回
make rtk-disconnect # 切断
```

## 次セッション（Session 163）でやること

1. `make rtk-debug` で100ms待機の効果検証
2. まだ失敗するなら別アプローチを検討

---

*作成: 2026-03-13*
