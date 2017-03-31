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
EELAYER 25 0
EELAYER END
$Descr A4 11693 8268
encoding utf-8
Sheet 2 2
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
L IC_CPLD_XC2C64A IC?
U 1 1 58CFBA86
P 5600 3800
F 0 "IC?" H 6650 5050 50  0000 L BNN
F 1 "IC_CPLD_XC2C64A" H 5600 3350 50  0000 C BNN
F 2 "VQ-44" H 5600 4150 50  0001 C CNN
F 3 "" H 5600 3800 60  0001 C CNN
	1    5600 3800
	1    0    0    -1  
$EndComp
$Comp
L MCP1703A-1802/MB U?
U 1 1 58CFBAF4
P 1250 1050
F 0 "U?" H 1400 800 50  0000 C CNN
F 1 "MCP1703A-1802/MB" H 1250 1200 50  0000 C CNN
F 2 "TO_SOT_Packages_SMD:SOT89-3_Housing" H 1300 1300 50  0001 C CNN
F 3 "" H 1250 1000 50  0001 C CNN
	1    1250 1050
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR?
U 1 1 58CFBC09
P 1250 1350
F 0 "#PWR?" H 1250 1100 50  0001 C CNN
F 1 "GND" H 1250 1200 50  0000 C CNN
F 2 "" H 1250 1350 50  0001 C CNN
F 3 "" H 1250 1350 50  0001 C CNN
	1    1250 1350
	1    0    0    -1  
$EndComp
Wire Wire Line
	800  950  800  1050
Wire Wire Line
	800  1050 950  1050
Wire Wire Line
	1550 1050 1700 1050
Text Label 1700 1050 0    60   ~ 0
+1V8
$Comp
L GND #PWR?
U 1 1 58CFBD9B
P 5600 5600
F 0 "#PWR?" H 5600 5350 50  0001 C CNN
F 1 "GND" H 5600 5450 50  0000 C CNN
F 2 "" H 5600 5600 50  0001 C CNN
F 3 "" H 5600 5600 50  0001 C CNN
	1    5600 5600
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR?
U 1 1 58CFBDB3
P 7100 4100
F 0 "#PWR?" H 7100 3850 50  0001 C CNN
F 1 "GND" H 7100 3950 50  0000 C CNN
F 2 "" H 7100 4100 50  0001 C CNN
F 3 "" H 7100 4100 50  0001 C CNN
	1    7100 4100
	0    -1   -1   0   
$EndComp
$Comp
L GND #PWR?
U 1 1 58CFBDCB
P 4100 3600
F 0 "#PWR?" H 4100 3350 50  0001 C CNN
F 1 "GND" H 4100 3450 50  0000 C CNN
F 2 "" H 4100 3600 50  0001 C CNN
F 3 "" H 4100 3600 50  0001 C CNN
	1    4100 3600
	0    1    1    0   
$EndComp
$Comp
L +1V8 #PWR?
U 1 1 58CFBED9
P 5400 5300
F 0 "#PWR?" H 5400 5150 50  0001 C CNN
F 1 "+1V8" H 5400 5440 50  0000 C CNN
F 2 "" H 5400 5300 50  0001 C CNN
F 3 "" H 5400 5300 50  0001 C CNN
	1    5400 5300
	-1   0    0    1   
$EndComp
Wire Wire Line
	4100 3600 4200 3600
Wire Wire Line
	5400 5200 5400 5300
Wire Wire Line
	7000 4100 7100 4100
$Comp
L +3V3 #PWR?
U 1 1 58CFBF6E
P 3800 3900
F 0 "#PWR?" H 3800 3750 50  0001 C CNN
F 1 "+3V3" H 3800 4040 50  0000 C CNN
F 2 "" H 3800 3900 50  0001 C CNN
F 3 "" H 3800 3900 50  0001 C CNN
	1    3800 3900
	0    -1   -1   0   
$EndComp
$Comp
L +3V3 #PWR?
U 1 1 58CFBF96
P 7400 4000
F 0 "#PWR?" H 7400 3850 50  0001 C CNN
F 1 "+3V3" H 7400 4140 50  0000 C CNN
F 2 "" H 7400 4000 50  0001 C CNN
F 3 "" H 7400 4000 50  0001 C CNN
	1    7400 4000
	0    1    1    0   
$EndComp
Wire Wire Line
	7400 4000 7000 4000
Wire Wire Line
	3800 3900 4200 3900
Wire Bus Line
	7800 1900 7800 5900
Wire Bus Line
	7800 5900 5750 5900
Wire Wire Line
	5800 1900 5800 2400
Wire Wire Line
	5900 1900 5900 2400
Wire Wire Line
	6100 1900 6100 2400
Wire Wire Line
	7800 3300 7000 3300
Wire Wire Line
	7800 3400 7000 3400
Wire Wire Line
	7800 3500 7000 3500
Wire Wire Line
	7800 3600 7000 3600
Wire Wire Line
	7800 3700 7000 3700
Wire Wire Line
	7800 3800 7000 3800
Wire Wire Line
	7800 3900 7000 3900
Wire Wire Line
	7800 4300 7000 4300
Wire Wire Line
	6100 5900 6100 5200
Wire Wire Line
	6000 5900 6000 5200
Wire Wire Line
	5900 5900 5900 5200
Wire Wire Line
	5800 5900 5800 5200
Wire Wire Line
	5700 1900 5700 2400
Text Label 7050 3300 0    60   ~ 0
ADDR4
Text Label 7050 3400 0    60   ~ 0
ADDR5
Text Label 7050 3500 0    60   ~ 0
ADDR6
Text Label 7050 3600 0    60   ~ 0
ADDR7
Text Label 7050 3700 0    60   ~ 0
ADDR8
Text Label 7050 3800 0    60   ~ 0
ADDR9
Text Label 7050 3900 0    60   ~ 0
ADDR10
Wire Bus Line
	5650 1900 7900 1900
Text Label 5700 2250 1    60   ~ 0
ADDR0
Text Label 5800 2250 1    60   ~ 0
ADDR1
Text Label 5900 2250 1    60   ~ 0
ADDR2
Text Label 6100 2250 1    60   ~ 0
ADDR3
Text Label 7050 4300 0    60   ~ 0
ADDR11
Text Label 6100 5200 3    60   ~ 0
ADDR12
Text Label 6000 5200 3    60   ~ 0
ADDR13
Text Label 5900 5200 3    60   ~ 0
ADDR14
Text Label 5800 5200 3    60   ~ 0
ADDR15
Text Label 7900 1900 0    60   ~ 0
ADDR
Wire Wire Line
	5600 5200 5600 5600
$Comp
L +3V3 #PWR?
U 1 1 58D51BB2
P 800 950
F 0 "#PWR?" H 800 800 50  0001 C CNN
F 1 "+3V3" H 800 1090 50  0000 C CNN
F 2 "" H 800 950 50  0001 C CNN
F 3 "" H 800 950 50  0001 C CNN
	1    800  950 
	1    0    0    -1  
$EndComp
$EndSCHEMATC
