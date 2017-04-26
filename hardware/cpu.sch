EESchema Schematic File Version 2
LIBS:power
LIBS:device
LIBS:transistors
LIBS:conn
LIBS:linear
LIBS:regul
LIBS:74xx
LIBS:cmos4000
LIBS:adc-dac
LIBS:memory
LIBS:xilinx
LIBS:microcontrollers
LIBS:dsp
LIBS:microchip
LIBS:analog_switches
LIBS:motorola
LIBS:texas
LIBS:intel
LIBS:audio
LIBS:interface
LIBS:digital-audio
LIBS:philips
LIBS:display
LIBS:cypress
LIBS:siliconi
LIBS:opto
LIBS:atmel
LIBS:contrib
LIBS:valves
LIBS:pda6502v2
LIBS:pda6502v2-cache
EELAYER 25 0
EELAYER END
$Descr A4 11693 8268
encoding utf-8
Sheet 3 5
Title ""
Date ""
Rev ""
Comp ""
Comment1 ""
Comment2 ""
Comment3 ""
Comment4 ""
$EndDescr
$Comp
L W65C02S U301
U 1 1 58ED8550
P 3700 3950
F 0 "U301" H 3950 5050 60  0000 C CNN
F 1 "W65C02S" H 3700 3950 60  0000 C CNN
F 2 "" H 3700 3950 60  0001 C CNN
F 3 "" H 3700 3950 60  0001 C CNN
	1    3700 3950
	1    0    0    -1  
$EndComp
Wire Wire Line
	3000 3800 2500 3800
Wire Wire Line
	3000 3900 2500 3900
Wire Wire Line
	3000 4000 2500 4000
Wire Wire Line
	3000 4100 2500 4100
Wire Wire Line
	3000 4200 2500 4200
Wire Wire Line
	3000 4300 2500 4300
Wire Wire Line
	3000 4400 2500 4400
Wire Wire Line
	3000 4500 2500 4500
Wire Wire Line
	3000 4600 2500 4600
Wire Wire Line
	3000 4700 2500 4700
Wire Wire Line
	3000 4800 2500 4800
Wire Wire Line
	3000 4900 2500 4900
Text Label 2550 3800 0    60   ~ 0
ADDR0
Text Label 2550 3900 0    60   ~ 0
ADDR1
Text Label 2550 4000 0    60   ~ 0
ADDR2
Text Label 2550 4100 0    60   ~ 0
ADDR3
Text Label 2550 4200 0    60   ~ 0
ADDR4
Text Label 2550 4300 0    60   ~ 0
ADDR5
Text Label 2550 4400 0    60   ~ 0
ADDR6
Text Label 2550 4500 0    60   ~ 0
ADDR7
Text Label 2550 4600 0    60   ~ 0
ADDR8
Text Label 2550 4700 0    60   ~ 0
ADDR9
Text Label 2550 4800 0    60   ~ 0
ADDR10
Text Label 2550 4900 0    60   ~ 0
ADDR11
Entry Wire Line
	2400 3900 2500 3800
Entry Wire Line
	2400 4000 2500 3900
Entry Wire Line
	2400 4100 2500 4000
Entry Wire Line
	2400 4200 2500 4100
Entry Wire Line
	2400 4300 2500 4200
Entry Wire Line
	2400 4400 2500 4300
Entry Wire Line
	2400 4500 2500 4400
Entry Wire Line
	2400 4600 2500 4500
Entry Wire Line
	2400 4700 2500 4600
Entry Wire Line
	2400 4800 2500 4700
Entry Wire Line
	2400 4900 2500 4800
Entry Wire Line
	2400 5000 2500 4900
Wire Bus Line
	2400 3900 2400 5100
Text HLabel 2400 5100 3    60   Output ~ 0
ADDR[0..15]
$Comp
L GND #PWR302
U 1 1 58ED884A
P 4400 5000
F 0 "#PWR302" H 4400 4750 50  0001 C CNN
F 1 "GND" H 4400 4850 50  0000 C CNN
F 2 "" H 4400 5000 50  0001 C CNN
F 3 "" H 4400 5000 50  0001 C CNN
	1    4400 5000
	1    0    0    -1  
$EndComp
Wire Wire Line
	4400 4900 4400 5000
Text Label 4500 4500 0    60   ~ 0
ADDR15
Text Label 4500 4600 0    60   ~ 0
ADDR14
Text Label 4500 4700 0    60   ~ 0
ADDR13
Text Label 4500 4800 0    60   ~ 0
ADDR12
Wire Wire Line
	4400 4500 4900 4500
Wire Wire Line
	4400 4600 4900 4600
Wire Wire Line
	4400 4700 4900 4700
Wire Wire Line
	4400 4800 4900 4800
Entry Wire Line
	4900 4500 5000 4600
Entry Wire Line
	4900 4600 5000 4700
Entry Wire Line
	4900 4700 5000 4800
Entry Wire Line
	4900 4800 5000 4900
Wire Bus Line
	5000 4600 5000 5100
