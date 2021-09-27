`include "bifrost.v"

`timescale 100ns/10ns

module bifrost_tb;

  reg clock = 0;
  reg flash_miso = 1'bZ;
  reg [18:0] addr = 16'h0000;
  reg [7:0] data = 8'h00;
  reg rw = 1;

  bifrost dut(
    .clock(clock),
    .flash_miso(flash_miso),
    .addr(addr),
    .data(data),
    .rw(rw)
  );

  // just run the clock for 1 second, monitoring changes.
  initial begin
    $dumpfile("bifrost_tb.vcd");
    $dumpvars(0, bifrost_tb);
    repeat(2_000_000) begin // 2M half-cycles -> 1 sec @ 1MHz
      clock = #5 ~clock; // 500ns high, 500ns low -> 1 MHz
      addr++;
    end

    $finish;
  end

endmodule
