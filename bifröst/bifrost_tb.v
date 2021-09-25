`include "bifrost.v"

`timescale 100ns/10ns

module bifrost_tb;

  reg clock = 0;
  reg flash_miso = 1'bZ;

  bifrost dut(
    .clock(clock),
    .flash_miso(flash_miso)
  );

  // just run the clock for 1 second, monitoring changes.
  initial begin
    $dumpfile("bifrost_tb.vcd");
    $dumpvars(0, bifrost_tb);
    //$monitor("[%10t] animating:%b leds:%b", $time, dut.animating, dut.leds);
    repeat(2_000_000) begin // 2M half-cycles -> 1 sec @ 1MHz
      clock = #5 ~clock; // 500ns high, 500ns low -> 1 MHz
    end

    $finish;
  end

endmodule
