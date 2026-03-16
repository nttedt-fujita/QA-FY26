# PDF抽出: u-blox-F9-HPG-1.32_InterfaceDescription_UBX-22008968.pdf

**総ページ数**: 305
**抽出ページ**: 63, 64, 65, 66, 67, 68

---

## Page 63

```
u-blox F9 HPG 1.32 - Interface Description
3.9.2.1 Message not acknowledged
Message
UBX-ACK-NAK
Message not acknowledged
Type
Output
Comment
Output upon processing of an input message. A UBX-ACK-NAK is sent as soon as possible but at least within
one second.
Header
Class
ID
Length (Bytes)
Payload
Checksum
Message
structure
0xb5 0x62
0x05
0x00
2
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
clsID
-
-
Class ID of the Not-Acknowledged Message
1
U1
msgID
-
-
Message ID of the Not-Acknowledged Message
3.10 UBX-CFG (0x06)
The messages in the UBX-CFG class are used to conﬁgure the receiver and poll current conﬁguration
values as well as for sending commands to the receiver. Unless stated otherwise, any message in
this class sent to the receiver is either acknowledged (by a UBX-ACK-ACK message) if processed
successfully or rejected (with a UBX-ACK-NAK message) if processed unsuccessfully.
3.10.1 UBX-CFG-ANT (0x06 0x13)
3.10.1.1 Antenna control settings
Message
UBX-CFG-ANT
Antenna control settings
Type
Get/set
Comment
This message is deprecated in protocol versions greater than 23.01. Use UBX-CFG-VALSET, UBX-CFG-
VALGET, UBX-CFG-VALDEL instead.
See the Legacy UBX Message Fields Reference for the corresponding conﬁguration item.
This message allows the user to conﬁgure the antenna supervisor.
The antenna supervisor can be used to detect the status of an active antenna and control it. It can be
used to turn oﬀ the supply to the antenna in the event of a short cirquit (for example) or to manage power
consumption in power save mode.
Refer to antenna supervisor conﬁguration in the integration manual for more information regarding the
behavior of the antenna supervisor.
Note that not all pins can be used for antenna supervisor operation, the default pins are recommended.
Consult the integration manual if you need to use the other pins.
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
0x13
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
flags
-
-
Antenna ﬂag mask
bit 0   U:1
svcs
-
-
Enable antenna supply voltage control signal
bit 1   U:1
scd
-
-
Enable short circuit detection
bit 2   U:1
ocd
-
-
Enable open circuit detection
bit 3   U:1
pdwnOnSCD
-
-
Power down antenna supply if short circuit is detected.
(only in combination with bit 1)
bit 4   U:1
recovery
-
-
Enable automatic recovery from short state
UBX-22008968 - R01
 
3 UBX protocol
Page 63 of 305
C1-Public
```

---

## Page 64

