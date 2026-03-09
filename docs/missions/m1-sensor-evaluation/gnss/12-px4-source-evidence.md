# PX4ソースコード調査エビデンス

**調査日**: 2026-03-09（Session 57）
**目的**: L1/L2別C/N0がフライトログから取得可能か確認
**結論**: **取得不可**（PX4がUBX-NAV-SIGを処理していない）

---

## 1. 調査対象リポジトリ

| リポジトリ | URL | 役割 |
|-----------|-----|------|
| PX4-GPSDrivers | https://github.com/PX4/PX4-GPSDrivers | GPSドライバ（UBX解析） |
| PX4-Autopilot | https://github.com/PX4/PX4-Autopilot | PX4本体（uORB定義含む） |

---

## 2. エビデンス: uORBメッセージ定義

### 2-1. SatelliteInfo.msg

**URL**: https://github.com/PX4/PX4-Autopilot/blob/main/msg/SatelliteInfo.msg

**原文抜粋**:
```
uint8 SAT_INFO_MAX_SATELLITES = 20
uint8 count             # Number of satellites visible to the receiver
uint8[20] svid          # Space vehicle ID
uint8[20] used          # 0: Satellite not used, 1: used for navigation
uint8[20] elevation     # Elevation (0: right on top of receiver, 90: on the horizon) of satellite
uint8[20] azimuth       # Direction of satellite, 0: 0 deg, 255: 360 deg.
uint8[20] snr           # dBHz, Signal to noise ratio of satellite C/N0, range 0..99
```

**確認事項**:
- `snr` は1衛星1値（配列サイズ20）
- **L1/L2の区別なし**
- 信号ID（sigId）フィールドが存在しない

---

### 2-2. SensorGps.msg

**URL**: https://github.com/PX4/PX4-Autopilot/blob/main/msg/SensorGps.msg

**原文抜粋**（jamming関連）:
```
uint8 JAMMING_STATE_UNKNOWN = 0
uint8 JAMMING_STATE_OK = 1
uint8 JAMMING_STATE_WARNING = 2
uint8 JAMMING_STATE_CRITICAL = 3

uint8 jamming_state         # indicates whether jamming has been detected or suspected
int32 jamming_indicator     # indicates jamming is occurring
```

**原文抜粋**（fix_type関連）:
```
uint8 FIX_TYPE_NONE = 1
uint8 FIX_TYPE_2D = 2
uint8 FIX_TYPE_3D = 3
uint8 FIX_TYPE_RTCM_CODE_DIFFERENTIAL = 4
uint8 FIX_TYPE_RTK_FLOAT = 5
uint8 FIX_TYPE_RTK_FIXED = 6
```

---

## 3. エビデンス: GPSドライバ（UBX解析）

### 3-1. ubx.h — メッセージID定義

**URL**: https://github.com/PX4/PX4-GPSDrivers/blob/main/src/ubx.h

**原文抜粋**（NAV-SAT定義あり）:
```c
#define UBX_ID_NAV_SAT        0x35
#define UBX_MSG_NAV_SAT       ((UBX_CLASS_NAV) | UBX_ID_NAV_SAT << 8)
```

**確認事項**:
- `UBX_ID_NAV_SAT` (0x35) は定義されている
- **`UBX_ID_NAV_SIG` (0x43) の定義がない** ← 重要

---

### 3-2. ubx.h — NAV-SAT構造体定義

**URL**: https://github.com/PX4/PX4-GPSDrivers/blob/main/src/ubx.h

**原文抜粋**:
```c
typedef struct {
    uint32_t iTOW;           /**< GPS Time of Week [ms] */
    uint8_t  version;        /**< Message version (1) */
    uint8_t  numSvs;         /**< Number of Satellites */
    uint16_t reserved;
} ubx_payload_rx_nav_sat_part1_t;

typedef struct {
    uint8_t  gnssId;         /**< GNSS identifier */
    uint8_t  svId;           /**< Satellite ID */
    uint8_t  cno;            /**< Carrier to Noise Ratio (Signal Strength) [dbHz] */
    int8_t   elev;           /**< Elevation [deg] range: +/-90 */
    int16_t  azim;           /**< Azimuth [deg] range: 0-360 */
    int16_t  prRes;          /**< Pseudo range residual [0.1 m] */
    uint32_t flags;
} ubx_payload_rx_nav_sat_part2_t;
```

