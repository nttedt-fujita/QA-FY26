# NTRIP + UBXポーリング テストレポート

---
作成: Session 159 (2026-03-13)
---

## テスト概要

NTRIP接続中にgnss-state API（UBXポーリング）が正常動作するかを検証。

## テスト環境

- バックエンド: m1-gnss (Rust/Actix-web)
- デバイス: ZED-F9P (USB接続)
- NTRIPキャスター: d-gnss.jp:2101/RRSGD
- ネットワーク: モバイル回線（会社ネットワークはポート2101ブロック）

## テスト手順

1. `make connect` - デバイス接続
2. `make ntrip-connect` - NTRIP接続
3. `make rtk-poll` - gnss-state 5回ポーリング

## テスト結果

| 回 | 結果 | 応答時間 | エラー数 |
|----|------|----------|----------|
| 1 | ✅ 成功 | 6111ms | 0 |
| 2 | ❌ 失敗 | 309ms | 6 |
| 3 | ✅ 成功 | 5615ms | 0 |
| 4 | ❌ 失敗 | 295ms | 6 |
| 5 | ✅ 成功 | 5635ms | 0 |

**パターン**: 成功 → 失敗 → 成功 → 失敗 → 成功（交互）

## 失敗時のログ（抜粋）

```
01:36:03.604721Z [NTRIP-RTCM] ロック取得開始...
01:36:03.604822Z [NTRIP-RTCM] ロック取得成功 (0ms)
01:36:03.637388Z [GNSS-STATE] API呼び出し開始
01:36:03.637443Z [GNSS-STATE] DeviceManagerロック取得開始...
01:36:03.804027Z [NTRIP-RTCM] 転送完了: 874 bytes (199ms)
01:36:03.804214Z [GNSS-STATE] ロック取得成功 (166ms)
01:36:03.824596Z NMEA OFF送信失敗（続行）: IOエラー: Operation timed out
01:36:03.844990Z [GNSS-STATE] NAV-PVT失敗: 送信エラー - IOエラー: Operation timed out (20ms)
（以下、6メッセージ全て失敗）
```

## 観察された事象

1. **RTCM転送時間**: 874 bytes転送に199ms
2. **gnss-stateロック待機**: RTCM転送完了まで166ms待機
3. **失敗時のタイムアウト**: 20msで送信エラー（通常は2000ms）
4. **成功時のRTCMロック待機**: gnss-state完了まで5500ms以上待機

## 成功時のログ（抜粋）

```
01:35:57.021515Z [NTRIP-RTCM] ロック取得開始...
（gnss-stateがロック保持中のため待機）
01:36:02.609256Z [NTRIP-RTCM] ロック取得成功 (5587ms)
01:36:02.609335Z [NTRIP-RTCM] 転送完了: 236 bytes (0ms)
```

## 結論

- NTRIP接続中にgnss-state APIを呼び出すと、**約50%の確率で失敗**する
- 失敗はRTCM転送とgnss-stateのタイミング競合によるもの
- これが「NTRIP接続後に屋外検査が失敗する」問題の原因

## 次のアクション

- 競合の詳細メカニズム解析
- 対策の検討（排他制御、リトライ、NTRIP一時停止など）