```
u-blox F9 HPG 1.32 - Interface Description
2
X2
pins
-
-
Antenna pin conﬁguration
bits 4…0   U:5
pinSwitch
-
-
PIO-pin used for switching antenna supply
bits 9…5   U:5
pinSCD
-
-
PIO-pin used for detecting a short in the antenna
supply
bits 14…10   U:5
pinOCD
-
-
PIO-pin used for detecting open/not connected
antenna
bit 15   U:1
reconfig
-
-
if set to one, and this command is sent to the receiver,
the receiver will reconﬁgure the pins as speciﬁed.
3.10.2 UBX-CFG-CFG (0x06 0x09)
3.10.2.1 Clear, save and load conﬁgurations
Message
UBX-CFG-CFG
Clear, save and load conﬁgurations
Type
Command
Comment
See Receiver conﬁguration for a detailed description on how receiver conﬁguration should be used. The
behavior of this message has changed for protocol versions greater than 23.01. Use UBX-CFG-VALSET and
UBX-CFG-VALDEL with the appropriate layers instead. These new messages support selective saving and
clearing to retain the behavior removed from this message. The three masks which were used to clear, save
and load a subsection of conﬁguration have lost their meaning. It is no longer possible to save or clear a
subsection of the conﬁguration using this message. The behavior of the masks is now:
•
if any bit is set in the clearMask: all conﬁguration in the selected non-volatile memory is deleted
•
if any bit is set in the saveMask: all current conﬁguration is stored (copied) to the selected layers
•
if any bit is set in the loadMask: The current conﬁguration is discarded and rebuilt from all the lower
layers
Note that commands can be combined. The sequence of execution is clear, save, then load.
☞ Old functionality of this message is not available in protocol versions greater than 23.01. Use UBX-CFG-
VALSET, UBX-CFG-VALGET, UBX-CFG-VALDEL instead.
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
0x09
12 + [0,1]
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
clearMask
-
-
Mask for conﬁguration to clear
bits 31…0   U:32
clearAll
-
-
Clear all saved conﬁguration from the selected non-
volatile memory if any bit is set
4
X4
saveMask
-
-
Mask for conﬁguration to save
bits 31…0   U:32
saveAll
-
-
Save all current conﬁguration to the selected non-
volatile memory if any bit is set
8
X4
loadMask
-
-
Mask for conﬁguration to load
bits 31…0   U:32
loadAll
-
-
Discard current conﬁguration and rebuilt it from lower
non-volatile memory layers if any bit is set
Start of optional group
12
X1
deviceMask
-
-
Mask which selects the memory devices for saving
and/or clearing operation
Note that if a deviceMask is not provided, the receiver
defaults the operation requested to battery-backed
RAM (BBR) and Flash (if available)
bit 0   U:1
devBBR
-
-
Battery-backed RAM
bit 1   U:1
devFlash
-
-
Flash
UBX-22008968 - R01
 
3 UBX protocol
Page 64 of 305
C1-Public
```

---

## Page 65

```
u-blox F9 HPG 1.32 - Interface Description
bit 2   U:1
devEEPROM
-
-
EEPROM (only supported for protocol versions less
than 14.00)
bit 4   U:1
devSpiFlash
-
-
SPI Flash (only supported for protocol versions less
than 14.00)
End of optional group
3.10.3 UBX-CFG-DAT (0x06 0x06)
3.10.3.1 Set user-deﬁned datum
Message
UBX-CFG-DAT
Set user-deﬁned datum
Type
Set
Comment
This message is deprecated in protocol versions greater than 23.01. Use UBX-CFG-VALSET, UBX-CFG-
VALGET, UBX-CFG-VALDEL instead.
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
0x06
44
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
R8
majA
-
m
Semi-major axis ( accepted range = 6,300,000.0 to
6,500,000.0 meters ).
8
R8
flat
-
-
1.0 / ﬂattening ( accepted range is 0.0 to 500.0 ).
16
R4
dX
-
m
X axis shift at the origin ( accepted range is +/- 5000.0
meters ).
20
R4
dY
-
m
Y axis shift at the origin ( accepted range is +/- 5000.0
meters ).
24
R4
dZ
-
m
Z axis shift at the origin ( accepted range is +/- 5000.0
meters ).
28
R4
rotX
-
s
Rotation about the X axis ( accepted range is +/- 20.0
milli-arc seconds ).
32
R4
rotY
-
s
Rotation about the Y axis ( accepted range is +/- 20.0
milli-arc seconds ).
36
R4
rotZ
-
s
Rotation about the Z axis ( accepted range is +/- 20.0
milli-arc seconds ).
40
R4
scale
-
ppm
Scale change ( accepted range is 0.0 to 50.0 parts per
million ).
3.10.3.2 Get currently deﬁned datum
Message
UBX-CFG-DAT
Get currently deﬁned datum
Type
Get
Comment
This message is deprecated in protocol versions greater than 23.01. Use UBX-CFG-VALSET, UBX-CFG-
VALGET, UBX-CFG-VALDEL instead.
See the Legacy UBX Message Fields Reference for the corresponding conﬁguration item.
Returns the parameters of the currently deﬁned datum. If no user-deﬁned datum has been set, this will
default to WGS84.
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
0x06
52
see below
CK_A CK_B
Payload description:
UBX-22008968 - R01
 
3 UBX protocol
Page 65 of 305
C1-Public
```

---

## Page 66

