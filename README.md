pda6502v2
=========

Version two (complete redesign) of https://github.com/pda/pda6502

A 3.3V single board computer with 6502 CPU, 64 KiB RAM, GPIO+UART+SPI I/O, CPLD
system controller / address decoder, serial EEPROM boot loader.

Version two aims to address the main pain points of version one:

* tedious software/bootloader development cycle, removing and reprogramming EEPROM.
* limited/slow serial communications (bit-banged via 6522 GPIO).
* no UART for serial console etc.
* inflexible address bus mapping with hard-wired logic gates.


Comparison table
----------------

|                       | v1                  | v2                             |
| --------------------- | ------------------- | ------------------------------ |
| Voltage               | 5.0V                | 3.3V                           |
| Address decode logic  | 7400-series chips   | CPLD                           |
| Schematic/layout tool | EAGLE               | KiCad                          |
| Boot                  | EEPROM              | CPLD loads boostrap from flash |
| Clock                 | 1 MHz osc           | ?                              |
| I/O                   | GPIO                | GPIO, UART, SPI                |


CPLD
----

A CPLD (Xilinx CoolRunner-II XC2C128A) acts as a system controller with two
primary functions;

### System startup

Startup code is stored in an on-board SPI serial EEPROM
(e.g. Microchip AT25128 / AT25512 / AT25M01) which can be written from an
external computer via SPI, e.g. using an off-board FTDI FT232H interface.

The system controller CPLD bootstraps the system by copying a fixed
size/location bootloader from serial EEPROM into main RAM before starting CPU.
The bootloader may load further code/data from the serial EEPROM.

This solves the main pain point of version one, in which program/bootloader
development involved removing and rewriting an EEPROM on every development
iteration.

### Address logic

Mapping of the address space to SRAM and I/O chips is also handled by the
system controller CPLD.  This provide very low propagation time allowing for
fast clock speed, and is in-circuit programmable for flexible address space /
logic design without hardware changes.

The CPLD has the entire address bus, data bus, and control signals, allowing
for arbitrary address mapping. For example a control register could be exposed
for state-based mapping, e.g. memory bank switching like the Commodore 64.

Minimal pin requirements:

| Signal  | Pins |
| ------- | ---- |
| ADDR    |  16  |
| DATA    |   8  |
| RWB     |   1  |
| CLK     |   1  |
| RESET   |   1  |
| SRAM CS |   1  |
| VIA CS  |   2  |
| SPI CS  |   2  |

This minimal set would fit on a XC2C64A-VQ44 with one pin to spare. However,
some other signals are nice to have:

* control over all IRQ signals;
  * also eliminates external AND logic for driven-high 6522 IRQ.
* per-chip control over RWB signal.
* RDY and SYNC signals from 6502 CPU for better single-step support etc.
* VPB (vector pull) from 6502 CPU for interrupts.

| Signal   | Pins |
| -------- | ---- |
| VIA IRQ  |   2  |
| SPI IRQ  |   2  |
| CPU IRQ  |   1  |
| VIA RWB  |   2  |
| SPI RWB  |   2  |
| SRAM RWB |   1  |
| CPU RDY  |   1  |
| CPU SYNC |   1  |
| CPU VPB  |   1  |

Brings total to 42, at which point XC2C128A-VQ100 is suitable, with plenty of
spare IO and double the macrocells. The pin count and 0.5mm pin spacing of
VQ100 makes soldering more difficult, but sparser pin utilization means layout
flexibility.


I/O
---

- SPI: 4 devices
- GPIO: 2 x 8-bit ports
- UART

### CPLD SPI controller

Another CPLD (Xilinx CoolRunner-II XC2C64A-VQ44) is mapped into the 6502
address space and programmed as SPI master using VHDL adapted from
http://6502.org/users/andre/spi65b/index.html

This hardware SPI communication is orders of magnitude faster than 6522
bit-banging, making SPI graphics output more viable.

### GPIO

WDC W65C22S (6522) VIA provides two 8-bit GPIO ports, as well as timers etc.

### UART

NXP SC28L91 3.3V UART PLCC44


Power supply
------------

The main system board expects a regulated 3.3 VDC, which may be done by a small
add-on board or module. All internal and external signals are 3.3V. CPLD chips
are powered via a 1.8V regulator.



Block diagram
-------------

![](docs/block.png)
