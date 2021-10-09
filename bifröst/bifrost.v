`include "blinken.v"
`include "adec.v"
`include "boot.v"

`timescale 100ns/10ns

module bifrost(
  input clock,
  input flash_miso,
  inout [18:0] addr,
  inout [7:0] data,
  inout rw,

  output wire busen,
  output setov,
  input vecpull,
  output ready, // inout really
  output irq,
  input mlock,
  output nmirq,
  input sync,

  input uart_irq,
  output uart_im,
  output uart_rdn,
  output uart_wrn,
  input uart_txbirq,
  input uart_rxbirq,
  input uart_txairq,
  input uart_rxairq,

  output wire reset_inv,
  output wire clockout,
  output wire flash_mosi,
  output wire flash_sck,
  output wire flash_cs,
  output wire [7:0] leds,
  output wire ram_cs,
  output wire via1_cs,
  output wire via2_cs,
  output wire uart_cs,
  output wire sid_cs,

  output wire [15:0] ext
);

wire bifrost_cs;

reg [7:0] leds_blinken;
wire animating;
blinken blinken(
  .clock(clockout),
  .leds(leds_blinken),
  .animating(animating)
);

adec adec(
  .clock(clockout),
  .addr(addr),
  .rw(rw),
  .ram_cs(ram_cs),
  .via1_cs(via1_cs),
  .via2_cs(via2_cs),
  .uart_cs(uart_cs),
  .sid_cs(sid_cs),
  .bifrost_cs(bifrost_cs)
);


wire booting;
wire [18:0] addr_boot;
wire [7:0] data_boot;
wire rw_boot;
wire flash_mosi_boot;
wire flash_sck_boot;
wire flash_cs_boot;
boot boot(
  .clock(clockout),
  .flash_so(flash_miso),
  .flash_si(flash_mosi_boot),
  .flash_sck(flash_sck_boot),
  .flash_cs_n(flash_cs_boot),
  .busen(busen),
  .address(addr_boot),
  .data(data_boot),
  .rw(rw_boot),
  .reset(ext[0]),
  .booting(booting)
);

// divide 8 MHz clock down to 1 MHz
reg [8:0] clock_divide = 9'b000000000;
always @(posedge clock) clock_divide++;
// 0: 4,000,000 Hz
// 1: 2,000,000 Hz
// 2: 1,000,000 Hz
// 3:   500,000 Hz
// 4:   250,000 Hz
// 5:   125,000 Hz
// 6:    62,500 Hz
// 7:    31,250 Hz
// 8:    15,625 Hz
assign clockout = clock_divide[2];

assign setov = 1'b1;
assign ready = 1'b1;
assign irq = 1'b1;
assign nmirq = 1'b1;
assign uart_rdn = 1'b1;
assign uart_wrn = 1'b1;
assign uart_im = 1'b1; // 80xxx/Intel mode
assign reset_inv = ~ext[0];

assign ext[15:1] = 15'bZZZZZZZZZZZZZZZ; // ext[0] is 6502 RESET hack

assign addr = booting ? addr_boot : 19'bZZZZZZZZZZZZZZZZZZZ;
assign data = booting ? data_boot : 8'bZZZZZZZZ;
assign rw = booting ? rw_boot : 1'bZ;
assign flash_mosi = booting ? flash_mosi_boot : 1'bZ;
assign flash_sck = booting ? flash_sck_boot : 1'bZ;
assign flash_cs = booting ? flash_cs_boot : 1'bZ;

reg [7:0] leds_reg = 8'b11000011;
always @(posedge clock) begin
  if (~bifrost_cs && ~rw && addr[7:0] == 8'h00) begin
    leds_reg <= data;
  end
end
assign leds = animating ? leds_blinken : leds_reg;

endmodule
