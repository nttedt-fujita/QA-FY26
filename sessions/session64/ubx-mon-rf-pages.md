# PDF抽出: アプリケーションノート_u-blox-F9-HPG-1.32_InterfaceDescription_UBX-22008968.pdf

**総ページ数**: 305
**抽出ページ**: 131, 132, 133, 134, 135

---

## Page 131

```
u-blox F9 HPG 1.32 - Interface Description
4 + n·20
U4
txBytes
-
bytes
Number of bytes ever sent
8 + n·20
U2
parityErrs
-
-
Number of 100 ms timeslots with parity errors
10 + n·20
U2
framingErrs
-
-
Number of 100 ms timeslots with framing errors
12 + n·20
U2
overrunErrs
-
-
Number of 100 ms timeslots with overrun errors
14 + n·20
U2
breakCond
-
-
Number of 100 ms timeslots with break conditions
16 + n·20
U1[4]
reserved0
-
-
Reserved
End of repeated group (N times)
3.14.7 UBX-MON-MSGPP (0x0a 0x06)
3.14.7.1 Message parse and process status
Message
UBX-MON-MSGPP
Message parse and process status
Type
Periodic/polled
Comment
This message is deprecated in this protocol version. Use UBX-MON-COMMS instead.
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
0x06
120
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
U2[8]
msg1
-
msgs
Number of successfully parsed messages for each
protocol on port0
16
U2[8]
msg2
-
msgs
Number of successfully parsed messages for each
protocol on port1
32
U2[8]
msg3
-
msgs
Number of successfully parsed messages for each
protocol on port2
48
U2[8]
msg4
-
msgs
Number of successfully parsed messages for each
protocol on port3
64
U2[8]
msg5
-
msgs
Number of successfully parsed messages for each
protocol on port4
80
U2[8]
msg6
-
msgs
Number of successfully parsed messages for each
protocol on port5
96
U4[6]
skipped
-
bytes
Number skipped bytes for each port
3.14.8 UBX-MON-PATCH (0x0a 0x27)
3.14.8.1 Installed patches
Message
UBX-MON-PATCH
Installed patches
Type
Polled
Comment
This message reports information about patches installed and currently enabled on the receiver. It does
not report on patches installed and then disabled. An enabled patch is considered active when the receiver
executes from the code space where the patch resides on. For example, a ROM patch is reported active only
when the system runs from ROM.
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
0x27
4 + nEntries·16
see below
CK_A CK_B
Payload description:
Byte offset
Type
Name
Scale
Unit
Description
UBX-22008968 - R01
 
3 UBX protocol
Page 131 of 305
C1-Public
```

---

## Page 132

```
u-blox F9 HPG 1.32 - Interface Description
0
U2
version
-
-
Message version (0x0001 for this version)
2
U2
nEntries
-
-
Total number of reported patches
Start of repeated group (nEntries times)
4 + n·16
X4
patchInfo
-
-
Status information about the reported patch
bit 0   U:1
activated
-
-
1: the patch is active, 0: otherwise
bits 2…1   U:2
location
-
-
Indicates where the patch is stored. 0: eFuse, 1: ROM,
2: BBR, 3: ﬁle system
8 + n·16
U4
comparator
Number
-
-
The number of the comparator
12 + n·16
U4
patchAddress
-
-
The address that is targeted by the patch
16 + n·16
U4
patchData
-
-
The data that is inserted at the patchAddress
End of repeated group (nEntries times)
3.14.9 UBX-MON-RF (0x0a 0x38)
3.14.9.1 RF information
Message
UBX-MON-RF
RF information
Type
Periodic/polled
Comment
Information for each RF block. There are as many RF blocks reported as bands supported by this receiver.
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
0x38
4 + nBlocks·24
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
nBlocks
-
-
The number of RF blocks included
2
U1[2]
reserved0
-
-
Reserved
Start of repeated group (nBlocks times)
4 + n·24
U1
blockId
-
-
RF block ID (0 = L1 band, 1 = L2 or L5 band depending
on product conﬁguration)
5 + n·24
X1
flags
-
-
Flags
bits 1…0   U:2
jammingState
-
-
output from Jamming/Interference Monitor (0 =
unknown or feature disabled, 1 = ok - no signiﬁcant
jamming, 2 = warning - interference visible but ﬁx
OK, 3 = critical - interference visible and no ﬁx). This
ﬂag is deprecated in protocol versions that support
UBX-SEC-SIG (version 0x02); instead jammingState in
UBX-SEC-SIG should be monitored.
6 + n·24
U1
antStatus
-
-
Status 
of 
the 
antenna 
supervisor 
state
machine (0x00=INIT, 0x01=DONTKNOW, 0x02=OK,
0x03=SHORT, 0x04=OPEN)
7 + n·24
U1
antPower
-
-
Current 
power 
status 
of 
antenna 
(0x00=OFF,
0x01=ON, 0x02=DONTKNOW)
8 + n·24
U4
postStatus
-
-
POST status word
12 + n·24
U1[4]
reserved1
-
-
Reserved
16 + n·24
U2
noisePerMS
-
-
Noise level as measured by the GPS core
UBX-22008968 - R01
 
3 UBX protocol
Page 132 of 305
C1-Public
```

