# PDF抽出: アプリケーションノート_u-blox-F9-HPG-1.32_InterfaceDescription_UBX-22008968.pdf

**総ページ数**: 305
**抽出ページ**: 18, 19, 20, 150, 151, 152, 153, 154

---

## Page 18

```
u-blox F9 HPG 1.32 - Interface Description
UBX-MON-VER.   Similarly, the NMEA-Standard-GGA is the NMEA standard message (sentence) with
the global positioning ﬁx data.
References to ﬁelds of the message add the ﬁeld name separated by a dot ("."), e.g. UBX-MON-
VER.swVersion.
Some messages use a fourth level of naming, called the message version. One example is the UBX-
MGA-GPS message for GPS assistance data, which exists in versions for ephemerides (UBX-MGA-
GPS-EPH) and almanacs (UBX-MGA-GPS-ALM).
Names of conﬁguration items are of the form CFG-GROUP-ITEM. For example, CFG-NAVSPG-
DYNMODEL refers to the navigation dynamic platform model the receiver uses. Constants add
a fourth level to the item name, such as CFG-NAVSPG-DYNMODEL-AUTOMOT for the automotive
platform model. In the context of describing an item's value, only the last part of the constant name
can be used (e.g. "set CFG-NAVSPG-DYNMODEL to PORT for portable applications").
1.5 GNSS, satellite, and signal identiﬁers
1.5.1 Overview
Some UBX protocol messages contain infomation about speciﬁc satellites. Any single satellite can
be identiﬁed by a gnssId ﬁeld indicating the GNSS the satellite is part of and an svId (SV for
space vehicle) ﬁeld indicating the number of the satellite in that system. Usually, the svId is the
native number associated with the satellite in the speciﬁc GNSS. For example the GLONASS SV4 is
identiﬁed as gnssId 6, svId 4, while the GPS SV4 is gnssId 0, svId 4.
GLONASS satellites can be tracked before they have been identiﬁed. In UBX messages, the unknown
satellites will be reported with svId 255. In NMEA messages, the unknown satellites will be
null (empty) ﬁelds. Product-related documentation and u-center will use R? to label unidentiﬁed
GLONASS satellites.
Signal identiﬁers are used when diﬀerent signals from the same GNSS satellite need to be
distinguished (e.g. in the UBX-NAV-SIG message). A separate sigId ﬁeld identiﬁes the signal. These
signal identiﬁers are only valid when combined with a GNSS identiﬁer (gnssId ﬁeld).
The NMEA protocol (version 4.10 and later) identiﬁes GNSS satellites with a one-digit system ID
and a two-digit satellite number. u-blox receivers support this method in their NMEA output when
"strict" SV numbering is selected. In most cases this is the default setting, but it can be checked or
changed using the Conﬁguration interface (see also NMEA GNSS, satellite, and signal numbering).
In order to support some GNSS (e.g. BeiDou, Galileo, QZSS), which are not supported by some or
all NMEA protocol versions, an "extended" SV numbering scheme can be enabled. This uses the
NMEA-deﬁned numbers where possible but adds other number ranges to support other GNSS. Note
however that these non-standard extensions require 3-digit numbers, which may not be supported
by some NMEA parsing software. For example, QZSS satellites use numbers in the range 193 to 202.
The NMEA standard deﬁnes signal identiﬁers to distinguish diﬀerent signals sent by a single
GNSS satellite (e.g. L2 CL and CM). u-blox positioning receivers use those identiﬁers for signal
identiﬁcation, as far as the corresponding standard is supported in a particular product.
UBX-22008968 - R01
 
1 General information
Page 18 of 305
C1-Public
```

---

## Page 19

