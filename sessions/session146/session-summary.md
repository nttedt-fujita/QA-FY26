# Session 146 サマリー

**日付**: 2026-03-12
**目的**: 定期出力無効化後の動作確認

---

## 実施内容

### 1. ログファイル出力を設定

- `env_logger` → `tracing` + `tracing-subscriber` + `tracing-appender` に変更
- ログ出力先: `logs/gnss-backend.log.YYYY-MM-DD`（日次ローテーション）
- コンソールとファイル両方に出力

### 2. 動作確認で問題発見

屋内検査後に屋外検査を実行したところ、統合APIが断続的に500エラー。

**原因**: NMEAデータがUBXポーリングを妨害
- 屋内検査終了時にNMEA ONに戻している
- 屋外検査ではNMEA ONのまま → UBXポーリングが不安定

### 3. 修正

**デバイス接続時にNMEA出力もOFFにする**

- `send_disable_nmea_output`関数を追加
- 接続処理で定期出力無効化の後にNMEA出力も無効化

---

## 変更ファイル

| ファイル | 内容 |
|----------|------|
| [Cargo.toml](../../prototype/m1-gnss/backend/Cargo.toml) | tracing関連クレート追加 |
| [main.rs](../../prototype/m1-gnss/backend/src/main.rs) | ログ初期化をtracing-subscriberに変更 |
| [device_api.rs](../../prototype/m1-gnss/backend/src/web/device_api.rs) | NMEA出力無効化を追加 |
| [manager.rs](../../prototype/m1-gnss/backend/src/device/manager.rs) | log→tracing置き換え |
| [engine.rs](../../prototype/m1-gnss/backend/src/inspection/engine.rs) | log→tracing置き換え |
| [ntrip_api.rs](../../prototype/m1-gnss/backend/src/web/ntrip_api.rs) | log→tracing置き換え |
| [nav_status_api.rs](../../prototype/m1-gnss/backend/src/web/nav_status_api.rs) | log→tracing置き換え |

---

## 残タスク

- 実機での動作確認（NMEA OFF修正後）
- ADR-012の更新

---

## 次セッション（Session 147）でやること

1. バックエンド再起動後の動作確認（NMEA OFF修正後）
2. 問題がなければADR-012を更新
3. 問題があれば修正

---

*作成: 2026-03-12 Session 146終了時*