---

## Page 133

```
u-blox F9 HPG 1.32 - Interface Description
18 + n·24
U2
agcCnt
-
-
AGC Monitor (counts SIGHI xor SIGLO, range 0 to
8191)
20 + n·24
U1
cwSuppression
-
-
CW interference suppression level, scaled (0=no CW
jamming, 255 = strong CW jamming)
21 + n·24
I1
ofsI
-
-
Imbalance of I-part of complex signal, scaled (-128
= max. negative imbalance, 127 = max. positive
imbalance)
22 + n·24
U1
magI
-
-
Magnitude of I-part of complex signal, scaled (0 = no
signal, 255 = max.magnitude)
23 + n·24
I1
ofsQ
-
-
Imbalance of Q-part of complex signal, scaled (-128
= max. negative imbalance, 127 = max. positive
imbalance)
24 + n·24
U1
magQ
-
-
Magnitude of Q-part of complex signal, scaled (0 = no
signal, 255 = max.magnitude)
25 + n·24
U1[3]
reserved2
-
-
Reserved
End of repeated group (nBlocks times)
3.14.10 UBX-MON-RXBUF (0x0a 0x07)
3.14.10.1 Receiver buﬀer status
Message
UBX-MON-RXBUF
Receiver buﬀer status
Type
Periodic/polled
Comment
This message is deprecated in this protocol version. Use UBX-MON-COMMS instead.
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
0x07
24
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
U2[6]
pending
-
bytes
Number of bytes pending in receiver buﬀer for each
target
12
U1[6]
usage
-
%
Maximum usage receiver buﬀer during the last
sysmon period for each target
18
U1[6]
peakUsage
-
%
Maximum usage receiver buﬀer for each target
3.14.11 UBX-MON-RXR (0x0a 0x21)
3.14.11.1 Receiver status information
Message
UBX-MON-RXR
Receiver status information
Type
Output
Comment
The receiver ready message is sent when the receiver changes from or to backup mode.
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
0x21
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
X1
flags
-
-
Receiver status ﬂags
UBX-22008968 - R01
 
3 UBX protocol
Page 133 of 305
C1-Public
```

---

## Page 134

