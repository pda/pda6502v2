`timescale 1ns/100ps

// module boot reads data from SPI EEPROM and writes it to RAM.
module boot(
  input clock,
  input flash_so,

  output flash_si = 0,
  output flash_sck = 0,
  output flash_cs_n = 1,
  output flash_wp_n = 1,
  output flash_hold_n = 1,

  output reg [18:0] address,
  output reg [7:0] data,
  output reg rw,
  output busen = 1,  // HIGH = 6502 bus output disabled

  output clock_stop = 0 // LOW = stop
);

// The 25AA512 model used in the testbench has 16-bit addressing,
// while the real AT25M01 / W25Q80 EEPROMs have 24-bit addressing.
parameter EEPROM_ADDRESS_BITS = 24;

reg [7:0] phase = 0;
reg [7:0] step = 0;
reg [10:0] offset = 0;

reg [31:0] spi_buffer;
reg [7:0] spi_bits = 0;

// Hold BE (Bus Enable) low.
// SPI shift out power up cmd, read cmd, address (0x080000).
// (use 0x040000 for now becaue the test model is only 512 KiB)
// SPI shift in 2048 bytes, write to RAM from 0x0E000.
// Release BE (Bus Enable) back to high.
// Start 6502 clock
//
//   BE:0
//   CS:0
//   shift out: 0xAB (POWER ON)
//   CS:1
//   CS:0
//   shift out READ: 0x03
//   shift out ADDR: 0x04 0x00 0x00
//   for i=0; i<2048; i++
//     shift in byte
//     ADDR:0x0E000+i
//     DATA:(byte read from EEPROM)
//     RW:0
//     RW:1
//   CS:1
//   BE:1

always @(posedge clock) begin
  case(phase)
    0: begin // initial phase
      phase <= phase+1;
    end
    1: begin // disable bus
      busen <= 0;
      phase <= phase+1;
    end
    2: begin // power on
      case(step)
        0: begin
          flash_cs_n <= 0;
          step <= step+1;
        end
        1: begin
          spi_buffer <= 8'hAB; // "Release Power Down / Device ID"
          spi_bits <= 8;
          step <= step+1;
        end
        2: begin // wait for SPI to complete
          if (spi_bits == 0) step <= step+1;
        end
        3: begin
          flash_cs_n <= 1;
          step <= step+1;
        end
        4: begin
          // wait 100 µS (800 cycles @ 8 MHz) for 25AA512 tREL
          // (W25Q80 tRES2 is only 3 µS, but the testbench uses 25AA512)
          // (using the offset register, it's otherwise unused in this phase)
          offset <= offset+1;
          if (offset >= 800) begin
            offset <= 0;
            step <= step+1;
          end
        end
        5: begin
          phase <= phase+1;
          step <= 0;
        end
      endcase
    end
    3: begin // read addr
      case(step)
        0: begin
          flash_cs_n <= 0;
          step <= step+1;
        end
        1: begin
          if (EEPROM_ADDRESS_BITS == 24) begin
            spi_buffer[31:24] <= 8'h03; // "Read Data from Memory”
            spi_buffer[23:00] <= 24'h080000; // from 0x80000 (512 KiB offset)
            spi_bits <= 32;
          end
          else if (EEPROM_ADDRESS_BITS == 16) begin
            spi_buffer[23:16] <= 8'h03; // "Read Data from Memory”
            spi_buffer[15:00] <= 16'hE000; // from 0xE000
            spi_bits <= 24;
          end
          step <= step+1;
        end
        2: begin // wait for SPI to complete
          if (spi_bits == 0) step <= step+1;
        end
        3: begin
          phase <= phase+1;
          step <= 0;
          offset <= 0;
        end
      endcase
    end
    4: begin // read data
      case(step)
        0: begin
          spi_buffer <= 8'h00;
          spi_bits <= 8;
          step <= step+1;
        end
        1: begin
          if (spi_bits == 0) step <= step+1;
        end
        2: begin
          address <= 16'hE000 + offset;
          data <= spi_buffer;
          rw <= 1;
          step <= step+1;
        end
        3: begin
          step <= 0;
          if (offset == 2048) begin
            phase <= phase+1;
          end
          offset <= offset+1;
        end
      endcase
    end
    5: begin // read finish
      phase <= phase+1;
    end
    6: begin // enable bus
      busen <= 1;
      phase <= phase+1;
    end
    7: begin // start 6502
      clock_stop <= 1;
      phase <= phase+1;
    end
    7: begin // END
    end
  endcase
end

always @(negedge clock) begin
  if (spi_bits > 0) begin
    if (flash_sck == 0) begin // prep for MOSI on rising clock
      flash_si <= (spi_buffer[spi_bits-1]);
    end
  end
end

always @(posedge clock) begin
  if (spi_bits > 0) begin
    flash_sck <= ~flash_sck;
  end
end

always @(negedge flash_sck) begin
  spi_buffer[spi_bits-1] <= flash_so;
  spi_bits <= spi_bits-1;
end

endmodule
