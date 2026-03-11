# PDF抽出: NtripDocumentation.pdf

**総ページ数**: 22
**抽出ページ**: 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15

---

## Page 1

```
Networked Transport of RTCM via Internet Protocol  
(Ntrip) 
 
  
Version 1.0 
 
 
 
 
 
 
 
 
 
The material provided here is not the official RTCM documentation! This can only be ordered from  
http://www.rtcm.org/orderinfo.php
```

---

## Page 2

```
ii 
 
TABLE OF CONTENTS 
 
 
 
1 
Introduction ..........................................................................................................................1-1 
2 
SYSTEM CONCEPT ...........................................................................................................2-1 
3 
System Elements.................................................................................................................3-1 
3.1 
General ........................................................................................................................3-1 
3.2 
NtripSource ..................................................................................................................3-1 
3.3 
NtripServer...................................................................................................................3-1 
3.4 
NtripCaster...................................................................................................................3-1 
3.5 
NtripClient ....................................................................................................................3-2 
4 
Server Communication ........................................................................................................4-1 
5 
Client Communication..........................................................................................................5-1 
5.1 
General ........................................................................................................................5-1 
5.2 
Basic Authentication Scheme......................................................................................5-1 
5.3 
Digest Authentication Scheme ....................................................................................5-2 
5.4 
NMEA Request Messages...........................................................................................5-2 
6 
Source-Table .......................................................................................................................6-1 
7 
References...........................................................................................................................7-1 
APPENDIX A – Example Source-Table ......................................................................................A-1 
APPENDIX B – Format Specifications ........................................................................................B-1 
APPENDIX C – Example Client Source Code ............................................................................C-1
```

---

## Page 3

```
iii 
 
 
PREFACE 
 
Networked Transport of RTCM via Internet Protocol (Ntrip) is an application-level protocol that 
supports streaming Global Navigation Satellite System (GNSS) data over the Internet. Ntrip is a 
generic, stateless protocol based on the Hypertext Transfer Protocol HTTP/1.1. The HTTP 
objects are extended to GNSS data streams. 
 
Ntrip is designed to disseminate differential correction data or other kinds of GNSS streaming 
data to stationary or mobile users over the Internet, allowing simultaneous PC, Laptop, PDA, or 
receiver connections to a broadcasting host. Ntrip supports wireless Internet access through 
Mobile IP Networks like GSM, GPRS, EDGE, or UMTS. 
 
Ntrip consists of three system software components: NtripClients, NtripServers and NtripCasters. 
The NtripCaster is the actual HTTP server program, while NtripClient and NtripServer act as 
HTTP clients. 
 
Ntrip is meant to be an open non-proprietary protocol. Major characteristics of Ntrip’s 
dissemination technique are the following: 
 
• It is based on the popular HTTP streaming standard; it is comparatively easy to implement 
when limited client and server platform resources are available.  
• Its application is not limited to one particular plain or coded stream content; it has the ability 
to distribute any kind of GNSS data.  
• It has the potential to support mass usage; it can disseminate hundreds of streams 
simultaneously for up to a thousand users when applying modified Internet Radio 
broadcasting software.  
• Regarding security needs, stream providers and users are not necessarily in direct contact, 
and streams are usually not blocked by firewalls or proxy servers protecting Local Area 
Networks.  
• It enables streaming over any mobile IP network because it uses TCP/IP.  
This documentation describes Ntrip Version 1.0. Further Ntrip versions may be issued as the 
need arises.
```

---

## Page 4

```
` 
 
 
 
1-1 
 
 
1 INTRODUCTION 
 
