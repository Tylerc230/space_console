EESchema Schematic File Version 4
EELAYER 30 0
EELAYER END
$Descr A4 11693 8268
encoding utf-8
Sheet 1 1
Title "Space Console"
Date ""
Rev ""
Comp ""
Comment1 ""
Comment2 ""
Comment3 ""
Comment4 ""
$EndDescr
$Comp
L mysensors_arduino:ArduinoProMini IC1
U 1 1 5FB7EAFA
P 5250 4150
F 0 "IC1" H 5300 5600 40  0000 C CNN
F 1 "ArduinoProMini" H 5300 5524 40  0000 C CNN
F 2 "mysensors_arduino:pro_mini_china" H 5250 4150 30  0001 C CIN
F 3 "http://www.arduino.cc/en/uploads/Main/Arduino-Pro-Mini-schematic.pdf" H 5300 5433 60  0000 C CNN
	1    5250 4150
	1    0    0    -1  
$EndComp
$Comp
L power:VCC #PWR01
U 1 1 5FB7FD72
P 4350 3050
F 0 "#PWR01" H 4350 2900 50  0001 C CNN
F 1 "VCC" V 4365 3177 50  0000 L CNN
F 2 "" H 4350 3050 50  0001 C CNN
F 3 "" H 4350 3050 50  0001 C CNN
	1    4350 3050
	0    -1   -1   0   
$EndComp
$Comp
L power:GND #PWR02
U 1 1 5FB80DDB
P 4350 3350
F 0 "#PWR02" H 4350 3100 50  0001 C CNN
F 1 "GND" V 4355 3222 50  0000 R CNN
F 2 "" H 4350 3350 50  0001 C CNN
F 3 "" H 4350 3350 50  0001 C CNN
	1    4350 3350
	0    1    1    0   
$EndComp
Text Label 6250 3250 0    50   ~ 0
LeftFrontLED
Text Label 6250 3350 0    50   ~ 0
RightFrontLED
Text Label 6250 3450 0    50   ~ 0
LeftMiddleLED
Text Label 6250 3550 0    50   ~ 0
RightMiddleLED
Text Label 6250 3650 0    50   ~ 0
RearLED
Text Label 6250 3750 0    50   ~ 0
SwitchA
Text Label 6250 3850 0    50   ~ 0
SwitchB
Text Label 6250 3950 0    50   ~ 0
SwitchC
Text Label 6250 4650 0    50   ~ 0
HornOut
Text Label 6250 4450 0    50   ~ 0
KnobB1
Text Label 6250 4350 0    50   ~ 0
KnobB0
Text Label 6250 4250 0    50   ~ 0
KnobA1
Text Label 6250 4050 0    50   ~ 0
KnobA0
$Comp
L Connector:Screw_Terminal_01x03 J1
U 1 1 5FB82450
P 8050 3000
F 0 "J1" H 8130 3042 50  0000 L CNN
F 1 "Screw_Terminal_01x03" H 8130 2951 50  0000 L CNN
F 2 "" H 8050 3000 50  0001 C CNN
F 3 "~" H 8050 3000 50  0001 C CNN
	1    8050 3000
	1    0    0    -1  
$EndComp
$Comp
L Connector:Screw_Terminal_01x03 J2
U 1 1 5FB83151
P 8050 3350
F 0 "J2" H 8130 3392 50  0000 L CNN
F 1 "Screw_Terminal_01x03" H 8130 3301 50  0000 L CNN
F 2 "" H 8050 3350 50  0001 C CNN
F 3 "~" H 8050 3350 50  0001 C CNN
	1    8050 3350
	1    0    0    -1  
$EndComp
$Comp
L Connector:Screw_Terminal_01x03 J3
U 1 1 5FB83CFA
P 8050 3700
F 0 "J3" H 8130 3742 50  0000 L CNN
F 1 "Screw_Terminal_01x03" H 8130 3651 50  0000 L CNN
F 2 "" H 8050 3700 50  0001 C CNN
F 3 "~" H 8050 3700 50  0001 C CNN
	1    8050 3700
	1    0    0    -1  
$EndComp
$Comp
L Connector:Screw_Terminal_01x03 J4
U 1 1 5FB847FB
P 8050 4050
F 0 "J4" H 8130 4092 50  0000 L CNN
F 1 "Screw_Terminal_01x03" H 8130 4001 50  0000 L CNN
F 2 "" H 8050 4050 50  0001 C CNN
F 3 "~" H 8050 4050 50  0001 C CNN
	1    8050 4050
	1    0    0    -1  
$EndComp
$Comp
L Connector:Screw_Terminal_01x03 J5
U 1 1 5FB8539C
P 8050 4450
F 0 "J5" H 8130 4492 50  0000 L CNN
F 1 "Screw_Terminal_01x03" H 8130 4401 50  0000 L CNN
F 2 "" H 8050 4450 50  0001 C CNN
F 3 "~" H 8050 4450 50  0001 C CNN
	1    8050 4450
	1    0    0    -1  
