# PDF抽出: アプリケーションノート_u-blox-F9-HPG-1.32_InterfaceDescription_UBX-22008968.pdf

**総ページ数**: 305
**抽出ページ**: 79, 80, 81, 82, 83, 84, 85, 86, 87

---

## Page 79

```
u-blox F9 HPG 1.32 - Interface Description
bits 2…0   U:3
profile
-
-
Proﬁle type (0=running, 1=cycling, 2=swimming,
3=car, 4=custom)
6
U1[6]
reserved1
-
-
Reserved
12
U1
cogMaxSpeed
1e-1
m/s
Speed below which course-over-ground (COG) is
computed with the low-speed COG ﬁlter
13
U1
cogMaxPosAcc
-
m
Maximum acceptable position accuracy for computing
COG with the low-speed COG ﬁlter
14
U1[2]
reserved2
-
-
Reserved
16
U1
velLpGain
-
-
Velocity low-pass ﬁlter level, range 0..255
17
U1
cogLpGain
-
-
COG low-pass ﬁlter level (at speed < 8 m/s), range
0..255
18
U1[2]
reserved3
-
-
Reserved
3.10.15 UBX-CFG-PRT (0x06 0x00)
3.10.15.1 Polls the conﬁguration for one I/O port
Message
UBX-CFG-PRT
Polls the conﬁguration for one I/O port
Type
Poll request
Comment
This message is deprecated in protocol versions greater than 23.01. Use UBX-CFG-VALSET, UBX-CFG-
VALGET, UBX-CFG-VALDEL instead.
See the Legacy UBX Message Fields Reference for the corresponding conﬁguration item.
Sending this message with a port ID as payload results in having the receiver return the conﬁguration for the
speciﬁed port.
Header
Class
ID
Length (Bytes)
Payload
Checksum
Message
structure
0xb5 0x62
0x06
0x00
1
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
PortID
-
-
Port identiﬁer number (see the other versions of CFG-
PRT for valid values)
3.10.15.2 Port conﬁguration for UART ports
Message
UBX-CFG-PRT
Port conﬁguration for UART ports
Type
Get/set
Comment
This message is deprecated in protocol versions greater than 23.01. Use UBX-CFG-VALSET, UBX-CFG-
VALGET, UBX-CFG-VALDEL instead.
See the Legacy UBX Message Fields Reference for the corresponding conﬁguration item.
Several conﬁgurations can be concatenated to one input message. In this case the payload length can be a
multiple of the normal length (see the other versions of CFG-PRT). Output messages from the module contain
only one conﬁguration unit.
Note that this message can aﬀect baud rate and other transmission parameters. Because there may be
messages queued for transmission there may be uncertainty about which protocol applies to such messages.
In addition a message currently in transmission may be corrupted by a protocol change. Host data reception
parameters may have to be changed to be able to receive future messages, including the acknowledge
message resulting from the CFG-PRT message.
Header
Class
ID
Length (Bytes)
Payload
Checksum
Message
structure
0xb5 0x62
0x06
0x00
20
see below
CK_A CK_B
Payload description:
UBX-22008968 - R01
 
3 UBX protocol
Page 79 of 305
C1-Public
```

---

## Page 80

