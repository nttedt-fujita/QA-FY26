# PDF抽出: ZED-F9P_IntegrationManual_UBX-18010802.pdf

**総ページ数**: 129
**抽出ページ**: 83, 84, 85

---

## Page 83

```
ZED-F9P - Integration manual
A ﬁrmware image is a binary ﬁle containing the software to be run by the GNSS receiver. A ﬁrmware
update is the process of transferring a ﬁrmware image to the receiver and storing it in non-volatile
ﬂash memory.
Contact u-blox for more information on ﬁrmware update.
3.18 Spectrum analyzer
Supported from ﬁrmware version HPG 1.13 onwards
The UBX-MON-SPAN message provides a low-resolution RF spectrum analyzer function suﬃcient
to identify noise or jammers in the receiver's reception band(s). It can be used for monitoring
potential interferers during customer integration and in normal operation to identify interference,
e.g. when the UBX-SEC-SIG or UBX-MON-RF message detects a possible jamming threat. See
Jamming/interference indicator for more details. u-center provides a visualization of the message
spectrum output(s) with features for max-hold and averaging.
This message is intended for comparative analysis of the RF spectrum rather than absolute
and precise measurement.
The message is output once per second when enabled. Depending on the receiver type, one or two
measurement blocks will be output, indicated by the numRfBlocks ﬂag ﬁeld. The ﬁrst block provides
L1 spectrum data which can be followed by an L2 or L5 block with multi-band receivers.
Each block comprises the following data:
•
256 spectrum data points (0.25 dB units)
•
Spectrum span (Hz)
•
Spectrum bin resolution (Hz)
•
Center frequency (Hz)
•
PGA setting (dB)
The frequency of each point can be calculated by Freq(i) = center frequency + spectrum span *
(i-128) / 256, where i=0-255. The number of points = span/resolution and is scaled in units of 0.25
dB. Changes in the PGA gain value can indicate an increased input in RF signal activity compared
to normal operation.
Figure 37 shows the spectrum view in u-center with the view/hold option selected. The red line
represents the frozen spectrum before modifying the external gain, while the black line represents
the current measurement.
UBX-18010802 - R16
 
3 Receiver functionality
Page 83 of 129
C1-Public
```

---

## Page 84

```
ZED-F9P - Integration manual
Figure 37: Spectrum analyzer view in u-center with the option view/hold selected
The span frequency depends on the number of constellations enabled which impacts the
spectrum resolution owing to a ﬁxed set of points. For further details about this message see
the Interface description [2].
A spur may be visible at the center frequency, this comes internally from the receiver and does
not cause any performance degradation.
3.19 Production test
u-blox focuses on high quality for its products. To achieve this, we only supply fully tested units.
At the end of the production process, every unit is tested. Defective units are analyzed in detail to
continuously improve the production quality.
This is achieved with automatic test equipment, which delivers a detailed test report for each unit.
The following measurements are done:
•
Digital self-test (software download, veriﬁcation of FLASH ﬁrmware, etc.)
•
Measurement of voltages and currents
•
Measurement of RF characteristics (e.g. C/No)
Thanks to the 100 % test coverage done by u-blox, the OEM manufacturer doesn’t need to
repeat ﬁrmware tests or measurements of the GNSS parameters/characteristics (e.g. TTFF) in the
production test.
The OEM manufacturer can focus on testing:
•
Overall sensitivity of the device (including antenna, if applicable)
•
Communication to a host controller
3.19.1 Connected sensitivity test
The best way to test the sensitivity of a positioning device is with the use of a GNSS simulator. It
assures reliable and constant signals at every measurement.
Guidelines for sensitivity tests:
•
Connect a GNSS simulator to the OEM product
•
Choose the power level in a way that the “Golden Device” would report a C/No ratio of 38-40
dBHz
•
Power up the DUT (Device Under Test) and allow enough time for the acquisition
UBX-18010802 - R16
 
3 Receiver functionality
Page 84 of 129
C1-Public
```

---

## Page 85

```
ZED-F9P - Integration manual
•
Read the C/No value from the NMEA GSV or the UBX-NAV-SAT message (e.g. with u-center)
•
Compare the results to a “Golden Device”, a u-blox Evaluation Kit or Application Board.
3.19.2 Go/No go tests for integrated devices
•
For best results, place the device in an outdoor position with excellent sky view (HDOP < 3.0).
•
Let the receiver acquire satellites and compare the signal strength with a “Golden Device”. As
the electro-magnetic ﬁeld of a redistribution antenna is not homogenous, indoor tests are not
reliable in most cases.
These kinds of tests are useful as a go/no go test but not for sensitivity measurements.
UBX-18010802 - R16
 
3 Receiver functionality
Page 85 of 129
C1-Public
```

---