Since the deliberate degradation of GPS signals, called Selective Availability, was turned off, 
stand-alone GPS receivers typically experience position errors of 10-15 meters, and these errors 
follow a random walk pattern.  While this accuracy is adequate for a wide variety of applications, 
there are other applications where meter-level, or even centimeter-level, accuracy is desired.  For 
such high-accuracy applications, a Differential Global Navigation Satellite System (DGNSS) can 
be employed successfully.  A DGNSS improves the accuracy of position, velocity, and time by 
providing measurements or correction data from one or more reference stations to mobile 
receivers.  The position of each reference station is accurately known, so its measurements act as 
a calibration for other nearby receivers, because the major error sources are common: satellite 
ephemeris and clock, tropospheric and ionospheric delay.  The DGNSS accuracy degrades as the 
distance between user and reference receivers increases, so for some applications, information 
from multiple reference stations is desirable.   
A DGNSS can utilize differential corrections for any GNSS (GPS, GLONASS, Galileo) to 
achieve meter-level accuracy, or it can use Real Time Kinematic (RTK) information to achieve 
decimeter or centimeter accuracy.  The DGNSS data needs to be refreshed every few seconds to 
keep up with atmospheric changes.   
RTCM-104 DGNSS standards are used worldwide. Many popular GNSS receivers accept 
RTCM-104 differential correction messages, and many special-purpose RTK receivers use the 
RTK messages.  The RTCM SC-104 standards define messages that contain reference station 
and system data.  These messages are typically broadcast over a transmission link and received 
by mobile users, which then apply the information contained in the messages to achieve high-
accuracy operation. The data can be transmitted over any number of communication channels, 
e.g., via radio transmission (LF, MF, HF, UHF), or via a mobile communication network, using 
different communication protocols.   
The introduction and deployment of wireless Internet capability has opened up the potential of 
disseminating these messages over the Internet.  This document defines a standard for an HTTP-
based protocol for the dissemination of DGNSS data (or other kinds of GNSS streaming data) to 
stationary or mobile receivers over the Internet.  It enables simultaneous PC, Laptop, PDA, or 
receiver connections to a broadcasting host, via IP Networks such as GSM, GPRS, EDGE, or 
UMTS. The protocol development is the result of a feasibility study on real-time streaming of 
differential GPS corrections as described in [1]. Because the primary application will be the 
dissemination of RTCM SC-104 messages, the system as a whole presents a transport protocol 
that will be referred to herein as the Ntrip Protocol (Networked Transport of RTCM via Internet 
Protocol). 
The basic data streaming is accomplished by TCP/IP protocol stack. Several demonstrations 
based on a plain Serial-to-TCP conversion of streaming data on the reference-side (server) and 
TCP-to-Serial re-conversion on the rover-side (client) have shown the suitability of the TCP/IP 
protocol for streaming data to mobile IP clients [2].
```

---

## Page 5

```
` 
 
 
 
2-1 
 
2 SYSTEM CONCEPT 
 
The Hypertext Transfer Protocol (HTTP) is designed as an application-level protocol for 
distributed collaborative hypermedia information systems, but it can also be used for linear 
streaming media. HTTP is primarily used for bulk traffic, where each object has a clearly defined 
beginning and end.  Although widely used for IP streaming applications, which include the 
RTCM application, it is not designed for such uses.  The basic unit of HTTP communication, 
consisting of a structured sequence of octets matching the syntax, is defined in the protocol and 
transmitted via a TCP/IP connection. Client and server must understand HTTP request messages, 
and must answer with adequate HTTP respond messages. 
Ntrip, which uses HTTP, is implemented in three programs: NtripClient, NtripServer and 
NtripCaster, where the NtripCaster is the real HTTP (splitter-) server. NtripClient and 
NtripServer act as HTTP client programs. 
In its message format and status code, the NtripClient-NtripCaster communication is based on 
HTTP 1.1 communication [3], where Ntrip uses only non-persistent connections. The 
NtripServer-NtripCaster communication deliberately deviates from HTTP by a new message 
format, which is called “SOURCE”, and a new status-code, which is called “ERROR - Bad 
Password”. 
A loss of the TCP connection between communicating system-components (NtripClient-
NtripCaster, NtripServer-NtripCaster) will be automatically recognized by the TCP-sockets. This 
effect can be used to trigger software events such as an automated reconnection. 
This Ntrip system (see Figure 1) consists of the following elements: 
• NtripSources, which generate data streams at a specific location, 
• NtripServers, which transfer the data streams from a source to the NtripCaster, 
• NtripCaster, the major system component, and 
• NtripClients, which finally access data streams of desired NtripSources on the NtripCaster. 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
Figure 1. 
Ntrip Streaming System 
 
NtripClient 1 
NtripClient N
NtripServer 1 
NtripServer M
NtripSource 1
NtripSource L
HTTP Streams 
HTTP Streams
NtripCaster 
Administration 
HTTP/Telnet
```

---

## Page 6

```
` 
 
 
 
2-2 
 
NtripServers define a source ID called "mountpoint" for every streamed NtripSource. Several 
NtripClients can access the data of desired NtripSources at the same time by requesting a source 
by its mountpoint on the NtripCaster. If implemented in the NtripCaster program, authorized 
personnel may remotely control the NtripCaster via a password-protected Telnet session, or 
receive status information via a password-protected HTTP session using an Internet Browser. 
 
