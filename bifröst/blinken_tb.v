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
    repeat(2_000_000) begin // 2M half-cycles -> 1 sec @ 1MHz
      clock = #5 ~clock; // 500ns high, 500ns low -> 1 MHz
    end

    $finish;
  end

endmodule
