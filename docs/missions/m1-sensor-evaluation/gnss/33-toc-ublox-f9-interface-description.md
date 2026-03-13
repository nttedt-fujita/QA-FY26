# u-blox F9 HPG 1.32 Interface Description — 目次

---
作成: Session 171 (2026-03-13)
出典: u-blox-F9-HPG-1.32_InterfaceDescription_UBX-22008968.pdf
---

**ドキュメント情報**:
- タイトル: u-blox F9 HPG 1.32 — Interface Description
- ドキュメント番号: UBX-22008968
- リビジョン: R01 (02-May-2022)
- 総ページ数: 305

---

## 目次

### 1 General information (p.15)

- 1.1 Document overview
- 1.2 Firmware and protocol versions
- 1.3 Receiver configuration
- 1.4 Message naming
- 1.5 GNSS, satellite, and signal identifiers
  - 1.5.1 Overview
  - 1.5.2 GNSS identifiers
  - 1.5.3 Satellite identifiers
  - 1.5.4 Signal identifiers
- 1.6 Message types

### 2 NMEA protocol (p.22)

- 2.1 NMEA frame structure
- 2.2 NMEA protocol configuration
- 2.3 NMEA-proprietary messages
- 2.4 NMEA multi-GNSS operation
- 2.5 NMEA data fields
  - 2.5.1 NMEA Talker ID
  - 2.5.2 NMEA extra fields
  - 2.5.3 NMEA latitude and longitude format
  - 2.5.4 NMEA GNSS, satellite, and signal numbering
  - 2.5.5 NMEA position fix flags
  - 2.5.6 NMEA output of invalid or unknown data
- 2.6 NMEA messages overview
- 2.7 Standard messages (DTM, GAQ, GBQ, GBS, GGA, GLL, GLQ, GNQ, GNS, GPQ, GQQ, GRS, GSA, GST, GSV, RLM, RMC, TXT, VLW, VTG, ZDA)
- 2.8 Secondary output messages (GGA, GLL, GNS, GSA, RMC, VTG, ZDA)
- 2.9 PUBX messages (CONFIG, POSITION, RATE, SVSTATUS, TIME)

### 3 UBX protocol (p.53)

- 3.1 UBX protocol key features
- 3.2 UBX frame structure
- 3.3 UBX payload definition rules
- 3.4 UBX checksum
- 3.5 UBX message flow
  - 3.5.1 UBX acknowledgement
  - 3.5.2 UBX polling mechanism
- 3.6 GNSS, satellite, and signal numbering
- 3.7 UBX message example
- 3.8 UBX messages overview
- 3.9 UBX-ACK (0x05) — p.62
- 3.10 UBX-CFG (0x06) — p.63
  - CFG-ANT, CFG-CFG, CFG-DAT, CFG-DGNSS, CFG-GEOFENCE, CFG-GNSS, CFG-INF, CFG-ITFM, CFG-LOGFILTER, CFG-MSG, CFG-NAV5, CFG-NAVX5, CFG-NMEA, CFG-ODO, CFG-PRT, CFG-PWR, CFG-RATE, CFG-RINV, CFG-RST, CFG-SBAS, CFG-TMODE3, CFG-TP5, CFG-USB, CFG-VALDEL, CFG-VALGET, CFG-VALSET
- 3.11 UBX-INF (0x04) — p.98
- 3.12 UBX-LOG (0x21) — p.100
- 3.13 UBX-MGA (0x13) — p.106
- 3.14 UBX-MON (0x0a) — p.126
  - MON-COMMS, MON-GNSS, MON-HW, MON-HW2, MON-HW3, MON-IO, MON-MSGPP, MON-PATCH, MON-RF, MON-RXBUF, MON-RXR, MON-SPAN, MON-SYS, MON-TXBUF, MON-VER
