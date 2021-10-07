`timescale 1ns/100ps

// module boot reads data from SPI EEPROM and writes it to RAM.
module boot(
  input clock,
  input flash_so,

  output reg flash_si = 1'b0,
  output reg flash_sck = 1'b0,
  output reg flash_cs_n = 1'b1,

  output reg [18:0] address = 19'b0,
  output reg [7:0] data = 8'b0,
  output reg rw = 1'b1,
  output reg busen = 1'b1,  // HIGH = 6502 bus output enabled
  output reg reset = 1'b1,

  output reg booting = 1'b1
);

// The 25AA512 model used in the testbench has 16-bit addressing,
// while the real AT25M01 / W25Q80 EEPROMs have 24-bit addressing.
parameter EEPROM_ADDRESS_BITS = 24;

reg [3:0] state = 0;
reg [15:0] offset = 0;
reg [31:0] spi_buffer;
reg [7:0] spi_bits = 0;

// states
localparam s_cpu_disable       =  0; // stop 6502 clock, tell it to release bus
localparam s_eeprom_power      =  1; // EEPROM power-up: prepare command
localparam s_eeprom_power_send =  2; //                  SPI send
localparam s_eeprom_power_wait =  3; //                  wait specified time for power-up
localparam s_eeprom_read       =  4; // EEPROM read: prepare cmd & addr
localparam s_eeprom_read_send  =  5; //              wait for SPI send, then trigger first byte read
localparam s_ram_write         =  6; //              read SPI data, write to RAM
localparam s_ram_write_finish  =  7; //              finish write to RAM, loop to s_ram_write until all bytes done
localparam s_cleanup           =  8; // Set everything back to a safe state.
localparam s_reset             =  9; // Reset the 6502 system
localparam s_done              = 10; // Terminal no-op.

always @(posedge clock) begin
  if (spi_bits > 0) begin
    flash_sck <= ~flash_sck;
    if (flash_sck == 1) begin // negedge
      spi_buffer[spi_bits-1] <= flash_so;
      spi_bits <= spi_bits-1;
    end
  end
  else begin
    case(state)
      s_cpu_disable: begin // initial state: disable CPU
        busen <= 0;
        state <= s_eeprom_power;
      end
      s_eeprom_power: begin
        flash_cs_n <= 0;
        spi_buffer <= 8'hAB; // "Release Power Down / Device ID"
        spi_bits <= 8;
        state <= s_eeprom_power_send;
      end
      s_eeprom_power_send: begin
        if (spi_bits == 0) begin
          flash_cs_n <= 1;
          state <= s_eeprom_power_wait;
        end
      end
      s_eeprom_power_wait: begin
        // wait 100 µS (800 cycles @ 8 MHz) for 25AA512 tREL
        // (W25Q80 tRES2 is only 3 µS, but the testbench uses 25AA512)
        // (using the offset register, it's otherwise unused in this state)
        offset <= offset+1;
        if (offset >= 800) begin
          offset <= 0;
          state <= s_eeprom_read;
        end
      end
      s_eeprom_read: begin
        flash_cs_n <= 0;
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
        state <= s_eeprom_read_send;
      end
      s_eeprom_read_send: begin
        if (spi_bits == 0) begin
          offset <= 0;
          spi_bits <= 8; // start reading a byte
          state <= s_ram_write;
        end
      end
      s_ram_write: begin // read data
        if (spi_bits == 0) begin
          address <= 16'hE000 + offset;
          data <= spi_buffer;
          rw <= 1'b0;
          state <= s_ram_write_finish;
        end
      end
      s_ram_write_finish: begin
        rw <= 1'b1;
        if (offset < 16'h1FFF) begin
          spi_bits <= 8; // start reading next byte
          state <= s_ram_write;
          offset <= offset+1;
        end
        else begin
          offset <= 0;
          state <= s_cleanup;
        end
      end
      s_cleanup: begin // read finish
        // TODO: put EEPROM back into power-down state?
        booting <= 0;
        flash_sck <= 1'bZ;
        flash_cs_n <= 1'bZ;
        address <= 19'bZZZZZZZZZZZZZZZZZZ;
        data <= 8'bZZZZZZZZ;
        rw <= 1'bZ;
        busen <= 1;
        state <= s_reset;
      end
      s_reset: begin
        // "The RESB signal must be held low for at least two clock cycles"
        if (offset == 0) reset <= 0;
        if (offset < 4) offset <= offset+1;
        else begin
          reset <= 1;
          offset <= 0;
          state <= s_done;
        end
      end
      s_done: begin
        // nothing
      end
    endcase
  end
end

always @(negedge clock) begin
  if (booting) begin
    if (spi_bits > 0 && flash_sck == 0) begin
      flash_si <= (spi_buffer[spi_bits-1]); // prep for MOSI on rising clock
    end
  end
  else flash_si <= 1'bZ;
end

endmodule
