# PDF抽出: u-center-25.06_ReleaseNote_UBXDOC-304424225-19978.pdf

**総ページ数**: 15
**抽出ページ**: 1, 2, 3, 4, 5, 6, 7, 8, 9, 10

---

## Page 1

```

u-center 25.06
GNSS evaluation software for Windows
Release note
Abstract
This release note contains general information, features, improvements, changes and known
limitations for u-center 25.06.
UBXDOC-304424225-19978 - R01
 
C1-Public
www.u-blox.com
```

---

## Page 2

```
u-center 25.06 - Release note
Document information
Title
u-center 25.06
Subtitle
GNSS evaluation software for Windows
Document type
Release note
Document number
UBXDOC-304424225-19978
Revision and date
R01
04-Jun-2025
Disclosure restriction
C1-Public
u-blox or third parties may hold intellectual property rights in the products, names, logos and designs included in this
document. Copying, reproduction, or modiﬁcation of this document or any part thereof is only permitted with the express
written permission of u-blox. Disclosure to third parties is permitted for clearly public documents only.
The information contained herein is provided "as is" and u-blox assumes no liability for its use. No warranty, either express
or implied, is given, including but not limited to, with respect to the accuracy, correctness, reliability and ﬁtness for a
particular purpose of the information. This document may be revised by u-blox at any time without notice. For the most recent
documents and product statuses, visit www.u-blox.com.
Copyright © 2025, u-blox AG.
UBXDOC-304424225-19978 - R01
 
Document information
Page 2 of 15
C1-Public
```

---

## Page 3

```
u-center 25.06 - Release note
Contents
Document information............................................................................................................ 2
Contents.......................................................................................................................................3
1 General information..............................................................................................................4
1.1 Released ﬁles........................................................................................................................................... 4
1.2 Related documents.................................................................................................................................4
2 Features....................................................................................................................................5
2.1 Supported Microsoft Windows versions............................................................................................5
2.2 Supported receiver generations...........................................................................................................5
2.3 Supported protocol speciﬁcation version.......................................................................................... 5
2.4 External hardware support....................................................................................................................5
2.5 Multiple GNSS AssistNow Service...................................................................................................... 5
3 Improvements and changes...............................................................................................6
3.1 Version 25.06........................................................................................................................................... 6
3.2 Version 25.03........................................................................................................................................... 6
3.3 Version 24.10........................................................................................................................................... 6
3.4 Version 24.05........................................................................................................................................... 6
3.5 Version 24.04........................................................................................................................................... 6
3.6 Version 24.02........................................................................................................................................... 7
3.7 Version 23.08........................................................................................................................................... 7
3.8 Version 22.07........................................................................................................................................... 7
3.9 Version 22.05........................................................................................................................................... 7
3.10 Version 22.02.........................................................................................................................................7
4 Known limitations..................................................................................................................9
4.1 Known issues........................................................................................................................................... 9
5 Open source software licenses.......................................................................................10
5.1 Licenses.................................................................................................................................................. 10
Contact.......................................................................................................................................15
UBXDOC-304424225-19978 - R01
 
Contents
Page 3 of 15
C1-Public
```

---

## Page 4

```
u-center 25.06 - Release note
1 General information
The u-center GNSS evaluation software for automotive, mobile terminal and infrastructure
applications provides a powerful tool for evaluation, performance analysis and conﬁguration of the
u-blox GNSS receiver.
1.1 Released ﬁles
The following table lists the ﬁles that have been released.
File
Description
u-center_v25.06.exe
u-center installer including u-blox CDC-ACM (USB) (x64 bit)
driver v1.2.0.8 for Windows 7 and 8
u-center-
v25.06_ReleaseNote_UBXDOC-304424225-19688.pdf
Release note
u-center_UserGuide_UBX-13005250.pdf
User guide
For Windows 10, use the Microsoft CDC-ACM (USB) driver available from the Windows Update.
u-blox sensor and VCP drivers have been removed from the release package.
1.2 Related documents
•
u-center user guide, UBX-13005250
UBXDOC-304424225-19978 - R01
 
1 General information
Page 4 of 15
C1-Public
```

---

## Page 5

