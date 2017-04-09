pda6502v2
=========

Version two (complete redesign) of https://github.com/pda/pda6502

A 3.3V single board computer with 6502 CPU, 64 KiB RAM, GPIO+UART+SPI I/O, CPLD
address decoder, and a microcontroller acting as boot loader, variable clock
source and program monitor/debugger.

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
| Boot                  | EEPROM              | microcontroller preloads SRAM  |
| Clock                 | 1 MHz osc           | microcontroller 0-10 MHz       |
| I/O                   | GPIO                | GPIO, UART, SPI                |


Microcontroller
---------------

A modern microcontroller (Atmel SAMD21) acts as a system controller with three
primary functions;

### Firmware management

The microcontroller has a USB interface allowing a host machine to
upload/download firmware and configuration into an on-board flash chip or
straight into system RAM.

This solves the main pain point of version one, in which program/bootloader
development involved removing and rewriting an EEPROM on every development
iteration.

### System startup

On startup, the microcontroller copies the program/bootloader from flash memory
into system RAM. This removes the need for an EEPROM chip, and the slow access
times generally associated with them, allowing for simpler and more flexible
design and faster potential system clock speed.

### Clock signal

Instead of a fixed oscillator, the microcontroller generates an adjustable
system clock signal between 0 Hz and 10 MHz. Combined with other control
signals this allows for single-clock-stepping and bus monitoring. Programming
the microcontroller with a 6502 instruction decoder and/or disassembler allows
for single-instruction stepping and program debugging.

At 3.3V the WDC W65C02S is expected to be stable at least to 8 MHz, subject to
design of the circuit and connected peripherals. With fast memory and address
decode, 10 MHz is not unrealistic.


CPLD address logic
------------------

Mapping of the address space to SRAM and I/O chips is handled by a Xilinx
CoolRunner-II XC2C128A-VQ100 CPLD.  This provide very low propagation time
allowing for fast clock speed, and is in-circuit programmable for a flexible
address space without hardware changes.

The address logic CPLD has the entire address bus, data bus, and control
signals, allowing for completely arbitrary address mapping including exposing a
control register for state-based mapping, e.g. memory bank switching like the
Commodore 64.

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


CPLD SPI controller
-------------------

Two more Xilinx CoolRunner-II XC2C64A-VQ44 CPLDs are mapped into the 6502
address space and programmed as SPI masters using VHDL adapted from
http://6502.org/users/andre/spi65b/index.html

This hardware SPI communication is orders of magnitude faster than 6522
bit-banging, making SPI graphics output more viable.


FPGA HDMI graphics (stretch goal)
---------------------------------

An FPGA could be added to the design, with HDMI output and access to the system
RAM between clock cycles, like the VIC-II in the Commodore 64. It would read
course-grained data (tiles, sprites) from RAM and rasterize them.


Power supply
------------

The main system board expects a regulated 3.3 VDC, which may be done by a small
add-on board or module. All internal and external signals are 3.3V. CPLD chips
are powered via a 1.8V regulator.

The microcontroller USB interface is electrically isolated from the board power
supply.


GPIO
----

Two WDC W65C22S (6522) VIAs provide two 8-bit GPIO ports each.


Block diagram
-------------

![](docs/block.png)