$EndComp
Wire Wire Line
	6250 3250 7200 3250
Wire Wire Line
	6250 3350 7850 3350
Wire Wire Line
	6250 3550 7500 3550
Wire Wire Line
	6250 3650 7250 3650
Wire Wire Line
	7250 3650 7250 4450
Wire Wire Line
	7500 3550 7500 4050
Wire Wire Line
	7500 4050 7850 4050
Wire Wire Line
	7250 4450 7850 4450
Wire Wire Line
	7200 3250 7200 3000
Wire Wire Line
	7200 3000 7850 3000
$Comp
L Connector:Screw_Terminal_01x02 J6
U 1 1 5FB9702F
P 8050 4800
F 0 "J6" H 8130 4792 50  0000 L CNN
F 1 "Screw_Terminal_01x02" H 8130 4701 50  0000 L CNN
F 2 "" H 8050 4800 50  0001 C CNN
F 3 "~" H 8050 4800 50  0001 C CNN
	1    8050 4800
	1    0    0    -1  
$EndComp
$Comp
L Connector:Screw_Terminal_01x02 J7
U 1 1 5FB97F65
P 8050 5150
F 0 "J7" H 8130 5142 50  0000 L CNN
F 1 "Screw_Terminal_01x02" H 8130 5051 50  0000 L CNN
F 2 "" H 8050 5150 50  0001 C CNN
F 3 "~" H 8050 5150 50  0001 C CNN
	1    8050 5150
	1    0    0    -1  
$EndComp
Wire Wire Line
	6250 3750 7100 3750
Wire Wire Line
	7100 3750 7100 4800
Wire Wire Line
	7100 4800 7850 4800
$Comp
L power:GND #PWR03
U 1 1 5FB99262
P 7850 4900
F 0 "#PWR03" H 7850 4650 50  0001 C CNN
F 1 "GND" V 7855 4772 50  0000 R CNN
F 2 "" H 7850 4900 50  0001 C CNN
F 3 "" H 7850 4900 50  0001 C CNN
	1    7850 4900
	0    1    1    0   
$EndComp
$Comp
L power:GND #PWR?
U 1 1 5FB9B39A
P 7850 3100
F 0 "#PWR?" H 7850 2850 50  0001 C CNN
F 1 "GND" V 7855 2972 50  0000 R CNN
F 2 "" H 7850 3100 50  0001 C CNN
F 3 "" H 7850 3100 50  0001 C CNN
	1    7850 3100
	0    1    1    0   
$EndComp
$Comp
L power:GND #PWR?
U 1 1 5FB9BFBF
P 7850 3450
F 0 "#PWR?" H 7850 3200 50  0001 C CNN
F 1 "GND" V 7855 3322 50  0000 R CNN
F 2 "" H 7850 3450 50  0001 C CNN
F 3 "" H 7850 3450 50  0001 C CNN
	1    7850 3450
	0    1    1    0   
$EndComp
$Comp
L power:GND #PWR?
U 1 1 5FB9C2F9
P 7850 3800
F 0 "#PWR?" H 7850 3550 50  0001 C CNN
F 1 "GND" V 7855 3672 50  0000 R CNN
F 2 "" H 7850 3800 50  0001 C CNN
F 3 "" H 7850 3800 50  0001 C CNN
	1    7850 3800
	0    1    1    0   
$EndComp
$Comp
L power:GND #PWR?
U 1 1 5FB9C647
P 7850 4150
F 0 "#PWR?" H 7850 3900 50  0001 C CNN
F 1 "GND" V 7855 4022 50  0000 R CNN
F 2 "" H 7850 4150 50  0001 C CNN
F 3 "" H 7850 4150 50  0001 C CNN
	1    7850 4150
	0    1    1    0   
$EndComp
$Comp
L power:GND #PWR?
U 1 1 5FB9CF71
P 7850 4550
F 0 "#PWR?" H 7850 4300 50  0001 C CNN
F 1 "GND" V 7855 4422 50  0000 R CNN
F 2 "" H 7850 4550 50  0001 C CNN
F 3 "" H 7850 4550 50  0001 C CNN
	1    7850 4550
	0    1    1    0   
$EndComp
$Comp
L power:VCC #PWR?
U 1 1 5FB9D84A
P 7850 2900
F 0 "#PWR?" H 7850 2750 50  0001 C CNN
F 1 "VCC" V 7865 3027 50  0000 L CNN
F 2 "" H 7850 2900 50  0001 C CNN
F 3 "" H 7850 2900 50  0001 C CNN
	1    7850 2900
	0    -1   -1   0   
$EndComp
$Comp
L power:VCC #PWR?
U 1 1 5FB9E272
P 7850 3250
F 0 "#PWR?" H 7850 3100 50  0001 C CNN
F 1 "VCC" V 7865 3377 50  0000 L CNN
F 2 "" H 7850 3250 50  0001 C CNN
F 3 "" H 7850 3250 50  0001 C CNN
	1    7850 3250
	0    -1   -1   0   