```
u-blox F9 HPG 1.32 - Interface Description
Note that the following sections are a generic overview for diﬀerent u-blox positioning
receivers. A particular product may not support all of the described GNSS identiﬁers,
satellite numbers, signal identiﬁers or combinations thereof.
1.5.2 GNSS identiﬁers
Table 1 lists each GNSS along with the GNSS identiﬁer (UBX protocol),  the NMEA system identiﬁers
(NMEA protocol), and abbreviations used in this document: 
GNSS
Abbreviations
UBX gnssId
NMEA system ID
2.3 - 4.0
4.10
4.11
GPS
GPS
G
0
1
1
1
SBAS
SBAS
S
1
1
1
1
Galileo
GAL
E
2
n/a
3
3
BeiDou
BDS
B
3
n/a
(4)1
4
IMES
IMES
I
4
n/a
n/a
n/a
QZSS
QZSS
Q
5
n/a
(1)1
5
GLONASS
GLO
R
6
2
2
2
NavIC
NavIC
N
7
n/a
n/a
6
Table 1: GNSS identiﬁers
See also NMEA Talker ID.
1.5.3 Satellite identiﬁers
The satellite numbering scheme for the UBX protocol is provided in Table 2. The satellite numbering
scheme for the NMEA protocol is provided in Table 3.
GNSS
SV Range
gnssId:svId
GPS
G1-G32
0:1-32
SBAS
S120-S158
1:120-158
Galileo
E1-E36
2:1-36
BeiDou
B1-B5
3:1-5
B6-B37
3:6-37
B38-B63
3:38-63
IMES
I1-I10
4:1-10
QZSS
Q1-Q10
5:1-10
GLONASS
R1-R32
6:1-32
R?
6:255
NavIC
N1-N7
7:1-7
Table 2: UBX protocol satellite numbering scheme
NMEA 2.3 - 4.0
NMEA 4.10
NMEA 4.11
GNSS
SV Range
strict
extended
strict
extended
strict
extended
GPS
G1-G32
1-32
1-32
1-32
1-32
1-32
1-32
SBAS
S120-S158
33-64
33-64, 152-158
33-64
33-64, 152-158
33-64
33-64, 152-158
Galileo
E1-E36
211-246
301-336
1-36
1-36
1-36
1-36
BeiDou
B1-B5
159-163
401-405
1-5
1-5
1-5
1-5
1 While not deﬁned by NMEA 4.10, u-blox receivers in this mode will use system ID 4 for BeiDou and, if extended satellite
numbering is enabled, system ID 1 for QZSS.
UBX-22008968 - R01
 
1 General information
Page 19 of 305
C1-Public
```

---

## Page 20

```
u-blox F9 HPG 1.32 - Interface Description
NMEA 2.3 - 4.0
NMEA 4.10
NMEA 4.11
GNSS
SV Range
strict
extended
strict
extended
strict
extended
B6-B37
33-64
406-437
6-37
6-37
6-37
6-37
B38-B63
n/a
438-463
38-63
38-63
38-63
38-63
IMES
I1-I10
173-182
173-182
n/a
173-182
n/a
173-182
QZSS
Q1-Q10
193-202
193-202
n/a
193-202
1-10
1-10
GLONASS
R1-R32
65-96
65-96
65-96
65-96
65-96
65-96
R?
255
null
null
null
null
null
NavIC
N1-N7
247-253
n/a
n/a
n/a
n/a
n/a
Table 3: NMEA protocol satellite numbering scheme
1.5.4 Signal identiﬁers
A summary of all the signal identiﬁcation schemes used in the NMEA protocol and  the UBX protocol
is provided in the following table. (Only a subset of the signals is supported by each product.) In
NMEA protocol, the system and signal identiﬁers are in hexadecimal format.   
UBX Protocol
NMEA Protocol 4.10
NMEA Protocol 4.11
Signal
gnssId
sigId
System ID
Signal ID
System ID
Signal ID
GPS L1C/A2
0
0
1
1
1
1
GPS L2 CL
0
3
1
6
1
6
GPS L2 CM
0
4
1
5
1
5
GPS L5 I
0
6
1
7
1
7
GPS L5 Q
0
7
1
8
1
8
SBAS L1C/A2
1
0
1
1
1
1
Galileo E1 C2
2
0
3
7
3
7
Galileo E1 B2
2
1
3
7
3
7
Galileo E5 aI
2
3
3
1
3
1
Galileo E5 aQ
2
4
3
1
3
1
Galileo E5 bI
2
5
3
2
3
2
Galileo E5 bQ
2
6
3
2
3
2
BeiDou B1I D12
3
0
(4)3
(1)4
4
1
BeiDou B1I D22
3
1
(4)3
(1)4
4
1
BeiDou B2I D1
3
2
(4)3
(3)4
4
B
BeiDou B2I D2
3
3
(4)3
(3)4
4
B
BeiDou B1C
3
5
(4)3
N/A
4
3
BeiDou B2a
3
7
(4)3
N/A
4
5
QZSS L1C/A2
5
0
(1)3
(1)4
5
1
QZSS L1S
5
1
(1)3
(4)4
5
4
QZSS L2 CM
5
4
(1)3
(5)4
5
5
2 UBX messages that do not have an explicit sigId ﬁeld contain information about the subset of signals marked.
3 While not deﬁned by NMEA 4.10, u-blox receivers in this mode will use system ID 4 for BeiDou and, if extended satellite
numbering is enabled, system ID 1 for QZSS.
4 BeiDou and QZSS signal ID are not deﬁned in the NMEA protocol version 4.10. Values shown in the table are only valid for
u-blox products and, for QZSS signal ID, if extended satellite numbering is enabled.
UBX-22008968 - R01
 
1 General information
Page 20 of 305
C1-Public
```

