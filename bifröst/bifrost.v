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
  inout ready,
  output irq,
  input mlock,
  output nmirq,
  input sync,
  input reset, // hrmm

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
  output wire sid_cs
);

blinken blinken(
  .clock(clock),
  .leds(leds)
);

adec adec(
  .clock(clock),
  .addr(addr),
  .rw(rw),
  .ram_cs(ram_cs),
  .via1_cs(via1_cs),
  .via2_cs(via2_cs),
  .uart_cs(uart_cs),
  .sid_cs(sid_cs)
);


wire booting;
boot boot(
  .clock(clock),
  .flash_so(flash_miso),
  .flash_si(flash_mosi),
  .flash_sck(flash_sck),
  .flash_cs_n(flash_cs),
  .busen(busen),
  .address(addr),
  .data(data),
  .rw(rw),
  .booting(booting)
);

// divide 8 MHz clock down to 1 MHz
reg [7:0] clock_divide = 8'b11111111;
always @(posedge clock) begin
  if (!booting) clock_divide++;
end
assign clockout = clock_divide[7]; // 7: 31,250 Hz (31 kHz)

//assign reset_inv = !reset;


assign setov = 1'b1;
assign ready = 1'b1;
assign reset_inv = !reset;
assign irq = 1'b1;
assign nmirq = 1'b1;
assign uart_rdn = 1'b1;
assign uart_wrn = 1'b1;
assign uart_im = 1'b1; // 80xxx/Intel mode

endmodule