**確認事項**:
- `cno` は1衛星1値
- `sigId`（信号ID）フィールドがない
- L1/L2を区別する構造になっていない

---

### 3-3. ubx.cpp — NAV-SAT処理

**URL**: https://github.com/PX4/PX4-GPSDrivers/blob/main/src/ubx.cpp

**原文抜粋**（C/N0の抽出部分）:
```cpp
_satellite_info->snr[sat_index] = static_cast<uint8_t>(_buf.payload_rx_nav_sat_part2.cno);
```

**確認事項**:
- 衛星ごとに1つのC/N0値を格納
- L1/L2を区別する処理がない

---

### 3-4. ubx.cpp — NAV-SIG未実装の確認

**URL**: https://github.com/PX4/PX4-GPSDrivers/blob/main/src/ubx.cpp

**確認方法**: ファイル内で "NAV_SIG" または "0x43" を検索

**結果**: **該当なし**

- `UBX_MSG_NAV_SIG` の case 文がない
- `payloadRxAddNavSig()` 相当の関数がない
- NAV-SIG用のペイロード構造体定義がない

---

## 4. エビデンス: TTFF（Time To First Fix）

### 4-1. ubx.h — NAV-STATUS構造体定義

**URL**: https://github.com/PX4/PX4-GPSDrivers/blob/main/src/ubx.h

**原文抜粋**:
```c
typedef struct {
    uint32_t iTOW;           /**< GPS Time of Week [ms] */
    uint8_t  gpsFix;         /**< GPSfix Type, range 0..5 */
    uint8_t  flags;          /**< Fix Status Flags */
    uint8_t  fixStat;        /**< Fix Status Information */
    uint8_t  flags2;         /**< Additional Flags */
    uint32_t ttff;           /**< Time to first fix [ms] */
    uint32_t msss;           /**< Milliseconds since startup/reset */
} ubx_payload_rx_nav_status_t;
```

**確認事項**:
- `ttff` フィールドは構造体に**定義されている**

---

### 4-2. ubx.cpp — NAV-STATUS処理

**URL**: https://github.com/PX4/PX4-GPSDrivers/blob/main/src/ubx.cpp

**原文抜粋**:
```cpp
case UBX_MSG_NAV_STATUS:
    UBX_TRACE_RXMSG("Rx NAV-STATUS");

    _gps_position->spoofing_state = (_buf.payload_rx_nav_status.flags2 &
        UBX_RX_NAV_STATUS_SPOOFDETSTATE_MASK) >>
        UBX_RX_NAV_STATUS_SPOOFDETSTATE_SHIFT;
```

**確認事項**:
- `spoofing_state` のみ抽出
- **`ttff` は抽出されていない**

---

### 4-3. SensorGps.msg — ttffフィールドの有無

**URL**: https://docs.px4.io/main/en/msg_docs/SensorGps.html

**確認事項**:
- SensorGps.msgに**ttffフィールドが存在しない**
- fix_type, satellites_used, jamming_state等はあるがTTFFはない

---

## 5. 結論

| 項目 | 結果 |
|------|------|
| PX4でのNAV-SIG対応 | **未実装** |
| L1/L2別C/N0のULog取得 | **不可** |
| PX4でのTTFF対応 | **未実装**（構造体にはあるが抽出していない） |
| TTFFのULog取得 | **不可** |
| 代替手段 | F9Pとの直接UBX通信が必要 |

---

## 5. 参照

- [ADR-004](../../../adr/ADR-004-gnss-tool-approach.md) — この調査に基づく方針決定
- [11-px4-uorb-mapping.md](11-px4-uorb-mapping.md) — PX4 uORBマッピング（更新済み）
- [08-ubx-protocol-index.md](08-ubx-protocol-index.md) — UBXプロトコル仕様（NAV-SIG: p.152-153）
