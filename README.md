pda6502v2
=========

pda6502v2 is a single-board homebrew computer built around the 8-bit
[6502][6502] CPU, varitions of which powered the venerable [Commodore 64][c64],
[Apple II][appleii], [Nintendo Entertainment System][nes] and lots more.

This repository contains all aspects of its development:

| Directory             | Description                      | Language / Tool |
| --------------------- | -------------------------------- | --------------- |
| [`kicad/`](kicad)     | hardware schematic / PCB design  | KiCad           |
| [`bifröst/`](bifröst) | BIFRÖST FPGA system controller   | Verilog         |
| [`os/`](os)           | pda6502v2 firmware/OS/software   | 6502 assembly   |
| [`emu/`](emu)         | pda6502v2 emulator               | Rust            |
| [`eeprog/`](eeprog)   | SPI EEPROM in-circuit programmer | C, Arduino C++  |

Hardware specifications
-----------------------

- 6502 CPU ([WDC 65C02][W65C02])
- “BIFRÖST” FPGA system controller ([Lattice ICE40HX4K][ice40])
- 512 KiB static RAM
- 2 x [6522 VIA][W65C22]; 4 x 8-bit ports; 32 GPIO pins
- [SID][sid] sound generator ([ARMSID][armsid])
- SPI controller (BIFRÖST FPGA)
- UART ([NXP SC28L92 dual UART][nxpuart])


Compared to original pda6502
----------------------------

pda6502v2 is redesigned from scratch to address the pain points encountered
creating and programming the original [pda6502][pda6502]
several years earlier:

* Tedious software/bootloader development cycle, physically removing and slowly
  reprogramming an EEPROM on every software iteration.
* Limited, very slow SPI communications (bit-banged via 6522 GPIO) not viable
  for SPI displays etc.
* No UART for serial console or other communications with modern systems.
* Inflexible address bus mapping with hard-wired logic gates.

These issues and more have been improved:


|                       | pda6502             | pda6502v2                             |
| --------------------- | ------------------- | ------------------------------------- |
| Bootloader / firmware | EEPROM              | In-circuit programmable Flash EEPROM  |
| SPI                   | 6522 bit-bang       | FPGA hardware accelerated             |
| UART                  | *none*              | Dual UART (SC28L92)                   |
| GPIO                  | 6522 VIA            | Dual 6522 VIA                         |
| Sound                 | *none*              | ARMSID (SID emulation)                |
| RAM                   | 32 KiB              | 512 KiB (FPGA can bank-switch)        |
| Clock                 | 1 MHz oscillator    | FPGA generated; variable              |
| Address decode logic  | 7400-series chips   | FPGA programmable                     |
| Voltage               | 5.0V                | 3.3V                                  |
| Schematic/layout tool | EAGLE               | KiCad                                 |


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

| Signals                                                                       | Count   |
| ----------------------------------------------------------------------------- | ------- |
| Bus: `ADDR 0..15` + extended `ADDR 16..18`                                    | **19**  |
| SPI: `MISO`, `MOSI`, `SCLK`, `SS` x 8                                         | **11**  |
| 6502: `CLK` `RWB` `BE` `IRQB` `MLB` `NMIB` `RDY` `SOB` `SYNC` `VPB` `RESB`    | **10**  |
| UART: `CS`, `IM`, `IRQ`, `RDN`, `RXAIRQ`, `RXBIRQ`, `TXAIRQ`, `TXBIRQ`, `WRN` | **9**   |
| Bus: `DATA 0..7`                                                              | **8**   |
| VIA1 & VIA2: `CS`, `IRQ`                                                      | **4**   |
| SRAM `CS`                                                                     | **1**   |
| `CLOCKSRC` (in from oscillator)                                               | **1**   |
| `~RESET`                                                                      | **1**   |

This brings the total I/O pin requirement to at least 64.

The ICE40HX1K-VQ100 FPGA lacks PLL; PLL could be useful?

The TQ144 package includes PLL and has the same 0.5 mm pitch, so shouldn't be
materially harder to hand-solder.

The ICE40HX**4**K is bigger/better, with immaterial cost difference at this
volume.

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

[ARMSID][armsid] emulates MOS6581 or MOS8580 SID.

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


Memory map
----------

Memory mapping is controlled by BIFRÖST, so it may be reconfigured.
Currently, everything is RAM except 0xD000–DFFF.

- `0xD400` SID (nobomi ARMSID)
- `0xDC00` VIA1 (WDC 65C22)
- `0xDC10` VIA2 (WDC 65C22)
- `0xDC20` UART (NXP SC28L92)

This roughly matches Commodore 64's I/O space.

Errata
------

R26 (0 ohm) and R27 (open) labels are swapped.
The one closer to LT3030 vreg is R26 and should be 0 ohm (jumper/shorted).
The one further from the LT3030 vreg is R27 and should be open (not connected).

EEPROM triple drama:
- 128 KiB isn't nearly enough; iCEstick has 4 MiB.
- missing pull-up on SPI CS to put FPGA into SPI master mode.
- MISO/MOSI lines from FPGA to EEPROM are swapped :(
The best solution to this mess is to leave the EEPROM footprint unpopulated,
and attach a small board to ICSP header with:
- a larger EEPROM,
- a 10K pull-up resistor on CS,
- a MISO/MOSI-correct ICSP header.
A prototype of this works using a Winbond W25Q80BVAIG 8Mbit SPI EEPROM.
(Silver lining: attempts to program the EEPROM while it's connected to the FPGA
have failed, despite efforts to make the FPGA tristate those SPI lines when not
in use. Being able to disconnect the EEPROM from the FPGA is useful. However
one of the pda6502v2 goals was uploading new 6502 code without touching the
board. Hopefully this can be fixed in Verilog.

RESET lines for BIFRÖST FPGA and the 6502 system are coupled.  This is a
terrible idea; BIFRÖST needs to reset the CPU without resetting itself.
Workaround: use a knife to physically cut the RESET trace coming from top-right
pin of RESET switch, next to pin 1 of ARMSID, and then airwire from EXT[0] to
6502 RESET.


[6502]: http://en.wikipedia.org/wiki/MOS_Technology_6502
[W65C02]: http://en.wikipedia.org/wiki/WDC_65C02
[W65C22]: http://en.wikipedia.org/wiki/WDC_65C22
[appleii]: https://en.wikipedia.org/wiki/Apple_II
[armsid]: https://www.nobomi.cz/8bit/armsid/index_en.php
[c64]: https://en.wikipedia.org/wiki/Commodore_64
[ice40]: https://www.latticesemi.com/iCE40
[nes]: https://en.wikipedia.org/wiki/Nintendo_Entertainment_System
[nxpuart]: https://www.nxp.com/products/interfaces/uarts/3-3-v-5-0-v-dual-universal-asynchronous-receiver-transmitter-duart:SC28L92
[sid]: https://en.wikipedia.org/wiki/MOS_Technology_6581
[pda6502]: https://github.com/pda/pda6502
