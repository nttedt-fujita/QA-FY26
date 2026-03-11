# PDF抽出: ZED-F9P_IntegrationManual_UBX-18010802.pdf

**総ページ数**: 129
**抽出ページ**: 16, 17, 18, 19, 20, 21, 22, 23, 24

---

## Page 16

```
ZED-F9P - Integration manual
Conﬁguration item
Description
Comments
CFG-HW-ANT_CFG_RECOVER
Enable auto recovery in the event of a
short circuit
To use this feature, enable short
circuit detection and CFG-HW-
ANT_CFG_PWRDOWN.
CFG-HW-ANT_SUP_SWITCH_PIN
PIO-Pin (PIO number) used for switching
antenna supply
It is recommended that you use the
default PIO and assigned pin
CFG-HW-ANT_SUP_SHORT_PIN
PIO-Pin (PIO number) used for detecting
a short circuit in the antenna supply
It is recommended that you use the
default PIO and assigned pin
Table 4: Antenna supervisor conﬁguration
It is possible to obtain the status of the antenna supervisor through the UBX-MON-RF message.
Moreover, any changes in the status of the antenna supervisor are reported to the host interface in
the form of notice messages. See the applicable Interface description [2] for antStatus and antPower
ﬁeld description.
Status
Description
OFF
Antenna is oﬀ
ON
Antenna is on
DONTKNOW
Antenna power status is not known
Table 5: Antenna power status
3.1.4.5 NMEA high precision mode
ZED-F9P supports NMEA high precision mode. This mode increases the number of signiﬁcant digits
of the position output; latitude and longitude will have seven digits after the decimal point, and
altitude will have three digits after the decimal point. By default it is not enabled since it violates
the NMEA standard. NMEA high precision mode cannot be used while in NMEA compatibility mode
or when NMEA output is limited to 82 characters. See conﬁguration item CFG-NMEA-HIGHPREC in
the applicable Interface description [2] for more details.
NMEA high precision mode is disabled by default meaning that the default NMEA output will be
insuﬃcient to report a high precision position.
3.1.5 RTK conﬁguration
RTK technology introduces the concept of a base6 and a rover. In such a setup, the base sends
corrections (complying with the RTCM 3.3 protocol) to the rover via a communication link. This
enables the rover to compute its position relative to the base with high accuracy.
When operating as a rover, the ZED-F9P can receive RTCM 3.3 corrections from another ZED-F9P
operating as a base, or via NTRIP from a VRS service provider operating a network of base receivers.
In this mode, the receiver coordinates will be expressed in the datum used by the RTCM correction
provider. For more information refer to the Reference frames section in the Appendix.
After describing the RTCM protocol and corresponding supported message types, this section
describes how to conﬁgure the ZED-F9P high precision receiver as a base or rover receiver. This
includes both the static base use case and the moving base use case.
See the ZED-F9P Moving Base application note [4] for more information on designing in, conﬁguring
and using a moving base setup.
6 The terms base, base station, reference and reference station can be used interchangeably
UBX-18010802 - R16
 
3 Receiver functionality
Page 16 of 129
C1-Public
```

---

## Page 17