An administrator running an NtripCaster is responsible for allowing new NtripServers to connect 
with new NtripSources. The administrator organizes all available NtripSources and defines all 
source IDs (mountpoints). 
 
NtripClients must be able to choose an NtripSource by its mountpoint on the NtripCaster. 
Therefore a source-table is introduced into, and maintained on, the NtripCaster. Each record of 
this source-table contains parameters describing attributes of a data stream, a network of data 
streams, or an NtripCaster. Stream attributes (identifier, coordinates, format, nav-system, 
mountpoint, etc.) are defined at the NtripServer side for each NtripSource. 
 
If an NtripClient sends an invalid or no mountpoint (no or not up-to-date source-table available 
for the client), the NtripCaster will upload an up-to-date source-table as an HTTP object instead 
of a GNSS data stream. Afterwards the NtripClient has the up-to-date source-table available and 
can connect to a GNSS data stream on the NtripCaster. 
 
The Ntrip system depends on direct communication between the responsible administrators of 
NtripCasters and NtripServers (e.g. via email). They must specify the parameters characterizing 
an NtripSource/mountpoint in the source-table. 
 
The RTCM SC-104 Committee is aware that the use of HTTP for Ntrip as described in this 
document differs from the HTTP standard.  The protocol described here utilizes a pragmatic 
approach that incorporates IP streaming, similar to techniques employed by internet radio and 
video conferencing.  The intended constituency for the RTCM information is the wireless mobile 
user community, which doesn’t use proxy servers.  While the protocol described here works with 
many proxy servers, their use should be avoided whenever possible.
```

---

## Page 7

```
` 
 
 
 
3-1 
 
3 SYSTEM ELEMENTS 
 
3.1 
GENERAL 
A DGNSS reference station in its simplest configuration consists of a GNSS receiver, located at 
a well-surveyed position. Because this stationary-operated GNSS receiver knows where the 
satellites are located in space at any point in time, as well as its own exact position, the receiver 
can compute theoretical distance and signal travel times between itself and each satellite. When 
these theoretical values are compared to real observations, differences represent errors in the 
received signals.  RTCM corrections are derived from these differences. Making these 
corrections available in real-time for mobile users is the major purpose of the Ntrip system 
elements, although Ntrip may be used as well for transporting other types of GNSS streaming 
data (such as RTK) over its system elements NtripServer, NtripCaster, and NtripClient.  
 
3.2 
NTRIPSOURCE 
The NtripSources provide continuous GNSS data (e.g. RTCM-104 corrections) as streaming 
data. A single source represents GNSS data referring to a specific location. Source description 
parameters as compiled in the source-table specify the format in use (e.g. RTCM 2.0, RTCM 2.1, 
Raw), the recognized navigation system (e.g. GPS, GPS+GLONASS), location coordinates and 
other information. 
Note: Every single NtripSource needs a unique mountpoint on an NtripCaster.  
 
3.3 
NTRIPSERVER 
The NtripServer is used to transfer GNSS data of an NtripSource to the NtripCaster. Before 
transmitting GNSS data to the NtripCaster using the TCP/IP connection, the NtripServer sends 
an assignment of the mountpoint.  
Server passwords and mountpoints must be defined by the administrator of the NtripCaster and 
handed over to the administrators of the participating NtripServers. An NtripServer in its 
simplest setup is a computer program running on a PC that sends correction data of an 
NtripSource (e.g. as received via the serial communication port from a GNSS receiver) to the 
NtripCaster. 
The Ntrip protocol may be used for the transport of RTCM data of a virtual reference station 
following the so-called VRS concept. Based on data from a number of reference stations, RTCM 
corrections are derived for a virtual point at the users approximate position. Data for this virtual 
reference station represent a single NtripSource that can be transmitted by an NtripServer.  
 
3.4 
NTRIPCASTER 
The NtripCaster is basically an HTTP server supporting a subset of HTTP request/response 
messages and adjusted to low-bandwidth streaming data (from 50 up to 500 Bytes/sec per 
stream). The NtripCaster accepts request-messages on a single port from either the NtripServer 
or the NtripClient. Depending on these messages, the NtripCaster decides whether there is 
streaming data to receive or to send.
```

---

## Page 8

```
` 
 
 
 
3-2 
 
An NtripServer could be a part of the NtripCaster program. If so, only the capability of receiving 
NtripClient messages has to be implemented into the combined NtripCaster/NtripServer. Built-in 
HTTP-based remote administration capability is an optional function.  
 
