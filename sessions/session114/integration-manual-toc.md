# PDF抽出: ZED-F9P_IntegrationManual_UBX-18010802.pdf

**総ページ数**: 129
**抽出ページ**: 1, 2, 3, 4, 5, 6, 7, 8, 9, 10

---

## Page 1

```

ZED-F9P
High precision GNSS module
Integration manual
Abstract
This document describes the features and application of the ZED-F9P, a
multi-band GNSS module with integrated RTK oﬀering centimeter-level
accuracy.
www.u-blox.com
UBX-18010802 - R16
C1-Public
```

---

## Page 2

```
ZED-F9P - Integration manual
Document information
Title
ZED-F9P
Subtitle
High precision GNSS module
Document type
Integration manual
Document number
UBX-18010802
Revision and date
R16
30-Oct-2024
Disclosure restriction
C1-Public
This document applies to the following products:
Type number
FW version
IN/PCN reference
RN reference
ZED-F9P-01B-01
HPG 1.12
UBX-19057484
UBX-19026698
ZED-F9P-02B-00
HPG 1.13
-
UBX-20019211
ZED-F9P-04B-01
HPG 1.32
UBX-22010309
UBX-22004887
ZED-F9P-15B-00
HPG L1L5 1.40
-
UBX-23010071
ZED-F9P-05B-00
HPG 1.50
-
UBXDOC-963802114-12826
ZED-F9P-05B-00
HPG 1.51
UBXDOC-963802114-13129
UBXDOC-963802114-13110
u-blox or third parties may hold intellectual property rights in the products, names, logos and designs included in this
document. Copying, reproduction, or modiﬁcation of this document or any part thereof is only permitted with the express
written permission of u-blox. Disclosure to third parties is permitted for clearly public documents only.
The information contained herein is provided "as is" and u-blox assumes no liability for its use. No warranty, either express
or implied, is given, including but not limited to, with respect to the accuracy, correctness, reliability and ﬁtness for a
particular purpose of the information. This document may be revised by u-blox at any time without notice. For the most recent
documents and product statuses, visit www.u-blox.com.
Copyright © 2024, u-blox AG.
UBX-18010802 - R16
 
Page 2 of 129
C1-Public
```

---

## Page 3

```
ZED-F9P - Integration manual
Contents
1 Integration manual overview............................................................................................. 6
2 System description...............................................................................................................7
2.1 Overview.................................................................................................................................................... 7
2.1.1  Correction services....................................................................................................................... 7
2.1.2  Real time kinematic......................................................................................................................7
2.2 Architecture..............................................................................................................................................9
2.2.1 Block diagram..................................................................................................................................9
2.2.2 Typical ZED-F9P application setups..........................................................................................9
3 Receiver functionality........................................................................................................12
3.1 Receiver conﬁguration.........................................................................................................................12
3.1.1 Changing the receiver conﬁguration....................................................................................... 12
3.1.2 Default GNSS conﬁguration...................................................................................................... 12
3.1.3 Default interface settings..........................................................................................................13
3.1.4 Basic receiver conﬁguration...................................................................................................... 14
3.1.5 RTK conﬁguration........................................................................................................................16
3.1.6 PPP-RTK conﬁguration............................................................................................................... 24
3.1.7 Legacy conﬁguration interface compatibility........................................................................ 28
3.1.8 Navigation conﬁguration............................................................................................................28
3.2 Primary and secondary output.......................................................................................................... 33
3.2.1 Introduction...................................................................................................................................33
3.2.2 Conﬁguration................................................................................................................................ 34
3.2.3 Expected output behavior..........................................................................................................35
3.2.4 Example use cases...................................................................................................................... 35
3.3 SBAS........................................................................................................................................................35
3.4 QZSS SLAS............................................................................................................................................ 37
3.4.1 Features......................................................................................................................................... 37
3.4.2 Conﬁguration................................................................................................................................ 38
3.5 Geofencing..............................................................................................................................................38
3.5.1 Introduction...................................................................................................................................38
3.5.2 Interface......................................................................................................................................... 38
3.5.3 Geofence state evaluation......................................................................................................... 39
3.5.4 Using a PIO for geofence state output................................................................................... 39
3.6 Logging....................................................................................................................................................39
3.6.1 Introduction...................................................................................................................................39
3.6.2 Setting the logging system up................................................................................................. 40
3.6.3 Information about the log..........................................................................................................40
3.6.4 Recording....................................................................................................................................... 41
3.6.5 Retrieval......................................................................................................................................... 42
3.6.6 Command message acknowledgment....................................................................................43
3.7 Protection level......................................................................................................................................43
3.7.1 Introduction...................................................................................................................................43
3.7.2 Interface......................................................................................................................................... 44
3.7.3 Expected behavior........................................................................................................................45
3.7.4 Example use cases...................................................................................................................... 46
UBX-18010802 - R16
 
Contents
Page 3 of 129
C1-Public
```

