# PDF抽出: u-blox-F9-HPG-1.32_InterfaceDescription_UBX-22008968.pdf

**総ページ数**: 305
**抽出ページ**: 223, 224, 225, 226, 227

---

## Page 223

```
u-blox F9 HPG 1.32 - Interface Description
6 Conﬁguration interface
This chapter describes the receiver conﬁguration interface.
6.1 Conﬁguration database
The conﬁguration database in the receiver's RAM holds the current conﬁguration, which is used
by the receiver at run-time. It is constructed on startup of the receiver from several sources of
conﬁguration. These sources are called Configuration Layers. The current conﬁguration is called
the RAM Layer. Any conﬁguration in any layer is organized as Configuration Items, where each
Conﬁguration Item is referenced to by a unique Configuration Key ID and holds a single Configuration
Value.
6.2 Conﬁguration items
The following ﬁgure shows the structure of a Configuration Item, which consists of a (Configuration)
Key ID and its (Configuration) Value:
size 
0x03
size 0x04
size 0x05
Conﬁguration Value: 1 byte (1bit), 2 bytes, 4 bytes or 8 bytes
1 byte
2 bytes
4 bytes
8 bytes
Conﬁguration Key ID (4 bytes = 32 bits)
size (3 bits) group (8 bits)
ID within group (12 bits)
reserved (1 + 4 + 4 = 9 bits)
31
0
8
16
24
size 0x02
- size 0x01
A Conﬁguration Key ID is a 32-bit integer value, which is split into the following parts:
•
Bit 31: Currently unused. Reserved for future use.
•
Bits 30…28: Three bits that indicate the storage size of a Conﬁguration Value (range
0x01-0x05, see below)
•
Bits 27…24: Currently unused. Reserved for future use.
•
Bits 23…16: Eight bits that deﬁne a unique group ID (range 0x01-0xfe)
•
Bits 15…12: Currently unused. Reserved for future use.
•
Bits 11…0: Twelve bits that deﬁne a unique item ID within a group (range 0x001-0xﬀe)
The entire 32-bit value is the unique Key ID, which uniquely identiﬁes a particular item. The numeric
representation of the Key ID uses the lower-case hexadecimal format, such as 0x20c400a1. An
easier, more readable text representation uses the form CFG-GROUP-ITEM. This is also referred to
as the (Configuration) Key Name.
Supported storage size identiﬁers (bits 30…28 of the Key ID) are:
•
0x01: one bit (the actual storage used is one byte, but only the least signiﬁcant bit is used)
•
0x02: one byte
•
0x03: two bytes
•
0x04: four bytes
UBX-22008968 - R01
 
6 Conﬁguration interface
Page 223 of 305
C1-Public
```

---

## Page 224