---

## Page 150

```
u-blox F9 HPG 1.32 - Interface Description
3.15.15.1 Reset odometer
Message
UBX-NAV-RESETODO
Reset odometer
Type
Command
Comment
This message resets the traveled distance computed by the odometer (see UBX-NAV-ODO).
UBX-ACK-ACK or UBX-ACK-NAK are returned to indicate success or failure.
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
0x10
0
see below
CK_A CK_B
Payload
This message has no payload.
3.15.16 UBX-NAV-SAT (0x01 0x35)
3.15.16.1 Satellite information
Message
UBX-NAV-SAT
Satellite information
Type
Periodic/polled
Comment
This message displays information about SVs that are either known to be visible or currently tracked by the
receiver. All signal related information corresponds to the subset of signals speciﬁed in Signal Identiﬁers.
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
0x35
8 + numSvs·12
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
version
-
-
Message version (0x01 for this version)
5
U1
numSvs
-
-
Number of satellites
6
U1[2]
reserved0
-
-
Reserved
Start of repeated group (numSvs times)
8 + n·12
U1
gnssId
-
-
GNSS 
identiﬁer 
(see 
Satellite 
Numbering) 
for
assignment
9 + n·12
U1
svId
-
-
Satellite identiﬁer (see Satellite Numbering) for
assignment
10 + n·12
U1
cno
-
dBHz
Carrier to noise ratio (signal strength)
11 + n·12
I1
elev
-
deg
Elevation (range: +/-90), unknown if out of range
12 + n·12
I2
azim
-
deg
Azimuth (range 0-360), unknown if elevation is out of
range
14 + n·12
I2
prRes
0.1
m
Pseudorange residual
16 + n·12
X4
flags
-
-
Bitmask
bits 2…0   U:3
qualityInd
-
-
Signal quality indicator:
•
0 = no signal
•
1 = searching signal
•
2 = signal acquired
•
3 = signal detected but unusable
•
4 = code locked and time synchronized
•
5, 6, 7 = code and carrier locked and time
synchronized
UBX-22008968 - R01
 
3 UBX protocol
Page 150 of 305
C1-Public
```

---

## Page 151

```
u-blox F9 HPG 1.32 - Interface Description
bit 3   U:1
svUsed
-
-
1 = Signal in the subset speciﬁed in Signal Identiﬁers
is currently being used for navigation
bits 5…4   U:2
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
bit 6   U:1
diffCorr
-
-
1 = diﬀerential correction data is available for this SV
bit 7   U:1
smoothed
-
-
1 = carrier smoothed pseudorange used
bits 10…8   U:3
orbitSource
-
-
Orbit source:
•
0 = no orbit information is available for this SV
•
1 = ephemeris is used
•
2 = almanac is used
•
3 = AssistNow Oﬄine orbit is used
•
4 = AssistNow Autonomous orbit is used
•
5, 6, 7 = other orbit information is used
bit 11   U:1
ephAvail
-
-
1 = ephemeris is available for this SV
bit 12   U:1
almAvail
-
-
1 = almanac is available for this SV
bit 13   U:1
anoAvail
-
-
1 = AssistNow Oﬄine data is available for this SV
bit 14   U:1
aopAvail
-
-
1 = AssistNow Autonomous data is available for this
SV
bit 16   U:1
sbasCorrUsed
-
-
1 = SBAS corrections have been used for a signal in the
subset speciﬁed in Signal Identiﬁers
bit 17   U:1
rtcmCorrUsed
-
-
1 = RTCM corrections have been used for a signal in
the subset speciﬁed in Signal Identiﬁers
bit 18   U:1
slasCorrUsed
-
-
1 = QZSS SLAS corrections have been used for a signal
in the subset speciﬁed in Signal Identiﬁers
bit 19   U:1
spartnCorrUsed
-
-
1 = SPARTN corrections have been used for a signal in
the subset speciﬁed in Signal Identiﬁers
bit 20   U:1
prCorrUsed
-
-
1 = Pseudorange corrections have been used for a
signal in the subset speciﬁed in Signal Identiﬁers
bit 21   U:1
crCorrUsed
-
-
1 = Carrier range corrections have been used for a
signal in the subset speciﬁed in Signal Identiﬁers
bit 22   U:1
doCorrUsed
-
-
1 = Range rate (Doppler) corrections have been used
for a signal in the subset speciﬁed in Signal Identiﬁers
bit 23   U:1
clasCorrUsed
-
-
1 = CLAS corrections have been used for a signal in the
subset speciﬁed in Signal Identiﬁers
End of repeated group (numSvs times)
3.15.17 UBX-NAV-SBAS (0x01 0x32)
3.15.17.1 SBAS status data
Message
UBX-NAV-SBAS
SBAS status data
Type
Periodic/polled
Comment
This message outputs the status of the SBAS sub system
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
0x32
12 + cnt·12
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
Page 151 of 305
C1-Public
```