```
u-center 25.06 - Release note
2 Features
2.1 Supported Microsoft Windows versions
u-center 25.06 runs on Microsoft Windows Vista and later versions including Windows 11.
2.2 Supported receiver generations
u-center 25.06 supports the following receiver generations:
•
u-blox 7
•
u-blox 8/M8
•
u-blox 9
2.3 Supported protocol speciﬁcation version
u-center 25.06 supports the u-blox protocol speciﬁcation from version 10.00 and later versions up
to (including) version 51.
2.4 External hardware support
The following external hardware is supported to interface with the receiver:
Interface
Device
DLL version
DDC/I2C
Totalphase Aardvark I2C/SPI Host Adapter
aardvark.dll v5.15 for Windows 32-bit
DDC/I2C
Diolan U2C-12 USB-I2C/SPI/GPIO Adapter
i2cbrdg.dll v1.0.0.9 for Windows 32-bit
SPI
Totalphase Aardvark I2C/SPI Host Adapter
aardvark.dll v5.15 for Windows 32-bit
SPI
Totalphase Cheetah SPI Host Adapter
aardvark.dll v5.15 for Windows 32-bit
SPI
Diolan U2C-12 USB-I2C/SPI/GPIO Adapter
i2cbrdg.dll v1.0.0.9 for Windows 32-bit
2.5 Multiple GNSS AssistNow Service
u-center 25.06 includes the multiple GNSS AssistNow Service (libMGA) version 24.02.
UBXDOC-304424225-19978 - R01
 
2 Features
Page 5 of 15
C1-Public
```

---

## Page 6

```
u-center 25.06 - Release note
3 Improvements and changes
For detailed information on any new messages or protocols, contact u-blox support or see product
Interface description.
3.1 Version 25.06
•
Bug ﬁx
•
Fixed a bug in the Universal GNSS Driver view where the ﬁx status was reported incorrectly.
3.2 Version 25.03
•
Features
•
New conﬁguration properties:
•
CFG-SFIMU-IMU_MNTALG_TOLERANCE
•
CFG-SFODO-IMU2VRP_LA_X
•
CFG-SFODO-IMU2VRP_LA_Y
•
CFG-SFODO-IMU2VRP_LA_Z
•
CFG-SFODO-TRACKGAUGE_REAR
•
New 'RAIL' option added to the CFG-NAVSPG-DYNMODEL conﬁguration property
•
Message UBX-NAV-ODO is now public
•
Improved Point Perfect performance
3.3 Version 24.10
•
Features
•
Added support for NMEA UTC message.
•
Added support for QZSS L1CB signal.
•
Improvements
•
Protocol speciﬁcation updates.
•
Updates to message UBX-RXM-SFRBX. Fixes and support for latest versions.
•
Updates to message UBX-NAV2-PVT. Fixes and support for latest versions.
•
Updates to message UBX-MON-GNSS. Fixes and support for latest versions.
•
Updates to message UBX-SEC-OSNMA. Fixes and support for latest versions.
•
Bug ﬁx
•
General ﬁxes for BDS B3I support.
•
General ﬁxes for GAL E6B support.
•
Fixes to hex console in GNSS conﬁguration view and corrected polling for signals.
3.4 Version 24.05
•
Bug ﬁx
•
Galileo OSNMA Merkle and public keys can now be generated with the correct size of the
integer.
3.5 Version 24.04
•
Features
•
Added support for u-blox A9 FW SME (Safe Measurement Engine) 1.6 and related UBX-
SUBX-* messages.
UBXDOC-304424225-19978 - R01
 
3 Improvements and changes
Page 6 of 15
C1-Public
```

---

## Page 7

