`timescale 100ns/10ns

module adec(
  input clock,
  input [18:0] addr,
  input rw,

  output wire ram_cs,
  output wire bifrost_cs,
  output wire via1_cs,
  output wire via2_cs,
  output wire uart_cs,
  output wire sid_cs
);

  assign ram_cs     = ~(clock && (addr <  16'hD000 || addr >  16'hDFFF));

  // C64 0xD000–D3FF: VIC-II

  // SID: 5-bit; 32 registers.  The C64 repeats them 32 times from 0xD400
  // onwards, presumably to simplify address decoding. We'll do the same.
  assign sid_cs     = ~(addr >= 16'hD400 && addr <= 16'hD7FF); // 5-bit; 32 registers

  // C64 0xD800–DBFF: color RAM

  // C64 0xDC00–DCFF: CIA#1; inputs (keyboard, joystick, mouse), datasette, IRQ control
  // C64 0xDD00–DDFF: CIA#2; serial bus, RS232, NMI control
  assign via1_cs    = ~(addr >= 16'hDC00 && addr <= 16'hDC0F); // 4-bit; 16 registers
  assign via2_cs    = ~(addr >= 16'hDC10 && addr <= 16'hDC1F); // 4-bit; 16 registers
  assign uart_cs    = ~(addr >= 16'hDC20 && addr <= 16'hDC2F); // 4-bit; 16 registers

  // C64 0xDE00–DEFF: I/O Area #1 (external/optional)
  // C64 0xDF00–DFFF: I/O Area #2 (external/optional)
  assign bifrost_cs = ~(addr >= 16'hDE00 && addr <= 16'hDEFF); // 8-bit; 255 registers

endmodule
