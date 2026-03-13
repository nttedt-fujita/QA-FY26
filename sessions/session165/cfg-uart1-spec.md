# PDF抽出: u-blox-F9-HPG-1.32_InterfaceDescription_UBX-22008968.pdf

**総ページ数**: 305
**抽出ページ**: 270, 271, 272

---

## Page 270

```
u-blox F9 HPG 1.32 - Interface Description
Constant
Value
Description
GAL
4
Galileo time reference
NAVIC
5
NavIC time reference
Table 57: Constants for CFG-TP-TIMEGRID_TP1
Constant
Value
Description
DRIVE_STRENGTH_2MA
0
2 mA drive strength
DRIVE_STRENGTH_4MA
1
4 mA drive strength
DRIVE_STRENGTH_8MA
2
8 mA drive strength
DRIVE_STRENGTH_12MA
3
12 mA drive strength
Table 58: Constants for CFG-TP-DRSTR_TP1
6.9.30 CFG-TXREADY: TX ready conﬁguration
Conﬁguration of the TX ready pin.
Conﬁguration item
Key ID
Type
Scale
Unit
Description
CFG-TXREADY-ENABLED
0x10a20001
L
-
-
Flag to indicate if TX ready pin mechanism
should be enabled
CFG-TXREADY-POLARITY
0x10a20002
L
-
-
The polarity of the TX ready pin: false:high-
active, true:low-active
CFG-TXREADY-PIN
0x20a20003
U1
-
-
Pin number to use for the TX ready functionality
CFG-TXREADY-THRESHOLD
0x30a20004
U2
-
-
Amount of data that should be ready on the
interface before triggering the TX ready pin
CFG-TXREADY-INTERFACE
0x20a20005
E1
-
-
Interface where the TX ready feature should be
linked to
See Table 60 below for a list of possible constants for this item.
Table 59: CFG-TXREADY conﬁguration items
Constant
Value
Description
I2C
0
I2C interface
SPI
1
SPI interface
Table 60: Constants for CFG-TXREADY-INTERFACE
6.9.31 CFG-UART1: Conﬁguration of the UART1 interface
Settings needed to conﬁgure the UART1 communication interface.
Conﬁguration item
Key ID
Type
Scale
Unit
Description
CFG-UART1-BAUDRATE
0x40520001
U4
-
-
The baud rate that should be conﬁgured on the
UART1
CFG-UART1-STOPBITS
0x20520002
E1
-
-
Number of stopbits that should be used on
UART1
See Table 62 below for a list of possible constants for this item.
CFG-UART1-DATABITS
0x20520003
E1
-
-
Number of databits that should be used on
UART1
See Table 63 below for a list of possible constants for this item.
CFG-UART1-PARITY
0x20520004
E1
-
-
Parity mode that should be used on UART1
See Table 64 below for a list of possible constants for this item.
CFG-UART1-ENABLED
0x10520005
L
-
-
Flag to indicate if the UART1 should be enabled
Table 61: CFG-UART1 conﬁguration items
UBX-22008968 - R01
 
6 Conﬁguration interface
Page 270 of 305
C1-Public
```

---

## Page 271

