# PDF抽出: アプリケーションノート_u-blox-F9-HPG-1.32_InterfaceDescription_UBX-22008968.pdf

**総ページ数**: 305
**抽出ページ**: 137, 138, 139, 140, 141, 142

---

## Page 137

```
u-blox F9 HPG 1.32 - Interface Description
3.15.1.1 Clock solution
Message
UBX-NAV-CLOCK
Clock solution
Type
Periodic/polled
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
0x01
0x22
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
GPS time of week of the navigation epoch. See
section Navigation epochs in the integration manual
for details.
See section iTOW timestamps in the integration
manual for details.
4
I4
clkB
-
ns
Clock bias
8
I4
clkD
-
ns/s
Clock drift
12
U4
tAcc
-
ns
Time accuracy estimate
16
U4
fAcc
-
ps/s
Frequency accuracy estimate
3.15.2 UBX-NAV-COV (0x01 0x36)
3.15.2.1 Covariance matrices
Message
UBX-NAV-COV
Covariance matrices
Type
Periodic/polled
Comment
This message outputs the covariance matrices for the position and velocity solutions in the topocentric
coordinate system deﬁned as the local-level North (N), East (E), Down (D) frame. As the covariance matrices
are symmetric, only the upper triangular part is output.
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
0x36
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
posCovValid
-
-
Position covariance matrix validity ﬂag
6
U1
velCovValid
-
-
Velocity covariance matrix validity ﬂag
7
U1[9]
reserved0
-
-
Reserved
16
R4
posCovNN
-
m^2
Position covariance matrix value p_NN
20
R4
posCovNE
-
m^2
Position covariance matrix value p_NE
24
R4
posCovND
-
m^2
Position covariance matrix value p_ND
28
R4
posCovEE
-
m^2
Position covariance matrix value p_EE
32
R4
posCovED
-
m^2
Position covariance matrix value p_ED
36
R4
posCovDD
-
m^2
Position covariance matrix value p_DD
40
R4
velCovNN
-
m^2/s^2
Velocity covariance matrix value v_NN
UBX-22008968 - R01
 
3 UBX protocol
Page 137 of 305
C1-Public
```

---

## Page 138

```
u-blox F9 HPG 1.32 - Interface Description
44
R4
velCovNE
-
m^2/s^2
Velocity covariance matrix value v_NE
48
R4
velCovND
-
m^2/s^2
Velocity covariance matrix value v_ND
52
R4
velCovEE
-
m^2/s^2
Velocity covariance matrix value v_EE
56
R4
velCovED
-
m^2/s^2
Velocity covariance matrix value v_ED
60
R4
velCovDD
-
m^2/s^2
Velocity covariance matrix value v_DD
3.15.3 UBX-NAV-DOP (0x01 0x04)
3.15.3.1 Dilution of precision
Message
UBX-NAV-DOP
Dilution of precision
Type
Periodic/polled
Comment
•
DOP values are dimensionless.
•
All DOP values are scaled by a factor of 100. If the unit transmits a value of e.g. 156, the DOP value is
1.56.
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
0x04
18
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
U2
gDOP
0.01
-
Geometric DOP
6
U2
pDOP
0.01
-
Position DOP
8
U2
tDOP
0.01
-
Time DOP
10
U2
vDOP
0.01
-
Vertical DOP
12
U2
hDOP
0.01
-
Horizontal DOP
14
U2
nDOP
0.01
-
Northing DOP
16
U2
eDOP
0.01
-
Easting DOP
3.15.4 UBX-NAV-EOE (0x01 0x61)
3.15.4.1 End of epoch
Message
UBX-NAV-EOE
End of epoch
Type
Periodic
Comment
This message is intended to be used as a marker to collect all navigation messages of an epoch. It is output
after all enabled NAV class messages (except UBX-NAV-HNR) and after all enabled NMEA messages.
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
0x61
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
UBX-22008968 - R01
 
3 UBX protocol
Page 138 of 305
C1-Public
```

---

