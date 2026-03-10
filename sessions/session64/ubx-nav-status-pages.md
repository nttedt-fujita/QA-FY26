# PDF抽出: アプリケーションノート_u-blox-F9-HPG-1.32_InterfaceDescription_UBX-22008968.pdf

**総ページ数**: 305
**抽出ページ**: 154, 155, 156, 157, 158

---

## Page 154

```
u-blox F9 HPG 1.32 - Interface Description
17 + n·16
U1
ionoModel
-
-
Ionospheric model used:
•
0 = no model
•
1 = Klobuchar model transmitted by GPS
•
2 = SBAS model
•
3 = Klobuchar model transmitted by BeiDou
•
8 = Iono delay derived from dual frequency
observations
18 + n·16
X2
sigFlags
-
-
Signal related ﬂags
bits 1…0   U:2
health
-
-
Signal health ﬂag:
•
0 = unknown
•
1 = healthy
•
2 = unhealthy
bit 2   U:1
prSmoothed
-
-
1 = Pseudorange has been smoothed
bit 3   U:1
prUsed
-
-
1 = Pseudorange has been used for this signal
bit 4   U:1
crUsed
-
-
1 = Carrier range has been used for this signal
bit 5   U:1
doUsed
-
-
1 = Range rate (Doppler) has been used for this signal
bit 6   U:1
prCorrUsed
-
-
1 = Pseudorange corrections have been used for this
signal
bit 7   U:1
crCorrUsed
-
-
1 = Carrier range corrections have been used for this
signal
bit 8   U:1
doCorrUsed
-
-
1 = Range rate (Doppler) corrections have been used
for this signal
20 + n·16
U1[4]
reserved1
-
-
Reserved
End of repeated group (numSigs times)
3.15.19 UBX-NAV-SLAS (0x01 0x42)
3.15.19.1 QZSS L1S SLAS status data
Message
UBX-NAV-SLAS
QZSS L1S SLAS status data
Type
Periodic/polled
Comment
This message outputs the status of the QZSS L1S SLAS sub system
Header
Class
ID
Length (Bytes)
Payload
Checksum
Message
structure
0xb5 0x62
0x01
0x42
20 + cnt·8
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
iTOW
-
ms
GPS time of week of the navigation epoch.
See the description of iTOW for details.
4
U1
version
-
-
Message version (0x00 for this version)
5
U1[3]
reserved0
-
-
Reserved
8
I4
gmsLon
1e-3
deg
Longitude of the used ground monitoring station
12
I4
gmsLat
1e-3
deg
Latitude of the used ground monitoring station
16
U1
gmsCode
-
-
Code of the used ground monitoring station according
to the QZSS SLAS Interface Speciﬁcation, available
from qzss.go.jp/en/
17
U1
qzssSvId
-
-
Satellite identiﬁer of the QZS/GEO whose correction
data is used (see Satellite Numbering)
18
X1
serviceFlags
-
-
Flags regarding SLAS service
UBX-22008968 - R01
 
3 UBX protocol
Page 154 of 305
C1-Public
```

---

## Page 155

