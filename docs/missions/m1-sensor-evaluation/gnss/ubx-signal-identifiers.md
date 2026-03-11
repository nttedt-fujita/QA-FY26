# UBX Signal Identifiers（信号識別子）

**作成日**: 2026-03-11
**参照PDF**: UBX-22008968 (u-blox F9 HPG 1.32 Interface Description) p.18-21

---

## 概要

UBXプロトコルでは、同一衛星からの異なる信号（L1/L2/L5等）を`sigId`で区別する。
`sigId`は`gnssId`と組み合わせて使用する。

---

## GNSS識別子 (gnssId)

| gnssId | GNSS | 略称 |
|--------|------|------|
| 0 | GPS | G |
| 1 | SBAS | S |
| 2 | Galileo | E |
| 3 | BeiDou | B |
| 4 | IMES | I |
| 5 | QZSS | Q |
| 6 | GLONASS | R |
| 7 | NavIC | N |

---

## 信号識別子 (sigId)

### GPS (gnssId=0)

| sigId | Signal | 帯域 |
|-------|--------|------|
| 0 | L1C/A | L1 |
| 3 | L2 CL | L2 |
| 4 | L2 CM | L2 |
| 6 | L5 I | L5 |
| 7 | L5 Q | L5 |

### SBAS (gnssId=1)

| sigId | Signal | 帯域 |
|-------|--------|------|
| 0 | L1C/A | L1 |

### Galileo (gnssId=2)

| sigId | Signal | 帯域 |
|-------|--------|------|
| 0 | E1 C | E1（L1相当） |
| 1 | E1 B | E1（L1相当） |
| 3 | E5 aI | E5a |
| 4 | E5 aQ | E5a |
| 5 | E5 bI | E5b |
| 6 | E5 bQ | E5b |

**注意**: GalileoにはL2帯がない。E5a/E5bはL5相当。

### BeiDou (gnssId=3)

| sigId | Signal | 帯域 |
|-------|--------|------|
| 0 | B1I D1 | B1（L1相当） |
| 1 | B1I D2 | B1（L1相当） |
| 2 | B2I D1 | B2（L2相当） |
| 3 | B2I D2 | B2（L2相当） |
| 5 | B1C | B1（L1相当） |
| 7 | B2a | B2a |

### QZSS (gnssId=5)

| sigId | Signal | 帯域 |
|-------|--------|------|
| 0 | L1C/A | L1 |
| 1 | L1S | L1 |
| 4 | L2 CM | L2 |
| 5 | L2 CL | L2 |
| 8 | L5 I | L5 |
| 9 | L5 Q | L5 |

### GLONASS (gnssId=6)

| sigId | Signal | 帯域 |
|-------|--------|------|
| 0 | L1 OF | L1 |
| 2 | L2 OF | L2 |

### NavIC (gnssId=7)

| sigId | Signal | 帯域 |
|-------|--------|------|
| 0 | L5 A | L5 |

---

## L1/L2判定まとめ

### L1帯

| gnssId | GNSS | L1 sigId |
|--------|------|----------|
| 0 | GPS | 0 |
| 1 | SBAS | 0 |
| 2 | Galileo | 0, 1 |
| 3 | BeiDou | 0, 1, 5 |
| 5 | QZSS | 0, 1 |
| 6 | GLONASS | 0 |

### L2帯

| gnssId | GNSS | L2 sigId |
|--------|------|----------|
| 0 | GPS | 3, 4 |
| 3 | BeiDou | 2, 3 |
| 5 | QZSS | 4, 5 |
| 6 | GLONASS | 2 |

---

## 使用箇所

- NAV-SIG (0x01 0x43): 信号単位の情報（L1/L2別C/N0）
- NAV-SAT (0x01 0x35): 衛星単位の情報（sigIdフィールドなし）

---

## 参照

- UBX-22008968 p.18-21 (1.5 GNSS, satellite, and signal identifiers)
- [08-ubx-protocol-index.md](08-ubx-protocol-index.md)
