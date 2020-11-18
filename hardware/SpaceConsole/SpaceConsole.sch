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
L mysensors_arduino:ArduinoProMini IC?
U 1 1 5FB7EAFA
P 5250 4150
F 0 "IC?" H 5300 5600 40  0000 C CNN
F 1 "ArduinoProMini" H 5300 5524 40  0000 C CNN
F 2 "mysensors_arduino:pro_mini_china" H 5250 4150 30  0001 C CIN
F 3 "http://www.arduino.cc/en/uploads/Main/Arduino-Pro-Mini-schematic.pdf" H 5300 5433 60  0000 C CNN
	1    5250 4150
	1    0    0    -1  
$EndComp
$Comp
L power:VCC #PWR?
U 1 1 5FB7FD72
P 4350 3050
F 0 "#PWR?" H 4350 2900 50  0001 C CNN
F 1 "VCC" V 4365 3177 50  0000 L CNN
F 2 "" H 4350 3050 50  0001 C CNN
F 3 "" H 4350 3050 50  0001 C CNN
	1    4350 3050
	0    -1   -1   0   
$EndComp
$Comp
L power:GND #PWR?
U 1 1 5FB80DDB
P 4350 3350
F 0 "#PWR?" H 4350 3100 50  0001 C CNN
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
$EndSCHEMATC