```
ZED-F9P - Integration manual
3.1.5.1 RTCM corrections
RTCM is a standard-based binary protocol for the communication of GNSS correction information.
The ZED-F9P high precision receiver supports RTCM as speciﬁed by RTCM 10403.4, Diﬀerential
GNSS (Global Navigation Satellite Systems) Services – Version 4 (December 1, 2023).
The RTCM speciﬁcation is currently at version 3.4 and RTCM version 2 messages are not supported
by this standard.
To modify the RTCM input/output settings, see the conﬁguration section in the applicable Interface
description [2].
Users should be aware of the datum used by the correction source. The rover position will provide
coordinates in the correction source reference frame. This may need to be taken into account
when using the RTK rover position. See the Reference frames section in the Appendix for more
information.
3.1.5.2 List of supported RTCM input messages
Message type
Description
RTCM 1001
L1-only GPS RTK observables
Unsupported in ﬁrmware version HPG L1L5 1.40.
RTCM 1002
Extended L1-only GPS RTK observables
Unsupported in ﬁrmware version HPG L1L5 1.40.
RTCM 1003
L1/L2 GPS RTK observables
Unsupported in ﬁrmware version HPG L1L5 1.40.
RTCM 1004
Extended L1/L2 GPS RTK observables
Unsupported in ﬁrmware version HPG L1L5 1.40.
RTCM 1005
Stationary RTK reference station ARP
RTCM 1006
Stationary RTK reference station ARP with antenna height
RTCM 1007
Antenna descriptor
RTCM 1009
L1-only GLONASS RTK observables
Unsupported in ﬁrmware version HPG L1L5 1.40.
RTCM 1010
Extended L1-only GLONASS RTK observables
Unsupported in ﬁrmware version HPG L1L5 1.40.
RTCM 1011
L1/L2 GLONASS RTK observables
Unsupported in ﬁrmware version HPG L1L5 1.40.
RTCM 1012
Extended L1/L2 GLONASS RTK observables
Unsupported in ﬁrmware version HPG L1L5 1.40.
RTCM 1033
Receiver and Antenna Description
RTCM 1074
GPS MSM4
RTCM 1075
GPS MSM5
RTCM 1077
GPS MSM7
RTCM 1084
GLONASS MSM4
RTCM 1085
GLONASS MSM5
RTCM 1087
GLONASS MSM7
RTCM 1094
Galileo MSM4
RTCM 1095
Galileo MSM5
RTCM 1097
Galileo MSM7
RTCM 1124
BeiDou MSM4
UBX-18010802 - R16
 
3 Receiver functionality
Page 17 of 129
C1-Public
```

---

## Page 18

```
ZED-F9P - Integration manual
Message type
Description
RTCM 1125
BeiDou MSM5
RTCM 1127
BeiDou MSM7
RTCM 1230
GLONASS code-phase biases
RTCM 4072.0
Reference station PVT (u-blox proprietary RTCM Message)
Unsupported in ﬁrmware version HPG L1L5 1.40.
RTCM 4072.1
Additional reference station information (u-blox proprietary RTCM Message)
Only valid with ﬁrmware version HPG 1.12 and necessary for moving base operation.
Table 6: ZED-F9P supported input RTCM version 3.4 messages
3.1.5.3 List of supported RTCM output messages
Message type
Description
RTCM 1005
Stationary RTK reference station ARP
RTCM 1074
GPS MSM4
RTCM 1077
GPS MSM7
RTCM 1084
GLONASS MSM4
RTCM 1087
GLONASS MSM7
RTCM 1094
Galileo MSM4
RTCM 1097
Galileo MSM7
RTCM 1124
BeiDou MSM4
RTCM 1127
BeiDou MSM7
RTCM 1230
GLONASS code-phase biases
RTCM 4072.0
Reference station PVT (u-blox proprietary RTCM Message)
Unsupported in ﬁrmware version HPG L1L5 1.40.
RTCM 4072.1
Additional reference station information (u-blox proprietary RTCM Message)
Only valid with ﬁrmware version HPG 1.12 and necessary for moving base operation.
Table 7: ZED-F9P supported output RTCM version 3.4 messages
3.1.5.4  Rover operation
In its default conﬁguration, the ZED-F9P will attempt to provide the best positioning accuracy
depending on the received correction data. It will enter RTK ﬂoat mode shortly after it starts
receiving an input stream of RTCM correction messages. Once the rover has resolved the carrier
phase ambiguities, it will enter RTK ﬁxed mode. When in this mode, the relative position accuracy
between base and rover can be expected to be correct to the cm-level. The time period between RTK
ﬂoat and RTK ﬁxed operation is referred to as the convergence time. Note that the convergence time
is aﬀected by the baseline length as well as by multipath and satellite visibility at both rover and
base station.
The ZED-F9P should receive RTCM corrections matching its GNSS signal conﬁguration to function
optimally. The rover requires both base station observation (MSM4 or MSM7 messages) and
position message (RTCM 1005 or RTCM 1006) in order to attempt ambiguity ﬁxes. The rover will
attempt to provide RTK ﬁxed operation when suﬃcient number of ambiguities are resolved. If phase
lock on suﬃcient number of signals cannot be maintained, the rover will drop back to RTK ﬂoat
mode. The rover will continue to attempt to resolve carrier ambiguities and revert to RTK ﬁxed mode
once the minimum number of signals has been restored.
The RTK mode that an RTK rover operates in can be conﬁgured through the CFG-NAVHPG-
DGNSSMODE conﬁguration item. The following two RTK modes are available:
•
RTK ﬁxed: The rover will attempt to ﬁx ambiguities whenever possible.
UBX-18010802 - R16
 
3 Receiver functionality
Page 18 of 129
C1-Public
```

