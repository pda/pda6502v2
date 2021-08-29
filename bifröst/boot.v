`timescale 1ns/1ns

// module boot reads data from SPI EEPROM and writes it to RAM.
module boot(
  input wire clock,

  output reg flash_si = 0,
  input wire flash_so,
  output reg flash_sck = 0,
  output reg flash_cs_n = 1,
  output reg flash_wp_n = 1,
  output reg flash_hold_n = 1

  // TODO: RAM connections
);

endmodule