---

## Page 4

```
ZED-F9P - Integration manual
3.8 Communication interfaces................................................................................................................. 47
3.8.1 UART............................................................................................................................................... 48
3.8.2 I2C interface..................................................................................................................................49
3.8.3 SPI interface..................................................................................................................................52
3.8.4 USB interface................................................................................................................................53
3.9 Predeﬁned PIOs.....................................................................................................................................54
3.9.1 D_SEL..............................................................................................................................................54
3.9.2 RESET_N........................................................................................................................................ 54
3.9.3 SAFEBOOT_N................................................................................................................................54
3.9.4 TIMEPULSE................................................................................................................................... 55
3.9.5 TX_READY..................................................................................................................................... 55
3.9.6 EXTINT............................................................................................................................................55
3.9.7 GEOFENCE_STAT interface....................................................................................................... 56
3.9.8 RTK_STAT interface.....................................................................................................................56
3.10 Antenna supervisor............................................................................................................................56
3.10.1 Antenna voltage control - ANT_OFF..................................................................................... 57
3.10.2 Antenna short detection - ANT_SHORT_N..........................................................................57
3.10.3 Antenna short detection auto recovery................................................................................58
3.10.4 Antenna open circuit detection - ANT_DETECT.................................................................58
3.11 Multiple GNSS assistance (MGA)...................................................................................................59
3.11.1 Authorization..............................................................................................................................59
3.11.2 Preserving MGA and operational data during power-oﬀ...................................................59
3.12 Clocks and time.................................................................................................................................. 60
3.12.1 Receiver local time.................................................................................................................... 60
3.12.2 Navigation epochs.....................................................................................................................60
3.12.3 iTOW timestamps..................................................................................................................... 61
3.12.4 GNSS times.................................................................................................................................61
3.12.5 Time validity................................................................................................................................61
3.12.6 UTC representation...................................................................................................................62
3.12.7 Leap seconds..............................................................................................................................62
3.12.8 Real-time clock...........................................................................................................................63
3.12.9 Date...............................................................................................................................................63
3.13 Timing functionality...........................................................................................................................64
3.13.1 Time pulse...................................................................................................................................64
3.13.2 Time mark................................................................................................................................... 67
3.14 Security.................................................................................................................................................68
3.14.1 Spooﬁng detection and monitoring.......................................................................................69
3.14.2 Jamming and interference detection and monitoring...................................................... 69
3.14.3 Spooﬁng and jamming indication..........................................................................................69
3.14.4 GNSS receiver security.............................................................................................................69
3.14.5 Galileo Open Service Navigation Message Authentication (OSNMA)............................ 70
3.15 u-blox protocol feature descriptions.............................................................................................. 74
3.15.1 Broadcast navigation data...................................................................................................... 74
3.16 Forcing a receiver reset.....................................................................................................................82
3.17 Firmware upload................................................................................................................................. 82
3.18 Spectrum analyzer.............................................................................................................................83
3.19 Production test................................................................................................................................... 84
3.19.1 Connected sensitivity test...................................................................................................... 84
3.19.2 Go/No go tests for integrated devices..................................................................................85
4 Design..................................................................................................................................... 86
UBX-18010802 - R16
 
Contents
Page 4 of 129
C1-Public
```

---

## Page 5

