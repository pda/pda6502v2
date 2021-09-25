`include "blinken.v"

`timescale 100ns/10ns

module blinken_tb;

  reg clock = 0;

  blinken dut(
    .clock(clock)
  );

  // just run the clock for 1 second, monitoring changes.
  initial begin
    //$dumpfile("blinken_tb.vcd");
    //$dumpvars(0, blinken_tb);
    $monitor("[%10t] animating:%b leds:%b", $time, dut.animating, dut.leds);
    repeat(40_000_000) begin
      clock = #5 ~clock;
    end

    $finish;
  end

endmodule
