`include "adec.v"

`timescale 100ns/10ns

module adec_tb;

  reg clock = 1;
  reg [18:0] addr = 16'hC000 - 10;
  reg rw = 0;

  reg [15:0] test_addr [0:7];
  reg [7:0] i;

  adec dut(
    .clock(clock),
    .addr(addr),
    .rw(rw)
  );

  initial begin
    $dumpfile("adec_tb.vcd");
    $dumpvars(0, adec_tb);

    test_addr[0] = 16'hC000;
    test_addr[1] = 16'hD000;
    test_addr[2] = 16'hD400;
    test_addr[3] = 16'hDC00;
    test_addr[4] = 16'hDC10;
    test_addr[5] = 16'hDC20;
    test_addr[6] = 16'hDE00;
    test_addr[7] = 16'hE000;

    for (i = 0; i <= 7; i = i+1) begin
      clock = 1;
      repeat (2) begin
        #1 addr = test_addr[i];
        $write("clock:%b addr:%h", clock, addr);
        if (!dut.ram_cs) $write(" RAM");
        if (!dut.sid_cs) $write(" SID");
        if (!dut.via1_cs) $write(" VIA1");
        if (!dut.via2_cs) $write(" VIA2");
        if (!dut.uart_cs) $write(" UART");
        $display();
        clock = 0;
      end
    end

    $finish;
  end

endmodule
