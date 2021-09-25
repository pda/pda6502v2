eeprog
=======

pda6502v2 EEPROM programmer, in two parts:

- Arduino sketch to interact with EEPROM via SPI, developed/tested on Arduino Zero (3.3V ARM).
- Host computer program, to upload/download/verify data via Arduino USB serial port.

The pda6502 EEPROM stores:

- the FPGA configuration bitstream at 0x000000.
- 6502 boot code, copied to RAM at startup by the FPGA.

Notes on pda6502v2 mistakes, as described in the main project README Errata section:

The FPGA SPI data lines (MISO/MOSI) are swapped, so the on-board EEPROM can't
work, and the 128KB one specced is way too small for just the FPGA bitstream
alone.

Instead, this has been developed/tested with a breakout board:

- ICSP header with MISO/MOSI swapped, connected to pda6502v2 via 6-pin ribbon cable.
- Winbond W25Q80BV 1MiB EEPROM (note: >= 3.6V; not 5V tolerant)
- 1K pull-up from CS to VCC to tell the iCE40 FPGA to pull config via SPI.
- ICSP header to connect to Arduino
    - MISO, MOSI, SCK to Arduino ICSP header.
    - CS to Arduino pin 10.
    - 3.3v & GND to Arduino power pins.
- Additionally:
    - Arduino 5V and GND powering pda6502v2.
    - Arduino pin 9 connceted to pda6502v2 RESET.

Usage:

```
eeprog upload bifrost.bin /dev/cu.cu.usbmodem144102 0x000000
```

```
pda@paulbookpro ~/code/pda6502v2/eeprog $ ./eeprog verify ../bifröst/bifrost.bin /dev/cu.usbmodem144102 0
Verify ../bifröst/bifrost.bin ← /dev/cu.usbmodem144102 0x000000–0x020FBC (135100 bytes)
^C
EEPROM> reset hold
RESET is held LOW
EEPROM> read 0x000000 135100
Reading 135100 bytes from 0x00000000:
EEPROM> reset release
RESET is released to Hi-Z
✔ verify successful: ../bifröst/bifrost.bin
```


Connecting directly to the Arduino interface:

```sh
screen /dev/cu.usbmodem144102 115200
# or:
minicom --baudrate 115200 --device /dev/cu.usbmodem144102
```

```
SPI EEPROM programmer

EEPROM> help
Commands:
  help
  info
  hexdump [addr] [length]
  write [addr] [length]

EEPROM> info
manufacturer_id:0xEF device_id:0x13
jedec manufacturer_id:0xEF type:0x40 capacity:0x14
unique_id:0x6084A163AB6C2DC8

EEPROM> hexdump 0 32
00000000  FF 00 00 FF 7E AA 99 7E  51 00 01 05 92 00 20 62 |....~..~Q..... b|
00000010  03 67 72 01 10 82 00 00  11 00 01 01 00 00 00 00 |.gr.............|
00000030

EEPROM> write 0 14
[data entry not echoed]
erasing sector 0x00000000
writing from 0x00000000
bytes written: 14

EEPROM> hd 0 14
00000000  48 65 6C 6C 6F 2C 20 45  45 50 52 4F 4D 21       |Hello, EEPROM!|
00000010
```