```
u-blox F9 HPG 1.32 - Interface Description
bit 0   U:1
awake
-
-
not in backup mode
3.14.12 UBX-MON-SPAN (0x0a 0x31)
3.14.12.1 Signal characteristics
Message
UBX-MON-SPAN
Signal characteristics
Type
Periodic/polled
Comment
This message is to be used as a basic spectrum analyzer, where it displays one spectrum for each of the
receiver's existing RF paths. The spectrum is conveyed with the following parameters: The frequency span
in Hz, the frequency bin resolution in Hz, the center frequency in Hz, and 256 bins with amplitude data.
Additionally, in order to give further insight on the signal captured by the receiver, the current gain of the
internal programmable gain ampliﬁer (PGA) is provided.
This message gives information for comparative analysis rather than absolute and precise spectrum
overview. Users should not expect highly accurate spectrum amplitude.
Note that the PGA gain is not included in the spectrum data but is available as a separate ﬁeld. Neither the
spectrum, nor the PGA gain considers the internal ﬁxed LNA gain or an external third-party LNA.
The center frequency at each bin, assuming a zero-based bin count, can be computed as
f(i) = center + span * (i - 127) / 256
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
0x31
4 + numRfBlocks·272
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
numRfBlocks
-
-
Number of RF blocks included
2
U1[2]
reserved0
-
-
Reserved
Start of repeated group (numRfBlocks times)
4 + n·272
U1[256]
spectrum
-
dB
Spectrum data (number of points = span/res)
260 + n·272
U4
span
-
Hz
Spectrum span
264 + n·272
U4
res
-
Hz
Resolution of the spectrum
268 + n·272
U4
center
-
Hz
Center of spectrum span
272 + n·272
U1
pga
-
dB
Programmable gain ampliﬁer
273 + n·272
U1[3]
reserved1
-
-
Reserved
End of repeated group (numRfBlocks times)
3.14.13 UBX-MON-SYS (0x0a 0x39)
3.14.13.1 Current system performance information
Message
UBX-MON-SYS
Current system performance information
Type
Periodic/polled
Comment
This message contains operationally relevant system information for monitoring purposes.
cpuLoadMax value is only valid, if 1 second output frequency is set.
Detailed information about ioUsage/ioUsageMax are available in UBX-MON-COMMS message.
tempValue has an accuracy of +/- 2 deg.
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
0x39
24
see below
CK_A CK_B
UBX-22008968 - R01
 
3 UBX protocol
Page 134 of 305
C1-Public
```

---

## Page 135

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
msgVer
-
-
Message Version (0x01)
1
U1
bootType
-
-
Boot type of master chip
0-Unknown
1-Cold Start
2-Watchdog
3-Hardware reset
4-Hardware backup
5-Software backup
6-Software reset
7-VIO fail
8-VDD_X fail
9-VDD_RF fail
10-V_CORE_HIGH fail
2
U1
cpuLoad
-
-
Highest actual load of realtime tasks of all CPUs in %
3
U1
cpuLoadMax
-
-
Maximal CPU load value in % seen since last restart
4
U1
memUsage
-
-
Highest actual dynamic memory usage of all CPUs in
%
5
U1
memUsageMax
-
-
Maximal dynamic memory usage in % seen since last
restart
6
U1
ioUsage
-
-
Highest actual IO bandwidth usage of all rx/tx
interfaces in %
7
U1
ioUsageMax
-
-
Maximal bandwidth usage of all rx/tx interfaces in %
seen since last restart
8
U4
runTime
-
sec
Time since last restart
12
U2
noticeCount
-
-
Number of notices occured since last restart
14
U2
warnCount
-
-
Number of warnings occured since last restart
16
U2
errorCount
-
-
Number of errors occured since last restart
18
I1
tempValue
-
-
Temperature value [C]
19
U1[5]
reserved0
-
-
Reserved
3.14.14 UBX-MON-TXBUF (0x0a 0x08)
3.14.14.1 Transmitter buﬀer status
Message
UBX-MON-TXBUF
Transmitter buﬀer status
Type
Periodic/polled
Comment
This message is deprecated in this protocol version. Use UBX-MON-COMMS instead.
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
0x08
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
U2[6]
pending
-
bytes
Number of bytes pending in transmitter buﬀer for
each target
12
U1[6]
usage
-
%
Maximum usage transmitter buﬀer during the last
sysmon period for each target
UBX-22008968 - R01
 
3 UBX protocol
Page 135 of 305
C1-Public
```

---
