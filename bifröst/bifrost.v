`include "blinken.v"
`include "adec.v"

`timescale 100ns/10ns

module bifrost(
  input clock,
  input flash_miso,
  input [18:0] addr,
  input [7:0] data,
  input rw,

  output wire clockout,
  output flash_mosi = 1'bZ,
  output flash_sck  = 1'bZ,
  output flash_cs   = 1'bZ,
  output [7:0] leds
);

// divide 8 MHz clock down to 1 MHz
reg [3:0] clock_divide = 0;
always @(posedge clock) begin
  clock_divide++;
end
assign clockout = clock_divide[3];

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
