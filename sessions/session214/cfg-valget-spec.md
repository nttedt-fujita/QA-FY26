# PDF抽出: u-blox-F9-HPG-1.32_InterfaceDescription_UBX-22008968.pdf

**総ページ数**: 305
**抽出ページ**: 96, 97, 98

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

## Page 98

```
u-blox F9 HPG 1.32 - Interface Description
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
U1
transaction
-
-
Transaction action to be applied
bits 1…0   U:2
action
-
-
Transaction action to be applied:
•
0 = Transactionless UBX-CFG-VALSET: In the
next UBX-CFG-VALSET, it can be either 0 or 1.
If a transaction has not yet been started, the
incoming conﬁguration is applied (if valid). If a
transaction has already been started, cancels
any started transaction and the incoming
conﬁguration is applied (if valid).
•
1 = (Re)Start set transaction: In the next
UBX-CFG-VALSET, it can be either 0, 1, 2 or
3. If a transaction has not yet been started, a
transaction will be started. If a transaction has
already been started, restarts the transaction,
eﬀectively removing all previous non-applied UBX-
CFG-VALSET messages.
•
2 = Set transaction ongoing: In the next UBX-
CFG-VALSET, it can be either 0, 1, 2 or 3.
•
3 = Apply and end a set transaction: In the next
UBX-CFG-VALSET, it can be either 0 or 1.
3
U1
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
3.11 UBX-INF (0x04)
Messages in the UBX-INF class are used to output strings from the ﬁrmware or application code. All
messages have an associated type to indicate the nature or priority of the message.
3.11.1 UBX-INF-DEBUG (0x04 0x04)
3.11.1.1 ASCII output with debug contents
Message
UBX-INF-DEBUG
ASCII output with debug contents
Type
Output
Comment
This message has a variable length payload, representing an ASCII string.
Header
Class
ID
Length (Bytes)
Payload
Checksum
Message
structure
0xb5 0x62
0x04
0x04
[0..n]
see below
CK_A CK_B
Payload description:
Byte offset
Type
Name
Scale
Unit
Description
Start of repeated group (N times)
0 + n
CH
str
-
-
ASCII Character
End of repeated group (N times)
3.11.2 UBX-INF-ERROR (0x04 0x00)
UBX-22008968 - R01
 
3 UBX protocol
Page 98 of 305
C1-Public
```

---
