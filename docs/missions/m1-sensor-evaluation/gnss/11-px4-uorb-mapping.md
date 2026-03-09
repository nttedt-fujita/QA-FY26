# PX4 uORBトピックとUBXメッセージの対応

**作成日**: 2026-03-09
**更新日**: 2026-03-09
**ステータス**: ✅ PX4ソースコード調査完了
**目的**: フライトログ経由でGNSSデータを取得する際の、データ変換経路を整理

---

## 1. 概要

F9PチップからPX4に送られるUBXメッセージが、どのuORBトピックに変換されるかを整理する。
これにより、フライトログ（ULog）から取得可能なデータと、直接UBX通信でないと取得できないデータを明確化する。

---

## 2. データフロー

```
[F9P] --UBX--> [PX4 GPSドライバ] --uORB--> [各モジュール]
                                      |
                                      +---> [ULog（SD保存）]
                                      +---> [MAVLink（GCS送信）]
```

---

## 3. 主要uORBトピック

### 3-1. sensor_gps

**説明**: GPSセンサーからの生データ

| フィールド | 型 | UBXメッセージ | 備考 |
|-----------|-----|--------------|------|
| lat | int32 | UBX-NAV-PVT | 緯度（1e-7 deg） |
| lon | int32 | UBX-NAV-PVT | 経度（1e-7 deg） |
| alt | int32 | UBX-NAV-PVT | 高度（mm） |
| alt_ellipsoid | int32 | UBX-NAV-PVT | 楕円体高度（mm） |
| s_variance_m_s | float | UBX-NAV-PVT | 速度分散 |
| c_variance_rad | float | — | コース分散 |
| fix_type | uint8 | UBX-NAV-PVT | 0=NoFix, 3=3D, 4=DGPS, 5=RTK Float, 6=RTK Fixed |
| eph | float | UBX-NAV-PVT | 水平精度（hAcc）（m） |
| epv | float | UBX-NAV-PVT | 垂直精度（vAcc）（m） |
| hdop | float | UBX-NAV-PVT | HDOP |
| vdop | float | UBX-NAV-PVT | VDOP |
| noise_per_ms | int32 | UBX-MON-RF | **要確認** |
| jamming_indicator | uint16 | UBX-MON-RF | jammingState |
| jamming_state | uint8 | UBX-MON-RF | 0=unknown, 1=ok, 2=warning, 3=critical |
| vel_n_m_s | float | UBX-NAV-PVT | 北方向速度 |
| vel_e_m_s | float | UBX-NAV-PVT | 東方向速度 |
| vel_d_m_s | float | UBX-NAV-PVT | 下方向速度 |
| cog_rad | float | UBX-NAV-PVT | Course over ground |
| satellites_used | uint8 | UBX-NAV-PVT | numSV |

### 3-2. vehicle_gps_position（廃止予定？）

PX4のバージョンによっては`sensor_gps`に統合されている可能性あり。**要確認**。

### 3-3. satellite_info

**説明**: 衛星ごとの詳細情報

| フィールド | 型 | UBXメッセージ | 備考 |
|-----------|-----|--------------|------|
| count | uint8 | — | 衛星数（最大40） |
| svid | uint8[40] | UBX-NAV-SAT | 衛星ID |
| used | uint8[40] | UBX-NAV-SAT | 使用中フラグ |
| elevation | uint8[40] | UBX-NAV-SAT | 仰角（deg） |
| azimuth | uint8[40] | UBX-NAV-SAT | 方位角（deg） |
| snr | uint8[40] | UBX-NAV-SAT | C/N0（dBHz）、**L1/L2区別なし** |

**調査結果（Session 57）**: L1/L2の区別はない。PX4 GPSドライバはUBX-NAV-SATのみ処理し、UBX-NAV-SIGは未実装。

---

## 4. ULogから取得可能なデータ（Session 57 調査確定）

| 評価項目 | ULog取得可否 | 備考 |
|----------|-------------|------|
| RTK状態（Fix/Float） | ✅ 可能 | fix_type = 5 or 6 |
| 水平精度（hAcc） | ✅ 可能 | eph フィールド |
| 垂直精度（vAcc） | ✅ 可能 | epv フィールド |
| 衛星数 | ✅ 可能 | satellites_used |
| ジャミング状態 | ✅ 可能 | jamming_state (0=unknown, 1=ok, 2=mitigated, 3=detected) |
| **受信感度（C/N0）** | ⚠️ 制限あり | satellite_info.snrで取得可能だが**L1/L2区別なし** |
| **L1/L2別C/N0** | ❌ 不可 | PX4がUBX-NAV-SIGを処理していない |
| **スペアナデータ** | ❌ 不可 | UBX-MON-SPANはuORB化されていない |
| **TTFF** | ❌ 不可 | UBX-NAV-STATUS.ttffはuORB化されていない |

---

## 5. 直接UBX通信が必要な項目（確定）

以下はフライトログ経由では取得**できない**ため、F9Pとの直接通信が**必須**：

| 評価項目 | 理由 | 必要なUBXメッセージ |
|----------|------|-------------------|
| L1/L2別の受信感度（C/N0） | PX4がUBX-NAV-SIGを処理していない | UBX-NAV-SIG |
| TTFF（初期FIX時間） | PX4がttffを抽出していない | UBX-NAV-STATUS |
| スペアナデータ | UBX-MON-SPANがuORB化されていない | UBX-MON-SPAN |
| 内部設定（FW/PROTVER/パラメータ） | 設定読み取りはドライバの役割外 | UBX-MON-VER, UBX-CFG-* |

**調査根拠（Session 57）**:
- PX4-GPSDrivers `ubx.cpp` を確認
- UBX-NAV-SIGの定義・処理コードが存在しない
- NAV-SATのみ実装（1衛星1C/N0値、L1/L2区別なし）
- NAV-STATUSからはspoofing_stateのみ抽出、ttffは未抽出
- 詳細エビデンス: [12-px4-source-evidence.md](12-px4-source-evidence.md)

---

## 6. 要調査項目

### 6-1. PX4ソースコード確認

| 確認項目 | 確認場所（推定） | 優先度 |
|----------|----------------|--------|
| GPSダンプ有効時のデータ内容 | `src/drivers/gps/` | 高 |
| UBX-NAV-SIG（L1/L2別C/N0）の対応 | `src/drivers/gps/devices/ubx/` | 高 |
| satellite_infoのsnrがL1/L2どちらか | uORB定義ファイル | 高 |
| UBX-NAV-STATUS.ttffの対応 | GPSドライバ | 中 |
| UBX-MON-SPANの対応 | GPSドライバ | 低（おそらく未対応） |

### 6-2. 実機確認

| 確認項目 | 方法 |
|----------|------|
| GPSダンプ有効時のULogに何が入るか | 実際にログ取得して確認 |
| pyulogでsatellite_infoが読めるか | pyulog + サンプルログ |

---

## 7. 参考リンク

- PX4 uORB メッセージ定義: https://github.com/PX4/PX4-Autopilot/tree/main/msg
- PX4 GPSドライバ: https://github.com/PX4/PX4-Autopilot/tree/main/src/drivers/gps
- pyulog: https://github.com/PX4/pyulog

---

## 8. 次のアクション

1. PX4ソースコードでGPSドライバを確認（UBX→uORB変換）
2. 実機でGPSダンプ有効化してULog取得
3. pyulogで内容確認、本ドキュメント更新

---

## 参照資料

- [08-ubx-protocol-index.md](08-ubx-protocol-index.md) — UBXプロトコル仕様索引
- [10-tool-design-notes.md](10-tool-design-notes.md) — ツール設計メモ
