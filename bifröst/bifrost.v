`include "blinken.v"
`include "adec.v"

`timescale 100ns/10ns

module bifrost(
  input clock,
  input flash_miso,
  input [18:0] addr,
  input [7:0] data,
  input rw,

  output flash_mosi = 1'bZ,
  output flash_sck  = 1'bZ,
  output flash_cs   = 1'bZ,
  output [7:0] leds
);

blinken blinken(
  .clock(clock),
  .leds(leds)
);

adec adec(
  .clock(clock),
  .addr(addr),
  .rw(rw)
);

endmodule
