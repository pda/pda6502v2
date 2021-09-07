`timescale 100ns/10ns

module blinken(
  input wire clock,
  output reg [7:0] leds = 0
);

// on start-up, animate the LEDs in
reg animating = 1;
reg [20:0] counter = 0; // 21-bit counter to reach 1,200,000 (for 12MHz)
always @(posedge clock) begin
  if (animating == 1) begin
    counter = counter + 1;
    if (counter == 2_000_000) begin
      counter <= 0;
      leds <= (leds << 1) | 1;
      if (leds == 8'h00) begin
        // after all LEDs have been on for 100ms, clear them and halt
        //animating = 0;
        leds <= 0;
      end
    end
  end
end

endmodule