---

## Page 19

```
ZED-F9P - Integration manual
•
RTK ﬂoat: The rover will estimate the ambiguities as ﬂoat but will make no attempts at ﬁxing
them.
The rover will stop using RTCM corrections that are older than 60s (default value) and will
drop back to a 3D or 3D/DGNSS mode. This is meant to prevent the computation of grossly
misleading diﬀerential solutions. If desired, this value can be changed through the CFG-NAVSPG-
CONSTR_DGNSSTO conﬁguration item.
The received correction messages stream should comply with the following:
•
The reference station ID in the reference station message (RTCM 1005 or RTCM 1006) must
match that used in the MSM observation messages. Otherwise, the rover cannot compute its
position.
•
In order to ﬁx GLONASS ambiguities the correction stream must contain RTCM message
1230 or 1033. Otherwise, the carrier ambiguities will be estimated as ﬂoat even when set to
operate in RTK ﬁxed mode. This will result in degraded performance, especially in challenging
environments.
CFG-RTCM-DF003_IN7 can be used to conﬁgure the desired reference station ID and CFG-RTCM-
DF003_IN_FILTER7 can be used to conﬁgure how strict the ﬁltering should be (RELAXED is the
recommended setting).
3.1.5.4.1  Conservative ambiguity ﬁx mode
The conservative ambiguity ﬁx mode oﬀers a tradeoﬀ for longer time to ﬁrst ﬁx or more ﬂoat
solutions in exchange for near absolute certainty when RTK is achieved. This mode is useful in
challenging environment, for instance surveying in urban environment, or when an extra degree of
certainty during ambiguity resolution is required.
To conﬁgure the conservative ambiguity ﬁx mode, set the value of CFG-NAVHPG-DGNSSMODE to
RTK_CAR.
This feature is available as of ﬁrmware version HPG 1.50.
3.1.5.4.2  Message output in RTK mode
When operating in RTK rover mode users should take note of the modiﬁed information within the
following NMEA and UBX messages:
•
NMEA-GGA: The quality ﬁeld will be 4 for RTK ﬁxed and 5 for RTK ﬂoat (see NMEA position ﬁx
ﬂags in interface description). The age of diﬀerential corrections and base station ID will be set.
•
NMEA-GLL, NMEA-VTG: The posMode indicator will be D for RTK ﬂoat and RTK ﬁxed (see
NMEA position ﬁx ﬂags in interface description).
•
NMEA-RMC, NMEA-GNS: The posMode indicator will be F for RTK ﬂoat and R for RTK ﬁxed (see
NMEA position ﬁx ﬂags in interface description).
•
UBX-NAV-PVT: The carrSoln ﬂag will be set to 1 for RTK ﬂoat and 2 for RTK ﬁxed. The age of
diﬀerential corrections will be reported.8
•
UBX-NAV-RELPOSNED
•
The diﬀSoln and relPosValid ﬂags will be set
•
The carrSoln ﬂag will be set to 1 for RTK ﬂoat and 2 for RTK ﬁxed
•
UBX-NAV-SAT
•
The diﬀCorr ﬂag will be set for satellites with valid RTCM data
•
The rtcmCorrUsed, prCorrUsed, and crCorrUsed ﬂags will be set for satellites for which the
RTCM corrections have been applied
7 CFG-RTCM-DF003_* items are supported from ﬁrmware version HPG 1.13 onwards
8 Age of diﬀerential corrections reported from ﬁrmware version HPG 1.30 onwards
UBX-18010802 - R16
 
3 Receiver functionality
Page 19 of 129
C1-Public
```