3.5 
NTRIPCLIENT 
An NtripClient will be accepted by and receive data from an NtripCaster, if the NtripClient sends 
the correct request message (TCP connection to the specified NtripCaster IP and listening Port). 
With respect to the message format and status code, the NtripClient-NtripCaster communication 
is fully compatible to HTTP 1.1, but Ntrip uses only non-persistent connections.
```

---

## Page 9

```
` 
 
 
 
4-1 
 
4 SERVER COMMUNICATION 
 
The NtripServer-NtripCaster communication extends HTTP by the additional message format 
“SOURCE” and the additional status-code “ERROR - Bad Password”. The password is not 
protected and therefore is based, as in the HTTP Basic Access Authentications scheme, on the 
assumption that the connection between the client and the server can be regarded as a trusted 
carrier. 
 
The NtripServer must connect to the NtripCaster using the IP and specified listening port of the 
NtripCaster. This means that the NtripCaster must be “up and running” before any source can 
connect. Before transmitting GNSS data to the NtripCaster using the TCP/IP connection, the 
NtripServer must send an Ntrip server message in order to get access and to specify a 
mountpoint. The Ntrip server message is shown in Figure 2. 
….. 
….. 
….. 
(See official RTCM documentation available from http://www.rtcm.org/orderinfo.php for further 
details.)
```

---

## Page 10

```
` 
 
 
 
5-1 
 
5 CLIENT COMMUNICATION 
 
5.1 
GENERAL 
A client’s request is designed as an HTTP message similar to the server message shown in 
Figure 2. Each client only needs to know the mountpoint of the desired data stream. The message 
for requesting a data stream is defined in Figure 3. The User-Agent request-header field must 
begin with the string NTRIP. 
….. 
….. 
….. 
(See official RTCM documentation available from http://www.rtcm.org/orderinfo.php for further 
details.) 
 
Via the information from the source-table, as maintained by the NtripCaster, NtripClients are 
enabled to choose the desired NtripSource/mountpoint from all available NtripSources/ 
mountpoints. An NtripClient can either store a source-table in memory (e.g. hard disk), or 
request a new source-table before requesting an NtripSource. The desired NtripSource can be 
chosen manually (e.g. based on identifier, nav-system, and format information) or by software 
(e.g. based on position, format, and nav-system information). 
 
Requesting unavailable mountpoints (out-of-date source-table) will automatically result in the 
caster replying with an up-to-date source-table. Therefore, mountpoints, as a synonym for a 
specific NtripSource, must be unique on an NtripCaster. Using a four-character-ID followed by 
an integer value (e.g. BUCU0 for Bucharest) as mountpoint parameter is recommended. 
 
An example for handling client messages in C programming language is given in Appendix C. 
This simple Linux/UNIX NtripClient program reads data from an NtripCaster and writes on 
standard output for further redirection of data to a file or COM-port. 
 
5.2 
BASIC AUTHENTICATION SCHEME 
For billing purposes, the NtripSources/mountpoints can be password-protected on an 
NtripCaster. In this protected case the HTTP communication differs from the non-protected case. 
Example: 
….. 
….. 
….. 
(See official RTCM documentation available from http://www.rtcm.org/orderinfo.php for further 
details.) 
 
