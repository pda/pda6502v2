`include "blinken.v"

`timescale 100ns/10ns

module bifrost(
  input clock,
  input flash_miso,
  output flash_mosi = 1'bZ,
  output flash_sck  = 1'bZ,
  output flash_cs   = 1'bZ,
  output [7:0] leds
);

blinken blinken(
  .clock(clock),
  .leds(leds)
);

endmodule