---

## Page 20

```
ZED-F9P - Integration manual
•
UBX-NAV-SIG
•
For signals to which the RTCM corrections have been applied, the correction source will be
set to RTCM3 OSR and the crUsed, prCorrUsed, and crCorrUsed ﬂags will be set
•
UBX-NAV-STATUS
•
The diﬀSoln ﬂag and the diﬀCorr ﬂag will be set
•
The carrSoln ﬂag will be set to 1 for RTK ﬂoat and 2 for RTK ﬁxed
•
If the baseline exceeds 100 km9, a UBX-INF-WARNING will be output, e.g. "WARNING: DGNSS
long baseline: 102.7 km"
An illustrated procedure for conﬁguring a rover using u-center is shown in the Appendix C.2.
3.1.5.5 Stationary base operation
The ZED-F9P high precision receiver default operation begins without producing any RTCM
messages. RTCM observation messages will be streamed as soon as they are conﬁgured for output.
However, any stationary reference position messages are output only when the base station position
has been initialized and is operating in time mode. Time mode sets the receiver to operate as a
stationary base station in ﬁxed position and only time is estimated.
The following procedures can be used to initialize the base station position:
•
Use the built-in survey-in procedure to estimate the position.
•
Enter coordinates independently generated or taken from an accurate position such as a survey
marker.
•
Use in rover mode while feeding the receiver corrections and then enter the resulting estimated
position coordinates as above.
3.1.5.5.1 Survey-in
Survey-in is a procedure that is carried out prior to entering time mode. It estimates the receiver
position by building a weighted mean of all valid 3D position solutions.
Two major parameters are required when conﬁguring:
•
A minimum observation time deﬁnes the minimum observation time independent of the
actual number of ﬁxes used for the position estimate. Values can range from one day for high
accuracy requirements to a few minutes for coarse position determination.
•
A 3D position standard deviation deﬁnes a limit on the spread of positions that contribute to
the calculated mean.
Survey-in ends when both requirements are successfully met. The receiver begins operation in time
mode and can output a base position message if conﬁgured. The survey-in status can be queried
using the UBX-NAV-SVIN message.
The base station receiver should not be fed RTCM corrections while it is in survey-in mode. If a
corrected position is desired, the base station coordinates should be pre-surveyed using RTCM
corrections; the resultant position can be used to set the base station in ﬁxed mode.
To conﬁgure a base station into survey-in mode (CFG-TMODE-MODE=SURVEY_IN), the following
items are required:
Conﬁguration item
Description
CFG-TMODE-MODE
Receiver mode (disabled, survey-in or ﬁxed)
CFG-TMODE-SVIN_MIN_DUR
Survey-in minimum duration
CFG-TMODE-SVIN_ACC_LIMIT
Survey-in position accuracy limit
Table 8: Conﬁguration items used for setting a base station into survey-in mode
9 With ﬁrmware version HPG 1.12 the maximum baseline is 50 km, e.g. "WARNING: DGNSS long baseline: 52.7 km"
UBX-18010802 - R16
 
3 Receiver functionality
Page 20 of 129
C1-Public
```

---

## Page 21