```
u-blox F9 HPG 1.32 - Interface Description
bit 0   U:1
gmsAvailable
-
-
1 = Ground monitoring station available
bit 1   U:1
qzssSv
Available
-
-
1 = Correction providing QZSS SV available
bit 2   U:1
testMode
-
-
1 = Currently used QZSS SV in test mode
19
U1
cnt
-
-
Number of pseudorange corrections following
Start of repeated group (cnt times)
20 + n·8
U1
gnssId
-
-
GNSS identiﬁer (see Satellite Numbering)
21 + n·8
U1
svId
-
-
Satellite identiﬁer (see Satellite Numbering)
22 + n·8
U1
reserved1
-
-
Reserved
23 + n·8
U1[3]
reserved2
-
-
Reserved
26 + n·8
I2
prc
-
cm
Pseudorange correction
End of repeated group (cnt times)
3.15.20 UBX-NAV-STATUS (0x01 0x03)
3.15.20.1 Receiver navigation status
Message
UBX-NAV-STATUS
Receiver navigation status
Type
Periodic/polled
Comment
See important comments concerning validity of position given in section Navigation output ﬁlters in the
integration manual.
Header
Class
ID
Length (Bytes)
Payload
Checksum
Message
structure
0xb5 0x62
0x01
0x03
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
iTOW
-
ms
GPS time of week of the navigation epoch.
See section iTOW timestamps in the integration
manual for details.
4
U1
gpsFix
-
-
GPSﬁx Type, this value does not qualify a ﬁx as valid
and within the limits. See note on ﬂag gpsFixOk below.
•
0x00 = no ﬁx
•
0x01 = dead reckoning only
•
0x02 = 2D-ﬁx
•
0x03 = 3D-ﬁx
•
0x04 = GPS + dead reckoning combined
•
0x05 = Time only ﬁx
•
0x06..0xﬀ = reserved
5
X1
flags
-
-
Navigation Status Flags
bit 0   U:1
gpsFixOk
-
-
1 = position and velocity valid and within DOP and ACC
Masks.
bit 1   U:1
diffSoln
-
-
1 = diﬀerential corrections were applied
bit 2   U:1
wknSet
-
-
1 = Week Number valid (see section Time validity in the
integration manual for details)
bit 3   U:1
towSet
-
-
1 = Time of Week valid (see section Time validity in the
integration manual for details)
6
X1
fixStat
-
-
Fix Status Information
bit 0   U:1
diffCorr
-
-
1 = diﬀerential corrections available
UBX-22008968 - R01
 
3 UBX protocol
Page 155 of 305
C1-Public
```

---

## Page 156

```
u-blox F9 HPG 1.32 - Interface Description
bit 1   U:1
carrSolnValid
-
-
1 = valid carrSoln
bits 7…6   U:2
mapMatching
-
-
map matching status:
•
00: none
•
01: valid but not used, i.e. map matching data was
received, but was too old
•
10: valid and used, map matching data was
applied
•
11: valid and used, map matching data was
applied. In case of sensor unavailability map
matching data enables dead reckoning. This
requires map matched latitude/longitude or
heading data.
7
X1
flags2
-
-
further information about navigation output
bits 1…0   U:2
psmState
-
-
power save mode state (not supported for protocol
versions less than 13.01)
•
0 = ACQUISITION [or when psm disabled]
•
1 = TRACKING
•
2 = POWER OPTIMIZED TRACKING
•
3 = INACTIVE
bits 4…3   U:2
spoofDetState
-
-
Spooﬁng detection state (not supported for protocol
versions less than 18.00)
•
0: Unknown or deactivated
•
1: No spooﬁng indicated
•
2: Spooﬁng indicated
•
3: Multiple spooﬁng indications
Note that the spooﬁng state value only reﬂects the
detector state for the current navigation epoch. As
spooﬁng can be detected most easily at the transition
from real signal to spooﬁng signal, this is also where
the detector is triggered the most. I.e. a value of 1 - No
spoofing indicated does not mean that the receiver is
not spoofed, it simply states that the detector was not
triggered in this epoch.
bits 7…6   U:2
carrSoln
-
-
Carrier phase range solution status:
•
0 = no carrier phase range solution
•
1 = carrier phase range solution with ﬂoating
ambiguities
•
2 = carrier phase range solution with ﬁxed
ambiguities
8
U4
ttff
-
ms
Time to ﬁrst ﬁx (millisecond time tag)
12
U4
msss
-
ms
Milliseconds since Startup / Reset
3.15.21 UBX-NAV-SVIN (0x01 0x3b)
3.15.21.1 Survey-in data
Message
UBX-NAV-SVIN
Survey-in data
Type
Periodic/polled
Comment
This message contains information about survey-in parameters.
Header
Class
ID
Length (Bytes)
Payload
Checksum
Message
structure
0xb5 0x62
0x01
0x3b
40
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
Page 156 of 305
C1-Public
```

---

## Page 157

