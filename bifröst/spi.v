`timescale 100ns/10ns

module spi(
  input clock_spi,
  input clock_sys,
  input [3:0] addr,
  input [7:0] data,
  input rw,
  input cs,
  input miso,
  output reg mosi = 1'b0,
  output reg sck = 1'b0,
  output reg [7:0] spi_cs = 8'b11111111,
  output [7:0] data_out,
  output data_out_en
);

reg [7:0] spi_buf = 8'h00;
reg [3:0] spi_bits = 4'd0;

always @(posedge clock_spi) begin
  if (spi_bits > 0) begin
    if (sck == 1) begin // negedge
      spi_buf[spi_bits-1] <= miso;
      spi_bits <= spi_bits-1;
    end
    sck <= ~sck;
  end

  // handle posedge clock_sys in this same process to avoid
  // multiple conflicting drivers for spi_buf and spi_bits.
  if (clock_sys) begin
    if (~cs && ~rw) begin
      case (addr)
        4'h0: begin
          spi_cs <= data;
        end
        4'h1: begin
          spi_buf <= data;
          spi_bits <= 4'd8;
        end
      endcase
    end
  end
end

always @(negedge clock_spi) begin
  if (spi_bits > 0 && sck == 0) begin
    mosi <= spi_buf[spi_bits-1]; // prep for MOSI on rising clock
  end
end

assign data_out =
  addr == 4'h0 ? spi_cs :
  addr == 4'h1 ? spi_buf :
  8'h00;

assign data_out_en = clock_sys && ~cs && rw;

endmodule
