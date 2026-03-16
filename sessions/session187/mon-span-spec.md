# PDF抽出: u-blox-F9-HPG-1.32_InterfaceDescription_UBX-22008968.pdf

**総ページ数**: 305
**抽出ページ**: 134, 135

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
