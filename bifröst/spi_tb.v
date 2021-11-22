`include "spi.v"

`timescale 100ns/10ns

module spi_tb;

  reg clock_spi = 0;
  wire clock_sys;
  reg [7:0] addr = 8'hXX;
  reg [7:0] data = 8'hZZ;
  reg rw = 1'b1;
  reg cs = 1'b1;
  reg miso = 1'b1;
  wire mosi;
  wire sck;
  wire [7:0] spi_cs;

  spi dut(
    .clock_spi(clock_spi),
    .clock_sys(clock_sys),
    .addr(addr),
    .data(data),
    .rw(rw),
    .cs(cs),
    .miso(miso),
    .mosi(mosi),
    .sck(sck),
    .spi_cs(spi_cs)
  );

  // simulate SPI clock running full speed (e.g. 8 MHz)
  // and system (6502) clock running slower (e.g. 4 MHz)
  reg [1:0] clockdiv = 2'b00;
  always @(clock_spi) clockdiv++;
  assign clock_sys = clockdiv[1];

  initial begin
    $dumpfile("spi_tb.vcd");
    $dumpvars(0, spi_tb);

    #5 clock_spi = 1;
    repeat (2) begin
      #5 clock_spi = 0;
      #5 clock_spi = 1;
    end

    // ----------------------------------------
    // 6502 read SPI CS (GPIO) register

    #2
    cs = 0;             // select SPI module (simulate address decoder)
    addr = 8'h10;       // CS/GPIO register address within SPI module
    rw = 1;             // 6502 reading this register

    // TODO: expect CS initial state 8'b11111111 on data_out with data_out_en

    // one more clock_spi to trigger clock_sys falling edge
    #3 clock_spi = 0;
    #5 clock_spi = 1;

    // 6502 read cycle completes
    #3
    cs = 1;             // deselect SPI module (simulate address decoder)
    addr = 8'hXX;       // addr bus no longer relevant

    // another clock_spi cycle to return clock_sys high
    #2 clock_spi = 0;
    #5 clock_spi = 1;

    // ----------------------------------------
    // 6502 write to SPI CS (GPIO) register

    #2
    cs = 0;             // select SPI module (simulate address decoder)
    addr = 8'h10;       // register address within SPI module
    data = 8'b11111110; // CS/GPIO pin state
    rw = 0;             // 6502 writing to this register

    // one more clock_spi to trigger clock_sys falling edge
    #3 clock_spi = 0;
    #5 clock_spi = 1;

    // 6502 write cycle completes
    #3
    cs = 1;             // deselect SPI module (simulate address decoder)
    addr = 8'hXX;       // addr bus no longer relevant
    data = 8'hZZ;       // data bus no longer relevant
    rw = 1;             // stop writing

    // another clock_spi cycle to return clock_sys high
    #2 clock_spi = 0;
    #5 clock_spi = 1;

    // ----------------------------------------
    // 6502 writes to SPI data register

    #2
    cs = 0;             // select SPI module (simulate address decoder)
    addr = 8'h11;       // register address within SPI module
    data = 8'b11011011; // an SPI byte to send
    rw = 0;             // 6502 writing to this register

    // one more clock_spi to trigger clock_sys falling edge
    #3 clock_spi = 0;
    #5 clock_spi = 1;

    // 6502 write cycle completes
    #3
    cs = 1;             // deselect SPI module (simulate address decoder)
    addr = 8'hXX;       // addr bus no longer relevant
    data = 8'hZZ;       // data bus no longer relevant
    rw = 1;             // stop writing

    // another clock_spi cycle to return clock_sys high
    #2 clock_spi = 0;
    #5 clock_spi = 1;

    miso = 0; // prep to send 8'b10101010 (8'hAA) over MISO
    // clock the data out
    repeat(8) begin
      #5 clock_spi = 0;
      miso = ~miso;
      #5 clock_spi = 1;
      #5 clock_spi = 0;
      #5 clock_spi = 1;
    end

    // ----------------------------------------
    // 6502 read SPI CS (GPIO) register

    #2
    cs = 0;             // select SPI module (simulate address decoder)
    addr = 8'h10;       // CS/GPIO register address within SPI module
    rw = 1;             // 6502 reading this register

    // TODO: expect CS initial state 8'b11111111 on data_out with data_out_en

    // one more clock_spi to trigger clock_sys falling edge
    #3 clock_spi = 0;
    #5 clock_spi = 1;

    // 6502 read cycle completes
    #3
    cs = 1;             // deselect SPI module (simulate address decoder)
    addr = 8'hXX;       // addr bus no longer relevant

    // another clock_spi cycle to return clock_sys high
    #2 clock_spi = 0;
    #5 clock_spi = 1;

    // ----------------------------------------
    // 6502 write to SPI CS (GPIO) register

    #2
    cs = 0;             // select SPI module (simulate address decoder)
    addr = 8'h10;       // register address within SPI module
    data = 8'b11111111; // CS/GPIO pin state
    rw = 0;             // 6502 writing to this register

    // one more clock_spi to trigger clock_sys falling edge
    #3 clock_spi = 0;
    #5 clock_spi = 1;

    // 6502 write cycle completes
    #3
    cs = 1;             // deselect SPI module (simulate address decoder)
    addr = 8'hXX;       // addr bus no longer relevant
    data = 8'hZZ;       // data bus no longer relevant
    rw = 1;             // stop writing

    // another clock_spi cycle to return clock_sys high
    #2 clock_spi = 0;
    #5 clock_spi = 1;


    // ----------------------------------------

    repeat(2) begin
      #5 clock_spi = 0;
      #5 clock_spi = 1;
    end

    $finish;
  end

endmodule