```
u-blox F9 HPG 1.32 - Interface Description
Constant
Value
Description
HALF
0
0.5 stopbits
ONE
1
1.0 stopbits
ONEHALF
2
1.5 stopbits
TWO
3
2.0 stopbits
Table 62: Constants for CFG-UART1-STOPBITS
Constant
Value
Description
EIGHT
0
8 databits
SEVEN
1
7 databits
Table 63: Constants for CFG-UART1-DATABITS
Constant
Value
Description
NONE
0
No parity bit
ODD
1
Add an odd parity bit
EVEN
2
Add an even parity bit
Table 64: Constants for CFG-UART1-PARITY
6.9.32 CFG-UART1INPROT: Input protocol conﬁguration of the UART1 interface
Input protocol enable ﬂags of the UART1 interface.
Conﬁguration item
Key ID
Type
Scale
Unit
Description
CFG-UART1INPROT-UBX
0x10730001
L
-
-
Flag to indicate if UBX should be an input
protocol on UART1
CFG-UART1INPROT-NMEA
0x10730002
L
-
-
Flag to indicate if NMEA should be an input
protocol on UART1
CFG-UART1INPROT-RTCM3X
0x10730004
L
-
-
Flag to indicate if RTCM3X should be an input
protocol on UART1
CFG-UART1INPROT-SPARTN
0x10730005
L
-
-
Flag to indicate if SPARTN should be an input
protocol on UART1
Table 65: CFG-UART1INPROT conﬁguration items
6.9.33 CFG-UART1OUTPROT: Output protocol conﬁguration of the UART1
interface
Output protocol enable ﬂags of the UART1 interface.
Conﬁguration item
Key ID
Type
Scale
Unit
Description
CFG-UART1OUTPROT-UBX
0x10740001
L
-
-
Flag to indicate if UBX should be an output
protocol on UART1
CFG-UART1OUTPROT-NMEA
0x10740002
L
-
-
Flag to indicate if NMEA should be an output
protocol on UART1
CFG-UART1OUTPROT-RTCM3X
0x10740004
L
-
-
Flag to indicate if RTCM3X should be an output
protocol on UART1
Table 66: CFG-UART1OUTPROT conﬁguration items
6.9.34 CFG-UART2: Conﬁguration of the UART2 interface
Settings needed to conﬁgure the UART2 communication interface.
Conﬁguration item
Key ID
Type
Scale
Unit
Description
CFG-UART2-BAUDRATE
0x40530001
U4
-
-
The baud rate that should be conﬁgured on the
UART2
UBX-22008968 - R01
 
6 Conﬁguration interface
Page 271 of 305
C1-Public
```

---

## Page 272

```
u-blox F9 HPG 1.32 - Interface Description
Conﬁguration item
Key ID
Type
Scale
Unit
Description
CFG-UART2-STOPBITS
0x20530002
E1
-
-
Number of stopbits that should be used on
UART2
See Table 68 below for a list of possible constants for this item.
CFG-UART2-DATABITS
0x20530003
E1
-
-
Number of databits that should be used on
UART2
See Table 69 below for a list of possible constants for this item.
CFG-UART2-PARITY
0x20530004
E1
-
-
Parity mode that should be used on UART2
See Table 70 below for a list of possible constants for this item.
CFG-UART2-ENABLED
0x10530005
L
-
-
Flag to indicate if the UART2 should be enabled
Table 67: CFG-UART2 conﬁguration items
Constant
Value
Description
HALF
0
0.5 stopbits
ONE
1
1.0 stopbits
ONEHALF
2
1.5 stopbits
TWO
3
2.0 stopbits
Table 68: Constants for CFG-UART2-STOPBITS
Constant
Value
Description
EIGHT
0
8 databits
SEVEN
1
7 databits
Table 69: Constants for CFG-UART2-DATABITS
Constant
Value
Description
NONE
0
No parity bit
ODD
1
Add an odd parity bit
EVEN
2
Add an even parity bit
Table 70: Constants for CFG-UART2-PARITY
6.9.35 CFG-UART2INPROT: Input protocol conﬁguration of the UART2 interface
Input protocol enable ﬂags of the UART2 interface.
Conﬁguration item
Key ID
Type
Scale
Unit
Description
CFG-UART2INPROT-UBX
0x10750001
L
-
-
Flag to indicate if UBX should be an input
protocol on UART2
CFG-UART2INPROT-NMEA
0x10750002
L
-
-
Flag to indicate if NMEA should be an input
protocol on UART2
CFG-UART2INPROT-RTCM3X
0x10750004
L
-
-
Flag to indicate if RTCM3X should be an input
protocol on UART2
CFG-UART2INPROT-SPARTN
0x10750005
L
-
-
Flag to indicate if SPARTN should be an input
protocol on UART2
Table 71: CFG-UART2INPROT conﬁguration items
6.9.36 CFG-UART2OUTPROT: Output protocol conﬁguration of the UART2
interface
Output protocol enable ﬂags of the UART2 interface.
UBX-22008968 - R01
 
6 Conﬁguration interface
Page 272 of 305
C1-Public
```

---