```
u-blox F9 HPG 1.32 - Interface Description
Byte offset
Type
Name
Scale
Unit
Description
0
U1
portID
-
-
Port identiﬁer number (see the integration manual for
valid UART port IDs)
1
U1
reserved0
-
-
Reserved
2
X2
txReady
-
-
TX ready PIN conﬁguration (not supported for protocol
versions less than 13.01)
bit 0   U:1
en
-
-
Enable TX ready feature for this port
bit 1   U:1
pol
-
-
Polarity
•
0 High-active
•
1 Low-active
bits 6…2   U:5
pin
-
-
PIO to be used (must not be in use by another function)
bits 15…7   U:9
thres
-
-
Threshold
The given threshold is multiplied by 8 bytes.
The TX ready PIN goes active after >= thres*8 bytes
are pending for the port and going inactive after the
last pending bytes have been written to hardware (0-4
bytes before end of stream).
•
0x000 no threshold
•
0x001 8byte
•
0x002 16byte
•
...
•
0x1FE 4080byte
•
0x1FF 4088byte
4
X4
mode
-
-
A bit mask describing the UART mode
bits 7…6   U:2
charLen
-
-
Character length
•
00 5bit (not supported)
•
01 6bit (not supported)
•
10 7bit (supported only with parity)
•
11 8bit
bits 11…9   U:3
parity
-
-
•
000 Even parity
•
001 Odd parity
•
10X No parity
•
X1X Reserved
bits 13…12   U:2
nStopBits
-
-
Number of Stop bits
•
00 1 Stop bit
•
01 1.5 Stop bit
•
10 2 Stop bit
•
11 0.5 Stop bit
8
U4
baudRate
-
Bits/s
Baud rate in bits/second
12
X2
inProtoMask
-
-
A mask describing which input protocols are active.
Each bit of this mask is used for a protocol. Through
that, multiple protocols can be deﬁned on a single port.
bit 0   U:1
inUbx
-
-
UBX protocol
bit 1   U:1
inNmea
-
-
NMEA protocol
bit 2   U:1
inRtcm
-
-
RTCM2 protocol
bit 5   U:1
inRtcm3
-
-
RTCM3 protocol (not supported for protocol versions
less than 20.00)
14
X2
outProtoMask
-
-
A mask describing which output protocols are active.
Each bit of this mask is used for a protocol. Through
that, multiple protocols can be deﬁned on a single port.
bit 0   U:1
outUbx
-
-
UBX protocol
UBX-22008968 - R01
 
3 UBX protocol
Page 80 of 305
C1-Public
```

---

## Page 81

```
u-blox F9 HPG 1.32 - Interface Description
bit 1   U:1
outNmea
-
-
NMEA protocol
bit 5   U:1
outRtcm3
-
-
RTCM3 protocol (not supported for protocol versions
less than 20.00)
16
X2
flags
-
-
Flags bit mask
bit 1   U:1
extendedTx
Timeout
-
-
Extended TX timeout: if set, the port will time out if
allocated TX memory >=4 kB and no activity for 1.5 s.
If not set the port will time out if no activity for 1.5 s
regardless on the amount of allocated TX memory (not
supported for protocol versions less than 13.01).
18
U1[2]
reserved1
-
-
Reserved
3.10.15.3 Port conﬁguration for USB port
Message
UBX-CFG-PRT
Port conﬁguration for USB port
Type
Get/set
Comment
This message is deprecated in protocol versions greater than 23.01. Use UBX-CFG-VALSET, UBX-CFG-
VALGET, UBX-CFG-VALDEL instead.
See the Legacy UBX Message Fields Reference for the corresponding conﬁguration item.
Several conﬁgurations can be concatenated to one input message. In this case the payload length can be a
multiple of the normal length (see the other versions of CFG-PRT). Output messages from the module contain
only one conﬁguration unit.
Header
Class
ID
Length (Bytes)
Payload
Checksum
Message
structure
0xb5 0x62
0x06
0x00
20
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
portID
-
-
Port identiﬁer number (= 3 for USB port)
1
U1
reserved0
-
-
Reserved
2
X2
txReady
-
-
TX ready PIN conﬁguration (not supported for protocol
versions less than 13.01)
bit 0   U:1
en
-
-
Enable TX ready feature for this port
bit 1   U:1
pol
-
-
Polarity
•
0 High-active
•
1 Low-active
bits 6…2   U:5
pin
-
-
PIO to be used (must not be in use by another function)
bits 15…7   U:9
thres
-
-
Threshold
The given threshold is multiplied by 8 bytes.
The TX ready PIN goes active after >= thres*8 bytes
are pending for the port and going inactive after the
last pending bytes have been written to hardware (0-4
bytes before end of stream).
•
0x000 no threshold
•
0x001 8byte
•
0x002 16byte
•
...
•
0x1FE 4080byte
•
0x1FF 4088byte
4
U1[8]
reserved1
-
-
Reserved
12
X2
inProtoMask
-
-
A mask describing which input protocols are active.
Each bit of this mask is used for a protocol. Through
that, multiple protocols can be deﬁned on a single port.
UBX-22008968 - R01
 
3 UBX protocol
Page 81 of 305
C1-Public
```

