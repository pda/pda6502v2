`include "boot.v"
`include "models/25AA512.v"

`timescale 1ns/1ns

module boot_tb;

reg clock = 0;

wire flash_si;
wire flash_so;
wire flash_sck; // rising:MOSI, falling:MISO
wire flash_cs_n;
reg flash_reset;

// `flash` is a Microchip 25AA512 512K SPI EEPROM,
// hopefully similar enough to W25Q80.
M25AA512 flash (
  .SI(flash_si),
  .SO(flash_so),
  .SCK(flash_sck),
  .CS_N(flash_cs_n),
  .WP_N(1'b1),
  .HOLD_N(1'b1),
  .RESET(flash_reset)
);

boot dut(
  .clock(clock),
  .flash_si(flash_si),
  .flash_so(flash_so),
  .flash_sck(flash_sck),
  .flash_cs_n(flash_cs_n)
);

// The 25AA512 model used in the testbench has 16-bit addressing,
// while the real AT25M01 / W25Q80 EEPROMs have 24-bit addressing.
defparam dut.EEPROM_ADDRESS_BITS = 16;

initial begin
  $dumpfile("boot_tb.vcd");
  $dumpvars(0, boot_tb);

  $display("hello boot");
  //$monitor("clock:%b", clock);
  $readmemh("models/data.txt", flash.MemoryBlock, 16'hE000, 16'hFFFF);

  flash.DeepPowerDown = 1;

  // 8 MHz @ timescale:1ns/1ns (62+63=125 ns period)
  repeat(10000) begin
    #62 clock = ~clock;
    #63 clock = ~clock;
  end

  $finish;
end

endmodule
