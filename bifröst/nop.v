`timescale 1ns/1ns

module nop(
  input clock,
  output wire clockout,

  output wire busen,
  output wire setov,
  output wire irq,
  output wire nmirq,
  output ready, // inout
  output reset6502,

  output wire ram_cs,
  output wire via1_cs,
  output wire via2_cs,
  output wire uart_cs,
  output wire sid_cs,

  inout [15:0] addr,
  inout [7:0] data,
  inout rw
);

reg data_en = 1'b1; // HIGH: drive data, LOW: tri-state Hi-Z
reg addr_en = 1'b0; // HIGH: drive addr, LOW: tri-state Hi-Z

// divide 8 MHz clock
reg [7:0] clock_divide = 8'b00000000;
reg [5:0] reset_counter = 6'b000000;
always @(posedge clock) begin
  clock_divide++;

  // increment reset_counter, leave an impossible value for addr_en/data_en.
  if (reset_counter < 6'b111110) reset_counter++;

  // this is inside an always block and predicated on an impossible
  // reset_counter value so that the synthesiser doesn't optimise away
  // the tri-state buffers (maybe?)
  addr_en <= (reset_counter > 6'b111111) ? 1'b1 : 1'b0; // LHS impossible
  data_en <= (reset_counter > 6'b111111) ? 1'b0 : 1'b1; // LHS impossible
end
assign clockout = clock_divide[2]; // 1 MHz
assign reset6502 = reset_counter[5]; // high until 2^5=64 @ 8MHz; 8 @ 1 MHz

// outputs to 6502
assign busen = 1'b1;
assign setov = 1'b1;
assign irq =   1'b1;
assign nmirq = 1'b1;
assign ready = 1'b1;

// chip selects
assign ram_cs =  1'b1;
assign via1_cs = 1'b1;
assign via2_cs = 1'b1;
assign uart_cs = 1'b1;
assign sid_cs =  1'b1;

// inouts to 6502
assign data = data_en ? 8'hEA : 8'bZZZZZZZZ;
assign addr = addr_en ? 16'b0000000000000000 : 16'bZZZZZZZZZZZZZZZZ;
assign rw = addr_en ? 1'b1 : 1'bZ;

endmodule