```
ZED-F9P - Integration manual
3.1.5.5.2 Fixed position
An alternative to the survey-in procedure is to manually enter the receiver’s coordinates. Any error in
the base station position will directly translate into rover position errors. The base station position
accuracy should therefore match or exceed the desired rover absolute position accuracy.
To conﬁgure Fixed mode (CFG-TMODE-MODE=FIXED), the following items are relevant:
Conﬁguration item
Description
CFG-TMODE-MODE
Receiver mode (disabled or survey-in or ﬁxed)
CFG-TMODE-POS_TYPE
Determines whether the ARP position is given in ECEF or LAT/LON/HEIGHT
CFG-TMODE-ECEF_X
ECEF X coordinate of the ARP position
CFG-TMODE-ECEF_Y
ECEF Y coordinate of the ARP position
CFG-TMODE-ECEF_Z
ECEF Z coordinate of the ARP position
CFG-TMODE-LAT
Latitude of the ARP position
CFG-TMODE-LON
Longitude of the ARP position
CFG-TMODE-HEIGHT
Height of the ARP position
CFG-TMODE-ECEF_X_HP
High-precision ECEF X coordinate of the ARP position
CFG-TMODE-ECEF_Y_HP
High-precision ECEF Y coordinate of the ARP position
CFG-TMODE-ECEF_Z_HP
High-precision ECEF Z coordinate of the ARP position
CFG-TMODE-LAT_HP
High-precision latitude of the ARP position
CFG-TMODE-LON_HP
High-precision longitude of the ARP position
CFG-TMODE-HEIGHT_HP
High-precision height of the ARP position
CFG-TMODE-FIXED_POS_ACC
Fixed position 3D accuracy estimate
Table 9: Conﬁguration items used for setting a base station into ﬁxed mode
Once the receiver is set in ﬁxed mode, select the position format to use: either LLH or ECEF with
optional high precision (mm) coordinates compared to the standard cm value.
For example, with CFG-TMODE-POS_TYPE=ECEF the base antenna position can be entered
to cm precision using CFG-TMODE-ECEF_X, CFG-TMODE-ECEF_Y, CFGTMODE-ECEF_Z. For
high precision (mm) coordinates use CFG-TMODEECEF_X_HP, CFG-TMODE-ECEF_Y_HP, CFG-
TMODE-ECEF_Z_HP. The same applies with corresponding coordinates used with CFG-TMODE-
POS_TYPE=LLH.
The "3D accuracy" estimate in "Fixed Position" and the "Position accuracy limit" in "Survey-in" will
aﬀect the rover absolute position accuracy. Note that the availability of the position accuracy does
not mitigate the error in the rover position, but only accounts for it when calculating the resulting
positioning accuracy.
In stationary base station mode a current position check is made with respect to the ﬁxed
coordinates. If the result indicates the ﬁxed position coordinates are incorrect, a UBX-INF-
WARNING message "Base station position seems incorrect" is issued. The message is output
when the coordinates are incorrect by more than ~50 m up to 25 km.
If the base station is moved during operation then new position coordinates must be conﬁgured.
An illustrated procedure for conﬁguring a base receiver using u-center is shown in the Appendix C.1.
3.1.5.5.3 Base station: RTCM output conﬁguration
The desired RTCM messages must be selected and conﬁgured for the corresponding GNSS
constellations received. The recommended list of RTCM output messages for a base operating in
default GNSS conﬁguration are:
•
RTCM 1005 Stationary RTK reference station ARP
UBX-18010802 - R16
 
3 Receiver functionality
Page 21 of 129
C1-Public
```

---

## Page 22