---

## Page 82

```
u-blox F9 HPG 1.32 - Interface Description
bit 0   U:1
inUbx
-
-
UBX protocol
bit 1   U:1
inNmea
-
-
NMEA protocol
bit 2   U:1
inRtcm
-
-
RTCM2 protocol
bit 5   U:1
inRtcm3
-
-
RTCM3 protocol (not supported for protocol versions
less than 20.00)
14
X2
outProtoMask
-
-
A mask describing which output protocols are active.
Each bit of this mask is used for a protocol. Through
that, multiple protocols can be deﬁned on a single port.
bit 0   U:1
outUbx
-
-
UBX protocol
bit 1   U:1
outNmea
-
-
NMEA protocol
bit 5   U:1
outRtcm3
-
-
RTCM3 protocol (not supported for protocol versions
less than 20.00)
16
U1[2]
reserved2
-
-
Reserved
18
U1[2]
reserved3
-
-
Reserved
3.10.15.4 Port conﬁguration for SPI port
Message
UBX-CFG-PRT
Port conﬁguration for SPI port
Type
Get/set
Comment
This message is deprecated in protocol versions greater than 23.01. Use UBX-CFG-VALSET, UBX-CFG-
VALGET, UBX-CFG-VALDEL instead.
See the Legacy UBX Message Fields Reference for the corresponding conﬁguration item.
Several conﬁgurations can be concatenated to one input message. In this case the payload length can be a
multiple of the normal length. Output messages from the module contain only one conﬁguration unit.
Header
Class
ID
Length (Bytes)
Payload
Checksum
Message
structure
0xb5 0x62
0x06
0x00
20
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
portID
-
-
Port identiﬁer number (= 4 for SPI port)
1
U1
reserved0
-
-
Reserved
2
X2
txReady
-
-
TX ready PIN conﬁguration (not supported for protocol
versions less than 13.01)
bit 0   U:1
en
-
-
Enable TX ready feature for this port
bit 1   U:1
pol
-
-
Polarity
•
0 High-active
•
1 Low-active
bits 6…2   U:5
pin
-
-
PIO to be used (must not be in use by another function)
bits 15…7   U:9
thres
-
-
Threshold
The given threshold is multiplied by 8 bytes.
The TX ready PIN goes active after >= thres*8 bytes
are pending for the port and going inactive after the
last pending bytes have been written to hardware (0-4
bytes before end of stream).
•
0x000 no threshold
•
0x001 8byte
•
0x002 16byte
•
...
•
0x1FE 4080byte
•
0x1FF 4088byte
UBX-22008968 - R01
 
3 UBX protocol
Page 82 of 305
C1-Public
```

---

## Page 83

