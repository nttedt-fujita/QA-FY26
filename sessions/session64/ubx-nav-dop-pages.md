# PDF抽出: アプリケーションノート_u-blox-F9-HPG-1.32_InterfaceDescription_UBX-22008968.pdf

**総ページ数**: 305
**抽出ページ**: 147, 148, 149, 150, 151

---

## Page 147

```
u-blox F9 HPG 1.32 - Interface Description
•
5 = Inactive
bit 5   U:1
headVehValid
-
-
1 = heading of vehicle is valid, only set if the receiver is
in sensor fusion mode
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
(not supported for protocol versions less than 20.00)
22
X1
flags2
-
-
Additional ﬂags
bit 5   U:1
confirmedAvai
-
-
1 = information about UTC Date and Time of Day
validity conﬁrmation is available (see section Time
validity in the integration manual for details)
This ﬂag is only supported in Protocol Versions 19.00,
19.10, 20.10, 20.20, 20.30, 22.00, 23.00, 23.01, 27 and
28.
bit 6   U:1
confirmedDate
-
-
1 = UTC Date validity could be conﬁrmed (see section
Time validity in the integration manual for details)
bit 7   U:1
confirmedTime
-
-
1 = UTC Time of Day could be conﬁrmed (see section
Time validity in the integration manual for details)
23
U1
numSV
-
-
Number of satellites used in Nav Solution
24
I4
lon
1e-7
deg
Longitude
28
I4
lat
1e-7
deg
Latitude
32
I4
height
-
mm
Height above ellipsoid
36
I4
hMSL
-
mm
Height above mean sea level
40
U4
hAcc
-
mm
Horizontal accuracy estimate
44
U4
vAcc
-
mm
Vertical accuracy estimate
48
I4
velN
-
mm/s
NED north velocity
52
I4
velE
-
mm/s
NED east velocity
56
I4
velD
-
mm/s
NED down velocity
60
I4
gSpeed
-
mm/s
Ground Speed (2-D)
64
I4
headMot
1e-5
deg
Heading of motion (2-D)
68
U4
sAcc
-
mm/s
Speed accuracy estimate
72
U4
headAcc
1e-5
deg
Heading accuracy estimate (both motion and vehicle)
76
U2
pDOP
0.01
-
Position DOP
78
X2
flags3
-
-
Additional ﬂags
bit 0   U:1
invalidLlh
-
-
1 = Invalid lon, lat, height and hMSL
bits 4…1   U:4
lastCorrection
Age
-
-
Age of the most recently received diﬀerential
correction:
•
0 = Not available
•
1 = Age between 0 and 1 second
•
2 = Age between 1 (inclusive) and 2 seconds
•
3 = Age between 2 (inclusive) and 5 seconds
•
4 = Age between 5 (inclusive) and 10 seconds
•
5 = Age between 10 (inclusive) and 15 seconds
•
6 = Age between 15 (inclusive) and 20 seconds
•
7 = Age between 20 (inclusive) and 30 seconds
•
8 = Age between 30 (inclusive) and 45 seconds
•
9 = Age between 45 (inclusive) and 60 seconds
UBX-22008968 - R01
 
3 UBX protocol
Page 147 of 305
C1-Public
```

---

## Page 148