```
ZED-F9P - Integration manual
4.1 Pin assignment......................................................................................................................................86
4.2 Power supply..........................................................................................................................................88
4.2.1 VCC: Main supply voltage.......................................................................................................... 88
4.2.2 V_BCKP: Backup supply voltage............................................................................................... 88
4.2.3 ZED-F9P power supply............................................................................................................... 89
4.3 ZED-F9P minimal design.................................................................................................................... 89
4.4 Antenna...................................................................................................................................................90
4.4.1 Active Antenna Power Supply...................................................................................................92
4.4.2 Antenna supervisor circuit.........................................................................................................95
4.5 EOS/ESD precautions.......................................................................................................................... 97
4.5.1 ESD protection measures.......................................................................................................... 97
4.5.2 EOS precautions...........................................................................................................................97
4.5.3 Safety precautions...................................................................................................................... 98
4.6 Electromagnetic interference on I/O lines.......................................................................................98
4.6.1 General notes on interference issues......................................................................................98
4.6.2 In-band interference mitigation................................................................................................99
4.6.3 Out-of-band interference........................................................................................................... 99
4.7 Layout....................................................................................................................................................100
4.7.1 Placement....................................................................................................................................100
4.7.2 Thermal management..............................................................................................................100
4.7.3 Package footprint, copper and paste mask.........................................................................100
4.7.4 Layout guidance.........................................................................................................................102
4.8 Design guidance................................................................................................................................. 104
4.8.1 General considerations.............................................................................................................104
4.8.2 Backup battery...........................................................................................................................104
4.8.3 RF front-end circuit options....................................................................................................105
4.8.4 Antenna/RF input...................................................................................................................... 105
4.8.5 Ground pads................................................................................................................................106
4.8.6 Schematic design......................................................................................................................106
4.8.7 Layout design-in guideline.......................................................................................................106
4.8.8 I2C design recommendations.................................................................................................106
5 Product handling...............................................................................................................108
5.1 ESD handling precautions................................................................................................................108
5.2 Soldering...............................................................................................................................................109
5.3 Tapes.....................................................................................................................................................112
5.4 Reels...................................................................................................................................................... 113
Appendix..................................................................................................................................114
A Glossary................................................................................................................................................... 114
B Reference frames.................................................................................................................................. 115
C RTK conﬁguration procedures with u-center.................................................................................. 115
C.1 Base conﬁguration with u-center..............................................................................................115
C.2 Rover conﬁguration with u-center............................................................................................ 119
D Stacked patch antenna........................................................................................................................123
Related documents..............................................................................................................127
Revision history.................................................................................................................... 128
UBX-18010802 - R16
 
Contents
Page 5 of 129
C1-Public
```

---

## Page 6

```
ZED-F9P - Integration manual
1 Integration manual overview
This document is an important source of information on all aspects of ZED-F9P system, software
and hardware design. The purpose of this document is to provide guidelines for a successful
integration of the receiver with the customer's end product.
UBX-18010802 - R16
 
1 Integration manual overview
Page 6 of 129
C1-Public
```

---

## Page 7

```
ZED-F9P - Integration manual
2 System description
2.1 Overview
The ZED-F9P positioning module features the u-blox F9 receiver platform, which provides multi-
band GNSS to high-volume industrial applications. ZED-F9P has integrated u-blox multi-band RTK
and PPP-RTK1 technologies for centimeter-level accuracy. The module enables precise navigation
and automation of moving machinery in industrial and consumer-grade products in a compact
surface-mounted form factor of only
The ZED-F9P includes moving base support, allowing both base and rover to move while computing
the position between them. The moving base is ideal for UAV applications where the UAV is
programmed to follow its owner or to land on a moving platform. It is also well suited to attitude
sensing applications where both base and rover modules are mounted on the same moving platform
and the relative position is used to derive attitude information for the vehicle or tool.
Firmware version HPG L1L5 1.40 does not support the moving base feature.
2.1.1  Correction services
u-blox ZED-F9P can use diﬀerent types of correction services which broadly fall into two categories,
based either on an Observation Space Representation (OSR) or a State Space Representation
(SSR) of the errors. These categories use diﬀerent techniques, delivery mechanisms, and core
technologies to solve the same problem – the mitigation of key GNSS errors in order to enable high
precision GNSS:
1.
OSR services provide GNSS observations from the nearest reference station, or virtual
reference station (VRS) to the rover. A communication link from reference station or VRS
service provider to the rover is needed. The data is dependent on the position of the rover.
2.
SSR services rely on GNSS reference station network to model key errors (such as satellite or
atmospheric errors) over large geographical regions and provide corrections to the rover via
broadcast link such as internet or satellite L-band. All receivers receive the same data over a
large area.
In this document, the following terminology is used: RTK refers to OSR-based solution (using RTCM
corrections) while PPP-RTK refers to SSR-based solution (using SPARTN or CLAS corrections).
2.1.2  Real time kinematic
u-blox ZED-F9P high precision receiver takes GNSS precision to the next level:
•
Delivers accuracy down to the centimeter level: 0.01 m + 1 ppm CEP
•
Fast time to ﬁrst ﬁx and robust performance with multi-band, multi-constellation reception
•
Compatible with leading correction services for global coverage and versatility
Some typical applications for the ZED-F9P are shown below:
1 PPP-RTK technology is only available from ﬁrmware HPG1.30 onwards
UBX-18010802 - R16
 
2 System description
Page 7 of 129
C1-Public
```

