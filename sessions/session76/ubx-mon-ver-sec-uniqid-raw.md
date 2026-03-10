# PDF抽出: アプリケーションノート_u-blox-F9-HPG-1.32_InterfaceDescription_UBX-22008968.pdf

**総ページ数**: 305
**抽出ページ**: 135, 136, 194, 195

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

## Page 136

```
u-blox F9 HPG 1.32 - Interface Description
18
U1[6]
peakUsage
-
%
Maximum usage transmitter buﬀer for each target
24
U1
tUsage
-
%
Maximum usage of transmitter buﬀer during the last
sysmon period for all targets
25
U1
tPeakusage
-
%
Maximum usage of transmitter buﬀer for all targets
26
X1
errors
-
-
Error bitmask
bits 5…0   U:6
limit
-
-
Buﬀer limit of corresponding target reached
bit 6   U:1
mem
-
-
Memory Allocation error
bit 7   U:1
alloc
-
-
Allocation error (TX buﬀer full)
27
U1
reserved0
-
-
Reserved
3.14.15 UBX-MON-VER (0x0a 0x04)
3.14.15.1 Receiver and software version
Message
UBX-MON-VER
Receiver and software version
Type
Polled
Comment
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
0x04
40 + [0..n]·30
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
CH[30]
swVersion
-
-
Nul-terminated software version string.
30
CH[10]
hwVersion
-
-
Nul-terminated hardware version string
Start of repeated group (N times)
40 + n·30
CH[30]
extension
-
-
Extended software information strings.
A series of nul-terminated strings. Each extension
ﬁeld is 30 characters long and contains varying
software information. Not all extension ﬁelds may
appear.
Examples of reported information: the software
version string of the underlying ROM (when the
receiver's ﬁrmware is running from ﬂash), the
ﬁrmware version, the supported protocol version, the
module identiﬁer, the ﬂash information structure
(FIS) ﬁle information, the supported major GNSS, the
supported augmentation systems.
See Firmware and protocol versions for details.
End of repeated group (N times)
3.15 UBX-NAV (0x01)
The messages in the UBX-NAV class are used to output navigation results and data, such as
position, altitude and velocity in a number of formats, and status ﬂags and accuracy estimate
ﬁgures, or satellite and signal information. The messages are generated with the conﬁgured
navigation rate.
3.15.1 UBX-NAV-CLOCK (0x01 0x22)
UBX-22008968 - R01
 
3 UBX protocol
Page 136 of 305
C1-Public
```

---

## Page 194

```
u-blox F9 HPG 1.32 - Interface Description
4 +
numKeys·8 +
n
U1
key
-
-
Key(s) payload. This is a concatenation of all keys as
raw bytes. The number of keys is deﬁned in 'numKeys'
ﬁeld. Each key length is deﬁned in its 'keyLengthBytes'
ﬁeld.
End of repeated group (N times)
3.18 UBX-SEC (0x27)
The messages in the UBX-SEC class are used for security features of the receiver.
3.18.1 UBX-SEC-UNIQID (0x27 0x03)
3.18.1.1 Unique chip ID
Message
UBX-SEC-UNIQID
Unique chip ID
Type
Output
Comment
This message is used to retrieve a unique chip identiﬁer (40 bits, 5 bytes).
Header
Class
ID
Length (Bytes)
Payload
Checksum
Message
structure
0xb5 0x62
0x27
0x03
9
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
U1[5]
uniqueId
-
-
Unique chip ID
3.19 UBX-TIM (0x0d)
The messages in the UBX-TIM class are used to output timing information from the receiver, such
as time pulse and time mark measurements.
3.19.1 UBX-TIM-TM2 (0x0d 0x03)
3.19.1.1 Time mark data
Message
UBX-TIM-TM2
Time mark data
Type
Periodic/polled
Comment
This message contains information for high precision time stamping / pulse counting.
The delay ﬁgures and timebase given in UBX-CFG-TP5 are also applied to the time results output in this
message.
Header
Class
ID
Length (Bytes)
Payload
Checksum
Message
structure
0xb5 0x62
0x0d
0x03
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
U1
ch
-
-
Channel (i.e. EXTINT) upon which the pulse was
measured
1
X1
flags
-
-
Bitmask
bit 0   U:1
mode
-
-
•
0=single
•
1=running
bit 1   U:1
run
-
-
•
0=armed
UBX-22008968 - R01
 
3 UBX protocol
Page 194 of 305
C1-Public
```

---

## Page 195

```
u-blox F9 HPG 1.32 - Interface Description
•
1=stopped
bit 2   U:1
newFallingEdge
-
-
New falling edge detected
bits 4…3   U:2
timeBase
-
-
•
0=Time base is Receiver time
•
1=Time base is GNSS time (the system according
to the conﬁguration in UBX-CFG-TP5 for tpIdx=0)
•
2=Time base is UTC (the variant according to the
conﬁguration in UBX-CFG-NAV5)
bit 5   U:1
utc
-
-
•
0=UTC not available
•
1=UTC available
bit 6   U:1
time
-
-
•
0=Time is not valid
•
1=Time is valid (Valid GNSS ﬁx)
bit 7   U:1
newRisingEdge
-
-
New rising edge detected
2
U2
count
-
-
Rising edge counter
4
U2
wnR
-
-
Week number of last rising edge
6
U2
wnF
-
-
Week number of last falling edge
8
U4
towMsR
-
ms
Tow of rising edge
12
U4
towSubMsR
-
ns
Millisecond fraction of tow of rising edge in
nanoseconds
16
U4
towMsF
-
ms
Tow of falling edge
20
U4
towSubMsF
-
ns
Millisecond fraction of tow of falling edge in
nanoseconds
24
U4
accEst
-
ns
Accuracy estimate
3.19.2 UBX-TIM-TP (0x0d 0x01)
3.19.2.1 Time pulse time data
Message
UBX-TIM-TP
Time pulse time data
Type
Periodic/polled
Comment
This message contains information on the timing of the next pulse at the TIMEPULSE0 output. The
recommended conﬁguration when using this message is to set both the measurement rate (CFG-RATE) and
the timepulse frequency (CFG-TP) to 1 Hz.
Header
Class
ID
Length (Bytes)
Payload
Checksum
Message
structure
0xb5 0x62
0x0d
0x01
16
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
U4
towMS
-
ms
Time pulse time of week according to time base
4
U4
towSubMS
2^-32
ms
Submillisecond part of towMS
8
I4
qErr
-
ps
Quantization error of time pulse
12
U2
week
-
weeks
Time pulse week number according to time base
14
X1
flags
-
-
Flags
bit 0   U:1
timeBase
-
-
•
0 = Time base is GNSS
•
1 = Time base is UTC
bit 1   U:1
utc
-
-
•
0 = UTC not available
•
1 = UTC available
bits 3…2   U:2
raim
-
-
(T)RAIM information
•
0 = Information not available
•
1 = Not active
UBX-22008968 - R01
 
3 UBX protocol
Page 195 of 305
C1-Public
```

---
