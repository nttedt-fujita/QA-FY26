# PDF抽出: u-blox-F9-HPG-1.32_InterfaceDescription_UBX-22008968.pdf

**総ページ数**: 305
**抽出ページ**: 93, 94, 95, 96, 97

---

## Page 93

```
u-blox F9 HPG 1.32 - Interface Description
12
CH[32]
vendorString
-
-
String containing the vendor name. 32 ASCII bytes
including 0-termination.
44
CH[32]
productString
-
-
String containing the product name. 32 ASCII bytes
including 0-termination.
76
CH[32]
serialNumber
-
-
String containing the serial number. 32 ASCII bytes
including 0-termination.
Changing the String ﬁelds requires special Host
drivers.
3.10.24 UBX-CFG-VALDEL (0x06 0x8c)
3.10.24.1 Delete conﬁguration item values
Message
UBX-CFG-VALDEL
Delete conﬁguration item values
Type
Set
Comment
Overview:
•
This message can be used to delete saved conﬁguration to eﬀectively revert the item values to defaults.
•
This message can delete saved conﬁguration from the ﬂash conﬁguration layer and the BBR
conﬁguration layer. The changes will not be eﬀective until these layers are loaded into the RAM layer.
•
This message is limited to containing a maximum of 64 keys up for deletion; i.e. N is a maximum of 64.
•
This message can be used multiple times and every time the result will be applied immediately. To send
this message multiple times with the result being applied at the end, see version 1 of UBX-CFG-VALDEL
that supports transactions.
•
This message does not check if the resulting conﬁguration is valid.
•
See Receiver conﬁguration for details.
This message returns a UBX-ACK-NAK and no conﬁguration is applied:
•
if any key is unknown to the receiver FW
•
if the layer's bitﬁeld does not specify a layer to delete a value from.
Notes:
•
If a key is sent multiple times within the same message, then the value is eﬀectively deleted only once.
•
Attempting to delete items that have not been set before, or that have already been deleted, is
considered a valid request.
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
0x8c
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
X1
layers
-
-
The layers where the conﬁguration should be deleted
from
bit 1   U:1
bbr
-
-
Delete conﬁguration from the BBR layer
bit 2   U:1
flash
-
-
Delete conﬁguration from the Flash layer
2
U1[2]
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
UBX-22008968 - R01
 
3 UBX protocol
Page 93 of 305
C1-Public
```

---

## Page 94

```
u-blox F9 HPG 1.32 - Interface Description
3.10.24.2 Delete conﬁguration item values (with transaction)
Message
UBX-CFG-VALDEL
Delete conﬁguration item values (with transaction)
Type
Set
Comment
Overview:
•
This message can be used to delete saved conﬁguration to eﬀectively revert them to defaults.
•
This message can delete saved conﬁguration from the ﬂash conﬁguration layer and the BBR
conﬁguration layer. The changes will not be eﬀective until these layers are loaded into the RAM layer.
•
This message is limited to containing a maximum of 64 keys up for deletion; i.e. N is a maximum of 64.
•
This message can be used multiple times with the result being managed within a transaction.
•
This message does not check if the resulting conﬁguration is valid.
•
See Receiver conﬁguration for details.
•
See version 0 of UBX-CFG-VALDEL for simpliﬁed version of this message.
This message returns a UBX-ACK-NAK, cancels any started transaction, and no conﬁguration is applied:
•
if any key within a transaction is unknown to the receiver FW
•
if an invalid transaction state transition is requested
•
if the layer's bitﬁeld changes within a transaction
•
if the layer's bitﬁeld does not specify a layer to delete a value from.
Notes:
•
Any request for another UBX-CFG- message type (including UBX-CFG-VALSET and UBX-CFG-VALGET)
will cancel any started transaction, and no conﬁguration is applied.
•
This message can be sent with no keys to delete for the purposes of managing the transaction state
transition.
•
If a key is sent multiple times within the same message or within the same transaction, then the value is
eﬀectively deleted only once.
•
Attempting to delete items that have not been set before, or that have already been deleted, is
considered a valid request.
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
0x8c
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
Message version (0x01 for this version)
1
X1
layers
-
-
The layers where the conﬁguration should be deleted
from
bit 1   U:1
bbr
-
-
Delete conﬁguration from the BBR layer
bit 2   U:1
flash
-
-
Delete conﬁguration from the Flash layer
2
X1
transaction
-
-
Transaction action to be applied:
bits 1…0   U:2
action
-
-
Transaction action to be applied:
•
0 = Transactionless UBX-CFG-VALDEL: In the
next UBX-CFG-VALDEL, it can be either 0 or 1.
If a transaction has not yet been started, the
incoming conﬁguration is applied. If a transaction
has already been started, cancels any started
transaction and the incoming conﬁguration is
applied.
•
1 = (Re)Start deletion transaction: In the next
UBX-CFG-VALDEL, it can be either 0, 1, 2 or
3. If a transaction has not yet been started, a
transaction will be started. If a transaction has
already been started, restarts the transaction,
eﬀectively removing all previous non-applied UBX-
CFG-VALDEL messages.
•
2 = Deletion transaction ongoing: In the next UBX-
CFG-VALDEL, it can be either 0, 1, 2 or 3.
•
3 = Apply and end a deletion transaction: In the
next UBX-CFG-VALDEL, it can be either 0 or 1.
UBX-22008968 - R01
 
3 UBX protocol
Page 94 of 305
C1-Public
```

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