---

## Page 8

```
ZED-F9P - Integration manual
Figure 1: Typical applications for the ZED-F9P
2.1.2.1 RTK modes of operation
The ZED-F9P supports the following modes of operation:
1.
ZED-F9P operating as a base: It provides RTCM correction data to a rover, or to a network of
rovers.
2.
ZED-F9P operating as a rover: It receives RTCM correction data from a ZED-F9P operating as
a base, or from a VRS service provider operating a network of base receivers.
2.1.2.2 PPP-RTK modes of operation
The ZED-F9P operating as a rover supports the following additional operation modes:
•
It can receive SPARTN correction data via internet from the data center of the GNSS reference
station network
•
It can receive SPARTN correction data via L-band satellite channel (a u-blox NEO-D9S may be
used for such purposes)
•
It can receive CLAS correction data transmitted from Japan's QZSS constellation (from u-blox
NEO-D9C message)
2.1.2.3 NTRIP - networked transport of RTCM via internet protocol
Networked Transport of RTCM via internet protocol, or NTRIP, is an open standard protocol for
streaming diﬀerential data and other kinds of GNSS streaming data over the internet in accordance
with speciﬁcations published by RTCM.
The NTRIP protocol is also used by SSR correction service providers to stream SSR correction data
over the internet (e.g. SPARTN corrections).
There are three major parts to the NTRIP system: The NTRIP client, the NTRIP server, and the NTRIP
caster:
1.
The NTRIP server is a PC or an on-board computer running NTRIP server software
communicating directly with a GNSS reference station. The NTRIP server serves as the
intermediary between the GNSS receiver (NTRIP Source) streaming correction data and the
NTRIP caster.
2.
The NTRIP caster is an HTTP server which receives streaming correction data from one or
more NTRIP servers and in turn streams the correction data to one or more NTRIP clients via
the internet.
3.
The NTRIP client receives streaming correction data from the NTRIP caster to apply as real-
time corrections to a GNSS receiver.
UBX-18010802 - R16
 
2 System description
Page 8 of 129
C1-Public
```

---

## Page 9

```
ZED-F9P - Integration manual
u-center GNSS evaluation software provides a NTRIP client and server application that can be used
to easily evaluate a ZED-F9P base or rover. Typically a u-center NTRIP client connects to an NTRIP
service provider over the internet. The u-center NTRIP client then provides the corrections to a ZED-
F9P rover connected to the local u-center application. VRS service is also supported by the u-center
NTRIP client.
2.2 Architecture
The ZED-F9P receiver provides all the necessary RF and baseband processing to enable  multi-band,
multi-constellation operation. The block diagram below shows the key functionality.
2.2.1 Block diagram
Figure 2: ZED-F9P block diagram
An active antenna is mandatory with the ZED-F9P.
2.2.2 Typical ZED-F9P application setups
Two application examples are illustrated below as typical system implementations. Both are
representative of a simple "short baseline" setup in which the base and rover receivers are within
a few hundred meters of each other. In Figure 3 and Figure 4 ZED-F9P is used as a base station
providing corrections to a ZED-F9P rover receiver.
Alternatively, the rover can use corrections provided over longer baselines from a correction stream
distributed as a subscription service. This method can use a single ﬁxed reference source which is
local (within 50 km) to the rover receiver or via a VRS service in which corrections are synthesized
based on the rover’s location.
UBX-18010802 - R16
 
2 System description
Page 9 of 129
C1-Public
```

---

## Page 10

```
ZED-F9P - Integration manual
The moving base feature also enables derivation of the vehicle orientation by mounting two or three
GNSS receivers on the same vehicle platform, i.e. by ﬁxing the position of the GNSS antennas relative
to each other, as illustrated in Figure 5.
Firmware version HPG L1L5 1.40 does not support the moving base feature.
Figure 3: ZED-F9P base and rover in a short baseline drone application
Figure 4: ZED-F9P base and rover in a short baseline robotic mower application
UBX-18010802 - R16
 
2 System description
Page 10 of 129
C1-Public
```

---
