# PDF抽出: u-center_Userguide_UBX-13005250.pdf

**総ページ数**: 74
**抽出ページ**: 1, 2, 3, 4, 5

---

## Page 1

```

u-center
GNSS evaluation software for Windows
User guide
Abstract
This document leads you through the eﬃcient use of the u-center
evaluation software, the powerful and easy to use tool from u-blox for
evaluating, performance analysis and conﬁguration of u-blox GNSS
positioning chips and modules.
www.u-blox.com
UBX-13005250 - R30
C1-Public
```

---

## Page 2

```
u-center - User guide
Document information
Title
u-center
Subtitle
GNSS evaluation software for Windows
Document type
User guide
Document number
UBX-13005250
Revision and date
R30
24-May-2022
Document status
Production information
Disclosure restriction
C1-Public
This document applies to the following products:
Product name
Type number
Firmware version
PCN reference
u-center
u-blox or third parties may hold intellectual property rights in the products, names, logos and designs included in this
document. Copying, reproduction, modiﬁcation or disclosure to third parties of this document or any part thereof is only
permitted with the express written permission of u-blox.
The information contained herein is provided "as is" and u-blox assumes no liability for its use. No warranty, either express
or implied, is given, including but not limited to, with respect to the accuracy, correctness, reliability and ﬁtness for a
particular purpose of the information. This document may be revised by u-blox at any time without notice. For the most recent
documents, visit www.u-blox.com.
Copyright © 2022, u-blox AG.
UBX-13005250 - R30
 
Page 2 of 74
C1-Public
Production information
```

---

## Page 3

```
u-center - User guide
Contents
1 Preface.......................................................................................................................................5
1.1 Overview.................................................................................................................................................... 5
1.2 Using this guide.......................................................................................................................................5
1.3 Technical support....................................................................................................................................5
1.3.1 Worldwide Web............................................................................................................................... 5
1.3.2 By email............................................................................................................................................ 5
1.3.3 Helpful information when contacting technical support.......................................................5
2 Features....................................................................................................................................7
3 Getting started...................................................................................................................... 8
3.1 General information about displayed values.....................................................................................8
3.2 Connecting an u-blox evaluation kit to the PC................................................................................. 8
3.3 Installing u-center...................................................................................................................................8
3.4 Connect to the receiver......................................................................................................................... 9
3.4.1 Select the port................................................................................................................................ 9
3.4.2 Select the baud rate (only for COM ports)................................................................................9
4 Concept and philosophy................................................................................................... 11
4.1 Color and satellite coding scheme....................................................................................................12
4.2 Operating modes.................................................................................................................................. 13
4.2.1 Online mode.................................................................................................................................. 14
4.2.2 Stop mode..................................................................................................................................... 14
4.2.3 Record mode................................................................................................................................. 14
4.2.4 Play mode.......................................................................................................................................14
4.2.5 Relations between modes..........................................................................................................14
4.2.6 Database limitations...................................................................................................................15
4.2.7 Receiver information................................................................................................................... 16
5 u-center menus and windows......................................................................................... 17
5.1 Main frame and toolbars.....................................................................................................................17
5.1.1 Standard menu bar..................................................................................................................... 17
5.1.2 Standard toolbar.......................................................................................................................... 26
5.1.3 Views toolbar................................................................................................................................ 26
5.1.4 Communication toolbar..............................................................................................................26
5.1.5 Logﬁle toolbar...............................................................................................................................26
5.1.6 Action toolbar............................................................................................................................... 27
5.1.7 Standard statusbar.....................................................................................................................27
5.2 Views and windows.............................................................................................................................. 28
5.2.1 Packet console..............................................................................................................................28
5.2.2 Binary console...............................................................................................................................29
5.2.3 Text console.................................................................................................................................. 30
5.2.4 Icons and text ﬁeld of console views.......................................................................................31
5.2.5 Regular expression evaluation.................................................................................................. 31
5.2.6 Messages view..............................................................................................................................33
5.2.7 Generation 9 conﬁguration view...............................................................................................35
5.2.8 Statistic view................................................................................................................................ 38
UBX-13005250 - R30
 
Contents
Page 3 of 74
C1-Public
Production information
```

