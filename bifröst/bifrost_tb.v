`include "bifrost.v"

`timescale 1ns/1ns

module bifrost_tb;

reg clock = 0;
reg flash_miso = 1'bZ;
wire [18:0] addr;
wire [7:0] data;
wire rw;

wire vecpull = 1'b1;
wire mlock = 1'b1;
wire sync = 1'b1;
wire reset = 1'b1;
wire uart_irq = 1'b1;
wire uart_txbirq = 1'b1;
wire uart_rxbirq = 1'b1;
wire uart_txairq = 1'b1;
wire uart_rxairq = 1'b1;

bifrost dut(
  .clock(clock),
  .flash_miso(flash_miso),
  .addr(addr),
  .data(data),
  .rw(rw),
  .vecpull(vecpull),
  .mlock(mlock),
  .sync(sync),
  .reset(reset),
  .uart_irq(uart_irq),
  .uart_txbirq(uart_txbirq),
  .uart_rxbirq(uart_rxbirq),
  .uart_txairq(uart_txairq),
  .uart_rxairq(uart_rxairq)
);

// just run the clock for 1 second, monitoring changes.
initial begin
  $dumpfile("bifrost_tb.vcd");
  $dumpvars(0, bifrost_tb);

  // 8 MHz @ timescale:1ns/1ns (62+63=125 ns period)
  repeat(150_000) begin
    #62 clock = ~clock;
    #63 clock = ~clock;
  end

  $finish;
end

endmodule