## Page 139

```
u-blox F9 HPG 1.32 - Interface Description
0
U4
iTOW
-
ms
GPS time of week of the navigation epoch.
See section iTOW timestamps in the integration
manual for details.
3.15.5 UBX-NAV-GEOFENCE (0x01 0x39)
3.15.5.1 Geofencing status
Message
UBX-NAV-GEOFENCE
Geofencing status
Type
Periodic/polled
Comment
This message outputs the evaluated states of all conﬁgured geofences for the current epoch's position.
See section Geofencing in the integration manual for feature details.
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
0x39
8 + numFences·2
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
status
-
-
Geofencing status
•
0 - Geofencing not available or not reliable
•
1 - Geofencing active
6
U1
numFences
-
-
Number of geofences
7
U1
combState
-
-
Combined (logical OR) state of all geofences
•
0 - Unknown
•
1 - Inside
•
2 - Outside
Start of repeated group (numFences times)
8 + n·2
U1
state
-
-
Geofence state
•
0 - Unknown
•
1 - Inside
•
2 - Outside
9 + n·2
U1
id
-
-
Geofence ID (0 = not available)
End of repeated group (numFences times)
3.15.6 UBX-NAV-HPPOSECEF (0x01 0x13)
3.15.6.1 High precision position solution in ECEF
Message
UBX-NAV-HPPOSECEF
High precision position solution in ECEF
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
0x13
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
UBX-22008968 - R01
 
3 UBX protocol
Page 139 of 305
C1-Public
```

---

## Page 140

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
See section iTOW timestamps in the integration
manual for details.
8
I4
ecefX
-
cm
ECEF X coordinate
12
I4
ecefY
-
cm
ECEF Y coordinate
16
I4
ecefZ
-
cm
ECEF Z coordinate
20
I1
ecefXHp
0.1
mm
High precision component of ECEF X coordinate. Must
be in the range of -99..+99. Precise coordinate in cm =
ecefX + (ecefXHp * 1e-2).
21
I1
ecefYHp
0.1
mm
High precision component of ECEF Y coordinate. Must
be in the range of -99..+99. Precise coordinate in cm =
ecefY + (ecefYHp * 1e-2).
22
I1
ecefZHp
0.1
mm
High precision component of ECEF Z coordinate. Must
be in the range of -99..+99. Precise coordinate in cm =
ecefZ + (ecefZHp * 1e-2).
23
X1
flags
-
-
Additional ﬂags
bit 0   U:1
invalidEcef
-
-
1 = Invalid ecefX, ecefY, ecefZ, ecefXHp, ecefYHp and
ecefZHp
24
U4
pAcc
0.1
mm
Position Accuracy Estimate
3.15.7 UBX-NAV-HPPOSLLH (0x01 0x14)
3.15.7.1 High precision geodetic position solution
Message
UBX-NAV-HPPOSLLH
High precision geodetic position solution
Type
Periodic/polled
Comment
See important comments concerning validity of position given in section Navigation output ﬁlters in the
integration manual.
This message outputs the Geodetic position in the currently selected ellipsoid. The default is the WGS84
Ellipsoid, but can be changed with the message CFG-NAVSPG-USE_USRDAT.
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
0x14
36
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
U1[2]
reserved0
-
-
Reserved
3
X1
flags
-
-
Additional ﬂags
bit 0   U:1
invalidLlh
-
-
1 = Invalid lon, lat, height, hMSL, lonHp, latHp,
heightHp and hMSLHp
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
lon
1e-7
deg
Longitude
12
I4
lat
1e-7
deg
Latitude
16
I4
height
-
mm
Height above ellipsoid.
UBX-22008968 - R01
 