```
u-blox F9 HPG 1.32 - Interface Description
Byte offset
Type
Name
Scale
Unit
Description
0
U2
datumNum
-
-
Datum number: 0 = WGS84, 0xFFFF = user-deﬁned
(extra values are deﬁned for protocol versions less
than 13.00)
2
CH[6]
datumName
-
-
ASCII string: WGS84 or USER (extra values are deﬁned
for protocol versions less than 13.00)
8
R8
majA
-
m
Semi-major axis ( accepted range = 6,300,000.0 to
6,500,000.0 meters ).
16
R8
flat
-
-
1.0 / ﬂattening ( accepted range is 0.0 to 500.0 ).
24
R4
dX
-
m
X axis shift at the origin ( accepted range is +/- 5000.0
meters ).
28
R4
dY
-
m
Y axis shift at the origin ( accepted range is +/- 5000.0
meters ).
32
R4
dZ
-
m
Z axis shift at the origin ( accepted range is +/- 5000.0
meters ).
36
R4
rotX
-
s
Rotation about the X axis ( accepted range is +/- 20.0
milli-arc seconds ).
40
R4
rotY
-
s
Rotation about the Y axis ( accepted range is +/- 20.0
milli-arc seconds ).
44
R4
rotZ
-
s
Rotation about the Z axis ( accepted range is +/- 20.0
milli-arc seconds ).
48
R4
scale
-
ppm
Scale change ( accepted range is 0.0 to 50.0 parts per
million ).
3.10.4 UBX-CFG-DGNSS (0x06 0x70)
3.10.4.1 DGNSS conﬁguration
Message
UBX-CFG-DGNSS
DGNSS conﬁguration
Type
Get/set
Comment
This message is deprecated in protocol versions greater than 23.01. Use UBX-CFG-VALSET, UBX-CFG-
VALGET, UBX-CFG-VALDEL instead.
See the Legacy UBX Message Fields Reference for the corresponding conﬁguration item.
This message allows the user to conﬁgure the DGNSS conﬁguration of the receiver.
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
0x70
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
U1
dgnssMode
-
-
Speciﬁes diﬀerential mode:
•
2 = RTK ﬂoat: No attempts are made to ﬁx
ambiguities.
•
3 = RTK ﬁxed: Ambiguities are ﬁxed whenever
possible.
1
U1[3]
reserved0
-
-
Reserved
3.10.5 UBX-CFG-GEOFENCE (0x06 0x69)
UBX-22008968 - R01
 
3 UBX protocol
Page 66 of 305
C1-Public
```

---

## Page 67

```
u-blox F9 HPG 1.32 - Interface Description
3.10.5.1 Geofencing conﬁguration
Message
UBX-CFG-GEOFENCE
Geofencing conﬁguration
Type
Get/set
Comment
This message is deprecated in protocol versions greater than 23.01. Use UBX-CFG-VALSET, UBX-CFG-
VALGET, UBX-CFG-VALDEL instead.
See the Legacy UBX Message Fields Reference for the corresponding conﬁguration item.
Gets or sets the geofencing conﬁguration.
If the receiver is sent a valid new conﬁguration, it will respond with a UBX-ACK-ACK message and immediately
change to the new conﬁguration. Otherwise the receiver will reject the request, by issuing a UBX-ACK-NAK
and continuing operation with the previous conﬁguration.
Note that the acknowledge message does not indicate whether the PIO conﬁguration has been successfully
applied (pin assigned), it only indicates the successful conﬁguration of the feature. The conﬁgured PIO must
be previously unoccupied for successful assignment.
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
0x69
8 + numFences·12
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
numFences
-
-
Number of geofences contained in this message. Note
that the receiver can only store a limited number of
geofences (currently 4).
2
U1
confLvl
-
-
Required conﬁdence level for state evaluation. This
value times the position's standard deviation (sigma)
deﬁnes the conﬁdence band.
•
0 = no conﬁdence required
•
1 = 68%
•
2 = 95%
•
3 = 99.7%
•
4 = 99.99%
3
U1
reserved0
-
-
Reserved
4
U1
pioEnabled
-
-
1 = Enable PIO combined fence state output, 0 =
disable
5
U1
pinPolarity
-
-
PIO pin polarity. 0 = Low means inside, 1 = Low means
outside. Unknown state is always high.
6
U1
pin
-
-
PIO pin number
7
U1
reserved1
-
-
Reserved
Start of repeated group (numFences times)
8 + n·12
I4
lat
1e-7
deg
Latitude of the geofence circle center
12 + n·12
I4
lon
1e-7
deg
Longitude of the geofence circle center
16 + n·12
U4
radius
1e-2
m
Radius of the geofence circle
End of repeated group (numFences times)
3.10.6 UBX-CFG-GNSS (0x06 0x3e)
3.10.6.1 GNSS system conﬁguration
Message
UBX-CFG-GNSS
GNSS system conﬁguration
Type
Get/set
UBX-22008968 - R01
 
3 UBX protocol
Page 67 of 305
C1-Public
```