```
u-blox F9 HPG 1.32 - Interface Description
•
0x05: eight bytes
Each Conﬁguration Item is of a certain type, which deﬁnes the interpretation of the raw binary data
(see also UBX data types):
•
U1, U2, U4, U8: unsigned little-endian integers of 8-, 16-, 32- and 64-bit widths
•
I1, I2, I4, I8: signed little-endian, two's complement integers of 8-, 16-, 32- and 64-bit widths
•
R4, R8: IEEE 754 single (32-bit) and double (64-bit) precision ﬂoats
•
E1, E2, E4: unsigned little-endian enumeration of 8-, 16-, and 32-bit widths
•
X1, X2, X4, X8: unsigned little-endian integers of 8-, 16-, 32- and 64-bit widths for bitﬁelds and
other binary data, such as strings
•
L: single-bit boolean (true = 1, false = 0), stored as U1
6.3 Conﬁguration layers
Several Configuration Layers exist. They are separate sources of Conﬁguration Items. Some of the
layers are read-only and others are modiﬁable. Layers are organized in terms of priority. Values in
a high-priority layer will replace values stored in low-priority layer. On startup of the receiver all
conﬁguration layers are read and the items within each layer are stacked up in order to create the
Current Configuration, which is used by the receiver at run-time.
The following conﬁguration layers are available (in order of priority, highest priority ﬁrst):
•
RAM: This layer contains items stored in volatile RAM. This is the Current Conﬁguration. The
value of any item can be set by the user at run-time (see UBX protocol interface) and it will
become eﬀective immediately.
•
BBR: This layer contains items stored in the battery-backed RAM. The contents in this layer are
preserved as long as a battery backup supply is provided during oﬀ periods. The value of any
item can be set by the user at run-time (see UBX protocol interface) and it will become eﬀective
upon a restart of the receiver.
•
Flash: This layer contains items stored permanently in the external ﬂash memory. This layer is
only available if there is a usable external ﬂash memory. The value of any item can be set by the
user at run-time (see UBX protocol interface) and it will become eﬀective upon a restart of the
receiver.
•
Default: This layer contains all items known to the running receiver software and their hard-
coded default values. Data in this layer is not writable.
The stacking of the conﬁguration items from the diﬀerent layers (sources) in order to construct the
Current Conﬁguration in the RAM Layer is depicted in the following ﬁgure. For each deﬁned item, i.e.
for each item in the Default Layer, the receiver software goes through the layers above and stacks all
the found items on top. Some items may not be present in every layer. The result is the RAM Layer
ﬁlled with all conﬁguration items given Conﬁguration Values coming from the highest priority layer
the corresponding item was present. In the example ﬁgure below bold text indicates the source of
the value in the Current Conﬁguration (the RAM Layer). Empty boxes mean that the layer can hold
the item but that it is not currently stored there. Boxes with text mean that an item is currently
stored in the layer.
UBX-22008968 - R01
 
6 Conﬁguration interface
Page 224 of 305
C1-Public
```

---

## Page 225

```
u-blox F9 HPG 1.32 - Interface Description
In the example ﬁgure above several items (e.g. the ﬁrst item) are only set in the Default Layer and
hence the default value ends up in Current Conﬁguration in the RAM Layer. The third item is present
in the Default, Flash and BBR Layers. The value from the BBR Layer has the highest priority and
therefore it ends up in the RAM Layer. On the other hand, the default value of the sixth item is
changed by the value in the Flash Layer. The value of the last item is changed in the RAM Layer only,
i.e. upon startup the value in the RAM Layer was the value from the Default Layer, but the user has
changed the value in the RAM Layer at run-time.
6.4 Conﬁguration interface access
The following sections describe the existing interfaces to access the Conﬁguration Database.
6.4.1 UBX protocol interface
The following UBX protocol messages are available to access the Conﬁguration Database:
•
UBX-CFG-VALGET to read conﬁguration items from the database
•
UBX-CFG-VALSET to set conﬁguration items in the database
•
UBX-CFG-VALDEL to delete conﬁguration items from the database
6.5 Conﬁguration data
Conﬁguration data is the binary representation of a list of Key ID and Value pairs. It is formed by
concatenating keys (U4 values) and values (variable type) without any padding. This format is used
in the UBX-CFG-VALSET and UBX-CFG-VALGET messages.
The ﬁgure below shows an example. The four Items (Key ID - Value pairs) on the left use the four
fundamental storage sizes: one byte (L, U1, I1, E1 and X1 types), 2 bytes (U2, I2, E2 and X2 types),
four byte (U4, I4, E4, X4 and R4 types) and eight bytes (U8, I8, X8 and R8 types). When concatenated
(right) the Key IDs and Values are not aligned and there is no padding.
UBX-22008968 - R01
 
6 Conﬁguration interface
Page 225 of 305
C1-Public
```

---

## Page 226

