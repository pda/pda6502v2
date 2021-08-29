pda6502v2
=========

Version two (complete redesign) of https://github.com/pda/pda6502

A single board computer with 6502 CPU, 512 KiB RAM, 32 GPIO pins, dual UART,
high speed SPI, SID sound, FPGA system controller, ICSP-programmable EEPROM
boot loader.

The main v1 pain points being addressed by v2:

* tedious software/bootloader development cycle, removing and reprogramming EEPROM.
* limited/slow serial communications (bit-banged via 6522 GPIO).
* no UART for serial console etc.
* inflexible address bus mapping with hard-wired logic gates.


Comparison table
----------------

|                       | v1                  | v2                             |
| --------------------- | ------------------- | ------------------------------ |
| Voltage               | 5.0V                | 3.3V                           |
| Address decode logic  | 7400-series chips   | FPGA                           |
| Schematic/layout tool | EAGLE               | KiCad                          |
| Boot                  | EEPROM              | FPGA loads boostrap from flash |
| Clock                 | 1 MHz               | variable, target 8 MHz         |
| I/O                   | GPIO                | GPIO, UART, SPI                |
| RAM                   | 32 KiB              | 512 KiB                        |
| Sound                 | (none)              | ARMSID (SID emulator)          |


FPGA system controller: BIFRÖST
-------------------------------

An FPGA (Lattice ICE40HX4K-TQ144) acts as a system controller (dubbed BIFRÖST)
with three primary functions;

### System startup

Startup code is stored in an on-board SPI serial EEPROM (Microchip AT25M01)
which can be written from an external computer via SPI ICSP header.

(Microchip AT25M01 replaced by ON Semiconductor CAT25M01 due to availability).

BIFRÖST bootstraps the system by copying a fixed size/location bootloader from
serial EEPROM into main RAM before starting CPU.  The bootloader may load
further code/data from the serial EEPROM.

This solves the main pain point of version one, in which program/bootloader
development involved removing and rewriting an EEPROM on every development
iteration.

### Address logic

Mapping of the address space to SRAM and I/O chips is also handled by BIFRÖST.
This provide very low propagation time allowing for fast clock speed, and is
in-circuit programmable for flexible address space / logic design without
hardware changes.

BIFRÖST has the entire address bus, data bus, control signals and clock,
allowing for arbitrary address mapping. For example a control register could be
exposed for state-based mapping, e.g. memory layer switching like the Commodore
64.

### BIFRÖST chip selection / considerations

BIFRÖST is implemented as a Lattice ICE40HX4K-TQ144 FPGA with a Microchip
AT25M01 SPI flash EEPROM for configuration storage. The EEPROM also acts as
the 6502 system ROM, loaded into RAM by BIFRÖST during boot.

Any signal that can be routed through / controlled by BIFRÖST should be, so
that design decisions are pushed from hardware schematic and into FPGA HDL
which can be easily altered after assembly.

BIFRÖST needs the following signals:

| Signal       | Pins |
| --------     | ---- |
| ADDR 0..15   |  16  |
| ADDR 16..18  |   3  |
| CLOCK        |   1  |
| CLOCK SRC    |   1  |
| CPU BUSEN    |   1  |
| CPU IRQ      |   1  |
| CPU MLOCK    |   1  |
| CPU NMIRQ    |   1  |
| CPU READY    |   1  |
| CPU SETOV    |   1  |
| CPU SYNC     |   1  |
| CPU VECPULL  |   1  |
| DATA         |   8  |
| RESET        |   1  |
| RESET (inv)  |   1  |
| RW           |   1  |
| SPI MISO     |   1  |
| SPI MOSI     |   1  |
| SPI SCLK     |   1  |
| SPI SS       |   8  |
| SRAM CS      |   2  |
| UART CS      |   1  |
| UART IM      |   1  |
| UART IRQ     |   1  |
| UART RDN     |   1  |
| UART RXAIRQ  |   1  |
| UART RXBIRQ  |   1  |
| UART TXAIRQ  |   1  |
| UART TXBIRQ  |   1  |
| UART WRN     |   1  |
| VIA1 CS      |   1  |
| VIA1 IRQ     |   1  |
| VIA2 CS      |   1  |
| VIA2 IRQ     |   1  |

This brings the total I/O pin requirement to at least 64.

The ICE40HX1K-VQ100 FPGA lacks PLL which might be useful?
The TQ144 package includes PLL and has the same 0.5 mm pitch, so shouldn't be
materially harder to hand-solder.

RAM
---

Alliance Memory AS6C4008-55PCN 512K x 8 SRAM with 55ns access time at 2.7~5.5V
in a PDIP-32 package provides 512 KiB RAM.

This is 8 times larger than the 6502's 16-bit address space; access to RAM
beyond the first 64 KiB is managed by BIFRÖST.

(Earlier during system design, a pair of AS6C62256 32K x 8 PDIP-28 was chosen
to provide a less overkill and more era-appropriate 64 KiB of RAM, then lure of
1 MiB RAM made that 2 x 512 KiB, then the second 512 KiB was removed to make
PCB space for SID.


I/O
---

- SPI: 8 devices
- GPIO: 2 x 6522 VIA providing total of 4 x 8-bit ports
- Dual UART
- SID (ARMSID) sound

### SPI

BIFRÖST maps registers into the 6502 address space, implementing SPI master
similar to http://6502.org/users/andre/spi65b/index.html

This hardware SPI communication is orders of magnitude faster than 6522
bit-banging, making SPI display output more viable.

### GPIO

A pair of WDC W65C22S (6522) VIA provide two 8-bit GPIO ports each, as well as timers etc.

### UART

NXP SC28L92 3.3V dual UART (SC28L92A1A: PLC44-44 package)

Useful information on [UARTs: REPLACING THE 65C51 on 6502.org forums](http://forum.6502.org/viewtopic.php?f=4&t=4587).

### SID Sound

[ARMSID](https://www.nobomi.cz/8bit/armsid/index_en.php) emulates MOS6581 or MOS8580 SID.

I believe this will only function correctly when running at ~1 MHz.

An email reply from the creator re. voltage and clock:

> The ARMSID has its own LDO regulator, so it needs at least 3.45 V (max. 5.5 V) on pin 24 for proper operation.
> The value of the output signals is logic 3.3 V, the inputs are tolerant to 5 V. Therefore, it is not a problem to connect it to a 3.3 V system.
> The clock signal is designed for use in the C64, so it is guaranteed in the range of 0.985 to 1.023 MHz and directly controls the emulation rate (just like the original SID).
> Lower frequencies are not a problem, higher frequencies can disable emulation. All write and read operations should take at least 350ns for proper operation.



Power supply & reset
--------------------

TLV1117-33 LDO linear regulator brings 4.75V–15V down to the main 3.3V / 800mA
supply, used for all internal/external I/O.

LT3030 dual LDO linear regulator brings the 3.3V supply down to 2.5V / 750mA
and 1.2V / 250mA for the additional FPGA voltage requirements.

The In-Circuit System Programming (ICSP) header can power the AT25M01 SPI flash
EEPROM (1.7–5.5V) only, isolated from the rest of the system via 1N4148 Diode.

Maxim DS1818-5 “3.3V EconoReset with Pushbutton” handles holding RESET
active-low during power-on, brown-out, and when a RESET button is pressed.


Block diagram
-------------

![](docs/block.png)
