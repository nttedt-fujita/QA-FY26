# Session 157 サマリー

**日付**: 2026-03-13

---

## 実施内容

1. **全二重通信の追加調査**
   - u-blox Interface Description p.270-274を抽出・確認
   - 「full duplex」の直接的記述は見つからなかったが、CFG-UARTxINPROT/OUTPROTが独立設定 → 全二重前提の設計

2. **DeviceManagerのコード確認**
   - `device/manager.rs`: ロック機構なし（SerialPortトレイトのread/write）
   - ロックは`AppState`の`Mutex<DeviceManager>`で実現

3. **NTRIP APIのコード確認**
   - `ntrip_api.rs`: RTCM転送時に`app_state.device_manager.lock()`を取得
   - ロック保持時間: write_dataの間のみ（数ms）
   - 理論上は並行動作可能

4. **要求の再確認**
   - ユーザーの本来の問題: NTRIP接続後に屋外検査すると「ポート使用中」で失敗
   - 今後の方針: バックエンドだけでNTRIP + UBXポーリングをテスト

---

## 作成ファイル

| ファイル | 内容 |
|----------|------|
| [ublox-uart-config.md](ublox-uart-config.md) | u-blox PDF p.270-274抽出（CFG-UART/USB設定） |

---

## 更新ファイル

| ファイル | 内容 |
|----------|------|
| [ntrip-rtk-spec-findings.md](../session156/ntrip-rtk-spec-findings.md) | 調査完了、Session 157の追加調査結果を反映 |

---

## 決定事項

| 項目 | 決定 |
|------|------|
| 次のアプローチ | バックエンドAPIだけでNTRIP + UBXポーリングをテスト |
| ADR-013 | 不要（現時点では設計変更なし、問題特定が先） |

---

## 次セッション（Session 158）でやること

1. **バックエンドAPIテスト用Makefileターゲット作成**
   - NTRIP接続 → UBXポーリング → 状態確認のシーケンス

2. **実機テスト**
   - curlでNTRIP接続
   - curlでUBXポーリング（NAV-PVTなど）
   - エラーが出るか確認

3. **問題切り分け**
   - 再現する → バックエンドの問題を修正
   - 再現しない → FE連携の問題を調査