## Page 96

```
u-blox F9 HPG 1.32 - Interface Description
4 + n·4
U4
keys
-
-
Conﬁguration key IDs of the conﬁguration items to be
retrieved
End of repeated group (N times)
3.10.25.2 Conﬁguration items
Message
UBX-CFG-VALGET
Conﬁguration items
Type
Polled
Comment
This message is output by the receiver to return requested conﬁguration data (key and value pairs).
See Receiver conﬁguration for details.
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
4 + [0..n]
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
layer
-
-
The layer from which the conﬁguration item was
retrieved:
•
0 - RAM layer
•
1 - BBR
•
2 - Flash
•
7 - Default
2
U2
position
-
-
Number of conﬁguration items skipped in the result
set before constructing this message (mirrors the
equivalent ﬁeld in the request message)
Start of repeated group (N times)
4 + n
U1
cfgData
-
-
Conﬁguration data (key and value pairs)
End of repeated group (N times)
3.10.26 UBX-CFG-VALSET (0x06 0x8a)
3.10.26.1 Set conﬁguration item values
Message
UBX-CFG-VALSET
Set conﬁguration item values
Type
Set
Comment
Overview:
•
This message is used to set a conﬁguration by providing conﬁguration data (a list of key and value
pairs), which identify the conﬁguration items to change, and their new values.
•
This message is limited to containing a maximum of 64 key-value pairs.
•
This message can be used multiple times and every time the result will be applied immediately. To send
this message multiple times with the result being applied at the end, see version 1 of UBX-CFG-VALSET
that supports transactions.
•
See Receiver conﬁguration for details.
This message returns a UBX-ACK-NAK and no conﬁguration is applied:
•
if any key is unknown to the receiver FW
•
if the layer's bitﬁeld does not specify a layer to save a value to
•
if the requested conﬁguration is not valid. The validity of a conﬁguration is checked only if the message
requests to apply the conﬁguration to the RAM conﬁguration layer.
Notes:
•
If a key is sent multiple times within the same message, then the value eventually being applied is the
last sent.
UBX-22008968 - R01
 
3 UBX protocol
Page 96 of 305
C1-Public
```

---

## Page 97

```
u-blox F9 HPG 1.32 - Interface Description
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
0x8a
4 + [0..n]
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
X1
layers
-
-
The layers where the conﬁguration should be applied
bit 0   U:1
ram
-
-
Update conﬁguration in the RAM layer
bit 1   U:1
bbr
-
-
Update conﬁguration in the BBR layer
bit 2   U:1
flash
-
-
Update conﬁguration in the Flash layer
2
U1[2]
reserved0
-
-
Reserved
Start of repeated group (N times)
4 + n
U1
cfgData
-
-
Conﬁguration data (key and value pairs)
End of repeated group (N times)
3.10.26.2 Set conﬁguration item values (with transaction)
Message
UBX-CFG-VALSET
Set conﬁguration item values (with transaction)
Type
Set
Comment
Overview:
•
This message is used to set a conﬁguration by providing conﬁguration data (a list of key and value
pairs), which identify the conﬁguration items to change, and their new values.
•
This message is limited to containing a maximum of 64 key-value pairs.
•
This message can be used multiple times with the result being managed within a transaction. Within
a transaction there is no limit on the number key-value pairs; a transaction is eﬀectively limited to the
number of known keys.
•
See Receiver conﬁguration for details.
•
See version 0 of UBX-CFG-VALSET for simpliﬁed version of this message.
This message returns a UBX-ACK-NAK, cancels any started transaction, and no conﬁguration is applied:
•
if any key within a transaction is unknown to the receiver FW
•
if an invalid transaction state transition is requested
•
if the layer's bitﬁeld changes within a transaction
•
if the layer's bitﬁeld does not specify a layer to save a value to
This message returns a UBX-ACK-NAK, and no conﬁguration is applied:
•
if the requested conﬁguration is not valid. While in a transaction context, only the last message that
requests to apply the transaction returns a UBX-ACK-NAK. The validity of a conﬁguration is checked
only if the message requests to apply the conﬁguration to the RAM conﬁguration layer. This also applies
to a transactionless request.
Notes:
•
Any request for another UBX-CFG-message type (including UBX-CFG-VALDEL and UBX-CFG-VALGET)
will cancel any started transaction, and no conﬁguration is applied.
•
This message can be sent with no key/values to set for the purposes of managing the transaction state
transition.
•
If a key is sent multiple times within the same message or within the same transaction, then the value
eventually being applied is the last sent.
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
0x8a
4 + [0..n]
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
X1
layers
-
-
The layers where the conﬁguration should be applied
bit 0   U:1
ram
-
-
Update conﬁguration in the RAM layer
UBX-22008968 - R01
 
3 UBX protocol
Page 97 of 305
C1-Public
```

---
