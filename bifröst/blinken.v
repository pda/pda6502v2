`timescale 100ns/10ns

module blinken(
  input wire clock,
  output reg [7:0] leds = 8'b0
);

// on start-up, animate the LEDs in
reg animating = 1;
reg [18:0] counter = 0; // 19-bit counter to reach 500,000
always @(posedge clock) begin
  if (animating == 1) begin
    counter = counter + 1;
    if (counter == 500_000) begin
      counter <= 0;
      if (leds[0] == 0) begin
        leds <= (leds>>1) | (1<<7);
      end
      else begin
        leds <= (leds>>1);
      end
      if (leds == 8'b00000001) begin
        // after all LEDs have been on for 100ms, clear them and halt
        animating = 0;
        leds <= 0;
      end
    end
  end
end

endmodule
