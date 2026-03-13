# PDF抽出: u-blox-F9-HPG-1.32_InterfaceDescription_UBX-22008968.pdf

**総ページ数**: 305
**抽出ページ**: 126, 127, 128, 129, 130

---

## Page 126

```
u-blox F9 HPG 1.32 - Interface Description
Payload description:
Byte offset
Type
Name
Scale
Unit
Description
0
U1
type
-
-
Message type (0x04 for this type)
1
U1
version
-
-
Message version (0x00 for this version)
2
U1[2]
reserved0
-
-
Reserved
4
U1[5]
healthCode
-
-
Each byte represents a QZSS SV (1-5). The 6 LSBs
of each byte contains the 6 bit health code from
subframes 4/5, data ID = 3, SV ID = 51
9
U1[3]
reserved1
-
-
Reserved
3.14 UBX-MON (0x0a)
The messages in the UBX-MON class are used to report the receiver status, such as hardware status
or I/O subsystem statistics.
3.14.1 UBX-MON-COMMS (0x0a 0x36)
3.14.1.1 Communication port information
Message
UBX-MON-COMMS
Communication port information
Type
Periodic/polled
Comment
Consolidated communications information for all ports. The size of the message is determined by the number
of ports that are in use on the receiver. A port is only included if communication, either send or receive, has
been initiated on that port.
Header
Class
ID
Length (Bytes)
Payload
Checksum
Message
structure
0xb5 0x62
0x0a
0x36
8 + nPorts·40
see below
CK_A CK_B
Payload description:
Byte offset
Type
Name
Scale
Unit
Description
0
U1
version
-
-
Message version (0x00 for this version)
1
U1
nPorts
-
-
Number of ports included
2
X1
txErrors
-
-
TX error bitmask
bit 0   U:1
mem
-
-
Memory Allocation error
bit 1   U:1
alloc
-
-
Allocation error (TX buﬀer full)
3
U1
reserved0
-
-
Reserved
4
U1[4]
protIds
-
The identiﬁers of the protocols reported in the msgs
array. 0: UBX, 1: NMEA, 2: RTCM2, 5: RTCM3, 6:
SPARTN, 0xFF: No protocol reported.
Start of repeated group (nPorts times)
8 + n·40
U2
portId
-
-
Unique 
identiﬁer 
for 
the 
port. 
See 
section
Communications ports in the integration manual for
details.
10 + n·40
U2
txPending
-
bytes
Number of bytes pending in transmitter buﬀer
12 + n·40
U4
txBytes
-
bytes
Number of bytes ever sent
16 + n·40
U1
txUsage
-
%
Maximum usage transmitter buﬀer during the last
sysmon period
17 + n·40
U1
txPeakUsage
-
%
Maximum usage transmitter buﬀer
18 + n·40
U2
rxPending
-
bytes
Number of bytes in receiver buﬀer
UBX-22008968 - R01
 
3 UBX protocol
Page 126 of 305
C1-Public
```

---

## Page 127

```
u-blox F9 HPG 1.32 - Interface Description
20 + n·40
U4
rxBytes
-
bytes
Number of bytes ever received
24 + n·40
U1
rxUsage
-
%
Maximum usage receiver buﬀer during the last
sysmon period
25 + n·40
U1
rxPeakUsage
-
%
Maximum usage receiver buﬀer
26 + n·40
U2
overrunErrs
-
-
Number of 100 ms timeslots with overrun errors
28 + n·40
U2[4]
msgs
-
msg
Number of successfully parsed messages for each
protocol. The reported protocols are identiﬁed through
the protIds ﬁeld.
36 + n·40
U1[8]
reserved1
-
-
Reserved
44 + n·40
U4
skipped
-
bytes
Number of skipped bytes
End of repeated group (nPorts times)
3.14.2 UBX-MON-GNSS (0x0a 0x28)
3.14.2.1 Information message major GNSS selection
Message
UBX-MON-GNSS
Information message major GNSS selection
Type
Polled
Comment
This message reports major GNSS selection. It does this by means of bit masks in U1 ﬁelds. Each bit in a bit
mask corresponds to one major GNSS. Augmentation systems are not reported.
Header
Class
ID
Length (Bytes)
Payload
Checksum
Message
structure
0xb5 0x62
0x0a
0x28
8
see below
CK_A CK_B
Payload description:
Byte offset
Type
Name
Scale
Unit
Description
0
U1
version
-
-
Message version (0x00 for this version)
1
X1
supported
-
-
A bit mask showing the major GNSS that can be
supported by this receiver
bit 0   U:1
GPSSup
-
-
GPS is supported
bit 1   U:1
GlonassSup
-
-
GLONASS is supported
bit 2   U:1
BeidouSup
-
-
BeiDou is supported
bit 3   U:1
GalileoSup
-
-
Galileo is supported
2
X1
defaultGnss
-
-
A bit mask showing the default major GNSS selection.
If the default major GNSS selection is currently
conﬁgured in the efuse for this receiver, it takes
precedence over the default major GNSS selection
conﬁgured in the executing ﬁrmware of this receiver.
bit 0   U:1
GPSDef
-
-
GPS is default-enabled
bit 1   U:1
GlonassDef
-
-
GLONASS is default-enabled
bit 2   U:1
BeidouDef
-
-
BeiDou is default-enabled
bit 3   U:1
GalileoDef
-
-
Galileo is default-enabled
3
X1
enabled
-
-
A bit mask showing the current major GNSS selection
enabled for this receiver
bit 0   U:1
GPSEna
-
-
GPS is enabled
bit 1   U:1
GlonassEna
-
-
GLONASS is enabled
bit 2   U:1
BeidouEna
-
-
BeiDou is enabled
bit 3   U:1
GalileoEna
-
-
Galileo is enabled
UBX-22008968 - R01
 
3 UBX protocol
Page 127 of 305
C1-Public
```