```
u-blox F9 HPG 1.32 - Interface Description
•
10 = Age between 60 (inclusive) and 90 seconds
•
11 = Age between 90 (inclusive) and 120 seconds
•
>=12 = Age greater or equal than 120 seconds
80
U1[4]
reserved0
-
-
Reserved
84
I4
headVeh
1e-5
deg
Heading of vehicle (2-D), this is only valid when
headVehValid is set, otherwise the output is set to the
heading of motion
88
I2
magDec
1e-2
deg
Magnetic declination. Only supported in ADR 4.10 and
later.
90
U2
magAcc
1e-2
deg
Magnetic declination accuracy. Only supported in ADR
4.10 and later.
3.15.14 UBX-NAV-RELPOSNED (0x01 0x3c)
3.15.14.1 Relative positioning information in NED frame
Message
UBX-NAV-RELPOSNED
Relative positioning information in NED frame
Type
Periodic/polled
Comment
This message contains the relative position vector from the reference station to the rover, including accuracy
ﬁgures, in the local topological system deﬁned at the reference station.
☞ The NED frame is deﬁned as the local topological system at the reference station. The relative position
vector components in this message, along with their associated accuracies, are given in that local topological
system.
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
0x3c
64
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
U1
reserved0
-
-
Reserved
2
U2
refStationId
-
-
Reference station ID. Must be in the range 0..4095.
4
U4
iTOW
-
ms
GPS time of week of the navigation epoch.
See section iTOW timestamps in the integration
manual for details.
8
I4
relPosN
-
cm
North component of relative position vector
12
I4
relPosE
-
cm
East component of relative position vector
16
I4
relPosD
-
cm
Down component of relative position vector
20
I4
relPosLength
-
cm
Length of the relative position vector
24
I4
relPosHeading
1e-5
deg
Heading of the relative position vector
28
U1[4]
reserved1
-
-
Reserved
32
I1
relPosHPN
0.1
mm
High-precision North component of relative position
vector.
Must be in the range -99 to +99.
The full North component of the relative position
vector, in units of cm, is given by
relPosN + (relPosHPN * 1e-2)
UBX-22008968 - R01
 
3 UBX protocol
Page 148 of 305
C1-Public
```

---

## Page 149

```
u-blox F9 HPG 1.32 - Interface Description
33
I1
relPosHPE
0.1
mm
High-precision East component of relative position
vector.
Must be in the range -99 to +99.
The full East component of the relative position vector,
in units of cm, is given by
relPosE + (relPosHPE * 1e-2)
34
I1
relPosHPD
0.1
mm
High-precision Down component of relative position
vector.
Must be in the range -99 to +99.
The full Down component of the relative position
vector, in units of cm, is given by
relPosD + (relPosHPD * 1e-2)
35
I1
relPosHP
Length
0.1
mm
High-precision component of the length of the relative
position vector.
Must be in the range -99 to +99.
The full length of the relative position vector, in units
of cm, is given by
relPosLength + (relPosHPLength * 1e-2)
36
U4
accN
0.1
mm
Accuracy of relative position North component
40
U4
accE
0.1
mm
Accuracy of relative position East component
44
U4
accD
0.1
mm
Accuracy of relative position Down component
48
U4
accLength
0.1
mm
Accuracy of length of the relative position vector
52
U4
accHeading
1e-5
deg
Accuracy of heading of the relative position vector
56
U1[4]
reserved2
-
-
Reserved
60
X4
flags
-
-
Flags
bit 0   U:1
gnssFixOK
-
-
A valid ﬁx (i.e within DOP & accuracy masks)
bit 1   U:1
diffSoln
-
-
1 if diﬀerential corrections were applied
bit 2   U:1
relPosValid
-
-
1 if relative position components and accuracies are
valid and, in moving base mode only, if baseline is valid
bits 4…3   U:2
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
bit 5   U:1
isMoving
-
-
1 if the receiver is operating in moving base mode
bit 6   U:1
refPosMiss
-
-
1 if extrapolated reference position was used to
compute moving base solution this epoch. (Flag set for
protocol versions 27.10, and 27.11, and 31.11)
bit 7   U:1
refObsMiss
-
-
1 if extrapolated reference observations were used to
compute moving base solution this epoch. (Flag set for
protocol versions 27.10, and 27.11, and 31.11)
bit 8   U:1
relPosHeading
Valid
-
-
1 if relPosHeading is valid
bit 9   U:1
relPos
Normalized
-
-
1 if the components of the relative position vector
(including the high-precision parts) are normalized
3.15.15 UBX-NAV-RESETODO (0x01 0x10)
UBX-22008968 - R01
 
3 UBX protocol
Page 149 of 305
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