The client password is different from the server password.  The client password is coded like the 
HTTP Basic Authentication Scheme [4] and allows the client access to protected content. The 
"basic" authentication scheme is based on the model that the user agent must authenticate with a 
user-ID and a password for each realm (here mountpoint). The realm value should be considered 
as an opaque string that can only be compared for equality with other realms on that server. The 
server will authorize the request only if it can validate the user-ID and password for the protected
```

---

## Page 11

```
` 
 
 
 
5-2 
 
space of the requested mountpoint. There are no optional authentication parameters. To receive 
authorization, the client sends the user-ID and password, separated by a single colon (":") 
character and within a “base64” [5] encoded string in the credentials. If the user agent wishes to 
send the user-ID "Aladdin" and password "open sesame", it would use the following header 
field: 
….. 
….. 
….. 
(See official RTCM documentation available from http://www.rtcm.org/orderinfo.php for further 
details.) 
 
5.3 
DIGEST AUTHENTICATION SCHEME 
For applications that require more security with user authentication, the Digest Access 
Authentication for HTTP specified in RFC 2617 [6] can be used as an alternative. 
 
Like Basic, Digest access authentication verifies that both communicating parties know a shared 
secret (a password). Unlike Basic, this verification can be done without sending the password in 
the clear, which is Basic's biggest weakness.  
 
Like Basic Access Authentication, the Digest scheme is based on a simple challenge-response 
paradigm. The Digest scheme challenges using a nonce value. A valid response contains a 
checksum (for Ntrip the MD5 [7] checksum is compulsory) of the username, the password, the 
given nonce value, the HTTP method, and the requested URI. Thus, the password is never sent in 
the clear. 
 
The Digest Access Authentication scheme is conceptually similar to the Basic scheme. The 
format of the modified WWW-Authenticate header line and the Authorization header line are 
specified in RFC 2617. [6] 
 
5.4 
NMEA REQUEST MESSAGES 
For some network-dependent applications it is necessary to send the position of the NtripClient 
to the NtripCaster. That position could be used by the NtripCaster to provide a data stream for a 
Virtual Reference Station (VRS) or to determine the best data stream to broadcast. Ntrip allows 
clients to send NMEA strings after the HTTP request for a data stream.  
 
If the <nmea> parameter is set to “1” (see Source-table section), the NtripCaster must receive at 
least one NMEA GGA string to prepare the data and start sending. The NtripClient is allowed to 
send more than one NMEA GGA string or NMEA strings of other type than GGA at any time. 
 
The following is an example of a request to a source-table named “vrs_bayern” to get a data 
stream generated for a virtual reference station with the coordinates of the NMEA string: 
….. 
….. 
….. 
(See official RTCM documentation available from http://www.rtcm.org/orderinfo.php for further 
details.)
```

---

## Page 12

```
` 
 
 
 
5-3 
 
Note that sending an NMEA string containing latitude and longitude information allows an 
NtripCaster to track the NtripClient’s position. The operator of an NtripCaster may wish to 
consider informing clients about this potential privacy problem.
```

---

## Page 13

```
` 
 
 
 
6-1 
 
6 SOURCE-TABLE 
The NtripCaster maintains a source-table containing information on available NtripSources, 
networks of NtripSources, and NtripCasters, to be sent to an NtripClient on request. Note that to 
request a source-table from the NtripCaster, the NtripClient uses the client message (see Fig. 3) 
while leaving out the mountpoint parameter.  
Source-table records are dedicated to one of the following: 
a) Data STReams (record type STR), 
b) CASters (record type CAS), or 
c) NETworks of data streams (record type NET). 
 
This structure is expandable by introducing new record types when necessary. Older NtripClient 
versions might ignore newly introduced record types. All NtripClients must be able to decode 
record type STR. Decoding types CAS and NET is an optional feature. 
 
All data fields in the source-table records are separated using the semicolon character “;” as a 
field delimiter. In case the semicolon character becomes part of the content of a data field, it 
must be quoted: “;”. The number of data fields in the source-table records is not fixed. 
Introducing additional data fields is allowed. The last data field always contains “Miscellaneous” 
information. 
 
The source-table is initiated by the HTTP/1.1 header fields 
 
Server: <NtripCasterIdentifier>/<NtripVersion><CR><LF> 
Content-Type: text/plain<CR><LF> 
Content-Length: <Content-Length><CR><LF> 
<CR><LF> 
 
followed by the actual source-table records. The “Server:” record contains 
 
• Before the slash: an NtripCaster identifier to be defined by the NtripCaster operator (see 
parameter #4 in Table 2) 
• After the slash: an Ntrip version number (e.g. NtripV1.0 or 1.0) in order to allow an 
NtripClient to understand which version of Ntrip is supported by a particular NtripCaster. 
 
The content-length indicates the size of the source-table records (decimal number of octets, e.g. 
“Content-Length: 243”). 
 
The end of the source-table is notified by the string: ENDSOURCETABLE 
 
Table 1: Format and Contents of Source-Table Records Describing a Data Stream 
 
# 
Record Parameter Meaning 
Format 
Examples 
1 
<type> = STR 
The following parameters of this 
record describe a data stream 
3 Characters 
STR (the only 
acceptable string) 
2 
<mountpoint> 
Caster mountpoint 
Characters, 100 max. 
LEIJ0 
LEIJ1 
WTZJ0
```

---

## Page 14

```
` 
 
 
 
6-2 
 
Table 1: Format and Contents of Source-Table Records Describing a Data Stream 
 