```
u-blox F9 HPG 1.32 - Interface Description
4
X4
mode
-
-
SPI Mode Flags
bits 2…1   U:2
spiMode
-
-
•
00 SPI Mode 0: CPOL = 0, CPHA = 0
•
01 SPI Mode 1: CPOL = 0, CPHA = 1
•
10 SPI Mode 2: CPOL = 1, CPHA = 0
•
11 SPI Mode 3: CPOL = 1, CPHA = 1
bits 13…8   U:6
ffCnt
-
-
Number of bytes containing 0xFF to receive before
switching oﬀ reception. Range: 0 (mechanism oﬀ) - 63
8
U1[4]
reserved1
-
-
Reserved
12
X2
inProtoMask
-
-
A mask describing which input protocols are active.
Each bit of this mask is used for a protocol. Through
that, multiple protocols can be deﬁned on a single port.
(The bitﬁeld inRtcm3 is not supported for protocol
versions less than 20.00)
bit 0   U:1
inUbx
-
-
bit 1   U:1
inNmea
-
-
bit 2   U:1
inRtcm
-
-
bit 5   U:1
inRtcm3
-
-
14
X2
outProtoMask
-
-
A mask describing which output protocols are active.
Each bit of this mask is used for a protocol. Through
that, multiple protocols can be deﬁned on a single port.
(The bitﬁeld outRtcm3 is not supported for protocol
versions less than 20.00)
bit 0   U:1
outUbx
-
-
bit 1   U:1
outNmea
-
-
bit 5   U:1
outRtcm3
-
-
16
X2
flags
-
-
Flags bit mask
bit 1   U:1
extendedTx
Timeout
-
-
Extended TX timeout: if set, the port will time out if
allocated TX memory >=4 kB and no activity for 1.5 s.
(not supported for protocol versions less than 13.01)
18
U1[2]
reserved2
-
-
Reserved
3.10.15.5 Port conﬁguration for I2C (DDC) port
Message
UBX-CFG-PRT
Port conﬁguration for I2C (DDC) port
Type
Get/set
Comment
This message is deprecated in protocol versions greater than 23.01. Use UBX-CFG-VALSET, UBX-CFG-
VALGET, UBX-CFG-VALDEL instead.
See the Legacy UBX Message Fields Reference for the corresponding conﬁguration item.
Several conﬁgurations can be concatenated to one input message. In this case the payload length can be a
multiple of the normal length (see the other versions of CFG-PRT). Output messages from the module contain
only one conﬁguration unit.
Header
Class
ID
Length (Bytes)
Payload
Checksum
Message
structure
0xb5 0x62
0x06
0x00
20
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
portID
-
-
Port identiﬁer number (= 0 for I2C (DDC) port)
1
U1
reserved0
-
-
Reserved
UBX-22008968 - R01
 
3 UBX protocol
Page 83 of 305
C1-Public
```

---

## Page 84

```
u-blox F9 HPG 1.32 - Interface Description
2
X2
txReady
-
-
TX ready PIN conﬁguration (not supported for protocol
versions less than 13.01)
bit 0   U:1
en
-
-
Enable TX ready feature for this port
bit 1   U:1
pol
-
-
Polarity
•
0 High-active
•
1 Low-active
bits 6…2   U:5
pin
-
-
PIO to be used (must not be in use by another function)
bits 15…7   U:9
thres
-
-
Threshold
The given threshold is multiplied by 8 bytes.
The TX ready PIN goes active after >= thres*8 bytes
are pending for the port and going inactive after the
last pending bytes have been written to hardware (0-4
bytes before end of stream).
•
0x000 no threshold
•
0x001 8byte
•
0x002 16byte
•
...
•
0x1FE 4080byte
•
0x1FF 4088byte
4
X4
mode
-
-
I2C (DDC) Mode Flags
bits 7…1   U:7
slaveAddr
-
-
Slave address
Range: 0x07 < slaveAddr < 0x78. Bit 0 must be 0
8
U1[4]
reserved1
-
-
Reserved
12
X2
inProtoMask
-
-
A mask describing which input protocols are active.
Each bit of this mask is used for a protocol. Through
that, multiple protocols can be deﬁned on a single port.
(The bitﬁeld inRtcm3 is not supported for protocol
versions less than 20.00)
bit 0   U:1
inUbx
-
-
bit 1   U:1
inNmea
-
-
bit 2   U:1
inRtcm
-
-
bit 5   U:1
inRtcm3
-
-
14
X2
outProtoMask
-
-
A mask describing which output protocols are active.
Each bit of this mask is used for a protocol. Through
that, multiple protocols can be deﬁned on a single port.
(The bitﬁeld outRtcm3 is not supported for protocol
versions less than 20.00)
bit 0   U:1
outUbx
-
-
bit 1   U:1
outNmea
-
-
bit 5   U:1
outRtcm3
-
-
16
X2
flags
-
-
Flags bit mask
bit 1   U:1
extendedTx
Timeout
-
-
Extended TX timeout: if set, the port will time out if
allocated TX memory >=4 kB and no activity for 1.5 s
(not supported for protocol versions less than 13.01).
18
U1[2]
reserved2
-
-
Reserved
3.10.16 UBX-CFG-PWR (0x06 0x57)
UBX-22008968 - R01
 
3 UBX protocol
Page 84 of 305
C1-Public
```