```
u-blox F9 HPG 1.32 - Interface Description
0
U1
version
-
-
Message version (0x00 for this version)
1
U1[3]
reserved0
-
-
Reserved
4
U4
iTOW
-
ms
GPS time of week of the navigation epoch.
See the description of iTOW for details.
8
U4
dur
-
s
Passed survey-in observation time
12
I4
meanX
-
cm
Current survey-in mean position ECEF X coordinate
16
I4
meanY
-
cm
Current survey-in mean position ECEF Y coordinate
20
I4
meanZ
-
cm
Current survey-in mean position ECEF Z coordinate
24
I1
meanXHP
-
0.1_mm
Current high-precision survey-in mean position ECEF
X coordinate. Must be in the range -99..+99.
The current survey-in mean position ECEF X
coordinate, in units of cm, is given by
meanX + (0.01 * meanXHP)
25
I1
meanYHP
-
0.1_mm
Current high-precision survey-in mean position ECEF
Y coordinate. Must be in the range -99..+99.
The current survey-in mean position ECEF Y
coordinate, in units of cm, is given by
meanY + (0.01 * meanYHP)
26
I1
meanZHP
-
0.1_mm
Current high-precision survey-in mean position ECEF
Z coordinate. Must be in the range -99..+99.
The current survey-in mean position ECEF Z
coordinate, in units of cm, is given by
meanZ + (0.01 * meanZHP)
27
U1
reserved1
-
-
Reserved
28
U4
meanAcc
-
0.1_mm
Current survey-in mean position accuracy
32
U4
obs
-
-
Number of position observations used during survey-
in
36
U1
valid
-
-
Survey-in position validity ﬂag, 1 = valid, otherwise 0
37
U1
active
-
-
Survey-in in progress ﬂag, 1 = in-progress, otherwise 0
38
U1[2]
reserved2
-
-
Reserved
3.15.22 UBX-NAV-TIMEBDS (0x01 0x24)
3.15.22.1 BeiDou time solution
Message
UBX-NAV-TIMEBDS
BeiDou time solution
Type
Periodic/polled
Comment
This message reports the precise BDS time of the most recent navigation solution including validity ﬂags and
an accuracy estimate.
Header
Class
ID
Length (Bytes)
Payload
Checksum
Message
structure
0xb5 0x62
0x01
0x24
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
U4
iTOW
-
ms
GPS time of week of the navigation epoch.
See section iTOW timestamps in the integration
manual for details.
4
U4
SOW
-
s
BDS time of week (rounded to seconds)
UBX-22008968 - R01
 
3 UBX protocol
Page 157 of 305
C1-Public
```

---

## Page 158

```
u-blox F9 HPG 1.32 - Interface Description
8
I4
fSOW
-
ns
Fractional part of SOW (range: +/-500000000).
The precise BDS time of week in seconds is:
SOW + fSOW * 1e-9
12
I2
week
-
-
BDS week number of the navigation epoch
14
I1
leapS
-
s
BDS leap seconds (BDS-UTC)
15
X1
valid
-
-
Validity Flags
bit 0   U:1
sowValid
-
-
1 = Valid SOW and fSOW (see section Time validity in
the integration manual for details)
bit 1   U:1
weekValid
-
-
1 = Valid week (see section Time validity in the
integration manual for details)
bit 2   U:1
leapSValid
-
-
1 = Valid leap second
16
U4
tAcc
-
ns
Time Accuracy Estimate
3.15.23 UBX-NAV-TIMEGAL (0x01 0x25)
3.15.23.1 Galileo time solution
Message
UBX-NAV-TIMEGAL
Galileo time solution
Type
Periodic/polled
Comment
This message reports the precise Galileo time of the most recent navigation solution including validity ﬂags
and an accuracy estimate.
Header
Class
ID
Length (Bytes)
Payload
Checksum
Message
structure
0xb5 0x62
0x01
0x25
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
U4
iTOW
-
ms
GPS time of week of the navigation epoch.
See section iTOW timestamps in the integration
manual for details.
4
U4
galTow
-
s
Galileo time of week (rounded to seconds)
8
I4
fGalTow
-
ns
Fractional part of the Galileo time of week (range:
+/-500000000).
The precise Galileo time of week in seconds is:
galTow + fGalTow * 1e-9
12
I2
galWno
-
-
Galileo week number
14
I1
leapS
-
s
Galileo leap seconds (Galileo-UTC)
15
X1
valid
-
-
Validity Flags
bit 0   U:1
galTowValid
-
-
1 = Valid galTow and fGalTow (see section Time validity
in the integration manual for details)
bit 1   U:1
galWnoValid
-
-
1 = Valid galWno (see section Time validity in the
integration manual for details)
bit 2   U:1
leapSValid
-
-
1 = Valid leapS
16
U4
tAcc
-
ns
Time Accuracy Estimate
3.15.24 UBX-NAV-TIMEGLO (0x01 0x23)
UBX-22008968 - R01
 
3 UBX protocol
Page 158 of 305
C1-Public
```

---
