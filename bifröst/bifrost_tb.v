`include "bifrost.v"

`timescale 1ns/1ns

module bifrost_tb;

reg clock = 0;
reg flash_miso = 1'bZ;
wire [18:0] addr = 19'hCAFE;
wire [7:0] data = 8'h42;
wire rw;

bifrost dut(
  .clock(clock),
  .flash_miso(flash_miso),
  .addr(addr),
  .data(data),
  .rw(rw),
  .vecpull(1'b1),
  .mlock(1'b1),
  .sync(1'b1),
  .uart_irq(1'b1),
  .uart_txbirq(1'b1),
  .uart_rxbirq(1'b1),
  .uart_txairq(1'b1),
  .uart_rxairq(1'b1)
);

// just run the clock for 1 second, monitoring changes.
initial begin
  $dumpfile("bifrost_tb.vcd");
  $dumpvars(0, bifrost_tb);

  // 8 MHz @ timescale:1ns/1ns (62+63=125 ns period)
  repeat(1_500_000) begin
    #62 clock = ~clock;
    #63 clock = ~clock;
  end

  $finish;
end

endmodule