---

## Page 4

```
5.2.9 Table view...................................................................................................................................... 39
5.2.10 Map view......................................................................................................................................40
5.2.11 Chart view................................................................................................................................... 46
5.2.12 Histogram view...........................................................................................................................49
5.2.13 Camera view................................................................................................................................51
5.2.14 Deviation map.............................................................................................................................51
5.2.15 Sky view....................................................................................................................................... 53
6 NTRIP...................................................................................................................................... 54
6.1 NTRIP client........................................................................................................................................... 54
6.2 NTRIP server/caster............................................................................................................................. 55
7 MQTT.......................................................................................................................................57
7.1 MQTT client........................................................................................................................................... 57
8 Google Earth server............................................................................................................59
9 Tools........................................................................................................................................ 61
9.1 Firmware update...................................................................................................................................61
9.2 Legacy Firmware update u-blox 5 - 8...............................................................................................63
9.3 Dump receiver diagnostics................................................................................................................. 64
9.4 GNSS conﬁguration..............................................................................................................................64
9.4.1 Read/Write conﬁguration ﬁles.................................................................................................. 65
9.4.2 Editing conﬁguration ﬁle............................................................................................................ 65
9.5 Preferences.............................................................................................................................................66
10 How To..................................................................................................................................67
10.1 Change baud rate of receiver.......................................................................................................... 67
10.2 Save parameters to receiver non-volatile memory (BBR/Flash)............................................... 67
10.2.1 Saving parameters with UBX-CFG-CFG................................................................................67
10.2.2 Saving parameters with GNSS conﬁguration......................................................................68
10.3 Recording/Playing a log ﬁle.............................................................................................................. 68
10.4 Conduct sensitivity tests................................................................................................................. 68
10.5 Read/Write conﬁguration ﬁles.........................................................................................................69
10.6 Set GNSS conﬁguration....................................................................................................................69
10.7 Change epoch detection method................................................................................................... 69
11 Troubleshooting................................................................................................................ 71
Related documents................................................................................................................ 72
Revision history.......................................................................................................................73
```

---

## Page 5

```
u-center - User guide
1 Preface
1.1 Overview
u-center is u-blox's powerful GNSS evaluation and visualization tool which can be downloaded free-
of-charge from our website (www.u-blox.com). This user guide provides a description of the features
of this software. It allows end users to assess and test u-blox GNSS positioning chips and modules
for navigation and positioning performance.
The purpose of u-center is to enable users to:
•
Conduct performance tests on u-blox and other GNSS devices.
•
Conﬁgure u-blox GNSS positioning chips and modules.
•
Update the ﬁrmware on GNSS modules.
•
Test the added performance provided by u-blox's AssistNow service.
1.2 Using this guide
This guide assumes that the user has basic computer skills and is familiar with the Windows
Graphical User Interface (GUI) and GNSS receiver environments.
The following symbols are used to highlight important information:
An index ﬁnger points out key information pertaining to integration and performance.
A warning symbol indicates actions that could negatively impact u-center behavior.
1.3 Technical support
If you have questions about installing or using u-center:
•
Read this user guide carefully.
•
Check our homepage (https://www.u-blox.com) to ensure that your GNSS device, ﬁrmware and
the u-center software are the latest versions.
•
Refer to our web based information service.
1.3.1 Worldwide Web
Our website (www.u-blox.com) is a rich pool of information. Product information and technical
documents can be accessed 24/7.
1.3.2 By email
If you have technical problems or cannot ﬁnd the required information in the provided documents,
contact the closest technical support oﬃce. To ensure that we process your request as soon as
possible, use our service pool email addresses rather than personal staﬀ email addresses. Contact
details are at the end of the document.
1.3.3 Helpful information when contacting technical support
When contacting technical support, have the following information ready:
•
Receiver type (e.g. NEO-M9N), ﬁrmware version (e.g. FW SPG 4.04), u-center release (e.g. u-
center 21.09).
•
Receiver conﬁguration and short description of the application.
UBX-13005250 - R30
 
1 Preface
Page 5 of 74
C1-Public
Production information
```

---