Text HLabel 5000 5100 3    60   Output ~ 0
ADDR[0..15]
Text Label 4900 3700 0    60   ~ 0
DATA0
Text Label 4900 3800 0    60   ~ 0
DATA1
Text Label 4900 3900 0    60   ~ 0
DATA2
Text Label 4900 4000 0    60   ~ 0
DATA3
Text Label 4900 4100 0    60   ~ 0
DATA4
Text Label 4900 4200 0    60   ~ 0
DATA5
Text Label 4900 4300 0    60   ~ 0
DATA6
Text Label 4900 4400 0    60   ~ 0
DATA7
Wire Wire Line
	4400 3700 5200 3700
Wire Wire Line
	4400 3800 5200 3800
Wire Wire Line
	4400 3900 5200 3900
Wire Wire Line
	4400 4000 5200 4000
Wire Wire Line
	4400 4100 5200 4100
Wire Wire Line
	4400 4200 5200 4200
Wire Wire Line
	4400 4300 5200 4300
Wire Wire Line
	4400 4400 5200 4400
Entry Wire Line
	5200 3700 5300 3800
Entry Wire Line
	5200 3800 5300 3900
Entry Wire Line
	5200 3900 5300 4000
Entry Wire Line
	5200 4000 5300 4100
Entry Wire Line
	5200 4100 5300 4200
Entry Wire Line
	5200 4200 5300 4300
Entry Wire Line
	5200 4300 5300 4400
Entry Wire Line
	5200 4400 5300 4500
Wire Bus Line
	5300 3800 5300 5100
Text HLabel 5300 5100 3    60   BiDi ~ 0
DATA[0..7]
Wire Wire Line
	3000 3700 1650 3700
Wire Wire Line
	1650 3700 1650 3600
$Comp
L +3V3 #PWR301
U 1 1 58ED8AFB
P 1650 3600
F 0 "#PWR301" H 1650 3450 50  0001 C CNN
F 1 "+3V3" H 1650 3740 50  0000 C CNN
F 2 "" H 1650 3600 50  0001 C CNN
F 3 "" H 1650 3600 50  0001 C CNN
	1    1650 3600
	1    0    0    -1  
$EndComp
Wire Wire Line
	3000 3000 2800 3000
Wire Wire Line
	3000 3100 2800 3100
Wire Wire Line
	3000 3200 2800 3200
Wire Wire Line
	3000 3300 2800 3300
Wire Wire Line
	3000 3400 2800 3400
Wire Wire Line
	3000 3500 2800 3500
Wire Wire Line
	3000 3600 2800 3600
Text HLabel 2800 3000 0    60   Output ~ 0
VPB
Text HLabel 2800 3100 0    60   BiDi ~ 0
RDY
Text HLabel 2800 3200 0    60   Output ~ 0
PHI1O
Text HLabel 2800 3300 0    60   Input ~ 0
IRQB
Text HLabel 2800 3400 0    60   Output ~ 0
MLB
Text HLabel 2800 3500 0    60   Input ~ 0
NMIB
Text HLabel 2800 3600 0    60   Output ~ 0
SYNC
Wire Wire Line
	4400 3000 4600 3000
Wire Wire Line
	4400 3100 4600 3100
Wire Wire Line
	4400 3200 4600 3200
Wire Wire Line
	4400 3300 4600 3300
Wire Wire Line
	4400 3400 4600 3400
Wire Wire Line
	4400 3600 4600 3600
Text HLabel 4600 3000 2    60   Input ~ 0
RESB
Text HLabel 4600 3100 2    60   Output ~ 0
PHI2O
Text HLabel 4600 3200 2    60   Input ~ 0
SOB
Text HLabel 4600 3300 2    60   Input ~ 0
PHI2
Text HLabel 4600 3400 2    60   Input ~ 0
BE
Text HLabel 4600 3600 2    60   Output ~ 0
RWB
$Comp
L C C301
U 1 1 58ED8CA9
P 6050 3500
F 0 "C301" H 6075 3600 50  0000 L CNN
F 1 "C" H 6075 3400 50  0000 L CNN
F 2 "" H 6088 3350 50  0001 C CNN
F 3 "" H 6050 3500 50  0001 C CNN
	1    6050 3500
	1    0    0    -1  
$EndComp
Wire Wire Line
	6050 3350 6050 3250
$Comp
L +3V3 #PWR303
U 1 1 58ED8D13
P 6050 3250
F 0 "#PWR303" H 6050 3100 50  0001 C CNN
F 1 "+3V3" H 6050 3390 50  0000 C CNN
F 2 "" H 6050 3250 50  0001 C CNN
F 3 "" H 6050 3250 50  0001 C CNN
	1    6050 3250
	1    0    0    -1  
$EndComp
Wire Wire Line
	6050 3650 6050 3750
$Comp
L GND #PWR304
U 1 1 58ED8D5F
P 6050 3750
F 0 "#PWR304" H 6050 3500 50  0001 C CNN
F 1 "GND" H 6050 3600 50  0000 C CNN
F 2 "" H 6050 3750 50  0001 C CNN
F 3 "" H 6050 3750 50  0001 C CNN
	1    6050 3750
	1    0    0    -1  
$EndComp
$EndSCHEMATC