$EndComp
$Comp
L power:VCC #PWR?
U 1 1 5FB9EB8F
P 7850 3600
F 0 "#PWR?" H 7850 3450 50  0001 C CNN
F 1 "VCC" V 7865 3727 50  0000 L CNN
F 2 "" H 7850 3600 50  0001 C CNN
F 3 "" H 7850 3600 50  0001 C CNN
	1    7850 3600
	0    -1   -1   0   
$EndComp
$Comp
L power:VCC #PWR?
U 1 1 5FB9F00F
P 7850 3950
F 0 "#PWR?" H 7850 3800 50  0001 C CNN
F 1 "VCC" V 7865 4077 50  0000 L CNN
F 2 "" H 7850 3950 50  0001 C CNN
F 3 "" H 7850 3950 50  0001 C CNN
	1    7850 3950
	0    -1   -1   0   
$EndComp
$Comp
L power:VCC #PWR?
U 1 1 5FB9F5DF
P 7850 4350
F 0 "#PWR?" H 7850 4200 50  0001 C CNN
F 1 "VCC" V 7865 4477 50  0000 L CNN
F 2 "" H 7850 4350 50  0001 C CNN
F 3 "" H 7850 4350 50  0001 C CNN
	1    7850 4350
	0    -1   -1   0   
$EndComp
Wire Wire Line
	7550 3700 7850 3700
Wire Wire Line
	6250 3450 7550 3450
Wire Wire Line
	7550 3450 7550 3700
$Comp
L myComponents:LM2596_Adj_Stepdown_Module U?
U 1 1 5FBA7ED4
P 7200 5650
F 0 "U?" H 7428 5326 50  0000 L CNN
F 1 "LM2596_Adj_Stepdown_Module" H 7428 5235 50  0000 L CNN
F 2 "" H 7200 5650 50  0001 C CNN
F 3 "" H 7200 5650 50  0001 C CNN
	1    7200 5650
	1    0    0    -1  
$EndComp
$Comp
L power:VCC #PWR?
U 1 1 5FBA8E0E
P 6950 6100
F 0 "#PWR?" H 6950 5950 50  0001 C CNN
F 1 "VCC" V 6965 6227 50  0000 L CNN
F 2 "" H 6950 6100 50  0001 C CNN
F 3 "" H 6950 6100 50  0001 C CNN
	1    6950 6100
	0    -1   -1   0   
$EndComp
$Comp
L power:GND #PWR?
U 1 1 5FBAA224
P 6950 6250
F 0 "#PWR?" H 6950 6000 50  0001 C CNN
F 1 "GND" V 6955 6122 50  0000 R CNN
F 2 "" H 6950 6250 50  0001 C CNN
F 3 "" H 6950 6250 50  0001 C CNN
	1    6950 6250
	0    1    1    0   
$EndComp
Text Label 7850 5150 2    50   ~ 0
Battery+
Text Label 6950 5800 2    50   ~ 0
Battery+
Text Label 7850 5250 2    50   ~ 0
Battery-
Text Label 6950 5950 2    50   ~ 0
Battery-
$Comp
L Connector:Conn_01x09_Male J?
U 1 1 5FBB2A1F
P 5450 6400
F 0 "J?" H 5558 6981 50  0000 C CNN
F 1 "Conn_01x09_Male" H 5558 6890 50  0000 C CNN
F 2 "" H 5450 6400 50  0001 C CNN
F 3 "~" H 5450 6400 50  0001 C CNN
	1    5450 6400
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR?
U 1 1 5FBB4227
P 5650 6000
F 0 "#PWR?" H 5650 5750 50  0001 C CNN
F 1 "GND" V 5655 5872 50  0000 R CNN
F 2 "" H 5650 6000 50  0001 C CNN
F 3 "" H 5650 6000 50  0001 C CNN
	1    5650 6000
	0    -1   -1   0   
$EndComp
$Comp
L power:VCC #PWR?
U 1 1 5FBB4E89
P 5650 6100
F 0 "#PWR?" H 5650 5950 50  0001 C CNN
F 1 "VCC" V 5665 6228 50  0000 L CNN
F 2 "" H 5650 6100 50  0001 C CNN
F 3 "" H 5650 6100 50  0001 C CNN
	1    5650 6100
	0    1    1    0   
$EndComp
Text Label 5650 6200 0    50   ~ 0
SwitchB
Text Label 5650 6300 0    50   ~ 0
SwitchC
Text Label 5650 6400 0    50   ~ 0
KnobA0
Text Label 5650 6500 0    50   ~ 0
KnobA1
Text Label 5650 6600 0    50   ~ 0
KnobB0
Text Label 5650 6700 0    50   ~ 0
KnobB1
Text Label 5650 6800 0    50   ~ 0
HornOut
$EndSCHEMATC