3 UBX protocol
Page 140 of 305
C1-Public
```

---

## Page 141

```
u-blox F9 HPG 1.32 - Interface Description
20
I4
hMSL
-
mm
Height above mean sea level
24
I1
lonHp
1e-9
deg
High precision component of longitude. Must be in the
range -99..+99. Precise longitude in deg * 1e-7 = lon +
(lonHp * 1e-2).
25
I1
latHp
1e-9
deg
High precision component of latitude. Must be in the
range -99..+99. Precise latitude in deg * 1e-7 = lat +
(latHp * 1e-2).
26
I1
heightHp
0.1
mm
High precision component of height above ellipsoid.
Must be in the range -9..+9. Precise height in mm =
height + (heightHp * 0.1).
27
I1
hMSLHp
0.1
mm
High precision component of height above mean sea
level. Must be in range -9..+9. Precise height in mm =
hMSL + (hMSLHp * 0.1)
28
U4
hAcc
0.1
mm
Horizontal accuracy estimate
32
U4
vAcc
0.1
mm
Vertical accuracy estimate
3.15.8 UBX-NAV-ODO (0x01 0x09)
3.15.8.1 Odometer solution
Message
UBX-NAV-ODO
Odometer solution
Type
Periodic/polled
Comment
This message outputs the traveled distance since last reset (see UBX-NAV-RESETODO) together with an
associated estimated accuracy and the total cumulated ground distance (can only be reset by a cold start
of the receiver).
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
0x09
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
See section iTOW timestamps in the integration
manual for details.
8
U4
distance
-
m
Ground distance since last reset
12
U4
totalDistance
-
m
Total cumulative ground distance
16
U4
distanceStd
-
m
Ground distance accuracy (1-sigma)
3.15.9 UBX-NAV-ORB (0x01 0x34)
3.15.9.1 GNSS orbit database info
Message
UBX-NAV-ORB
GNSS orbit database info
Type
Periodic/polled
Comment
Status of the GNSS orbit database knowledge.
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
0x34
8 + numSv·6
see below
CK_A CK_B
Payload description:
UBX-22008968 - R01
 
3 UBX protocol
Page 141 of 305
C1-Public
```

---

## Page 142

```
u-blox F9 HPG 1.32 - Interface Description
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
numSv
-
-
Number of SVs in the database
6
U1[2]
reserved0
-
-
Reserved
Start of repeated group (numSv times)
8 + n·6
U1
gnssId
-
-
GNSS ID
9 + n·6
U1
svId
-
-
Satellite ID
10 + n·6
X1
svFlag
-
-
Information Flags
bits 1…0   U:2
health
-
-
SV health:
•
0 = unknown
•
1 = healthy
•
2 = not healty
bits 3…2   U:2
visibility
-
-
SV health:
•
0 = unknown
•
1 = below horizon
•
2 = above horizon
•
3 = above elevation mask
11 + n·6
X1
eph
-
-
Ephemeris data
In products supporting L5 signals, the receiver may
store multiple ephemeris data sets per satellite.
ephUsability and ephSource ﬁelds show information
on one of the data sets. It is not possible to choose
which data set's status is shown.
bits 4…0   U:5
ephUsability
-
-
How long the receiver will be able to use the stored
ephemeris data from now on:
•
31 = The usability period is unknown
•
30 = The usability period is more than 450
minutes
•
30 > n > 0 = The usability period is between
(n-1)*15 and n*15 minutes
•
0 = Ephemeris can no longer be used
bits 7…5   U:3
ephSource
-
-
•
0 = not available
•
1 = GNSS transmission
•
2 = external aiding
•
3-7 = other
12 + n·6
X1
alm
-
-
Almanac data
bits 4…0   U:5
almUsability
-
-
How long the receiver will be able to use the stored
almanac data from now on:
•
31 = The usability period is unknown
•
30 = The usability period is more than 30 days
•
30 > n > 0 = The usability period is between n-1
and n days
•
0 = Almanac can no longer be used
bits 7…5   U:3
almSource
-
-
•
0 = not available
•
1 = GNSS transmission
•
2 = external aiding
•
3-7 = other
13 + n·6
X1
otherOrb
-
-
Other orbit data available
UBX-22008968 - R01
 
3 UBX protocol
Page 142 of 305
C1-Public
```

---
