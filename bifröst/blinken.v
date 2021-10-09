`timescale 100ns/10ns

module blinken(
  input wire clock,
  output reg [7:0] leds = 8'b0,
  output reg animating = 1
);

// on start-up, animate the LEDs in
reg [18:0] counter = 0;
always @(posedge clock) begin
  if (animating == 1) begin
    counter <= counter + 1;
    if (counter[16] == 1'b1) begin
      counter <= 0;
      if (leds == 8'b00000000) leds <= 8'b10000000;
      else leds <= leds>>1;
      if (leds == 8'b00000001) animating <= 1'b0;
    end
  end
end

endmodule