---

## Page 128

```
u-blox F9 HPG 1.32 - Interface Description
4
U1
simultaneous
-
-
Maximum number of concurrent major GNSS that can
be supported by this receiver
5
U1[3]
reserved0
-
-
Reserved
3.14.3 UBX-MON-HW (0x0a 0x09)
3.14.3.1 Hardware status
Message
UBX-MON-HW
Hardware status
Type
Periodic/polled
Comment
This message is deprecated in this protocol version. Use UBX-MON-HW3 and UBX-MON-RF instead.
Status of diﬀerent aspects of the hardware, such as antenna, PIO/peripheral pins, noise level, automatic gain
control (AGC)
Header
Class
ID
Length (Bytes)
Payload
Checksum
Message
structure
0xb5 0x62
0x0a
0x09
60
see below
CK_A CK_B
Payload description:
Byte offset
Type
Name
Scale
Unit
Description
0
X4
pinSel
-
-
Mask of pins set as peripheral/PIO
4
X4
pinBank
-
-
Mask of pins set as bank A/B
8
X4
pinDir
-
-
Mask of pins set as input/output
12
X4
pinVal
-
-
Mask of pins value low/high
16
U2
noisePerMS
-
-
Noise level as measured by the GPS core
18
U2
agcCnt
-
-
AGC monitor (counts SIGHI xor SIGLO, range 0 to
8191)
20
U1
aStatus
-
-
Status of the antenna supervisor state machine
(0=INIT, 1=DONTKNOW, 2=OK, 3=SHORT, 4=OPEN)
21
U1
aPower
-
-
Current power status of antenna (0=OFF, 1=ON,
2=DONTKNOW)
22
X1
flags
-
-
Flags
bit 0   U:1
rtcCalib
-
-
RTC is calibrated
bit 1   U:1
safeBoot
-
-
Safeboot mode (0 = inactive, 1 = active)
bits 3…2   U:2
jammingState
-
-
Output from jamming/interference monitor (0 =
unknown or feature disabled, 1 = ok - no signiﬁcant
jamming, 2 = warning - interference visible but ﬁx
OK, 3 = critical - interference visible and no ﬁx). This
ﬂag is deprecated in protocol versions that support
UBX-SEC-SIG (version 0x02); instead jammingState in
UBX-SEC-SIG should be monitored.
bit 4   U:1
xtalAbsent
-
-
RTC xtal has been determined to be absent (not
supported for protocol versions less than 18.00)
23
U1
reserved0
-
-
Reserved
24
X4
usedMask
-
-
Mask of pins that are used by the virtual pin manager
28
U1[17]
VP
-
-
Array of pin mappings for each of the 17 physical pins
45
U1
cwSuppression
-
-
CW interference suppression level, scaled (0 = no CW
jamming, 255 = strong CW jamming)
46
U1[2]
reserved1
-
-
Reserved
48
X4
pinIrq
-
-
Mask of pins value using the PIO Irq
UBX-22008968 - R01
 
3 UBX protocol
Page 128 of 305
C1-Public
```

---

## Page 129

