pda6502v2
=========

Version two of https://github.com/pda/pda6502, as a complete redesign.

Changes compared to version one:

|                       | v1                  | v2                             |
| --------------------- | ------------------- | ------------------------------ |
| Voltage               | 5.0V                | 3.3V                           |
| Address decode logic  | 7400-series chips   | CPLD                           |
| Schematic/layout tool | EAGLE               | KiCad                          |
| Boot                  | EEPROM              | microcontroller preloads SRAM  |
| Clock                 | 1 MHz osc           | microcontroller 0-10 MHz       |
| I/O                   | GPIO (2 x 8 bit)    | GPIO, UART, SPI                |
