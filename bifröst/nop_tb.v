`include "nop.v"

`timescale 1ns/1ns

module nop_tb;

reg clock_tb = 1'b0;
wire [15:0] addr_tb;
wire [7:0] data_tb;
wire rw_tb = 1'b0;

nop dut(
  .clock(clock_tb),
  .addr(addr_tb),
  .data(data_tb),
  .rw(rw_tb)
);

// just run the clock for 1 second, monitoring changes.
initial begin
  $dumpfile("nop_tb.vcd");
  $dumpvars(0, nop_tb);

  // 8 MHz @ timescale:1ns/1ns (62+63=125 ns period)
  repeat(1_000) begin
    #62 clock_tb = ~clock_tb;
    #63 clock_tb = ~clock_tb;
  end

  $finish;
end

endmodule
