`include "blinken.v"
`include "adec.v"
`include "boot.v"
`include "spi.v"

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
  output wire irq,
  input mlock,
  output nmirq,
  input sync,

  input via1_irq,
  input via2_irq,
  output wire via1_cs,
  output wire via2_cs,

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
  output wire uart_cs,
  output wire sid_cs,

  input spi_miso,
  output spi_mosi,
  output spi_clock,
  output [7:0] spi_cs,
  inout spi_exp0,
  inout spi_exp1,

  output wire [15:0] ext
);

// ext[15] is hijacked as 6502 reset line on the first
// revision of this board.
wire reset_fixup;
assign ext[15] = reset_fixup;

wire bifrost_cs;

wire [7:0] leds_blinken;
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
  .reset(reset_fixup),
  .booting(booting)
);

wire [7:0] spi_data_out;
wire spi_data_out_en;
spi spi(
  .clock_spi(clock),
  .clock_sys(clockout),
  .addr(addr[7:0]),
  .data(data),
  .rw(rw),
  .cs(bifrost_cs),
  .miso(spi_miso),
  .mosi(spi_mosi),
  .sck(spi_clock),
  .spi_cs(spi_cs),
  .data_out(spi_data_out),
  .data_out_en(spi_data_out_en)
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
assign nmirq = 1'b1;

// multiplex IRQ signals into 6502 IRQB.
assign irq = via1_irq & via2_irq & uart_irq;

assign ext[14:0] = 15'bZZZZZZZZZZZZZZZ; // ext[15] is 6502 reset_fixup

assign addr = booting ? addr_boot : 19'bZZZZZZZZZZZZZZZZZZZ;
assign rw = booting ? rw_boot : 1'bZ;
assign flash_mosi = booting ? flash_mosi_boot : 1'bZ;
assign flash_sck = booting ? flash_sck_boot : 1'bZ;
assign flash_cs = booting ? flash_cs_boot : 1'bZ;


reg [7:0] leds_reg = 8'b11000011;
reg [7:0] leds_src = 8'h00;
wire [7:0] blinken_irq = {
  via1_irq,
  via2_irq,
  uart_irq,
  uart_txairq,
  uart_rxairq,
  uart_txbirq,
  uart_rxbirq,
  irq
};
// write to BIFRÖST registers
always @(negedge clockout) begin
  if (~bifrost_cs && ~rw) begin
    case(addr[7:0])
      8'h00: leds_reg <= data;
      8'h01: leds_src <= data;
    endcase
  end
end
wire bifrost_reg_read = clockout && ~bifrost_cs && rw;
assign data = (booting || bifrost_reg_read) ? (
    booting ? data_boot :
    spi_data_out_en ? spi_data_out :
    addr[7:0] == 8'h00 ? leds_reg :
    addr[7:0] == 8'h01 ? leds_src :
    8'h00
  ) : 8'bZZZZZZZZ;

//assign leds = animating ? leds_blinken : leds_reg;
assign leds =
  animating ? leds_blinken :
  leds_src == 8'h00 ? leds_reg :
  leds_src == 8'h01 ? data :
  leds_src == 8'h02 ? addr[7:0] :
  leds_src == 8'h03 ? addr[15:8] :
  leds_src == 8'h04 ? blinken_irq :
  8'h00;

// UART
assign uart_rdn = ~(clockout & rw);
assign uart_wrn = ~(clockout & ~rw);
assign uart_im = 1'b1; // 80xxx/Intel mode
assign reset_inv = ~reset_fixup;

endmodule