- 3.15 UBX-NAV (0x01) — p.136
  - NAV-CLOCK, NAV-COV, NAV-DOP, NAV-EOE, NAV-GEOFENCE, NAV-HPPOSECEF, NAV-HPPOSLLH, NAV-ODO, NAV-ORB, NAV-PL, NAV-POSECEF, NAV-POSLLH, NAV-PVT, NAV-RELPOSNED, NAV-RESETODO, NAV-SAT, NAV-SBAS, NAV-SIG, NAV-SLAS, NAV-STATUS, NAV-SVIN, NAV-TIMEBDS, NAV-TIMEGAL, NAV-TIMEGLO, NAV-TIMEGPS, NAV-TIMELS, NAV-TIMEQZSS, NAV-TIMEUTC, NAV-VELECEF, NAV-VELNED
- 3.16 UBX-NAV2 (0x29) — p.164
- 3.17 UBX-RXM (0x02) — p.183
  - RXM-COR, RXM-MEASX, RXM-PMP, RXM-PMREQ, RXM-QZSSL6, RXM-RAWX, RXM-RLM, RXM-RTCM, RXM-SFRBX, RXM-SPARTN, RXM-SPARTNKEY
- 3.18 UBX-SEC (0x27) — p.194
- 3.19 UBX-TIM (0x0d) — p.194
- 3.20 UBX-UPD (0x09) — p.197

### 4 RTCM protocol (p.199)

- 4.1 RTCM introduction
- 4.2 RTCM 3.x configuration
- 4.3 RTCM messages overview
- 4.4 RTCM 3.3 messages (1001-1012, 1033, 1074-1127, 1230, 4072)

### 5 SPARTN protocol (p.216)

- 5.1 SPARTN introduction
- 5.2 SPARTN configuration
- 5.3 SPARTN messages overview
- 5.4 SPARTN messages (Type 0, 1, 2)

### 6 Configuration interface (p.223)

- 6.1 Configuration database
- 6.2 Configuration items
- 6.3 Configuration layers
- 6.4 Configuration interface access
- 6.5 Configuration data
- 6.6 Configuration transactions
- 6.7 Configuration reset behavior
- 6.8 Configuration overview
- 6.9 Configuration reference — p.228
  - CFG-BDS, CFG-GEOFENCE, CFG-HW, CFG-I2C, CFG-I2CINPROT, CFG-I2COUTPROT, CFG-INFMSG, CFG-ITFM, CFG-LOGFILTER, CFG-MOT, CFG-MSGOUT (p.234), CFG-NAV2, CFG-NAVHPG, CFG-NAVSPG, CFG-NMEA, CFG-ODO, CFG-QZSS, CFG-RATE, CFG-RINV, CFG-RTCM, CFG-SBAS, CFG-SEC, CFG-SIGNAL, CFG-SPARTN, CFG-SPI, CFG-SPIINPROT, CFG-SPIOUTPROT, CFG-TMODE, CFG-TP, CFG-TXREADY, **CFG-UART1 (p.270)**, CFG-UART1INPROT, CFG-UART1OUTPROT, CFG-UART2, CFG-UART2INPROT, CFG-UART2OUTPROT, CFG-USB, CFG-USBINPROT, CFG-USBOUTPROT
- 6.10 Legacy UBX message fields reference — p.274

### 付録

- Configuration defaults — p.280
- Related documents — p.303
- Revision history — p.304

---

## よく使うセクションへのクイックリファレンス

| 用途 | セクション | ページ |
|------|-----------|--------|
| UBXフレーム構造 | 3.2 | 53 |
| UBXポーリング | 3.5.2 | 56 |
| NAV-PVT | 3.15.13 | 145 |
| NAV-SAT | 3.15.16 | 150 |
| NAV-SIG | 3.15.18 | 152 |
| NAV-STATUS | 3.15.20 | 155 |
| MON-COMMS | 3.14.1 | 126 |
| MON-SPAN | 3.14.12 | 134 |
| MON-VER | 3.14.15 | 136 |
| CFG-VALSET | 3.10.26 | 96 |
| CFG-MSGOUT | 6.9.11 | 234 |
| CFG-UART1 | 6.9.31 | 270 |
| RTCM概要 | 4.3 | 199 |

---

*統合元: session156/ublox-toc.md, ublox-toc-2.md, session165/pdf-toc.md, pdf-toc-2.md*