---

## Page 85

```
u-blox F9 HPG 1.32 - Interface Description
3.10.16.1 Put receiver in a deﬁned power state
Message
UBX-CFG-PWR
Put receiver in a deﬁned power state
Type
Set
Comment
☞ This message is deprecated in protocol versions greater than 17. Use UBX-CFG-RST for GNSS start/stop
and UBX-RXM-PMREQ for software backup.
Header
Class
ID
Length (Bytes)
Payload
Checksum
Message
structure
0xb5 0x62
0x06
0x57
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
Message version (0x01 for this version)
1
U1[3]
reserved0
-
-
Reserved
4
U4
state
-
-
Enter system state
•
0x52554E20 = GNSS running
•
0x53544F50 = GNSS stopped
•
0x42434B50 = Software backup. USB interface
will be disabled, other wakeup source is needed.
3.10.17 UBX-CFG-RATE (0x06 0x08)
3.10.17.1 Navigation/measurement rate settings
Message
UBX-CFG-RATE
Navigation/measurement rate settings
Type
Get/set
Comment
This message is deprecated in protocol versions greater than 23.01. Use UBX-CFG-VALSET, UBX-CFG-
VALGET, UBX-CFG-VALDEL instead.
See the Legacy UBX Message Fields Reference for the corresponding conﬁguration item.
This message allows the user to alter the rate at which navigation solutions (and the measurements that they
depend on) are generated by the receiver. The calculation of the navigation solution will always be aligned to
the top of a second zero (ﬁrst second of the week) of the conﬁgured reference time system.
(Navigation period is an integer multiple of the measurement period for protocol versions greater than 17.00).
•
Each measurement triggers the measurements generation and, if available, raw data output.
•
The navRate value deﬁnes that every nth measurement triggers a navigation epoch.
•
The update rate has a direct inﬂuence on the power consumption. The more ﬁxes that are required, the
more CPU power and communication resources are required.
•
For most applications a 1 Hz update rate would be suﬃcient.
•
When using power save mode, measurement and navigation rate can diﬀer from the values conﬁgured
here.
Header
Class
ID
Length (Bytes)
Payload
Checksum
Message
structure
0xb5 0x62
0x06
0x08
6
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
U2
measRate
-
ms
The elapsed time between GNSS measurements,
which deﬁnes the rate, e.g. 100 ms => 10 Hz, 1000
ms => 1 Hz, 10000 ms => 0.1 Hz. Measurement
rate should be greater than or equal to 25 ms.
(Measurement rate should be greater than or equal to
50 ms for protocol versions less than 24.00).
UBX-22008968 - R01
 
3 UBX protocol
Page 85 of 305
C1-Public
```

---

## Page 86

