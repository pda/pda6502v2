`include "blinken.v"

`timescale 100ns/10ns

module blinken_tb;

  reg clock = 0;

  blinken dut(
    .clock(clock)
  );

  initial begin
    $dumpfile("blinken_tb.vcd");
    $dumpvars(0, blinken_tb);
    $monitor("[%10t] splashing:%b leds:%b", $time, dut.splashing, dut.leds);
  end

  initial begin
    repeat(2_000_000) begin // 2M half-cycles -> 1 sec @ 1MHz
      #5 // 500ns high, 500ns low -> 1 MHz
      clock = ~clock;
    end

    $finish;
  end

endmodule
