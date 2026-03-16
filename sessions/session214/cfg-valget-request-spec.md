# PDF抽出: u-blox-F9-HPG-1.32_InterfaceDescription_UBX-22008968.pdf

**総ページ数**: 305
**抽出ページ**: 95

---

## Page 95

```
u-blox F9 HPG 1.32 - Interface Description
3
U1
reserved0
-
-
Reserved
Start of repeated group (N times)
4 + n·4
U4
keys
-
-
Conﬁguration key IDs of the conﬁguration items to be
deleted
End of repeated group (N times)
3.10.25 UBX-CFG-VALGET (0x06 0x8b)
3.10.25.1 Get conﬁguration items
Message
UBX-CFG-VALGET
Get conﬁguration items
Type
Poll request
Comment
Overview:
•
This message is used to get conﬁguration values by providing a list of conﬁguration key IDs, which
identify the conﬁguration items to retrieve.
•
This message can specify the conﬁguration layer where the values of the speciﬁed conﬁguration items
are retrieved from.
•
This message is limited to containing a maximum of 64 key IDs.
•
See Receiver conﬁguration for details.
This message returns a UBX-ACK-NAK:
•
if any key is unknown to the receiver FW
•
if the layer ﬁeld speciﬁes an invalid layer to get the value from
•
if the keys array speciﬁes more than 64 key IDs.
Notes:
•
If a value is requested multiple times within the same poll request, then the reply will contain it multiple
times.
•
The provided keys can be complete key values (group and item speciﬁers) or wild-card speciﬁcations.
A complete key value will constitute a request for one key-value pair. A key value that has a valid group
speciﬁer and 0xﬀﬀ in the item part of the key value (bits 0-15) constitutes a request for all items in the
speciﬁed group. A key with a value of 0xﬀf in the group part of the key value (bits 16-27) is a request for
all items known to the receiver in all groups.
•
The response message is limited to containing a maximum of 64 key-value pairs. If there are wild-card
speciﬁcations then there may be more than 64 possible responses. In order to handle this, the 'position'
ﬁeld can specify that the response message should skip this number of key-value pairs before it starts
constructing the message. This allows a large set of values to be retrieved 64 at a time. If the response
contains less than 64 key-value pairs then all values have been reported, otherwise there may be more to
read.
•
It is not possible to retrieve conﬁguration values for the same conﬁguration item from multiple
conﬁguration layers. Separate poll requests must be made for each desired layer.
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
0x8b
4 + [0..n]·4
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
layer
-
-
The layer from which the conﬁguration items should
be retrieved:
•
0 - RAM layer
•
1 - BBR layer
•
2 - Flash layer
•
7 - Default layer
2
U2
position
-
-
Skip this many key values before constructing output
message
Start of repeated group (N times)
UBX-22008968 - R01
 
3 UBX protocol
Page 95 of 305
C1-Public
```

---