```
u-center 25.06 - Release note
3.6 Version 24.02
•
Features
•
Added new messages UBX-MGA-GAL-OSNMA MERKELE and UBX-MGA-GAL-OSNMA
PUBLIC to improve Galileo OS-NMA support.
•
Improvements
•
Updated UBX-SEC-OSNMA with new key status and other improvements.
•
Bug ﬁxes
•
Fixed the I2C PER SCK virtual pin number in message UBX-CFG-VP.
•
Fixed wrong payload structure for generation 10 in message UBX-MON-HW.
•
Fixed to make UBX-NAV-PVT message ﬁeld nmaFixStatus visible.
3.7 Version 23.08
•
Features
•
Improvements
•
Added support for L5 band signals in table view.
•
Added Galileo OS-NMA ﬂags in messages UBX-NAV-PVT, UBX-NAV2-SIG, and added new
message UBX-SEC-OSNMA.
•
Fixed PointPerfect timeout issue for network unavailability. Implemented initial logic for tile
and node changes and support for AssistNow updates.
•
Added protection level invalidity reason in case signal attenuation compensation (SAC) has
been enabled and correct message version for plTime ﬁeld in message UBX-NAV-PL.
•
Added authentication in message UBX-NAV-TIMEUTC to make sure that the conversion
between GNSS time and UTC time has been performed using OSNMA authenticated
conversion parameters.
•
Added initStatus ﬁeld in message UBX-ESF-STATUS.
•
Bug ﬁxes
•
Message UBX-CFG-TMODE2 is available now which was missing in u-center v22.05 and
v22.07.
3.8 Version 22.07
•
Features
•
AssistNow Oﬄine tool now provides MGA data for BeiDou and Galileo.
•
Bug ﬁxes
3.9 Version 22.05
•
Improvements
•
Improved MQTT tool with options to subscribe to key, AssistNow and data topics.
•
Adjusted UBX-RXM-SPARTNKEY message so users can't enable/disable it anymore as being
only the pollable message.
•
Bug ﬁxes
3.10 Version 22.02
•
Features
•
Added new messages UBX-MGA-INI-ATT, UBX-MGA-SF-INI, UBX-MGA-SF-INI2 to support
"Advanced calibration handling" feature.
•
Added new message UBX-RXM-COR to support multiple correction sources.
•
Position protection level reported values from message UBX-NAV-PL are added in chart and
statistic views.
UBXDOC-304424225-19978 - R01
 
3 Improvements and changes
Page 7 of 15
C1-Public
```

---

## Page 8

```
u-center 25.06 - Release note
•
Improvements
•
At successful MQTT connection, the client can now connect to the PointPerfect and
AssistNow Online services.
UBXDOC-304424225-19978 - R01
 
3 Improvements and changes
Page 8 of 15
C1-Public
```

---

## Page 9

```
u-center 25.06 - Release note
4 Known limitations
4.1 Known issues
•
Enumerating the ports on a computer with many ports can take a long time.
•
Using the protocol ﬁlter with "silent" or "no receiver" results in an empty message tree.
•
Full support for u-blox 10 generation receiver is not yet available.
•
Content in the MSG column of message UBX-RXM-SFRBX is speciﬁc to each constellation.
UBXDOC-304424225-19978 - R01
 
4 Known limitations
Page 9 of 15
C1-Public
```

---

## Page 10

```
u-center 25.06 - Release note
5 Open source software licenses
This chapter provides license information for the open source software components included in the
u-center software package.
5.1 Licenses
License number
License
1
paho.mqtt license
Eclipse Distribution License - v 1.0
Copyright (c) 2007, Eclipse Foundation, Inc. and its licensors.
All rights reserved.
Redistribution and use in source and binary forms, with or without modiﬁcation, are
permitted provided that the following conditions are met:
Redistributions of source code must retain the above copyright notice, this list of
conditions and the following disclaimer.
Redistributions in binary form must reproduce the above copyright notice, this
list of conditions and the following disclaimer in the documentation and/or other
materials provided with the distribution.
Neither the name of the Eclipse Foundation, Inc. nor the names of its contributors
may be used to endorse or promote products derived from this software without
speciﬁc prior written permission.
THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND
CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS
BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT
OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS;
OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
2
SQLite license, https://www.sqlite.org/copyright.html
3
Libjpeg license
LICENSE TERMS (ships as a part of the libjpeg package in the README ﬁle)
=============
1. We don't promise that this software works. (But if you ﬁnd any bugs,
please let us know!)
2. You can use this software for whatever you want. You don't have to pay us.
3. You may not pretend that you wrote this software. If you use it in a
program, you must acknowledge somewhere in your documentation that
you've used the IJG code.
In legalese:
UBXDOC-304424225-19978 - R01
 
5 Open source software licenses
Page 10 of 15
C1-Public
```

---
