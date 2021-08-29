`include "boot.v"
`include "models/25AA512.v"

`timescale 1ns/1ns

module boot_tb;

reg clock = 0;

wire flash_si;
wire flash_so;
wire flash_sck; // rising:MOSI, falling:MISO
wire flash_cs_n;
wire flash_wp_n;
wire flash_hold_n;
reg flash_reset;

// `flash` is a Microchip 25AA512 512K SPI EEPROM,
// hopefully similar enough to AT25M01.
M25AA512 flash (
  .SI(flash_si),
  .SO(flash_so),
  .SCK(flash_sck),
  .CS_N(flash_cs_n),
  .WP_N(flash_wp_n),
  .HOLD_N(flash_hold_n),
  .RESET(flash_reset)
);

boot dut(
  .clock(clock),
  .flash_si(flash_si),
  .flash_so(flash_so),
  .flash_sck(flash_sck),
  .flash_cs_n(flash_cs_n),
  .flash_wp_n(flash_wp_n),
  .flash_hold_n(flash_hold_n)
);

initial begin
  $display("hello boot");
  $monitor("clock:%b", clock);
  $readmemh("models/data.txt", flash.MemoryBlock, 0, 3);

  repeat(10) begin
    clock = #1 ~clock;
  end

  $finish;
end

endmodule