```
ZED-F9P - Integration manual
•
RTCM 1074 GPS MSM4
•
RTCM 1084 GLONASS MSM4
•
RTCM 1094 Galileo MSM4
•
RTCM 1124 BeiDou MSM4
•
RTCM 1230 GLONASS code-phase biases
The conﬁguration messages for these are shown in the Table 10.
The following conﬁguration items output the recommended messages for a default satellite
constellation setting. Note that these are given for the UART1 interface:
Conﬁguration item
Description
CFG-MSGOUT-
RTCM_3X_TYPE1005_UART1
Output rate of the RTCM-3X-TYPE1005 message on port UART1: RTCM base
station message
CFG-MSGOUT-
RTCM_3X_TYPE1074_UART1
Output rate of the RTCM-3X-TYPE1074 message on port UART1: RTCM GPS
MSM4 message
CFG-MSGOUT-
RTCM_3X_TYPE1084_UART1
Output rate of the RTCM-3X-TYPE1084 message on port UART1: RTCM GLONASS
MSM4 message
CFG-MSGOUT-
RTCM_3X_TYPE1094_UART1
Output rate of the RTCM-3X-TYPE1094 message on port UART1: RTCM Galileo
MSM4 message
CFG-MSGOUT-
RTCM_3X_TYPE1124_UART1
Output rate of the RTCM-3X-TYPE1124 message on port UART1: RTCM BeiDou
MSM4 message
CFG-MSGOUT-
RTCM_3X_TYPE1230_UART1
Output rate of the RTCM-3X-TYPE1230 message on port UART1: RTCM GLONASS
bias message
Table 10: Conﬁguration items used for typical RTCM output conﬁguration on UART1
CFG-RTCM-DF003_OUT10 can be used to conﬁgure the reference station ID that will be reported in
all RTCM messages containing the RTCM DF003 data ﬁeld.
The conﬁguration of the RTCM 3.3 correction stream must be made with the following guidance:
•
All observation messages must be broadcast at the same rate.
•
The RTCM 3.3 correction stream must contain the GLONASS code-phase biases message
(RTCM 1230) or the Receiver Antenna Description message (RTCM 1033) otherwise the
GLONASS ambiguities can only be estimated as ﬂoat, even in RTK ﬁxed mode.
•
The static reference station message (RTCM 1005 or RTCM 1006) does not need to be
broadcast at the same rate as the observation messages, however, a rover will not be able to
compute its position until it has received a valid reference station message.
•
The correction stream should only contain one type of observation messages per constellation.
When using a multi-constellation conﬁguration, all constellations should use the same type
of observation messages. Mixing MSM4 and MSM7 messages will possibly lead to incorrect
setting of the multiple message bit.
•
If the receiver is conﬁgured to output RTCM messages on several ports, they must all have the
same RTCM conﬁguration, otherwise, the MSM multiple message bit might not be set properly.
3.1.5.6 Base and rover conﬁguration for moving base RTK operation
Firmware version HPG L1L5 1.40 does not support the moving base feature.
The moving base (MB) mode diﬀers from the standard RTK operation in that the base station is
no longer stationary at a predetermined location. Both the reference station and rover receivers are
allowed to move while computing an accurate vector between the receiver antennas. This mode
enables the calculation of heading on dynamic or static platforms, plus provides a centimeter-level
accurate 3D vector for use in dynamic positioning applications, e.g. a UAV ”follow me” feature.
10 CFG-RTCM-DF003_* items are supported from ﬁrmware version HPG 1.13 onwards
UBX-18010802 - R16
 
3 Receiver functionality
Page 22 of 129
C1-Public
```

---

## Page 23

```
ZED-F9P - Integration manual
In the MB RTK mode, the base and rover receivers are referred to as MB base and MB rover
respectively.
This section describes how to conﬁgure the ZED-F9P high precision receiver in a moving base setup.
3.1.5.6.1 Base operation in moving base RTK mode
In addition to the rules described in RTCM output conﬁguration section above, the following moving
base speciﬁc rules apply:
•
The RTCM 3.3 stream must contain reference station message 4072.0 (position information)
and MSM411 or MSM7 observation messages, otherwise the rover will be unable to operate in
MB rover mode.
•
Message 4072.112 (timing information) is no longer necessary for moving base rover and
as such it is no longer used by a moving base rover. To reduce the bandwidth requirements
between a moving base rover and a moving base, it is recommended to disable RTCM 4072.1
output on the base.
•
Message 4072.0 must be sent for each epoch the MB base observations are sent.
•
To ensure that the moving base processing works, the MB base and rover must use the same
navigation update rate and measurement rate.
The desired RTCM messages must be selected and conﬁgured for the corresponding GNSS
constellations received. The recommended list of RTCM output messages for a base operating in
MB conﬁguration are:
•
RTCM 4072.0 Reference station PVT information
•
RTCM 1074 GPS MSM4
•
RTCM 1084 GLONASS MSM4
•
RTCM 1094 Galileo MSM4
•
RTCM 1124 BeiDou MSM4
•
RTCM 1230 GLONASS code-phase biases
With ﬁrmware version HPG 1.12, only MSM7 messages are supported and message 4072.1 is
needed.
Additionally, while an MB receiver operates as a base, it is able to simultaneously:
•
Apply RTCM 3.3 corrections and reach RTK ﬁxed mode.
•
Generate an MB correction stream for accompanying MB rover(s).
3.1.5.6.2 Rover operation in moving base RTK mode
While the MB RTK solution aims at estimating the relative position with centimeter-level accuracy,
the absolute position of each receiver is expected to be known with a standard GNSS accuracy of a
few meters, unless the base is receiving RTCM 3.3 corrections.
In addition to the rules described in the rover operation section, the following moving base speciﬁc
rules apply:
•
A moving base receiver typically experiences worse GNSS tracking than a static base receiver in
an open-sky environment and therefore the MB RTK performance may be degraded.
•
The MB rover will wait as long as possible to compute a navigation solution, possibly lowering
the navigation rate all the way to 1 Hz while doing so. If no time-matched observations are
received in time, the receiver will ﬂag the relative position as invalid.
11 MSM4 messages in moving base mode are supported from ﬁrmware version HPG 1.13 onwards
12 Message 4072.1 is no longer necessary from ﬁrmware version HPG 1.13 onwards
UBX-18010802 - R16
 
3 Receiver functionality
Page 23 of 129
C1-Public
```