# 
Record Parameter Meaning 
Format 
Examples 
3 
<identifier> 
Source identifier, e.g. name of city 
next to source location  
Characters, undefined 
length 
Frankfurt 
4 
<format> 
Data format 
RTCM, RAW, etc.  
Characters, undefined 
length 
RTCM 2 
RTCM 2.0 
RTCM 2.1 
RTCM 2.3 
RTCM 3 
RTCM SAPOS 
CMR 
CMR+ 
RAW 
RTCA 
5 
<format-details> 
E.g. RTCM message types or 
RAW data format etc., update 
periods in parenthesis in seconds 
Characters, undefined 
length 
1(1), 2(1), 3(30) 
MBEN(1) 
LB2 
6 
<carrier> 
Data stream contains carrier phase 
information 
0  =  No (e.g. for DGPS) 
1  =  Yes, L1 (e.g. for RTK) 
2  =  Yes, L1&L2 (e.g. for RTK) 
Integer 
0 
1 
2 
7 
<nav-system> 
 
Navigation system(s)  
Characters, undefined 
length 
GPS 
GPS+GLONASS 
GPS+EGNOS 
8 
<network> 
Network 
Characters, undefined 
length 
EUREF 
IGS 
IGLOS 
SAPOS 
GREF 
Misc 
9 
<country> 
Three character country code in ISO 
3166 
3 Characters 
DEU 
ITA 
ESP 
10 
<latitude> 
Position, latitude, north 
(approximate position in case of 
nmea = 1) 
Floating point number, 
two digits after decimal 
point 
40.12 
-12.14 
 
11 
<longitude> 
Position, longitude, east 
(approximate position in case of 
nmea = 1) 
Floating point number, 
two digits after decimal 
point 
10.12 
357.85 
 
12 
<nmea> 
Necessity for Client to send NMEA 
message with approximate position 
to Caster 
0 = Client must not send NMEA 
      message with approximate 
      position to Caster  
1 = Client must send NMEA 
      GGA message with 
      approximate position to 
      Caster 
Integer 
0 
1 
13 
<solution> 
Stream generated from single 
reference station or from networked 
reference stations 
0  =  Single base 
1  =  Network 
Integer 
0 
1
```

---

## Page 15

```
` 
 
 
 
6-3 
 
Table 1: Format and Contents of Source-Table Records Describing a Data Stream 
 
# 
Record Parameter Meaning 
Format 
Examples 
14 
<generator> 
Hard- or software generating data 
stream 
Characters, undefined 
length 
JPS Legacy E 
GPSNet 
15 
<compr-encryp> 
Compression/Encryption algorithm 
applied 
Characters, undefined 
length 
none 
 
16 
<authentication> 
Access protection for this particular 
data stream 
N  =  None 
B  =  Basic 
D  =  Digest 
1 Character 
N 
B 
D 
17 
<fee> 
User fee for receiving this particular 
data stream 
N  =  No user fee 
Y  =  Usage is charged 
1 Character 
N 
Y 
18 
<bitrate> 
Bit rate of data stream, bits per 
second  
Integer 
500 
5000 
… 
… 
 
 
 
… 
… 
 
 
 
n 
<misc> 
Miscellaneous information, last data 
field in record 
Characters, undefined 
length 
none 
Demo 
 
 
Table 2: Format and Contents of Source-Table Records Describing a Caster 
 
# 
Source-table 
Element 
Meaning 
Format 
Examples 
1 
<type> = CAS 
The following parameters of this 
record describe a Caster 
3 Characters 
CAS (the only 
acceptable string) 
2 
<host> 
Caster Internet host domain name or IP 
address 
Characters, 128 max. 
141.74.243.11 
euref-ip.ifag.de 
 
3 
<port> 
Port number  
Integer 
80 
2101 
4 
<identifier> 
Caster identifier, e.g. name of provider Characters, undefined 
length 
NTRIP Caster/0.5.3 
Trimble-iGate 
5 
<operator> 
Name of institution / agency / company 
operating the Caster 
Characters, undefined 
length 
BKG 
Geo++ 
6 
<nmea> 
Capability of Caster to receive NMEA 
message with approximate position 
from Client 
0 = Caster is not able to handle 
      incoming NMEA message 
      with approximate position 
      from Client 
1 = Caster is able to handle 
      incoming NMEA GGA 
      message with approximate 
      position from Client 
Integer 
0 
1 
7 
<country> 
Three character country code in ISO 
3166 
3 Characters 
DEU 
ITA 
ESP
```

---