---

## Page 152

```
u-blox F9 HPG 1.32 - Interface Description
0
U4
iTOW
-
ms
GPS time of week of the navigation epoch.
See the description of iTOW for details.
4
U1
geo
-
-
PRN Number of the GEO where correction and
integrity data is used from
5
U1
mode
-
-
SBAS Mode
•
0 Disabled
•
1 Enabled integrity
•
3 Enabled test mode
6
I1
sys
-
-
SBAS System (WAAS/EGNOS/...)
•
-1 Unknown
•
0 WAAS
•
1 EGNOS
•
2 MSAS
•
3 GAGAN
•
16 GPS
7
X1
service
-
-
SBAS Services available
bit 0   U:1
Ranging
-
-
GEO may be used as ranging source
bit 1   U:1
Corrections
-
-
GEO is providing correction data
bit 2   U:1
Integrity
-
-
GEO is providing integrity
bit 3   U:1
Testmode
-
-
GEO is in test mode
bit 4   U:1
Bad
-
-
Problem with signal or broadcast data indicated
8
U1
cnt
-
-
Number of SV data following
9
X1
statusFlags
-
-
SBAS status ﬂags
bits 1…0   U:2
integrityUsed
-
-
SBAS integrity used
•
0 = Unknown
•
1 = Integrity information is not available or SBAS
integrity is not enabled
•
2 = Receiver uses only GPS satellites for which
integrity information is available
10
U1[2]
reserved0
-
-
Reserved
Start of repeated group (cnt times)
12 + n·12
U1
svid
-
-
SV ID
13 + n·12
U1
reserved1
-
-
Reserved
14 + n·12
U1
udre
-
-
Monitoring status
15 + n·12
U1
svSys
-
-
System (WAAS/EGNOS/...)
same as SYS
16 + n·12
U1
svService
-
-
Services available
same as SERVICE
17 + n·12
U1
reserved2
-
-
Reserved
18 + n·12
I2
prc
-
cm
Pseudo Range correction in [cm]
20 + n·12
U1[2]
reserved3
-
-
Reserved
22 + n·12
I2
ic
-
cm
Ionosphere correction in [cm]
End of repeated group (cnt times)
3.15.18 UBX-NAV-SIG (0x01 0x43)
UBX-22008968 - R01
 
3 UBX protocol
Page 152 of 305
C1-Public
```

---

## Page 153

```
u-blox F9 HPG 1.32 - Interface Description
3.15.18.1 Signal information
Message
UBX-NAV-SIG
Signal information
Type
Periodic/polled
Comment
This message displays information about signals currently tracked by the receiver.
On the F9 platform the maximum number of signals is 120.
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
0x43
8 + numSigs·16
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
version
-
-
Message version (0x00 for this version)
5
U1
numSigs
-
-
Number of signals
6
U1[2]
reserved0
-
-
Reserved
Start of repeated group (numSigs times)
8 + n·16
U1
gnssId
-
-
GNSS 
identiﬁer 
(see 
Satellite 
Numbering) 
for
assignment
9 + n·16
U1
svId
-
-
Satellite identiﬁer (see Satellite Numbering) for
assignment
10 + n·16
U1
sigId
-
-
New style signal identiﬁer (see Signal Identiﬁers)
11 + n·16
U1
freqId
-
-
Only used for GLONASS: This is the frequency slot + 7
(range from 0 to 13)
12 + n·16
I2
prRes
0.1
m
Pseudorange residual
14 + n·16
U1
cno
-
dBHz
Carrier-to-noise density ratio (signal strength)
15 + n·16
U1
qualityInd
-
-
Signal quality indicator:
•
0 = no signal
•
1 = searching signal
•
2 = signal acquired
•
3 = signal detected but unusable
•
4 = code locked and time synchronized
•
5, 6, 7 = code and carrier locked and time
synchronized
16 + n·16
U1
corrSource
-
-
Correction source:
•
0 = no corrections
•
1 = SBAS corrections
•
2 = BeiDou corrections
•
3 = RTCM2 corrections
•
4 = RTCM3 OSR corrections
•
5 = RTCM3 SSR corrections
•
6 = QZSS SLAS corrections
•
7 = SPARTN corrections
•
8 = CLAS corrections
UBX-22008968 - R01
 
3 UBX protocol
Page 153 of 305
C1-Public
```

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