---

## Page 24

```
ZED-F9P - Integration manual
•
The achievable navigation update rate of the MB RTK solution is limited by the communication
link latency. Latency here refers to the delay we have in getting the RTCM messages on the
rover from the time they are sent from the base station.
•
It is a good practice to try and minimize the latency in the communication link, especially if
trying to achieve high navigation update rate. As a rule of thumb, the communication link
latency should be less than the desired navigation update period minus 50 ms.
•
If the communication link latency is too high to achieve the conﬁgured navigation update rate
the receiver will lower the navigation update rate until it reaches 1 Hz.
•
If the communication link latency is too high for 1 Hz rover operation, the receiver will declare
the relative position as invalid.
•
To ensure that the moving base processing works, the MB base and rover must use the same
navigation update rate and measurement rate.
In addition to the modiﬁed output described in the rover operation section, the following moving
base output message modiﬁcations will be observed:
UBX-NAV-RELPOSNED:
•
The isMoving ﬂag will be set
•
The relPosValid ﬂag will be set if the rover managed to get the time-matched observations in
time and process the moving base solution
•
The length of the relative position vector and its accuracy will be output
•
The heading for the relative position vector and its accuracy will be output
•
The relPosHeadingValid ﬂag will be set, unless the length of the relative position vector and/or
its accuracy are not suﬃcient to reliably derive the heading information
With ﬁrmware version HPG 1.12, the time-matched observations must be received within
700 ms. When the limit is exceeded, the MB reference observations and/or position will be
extrapolated for up to 3 s before falling back to standard GNSS operation. The refPosMiss
and refObsMiss ﬂags will be set for epochs during which the extrapolated base position or
observations have been used. These ﬂags are no longer supported from ﬁrmware version HPG
1.13 onwards.
3.1.6 PPP-RTK conﬁguration
Supported from ﬁrmware version HPG 1.30 onwards
3.1.6.1 SPARTN corrections
When operating as a rover, the ZED-F9P can receive SPARTN corrections:
•
via the internet from a service provider
•
via a host application that receives L-band satellite data (u-blox NEO-D9S may be used for this).
For more information, see section Multiple SPARTN sources.
If you choose PointPerfect service, contact your local u-blox technical support
3.1.6.1.1 SPARTN protocol
SPARTN is a binary protocol for the communication of SSR correction information.
ZED-F9P supports SPARTN as speciﬁed by SPARTN Interface Control Document – Version 2.0.1
(September, 2021).
To modify the SPARTN input/output settings, see the conﬁguration section in the applicable
Interface description [2].
UBX-18010802 - R16
 
3 Receiver functionality
Page 24 of 129
C1-Public
```

---
