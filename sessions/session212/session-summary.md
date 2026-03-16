# Session 212 サマリー

**目的**: CFG-CFGによる設定リセットAPIの実装

**日付**: 2026-03-16

---

## 実施内容

1. **CFG-CFGコマンド実装**（ubx/cfg_cfg.rs）
   - CFG-CFG (0x06 0x09) メッセージ生成機能
   - `reset_config_to_default()`: BBRクリア + ROMデフォルト復元
   - `save_config_to_bbr()`: RAM → BBR保存
   - テスト4件追加（全パス）

2. **設定リセットAPI実装**（web/reset_config_api.rs）
   - `POST /api/devices/{path}/reset-config`
   - CFG-CFG送信 → ACK/NAK確認
   - device_api.rsにルート追加

---

## 結果

- CFG-CFGコマンドとAPIの実装完了
- 実機テストは次セッションへ

---

## 作成・更新ファイル

| ファイル | 内容 |
|----------|------|
| [cfg_cfg.rs](../../prototype/m1-gnss/backend/src/ubx/cfg_cfg.rs) | CFG-CFGメッセージ生成（新規） |
| [reset_config_api.rs](../../prototype/m1-gnss/backend/src/web/reset_config_api.rs) | 設定リセットAPI（新規） |
| [ubx/mod.rs](../../prototype/m1-gnss/backend/src/ubx/mod.rs) | cfg_cfgモジュール追加 |
| [web/mod.rs](../../prototype/m1-gnss/backend/src/web/mod.rs) | reset_config_apiモジュール追加 |
| [device_api.rs](../../prototype/m1-gnss/backend/src/web/device_api.rs) | reset-configルート追加 |

---

## 未対応（次セッションへ）

1. 実機テスト（BBRクリア効果確認）
2. PX4のubx.cpp/ubx.h調査（定期出力設定の裏付け）

---

## メモ

ユーザーからの指摘：PX4（フライトコントローラー）がGNSSレシーバーの定期出力設定を行っている。ubx.cpp/ubx.hを調査することで、どのメッセージが設定されるか分かる可能性がある。

---

## 参照

- [Session 211 summary](../session211/session-summary.md)
- [38-ublox-config-management.md](../../docs/missions/m1-sensor-evaluation/gnss/38-ublox-config-management.md)