```
u-blox F9 HPG 1.32 - Interface Description
2
U2
navRate
-
cycles
The ratio between the number of measurements and
the number of navigation solutions, e.g. 5 means
ﬁve measurements for every navigation solution.
Maximum value is 127. (This parameter is ignored and
the navRate is ﬁxed to 1 for protocol versions less than
18.00).
4
U2
timeRef
-
-
The time system to which measurements are aligned:
•
0 = UTC time
•
1 = GPS time
•
2 = GLONASS time (not supported for protocol
versions less than 18.00)
•
3 = BeiDou time (not supported for protocol
versions less than 18.00)
•
4 = Galileo time (not supported for protocol
versions less than 18.00)
•
5 = NavIC time (not supported for protocol
versions less than 29.00)
3.10.18 UBX-CFG-RINV (0x06 0x34)
3.10.18.1 Contents of remote inventory
Message
UBX-CFG-RINV
Contents of remote inventory
Type
Get/set
Comment
This message is deprecated in protocol versions greater than 23.01. Use UBX-CFG-VALSET, UBX-CFG-
VALGET, UBX-CFG-VALDEL instead.
If N is greater than 30, the excess bytes are discarded.
See the Legacy UBX Message Fields Reference for the corresponding conﬁguration item.
Header
Class
ID
Length (Bytes)
Payload
Checksum
Message
structure
0xb5 0x62
0x06
0x34
1 + [0..n]
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
X1
flags
-
-
Flags
bit 0   U:1
dump
-
-
Dump data at startup. Does not work if ﬂag binary is
set.
bit 1   U:1
binary
-
-
Data is binary.
Start of repeated group (N times)
1 + n
U1
data
-
-
Data to store/stored in remote inventory.
End of repeated group (N times)
3.10.19 UBX-CFG-RST (0x06 0x04)
3.10.19.1 Reset receiver / Clear backup data structures
Message
UBX-CFG-RST
Reset receiver / Clear backup data structures
Type
Command
Comment
Do not expect this message to be acknowledged by the receiver.
•
Newer FW version will not acknowledge this message at all.
•
Older FW version will acknowledge this message but the acknowledge may not be sent completely
before the receiver is reset.
UBX-22008968 - R01
 
3 UBX protocol
Page 86 of 305
C1-Public
```

---

## Page 87

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
0x06
0x04
4
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
X2
navBbrMask
-
-
BBR sections to clear. The following special sets apply:
•
0x0000 Hot start
•
0x0001 Warm start
•
0xFFFF Cold start
bit 0   U:1
eph
-
-
Ephemeris
bit 1   U:1
alm
-
-
Almanac
bit 2   U:1
health
-
-
Health
bit 3   U:1
klob
-
-
Klobuchar parameters
bit 4   U:1
pos
-
-
Position
bit 5   U:1
clkd
-
-
Clock drift
bit 6   U:1
osc
-
-
Oscillator parameter
bit 7   U:1
utc
-
-
UTC correction + GPS leap seconds parameters
bit 8   U:1
rtc
-
-
RTC
bit 11   U:1
sfdr
-
-
SFDR Parameters (only available on the ADR/UDR/
HPS product variant) and weak signal compensation
estimates
bit 12   U:1
vmon
-
-
SFDR Vehicle Monitoring Parameter (only available on
the ADR/UDR/HPS product variant)
bit 13   U:1
tct
-
-
TCT Parameters (only available on the ADR/UDR/HPS
product variant)
bit 15   U:1
aop
-
-
Autonomous orbit parameters
2
U1
resetMode
-
-
Reset Type
•
0x00 = Hardware reset (watchdog) immediately
•
0x01 = Controlled software reset
•
0x02 = Controlled software reset (GNSS only)
•
0x04 = Hardware reset (watchdog) after
shutdown
•
0x08 = Controlled GNSS stop
•
0x09 = Controlled GNSS start
3
U1
reserved0
-
-
Reserved
3.10.20 UBX-CFG-SBAS (0x06 0x16)
3.10.20.1 SBAS conﬁguration
Message
UBX-CFG-SBAS
SBAS conﬁguration
Type
Get/set
Comment
This message is deprecated in protocol versions greater than 23.01. Use UBX-CFG-VALSET, UBX-CFG-
VALGET, UBX-CFG-VALDEL instead.
This message conﬁgures the SBAS receiver subsystem (i.e. WAAS, EGNOS, MSAS).
See SBAS conﬁguration settings description in the integration manual for a detailed description of how these
settings aﬀect receiver operation.
Header
Class
ID
Length (Bytes)
Payload
Checksum
Message
structure
0xb5 0x62
0x06
0x16
8
see below
CK_A CK_B
UBX-22008968 - R01
 
3 UBX protocol
Page 87 of 305
C1-Public
```

---