---

## Page 68

```
u-blox F9 HPG 1.32 - Interface Description
Comment
This message is deprecated in protocol versions greater than 23.01. Use UBX-CFG-VALSET, UBX-CFG-
VALGET, UBX-CFG-VALDEL instead.
See the Legacy UBX Message Fields Reference for the corresponding conﬁguration item.
Gets or sets the GNSS system channel sharing conﬁguration.
If the receiver is sent a valid new conﬁguration, it will respond with a UBX-ACK-ACK message and immediately
change to the new conﬁguration. Otherwise the receiver will reject the request, by issuing a UBX-ACK-NAK
and continuing operation with the previous conﬁguration.
Conﬁguration requirements:
•
It is necessary for at least one major GNSS to be enabled, after applying the new conﬁguration to the
current one.
•
It is also required that at least 4 tracking channels are available to each enabled major GNSS, i.e.
maxTrkCh must have a minimum value of 4 for each enabled major GNSS.
•
The number of tracking channels in use must not exceed the number of tracking channels available in
hardware, and the sum of all reserved tracking channels needs to be less than or equal to the number of
tracking channels in use.
Notes:
•
To avoid cross-correlation issues, it is recommended that GPS and QZSS are always both enabled or
both disabled.
•
Polling this message returns the conﬁguration of all supported GNSS, whether enabled or not; it may
also include GNSS unsupported by the particular product, but in such cases the enable ﬂag will always
be unset.
•
See section Satellite Numbering for a description of the GNSS IDs available.
•
Conﬁguration speciﬁc to the GNSS system can be done via other messages (e.g. UBX-CFG-SBAS).
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
0x3e
4 + numConﬁgBlocks·8
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
msgVer
-
-
Message version (0x00 for this version)
1
U1
numTrkChHw
-
-
Number of tracking channels available in hardware
(read only)
2
U1
numTrkChUse
-
-
(Read only for protocol versions greater than 23.00)
Number of tracking channels to use. Must be > 0,
<= numTrkChHw. If 0xFF, then number of tracking
channels to use will be set to numTrkChHw.
3
U1
numConfig
Blocks
-
-
Number of conﬁguration blocks following
Start of repeated group (numConfigBlocks times)
4 + n·8
U1
gnssId
-
-
System identiﬁer (see Satellite Numbering)
5 + n·8
U1
resTrkCh
-
-
(Read only for protocol versions greater than 23.00)
Number of reserved (minimum) tracking channels for
this system.
6 + n·8
U1
maxTrkCh
-
-
(Read only for protocol versions greater than 23.00)
Maximum number of tracking channels used for this
system. Must be > 0, >= resTrkChn, <= numTrkChUse
and <= maximum number of tracking channels
supported for this system.
7 + n·8
U1
reserved0
-
-
Reserved
8 + n·8
X4
flags
-
-
Bitﬁeld of ﬂags. At least one signal must be conﬁgured
in every enabled system.
bit 0   U:1
enable
-
-
Enable this system
bits 23…16   U:8
sigCfgMask
-
-
Signal conﬁguration mask
When gnssId is 0 (GPS)
•
0x01 = GPS L1C/A
UBX-22008968 - R01
 
3 UBX protocol
Page 68 of 305
C1-Public
```

---
