# PDF抽出: u-center_Userguide_UBX-13005250.pdf

**総ページ数**: 74
**抽出ページ**: 35, 36, 37, 38

---

## Page 35

```
u-center - User guide
5.2.6.1 Receiver output messages
Figure 28: Message display of an output message
Double-clicking on an output message enables or disables the periodic message update if
the communication protocol is active. This feature is currently only supported for the UBX
protocol.
5.2.7 Generation 9 conﬁguration view
The new u-blox Generation 9 conﬁguration view allows the users to check the current conﬁguration
of the receiver and change it if needed. This view can only be used to conﬁgure u-blox 9 generation
receivers.
By default, the conﬁguration values being edited come from (and can be written back to) an
attached receiver. This view consists of two diﬀerent sub-views, GNSS Conﬁguration and Advanced
Conﬁguration.
5.2.7.1 GNSS conﬁguration
This GNSS conﬁguration sub-view enables the users to poll and conﬁgure the basic and advanced
GNSS system conﬁgurations of the attached receiver. This sub-view describes sections below:
UBX-13005250 - R30
 
5 u-center menus and windows
Page 35 of 74
C1-Public
Production information
```

---

## Page 36

```
u-center - User guide
Figure 29: u-blox Generation 9 Advanced Conﬁguration View
Basic
This section shows the GNSS constellation conﬁguration of the receiver by pressing
Poll Active Conﬁguration. The receiver’s GNSS constellations can also be conﬁgured
by enabling/disabling the required satellite constellations, and then pressing Send
Conﬁguration. See position a.
Advanced
This section shows the GNSS constellation signal information conﬁgured in the
receiver by pressing Poll Active Conﬁguration. The receiver’s GNSS signals can also
be conﬁgured by enabling/disabling the required signals, and then pressing Send
Conﬁguration. See position b.
Show Hex/ Hide Hex
Shows or hides the hex values that make up the messages describing the receiver
conﬁguration just sent or received. See position c.
Status
Shows the status of the action taken. See position d.
Write to layer
Once selected the desired layer, Send conﬁguration button sends the GNSS
constellation and signal information to the receiver. See position e.
Poll Active Conﬁguration (RAM
Layer)
Polls the GNSS constellation and signal conﬁguration from RAM layer of the attached
receiver. See position f.
Table 15: Description of the buttons and sections in the GNSS Conﬁguration sub-view
5.2.7.2 Advanced Conﬁguration
In the Advanced Conﬁguration sub-view, all groups of conﬁguration items are displayed in a tree
structure. Expanding a group will show the RAM layer values for all readable conﬁguration items in
that group. Each item is read individually from the receiver as the group is expanded. Some item
values may not be known to the receiver in which case the value will be shown as "-".
If no receiver is attached or if the receiver does not support the new conﬁguration concept then no
values will be visible. If the receiver is not responding for some reason then close the group and open
it again to retry.
UBX-13005250 - R30
 
5 u-center menus and windows
Page 36 of 74
C1-Public
Production information
```

---

## Page 37

```
u-center - User guide
Figure 30: Advanced Conﬁguration view
Users can expand items of interest and u-center will attempt to read values for all the other layers
such as BBR, ﬂash, ROM, pin, etc. and display any that it ﬁnds. This sub-view contains sections
below:
Conﬁguration item search
To search for an item by name, type into the search text box just above the tree. The
search is case-insensitive. See position a.
Conﬁguration item tree view
All entries that contain the search text will be highlighted in red. The search will check
for a match in group and item names, titles and descriptions. If a group contains an
item which matches then the group will be highlighted as well. See position b.
Selected Conﬁguration Item
This section describes the selected item in more detail. The ﬁeld having the searched
item is shown in red. See position c.
Load diﬀerences from default
Click on Load receiver diﬀerences from default to read the conﬁguration values
set in the receiver. This can be used to duplicate the current settings in another
receiver. The operation depends on the working mode of the tool. If used in the
normal, attached to receiver mode then the "writes" list will be populated with any
settings in the FS or BBR layers. See position d.
Send conﬁg changes
"Send conﬁg changes" will send the current set of settings to the attached receiver.
A tick will appear next to the items which were successfully altered in the receiver.If
the receiver does not acknowledge the request, a cross appears. If there is no tick or a
cross, the receiver does not respond to the request. Items are sent in groups of values
to be sent to the same layer. If one value for a layer cannot be written, then all values
for that layer will fail to be written and will show a cross. See position e.
UBX-13005250 - R30
 
5 u-center menus and windows
Page 37 of 74
C1-Public
Production information
```

---

## Page 38

```
u-center - User guide
Items to delete
If an item that can be deleted from the receiver (if it is in the BBR or Flash layers) is
selected from the tree, then a button Delete will appear. If that button is selected
then it will be added to the list of deletions shown in the "Items to delete" section on
the right of the tree. See position f.
Items to set
If a writable layer item is selected, then press one of the layer buttons to add write
operations to a list of item writes. A value can be changed by clicking on it in the top
left sub-view area before writing. If two values for the same item are selected in the
same layer, then the earlier one will be highlighted in red to show that it will be ignored
as a duplicate setting. See position g.
Remove from list
Removes the selected item from one of the lists. See position h.
Clear lists
Removes all the items from all the lists. See position i.
Load from ﬁle...
Loads a group of settings from a readable text ﬁle. See position j.
Save to ﬁle...
To save the current list of settings to a readable text ﬁle, click Save to ﬁle... and
choose a ﬁle path. This will produce a ﬁle in the ASCII format. See position k.
Message hex codes
Lists the hex values for the UBX-CFG-VALSET message that will be constructed for
setting the selected conﬁguration properties. See position l.
Table 16: Description of the buttons and sections in the Advanced Conﬁguration sub-view
5.2.8 Statistic view
Figure 31: Statistic view
All available database values (transmitted from the device or calculated by u-center) are displayed.
The following statistics are displayed:
•
Current value
•
Minimum value
UBX-13005250 - R30
 
5 u-center menus and windows
Page 38 of 74
C1-Public
Production information
```

---
