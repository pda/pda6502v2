`include "bifrost.v"

`timescale 1ns/1ns

module bifrost_tb;

reg clock = 0;
reg flash_miso = 1'bZ;
wire [18:0] addr;
wire [7:0] data;
wire rw;
reg via1_irq = 1'b1;
reg via2_irq = 1'b1;
reg spi_miso = 1'b1;

reg [18:0] addr6502 = 19'hCAFE;
reg [7:0] data6502 = 8'h42;
reg rw6502 = 1'b1;
assign addr = dut.booting ? 19'hZZZ : addr6502;
assign data = dut.booting ? 8'hZZ   : data6502;
assign rw   = dut.booting ? 1'bZ    : rw6502;

bifrost dut(
  .clock(clock),
  .flash_miso(flash_miso),
  .addr(addr),
  .data(data),
  .rw(rw),
  .vecpull(1'b1),
  .mlock(1'b1),
  .sync(1'b1),
  .via1_irq(via1_irq),
  .via2_irq(via2_irq),
  .uart_irq(1'b1),
  .uart_txbirq(1'b1),
  .uart_rxbirq(1'b1),
  .uart_txairq(1'b1),
  .uart_rxairq(1'b1),
  .spi_miso(spi_miso)
);

initial begin
  $dumpfile("bifrost_tb.vcd");
  $dumpvars(0, bifrost_tb);

  // 8 MHz @ timescale:1ns/1ns (62+63=125 ns period)
  repeat(200_000) begin
    #62 clock = ~clock;
    #63 clock = ~clock;
    via1_irq = ~via1_irq;
  end

  // get the 6502 clock high
  #62 clock = ~clock;
  #63 clock = ~clock;

  addr6502 = 19'h0DE11;
  data6502 = 8'b10101010;
  rw6502 = 1'b0;

  #62 clock = ~clock;
  #63 clock = ~clock;

  addr6502 = 19'h00000;
  data6502 = 8'bZZZZZZZZ;
  rw6502 = 1'b1;

  spi_miso = 1; // prep to send 8'b01010101 (8'h55) over MISO
  // clock the data out
  repeat(8) begin
    #62 clock = 0;
    spi_miso = ~spi_miso;
    #63 clock = 1;
    #62 clock = 0;
    #63 clock = 1;
  end

  addr6502 = 19'h0DE11;
  #62 clock = ~clock;
  #63 clock = ~clock;
  #62 clock = ~clock;
  #63 clock = ~clock;

  $finish;
end

endmodule