```
u-blox F9 HPG 1.32 - Interface Description
52
X4
pullH
-
-
Mask of pins value using the PIO pull high resistor
56
X4
pullL
-
-
Mask of pins value using the PIO pull low resistor
3.14.4 UBX-MON-HW2 (0x0a 0x0b)
3.14.4.1 Extended hardware status
Message
UBX-MON-HW2
Extended hardware status
Type
Periodic/polled
Comment
This message is deprecated in this protocol version. Use UBX-MON-HW3 and UBX-MON-RF instead.
Status of diﬀerent aspects of the hardware such as Imbalance, Low-Level Conﬁguration and POST Results.
The ﬁrst four parameters of this message represent the complex signal from the RF front end. The following
rules of thumb apply:
•
The smaller the absolute value of the variable ofsI and ofsQ, the better.
•
Ideally, the magnitude of the I-part (magI) and the Q-part (magQ) of the complex signal should be the
same.
Header
Class
ID
Length (Bytes)
Payload
Checksum
Message
structure
0xb5 0x62
0x0a
0x0b
28
see below
CK_A CK_B
Payload description:
Byte offset
Type
Name
Scale
Unit
Description
0
I1
ofsI
-
-
Imbalance of I-part of complex signal, scaled (-128
= max. negative imbalance, 127 = max. positive
imbalance)
1
U1
magI
-
-
Magnitude of I-part of complex signal, scaled (0 = no
signal, 255 = max. magnitude)
2
I1
ofsQ
-
-
Imbalance of Q-part of complex signal, scaled (-128
= max. negative imbalance, 127 = max. positive
imbalance)
3
U1
magQ
-
-
Magnitude of Q-part of complex signal, scaled (0 = no
signal, 255 = max. magnitude)
4
U1
cfgSource
-
-
Source of low-level conﬁguration
(114 = ROM, 111 = OTP, 112 = conﬁg pins, 102 = ﬂash
image)
5
U1[3]
reserved0
-
-
Reserved
8
U4
lowLevCfg
-
-
Low-level conﬁguration (obsolete for protocol versions
greater than 15.00)
12
U1[8]
reserved1
-
-
Reserved
20
U4
postStatus
-
-
POST status word
24
U1[4]
reserved2
-
-
Reserved
3.14.5 UBX-MON-HW3 (0x0a 0x37)
3.14.5.1 I/O pin status
Message
UBX-MON-HW3
I/O pin status
Type
Periodic/polled
Comment
This message contains information speciﬁc to each HW I/O pin, for example whether the pin is set as Input
or Output.
For the antenna supervisor status and other RF status information, see the UBX-MON-RF message.
UBX-22008968 - R01
 
3 UBX protocol
Page 129 of 305
C1-Public
```

---

## Page 130

```
u-blox F9 HPG 1.32 - Interface Description
Header
Class
ID
Length (Bytes)
Payload
Checksum
Message
structure
0xb5 0x62
0x0a
0x37
22 + nPins·6
see below
CK_A CK_B
Payload description:
Byte offset
Type
Name
Scale
Unit
Description
0
U1
version
-
-
Message version (0x00 for this version)
1
U1
nPins
-
-
The number of I/O pins included
2
X1
flags
-
-
Flags
bit 0   U:1
rtcCalib
-
-
RTC is calibrated
bit 1   U:1
safeBoot
-
-
Safeboot mode (0 = inactive, 1 = active)
bit 2   U:1
xtalAbsent
-
-
RTC xtal has been determined to be absent
3
CH[10]
hwVersion
-
-
Zero-terminated hardware version string (same as
that returned in the UBX-MON-VER message)
13
U1[9]
reserved0
-
-
Reserved
Start of repeated group (nPins times)
22 + n·6
U2
pinId
-
-
Identiﬁer for the pin, including both external and
internal pins.
24 + n·6
X2
pinMask
-
-
Pin mask
bit 0   U:1
periphPIO
-
-
Pin is set to peripheral or PIO? 0=Peripheral 1=PIO
bits 3…1   U:3
pinBank
-
-
Bank the pin belongs to, where 0=A 1=B 2=C 3=D 4=E
5=F 6=G 7=H
bit 4   U:1
direction
-
-
Pin direction? 0=Input 1=Output
bit 5   U:1
value
-
-
Pin value? 0=Low 1=High
bit 6   U:1
vpManager
-
-
Used by virtual pin manager? 0=No 1=Yes
bit 7   U:1
pioIrq
-
-
Interrupt enabled? 0=No 1=Yes
bit 8   U:1
pioPullHigh
-
-
Using pull high resistor? 0=No 1=Yes
bit 9   U:1
pioPullLow
-
-
Using pull low resistor 0=No 1=Yes
26 + n·6
U1
VP
-
-
Virtual pin mapping
27 + n·6
U1
reserved1
-
-
Reserved
End of repeated group (nPins times)
3.14.6 UBX-MON-IO (0x0a 0x02)
3.14.6.1 I/O system status
Message
UBX-MON-IO
I/O system status
Type
Periodic/polled
Comment
This message is deprecated in this protocol version. Use UBX-MON-COMMS instead.
The size of the message is determined by the number of ports 'N' the receiver supports, i.e. on u-blox 5 the
number of ports is 6.
Header
Class
ID
Length (Bytes)
Payload
Checksum
Message
structure
0xb5 0x62
0x0a
0x02
[0..n]·20
see below
CK_A CK_B
Payload description:
Byte offset
Type
Name
Scale
Unit
Description
Start of repeated group (N times)
0 + n·20
U4
rxBytes
-
bytes
Number of bytes ever received
UBX-22008968 - R01
 
3 UBX protocol
Page 130 of 305
C1-Public
```

---
