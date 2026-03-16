# PDF抽出: u-center_Userguide_UBX-13005250.pdf

**総ページ数**: 74
**抽出ページ**: 64, 65, 66

---

## Page 64

```
u-center - User guide
Option
Description
Recommended
USB alternative update
method
Erases the ﬁrst sector of the ﬂash memory and restarts the receiver.
The receiver will then start from ROM and the ﬂash ﬁrmware can be
updated. Use this method if you want to update the ﬁrmware of a
receiver connected over USB.
disabled
Enter Safeboot before
update
Sends the command to enter safeboot. In this state, the receiver
boots from the internal ring oscillator and does not rely on any
external components. GNSS functionality is not started and the
receiver does not output any data. Don't use this method when
having the receiver connected over USB.
enabled
Send Training sequence
Sends the training sequence after safeboot was entered. This
synchronizes the internal ring oscillator so that a communication
can be established.
enabled
7.
Click OK.
9.3 Dump receiver diagnostics
This tool is used to dump the receiver diagnostic to a ﬁle. Use this tool if requested by the support
team.
Figure 57: Dump receiver diagnostics tool
9.4 GNSS conﬁguration
u-center is capable of getting the actual conﬁguration of a u-blox GNSS positioning chip or module
and storing it to an ASCII text ﬁle containing hexadecimal records. Such a ﬁle can be edited and
stored to a u-blox GNSS device again. In u-center Tools menu, select GNSS Conﬁguration to open
the GNSS Conﬁguration dialog box. The following functions are available:
•
Specify the name of a new conﬁguration ﬁle to store current conﬁguration from the u-blox GNSS
device
•
Specify the name of an existing conﬁguration ﬁle and load this conﬁguration into the u-blox GNSS
device
•
A ﬂag can be set to force storing the conﬁguration into a battery-backed RAM (BBR) or ﬂash,
applicable for u-blox 5 to u-blox 8/M8 only.
UBX-13005250 - R30
 
9 Tools
Page 64 of 74
C1-Public
Production information
```

---

## Page 65

```
u-center - User guide
Figure 58: GNSS conﬁguration tool
9.4.1 Read/Write conﬁguration ﬁles
1.
Connect to the device.
2.
Open "Tools > GNSS Conﬁguration".
3.
Select the u-blox receiver generation connected from the Generation drop down menu.
4.
To read an existing conﬁguration ﬁle, select the name of the ﬁle, then click Transfer ﬁle >
GNSS button. The GNSS Conﬁguration window then closes and the progress window pops
up, showing the conﬁguration being sent to the receiver. This progress window closes after
successful transfer.
5.
To write a new conﬁguration ﬁle, click " Transfer GNSS > File". The GNSS Conﬁguration window
closes and the progress window pops up, showing the conﬁgurations being polled and stored
into a local ﬁle in ASCII format.
Select Store conﬁguration into BBR/Flash checkbox if the parameters need to be stored
into the device's non-volatile memory (BBR/Flash). This option is applicable for u-blox 5 to u-
blox 8/M8 generation receivers.
Sending a conﬁguration to a u-blox GNSS device may fail due to a baud rate change on the
current serial port of the receiver to which the conﬁguration is being sent. If this happens,
simply change the u-center baud rate and send the conﬁguration again.
If reading or writing conﬁguration data fails too frequently, try to increase the number of retries u-
center should do on a single message if one fails.
The user can abort the transfer by clicking the Abort button. It is not possible to close the window
unless the transfer is complete or aborted by the user.
It is not recommended to read/write conﬁguration while the u-blox GNSS device is in sleep
mode.
9.4.2 Editing conﬁguration ﬁle
When clicking the Edit button in the GNSS Conﬁguration dialog, the Notepad editor opens (standard
Windows software). Conﬁgurations are stored the following way:
•
The ﬁrst line contains the version of the u-blox GNSS receiver where the conﬁguration is from.
Never change this line!
•
For the second and following lines, each line contains the same: <class ID>-<message ID> -
<hexadecimal byte code of the message>. The byte code consists of class and message
IDs (2 bytes), payload length (2 bytes), payload (payload length bytes). The sync characters and
the checksum are not included. They will be calculated automatically.
UBX-13005250 - R30
 
9 Tools
Page 65 of 74
C1-Public
Production information
```

---

## Page 66

```
u-center - User guide
Refer to u-blox Receiver Description including Protocol Specification for detailed information and
ranges.
Figure 59: Content of GNSS conﬁguration ﬁle
9.5 Preferences
The preferences tool can be used to conﬁgure a number of u-center parameters.
UBX-13005250 - R30
 
9 Tools
Page 66 of 74
C1-Public
Production information
```

---