```
u-blox F9 HPG 1.32 - Interface Description
Note that this is an arbitrary example and any number of items of any value storage size can be
concatenated the same way.
6.6 Conﬁguration transactions
The conﬁguration interface supports two mechanisms of conﬁguration: the ﬁrst is a transactionless
mechanism where sent conﬁguration changes are applied immediately to the conﬁguration layer(s)
requested. The second mechanism is a conﬁguration transaction.
A transaction oﬀers a way of queuing multiple conﬁguration changes. It is particularly useful where
diﬀerent conﬁguration keys depend on each other in such a way that sending one before the other
can cause the conﬁguration to be rejected. The queued conﬁguration change requests are stored
then checked collectively before being applied to the receiver.
A transaction can have the following states described in the ﬁgure below.
When starting a transaction, the user must specify the layer(s) the changes will be applied to.
This list of conﬁguration layer(s) must be observed throughout the transaction states. Modifying
the conﬁguration layer(s) mid-transaction will cause the transaction to be aborted and no queued
changes will be applied.
In the start transaction state, the receiver will lock the conﬁguration database so that changes from
another entity or message cannot be applied. It is possible to send a conﬁguration key-value pairs
with the start transaction state. These will be queued waiting to be applied.
In the ongoing state, a conﬁguration key and value must be sent. The receiver will abort the
transaction and not apply any changes if this condition is violated. Key-value pairs sent in the
ongoing state will be queued waiting to be applied.
In the apply state, the queued changes will be collectively checked and applied to the requested
conﬁguration layer(s). Note that any additional key-value pairs sent within the apply state will be
ignored.
Note that a transaction can only come from a single source, a UBX-CFG-VALSET message or a UBX-
CFG-VALDEL message. This means that in any given transaction it is not possible to mix a delete
UBX-22008968 - R01
 
6 Conﬁguration interface
Page 226 of 305
C1-Public
```

---

## Page 227

```
u-blox F9 HPG 1.32 - Interface Description
and a save request. Starting a transaction from a diﬀerent source will abort the current transaction
and no queued changes would be applied.
Refer to UBX-CFG-VALSET and UBX-CFG-VALDEL messages for a detailed description of how to
set up a conﬁguration transaction, its limitations and conditions that would cause the transaction
to be rejected.
6.7 Conﬁguration reset behavior
The RAM layer is always rebuilt from the layers below when the chip's processor comes out from
reset. When using UBX-CFG-RST the processor goes through a reset cycle with these reset types
(resetMode ﬁeld):
•
0x00 hardware reset (watchdog) immediately
•
0x01 controlled software reset
•
0x04 hardware reset (watchdog) after shutdown
See section Forcing a receiver reset in the integration manual.
6.8 Conﬁguration overview
Group
Description
CFG-BDS
BeiDou system conﬁguration
CFG-GEOFENCE
Geofencing conﬁguration
CFG-HW
Hardware conﬁguration
CFG-I2C
Conﬁguration of the I2C interface
CFG-I2CINPROT
Input protocol conﬁguration of the I2C interface
CFG-I2COUTPROT
Output protocol conﬁguration of the I2C interface
CFG-INFMSG
Information message conﬁguration
CFG-ITFM
Jamming and interference monitor conﬁguration
CFG-LOGFILTER
Data logger conﬁguration
CFG-MOT
Motion detector conﬁguration
CFG-MSGOUT
Message output conﬁguration
CFG-NAV2
Secondary output conﬁguration
CFG-NAVHPG
High precision navigation conﬁguration
CFG-NAVSPG
Standard precision navigation conﬁguration
CFG-NMEA
NMEA protocol conﬁguration
CFG-ODO
Odometer and low-speed course over ground ﬁlter conﬁguration
CFG-QZSS
QZSS system conﬁguration
CFG-RATE
Navigation and measurement rate conﬁguration
CFG-RINV
Remote inventory
CFG-RTCM
RTCM protocol conﬁguration
CFG-SBAS
SBAS conﬁguration
CFG-SEC
Security conﬁguration
CFG-SIGNAL
Satellite systems (GNSS) signal conﬁguration
CFG-SPARTN
SPARTN conﬁguration
CFG-SPI
Conﬁguration of the SPI interface
UBX-22008968 - R01
 
6 Conﬁguration interface
Page 227 of 305
C1-Public
```

---
