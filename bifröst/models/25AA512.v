// *******************************************************************************************************
// **                                                                                                   **
// **   25AA512.v - 25AA512 512K-BIT SPI SERIAL EEPROM (VCC = +1.8V TO +5.5V)                           **
// **                                                                                                   **
// *******************************************************************************************************
// **                                                                                                   **
// **                   This information is distributed under license from Young Engineering.           **
// **                              COPYRIGHT (c) 2009 YOUNG ENGINEERING                                 **
// **                                      ALL RIGHTS RESERVED                                          **
// **                                                                                                   **
// **                                                                                                   **
// **   Young Engineering provides design expertise for the digital world                               **
// **   Started in 1990, Young Engineering offers products and services for your electronic design      **
// **   project.  We have the expertise in PCB, FPGA, ASIC, firmware, and software design.              **
// **   From concept to prototype to production, we can help you.                                       **
// **                                                                                                   **
// **   http://www.young-engineering.com/                                                               **
// **                                                                                                   **
// *******************************************************************************************************
// **                                                                                                   **
// **   This information is provided to you for your convenience and use with Microchip products only.  **
// **   Microchip disclaims all liability arising from this information and its use.                    **
// **                                                                                                   **
// **   THIS INFORMATION IS PROVIDED "AS IS." MICROCHIP MAKES NO REPRESENTATION OR WARRANTIES OF        **
// **   ANY KIND WHETHER EXPRESS OR IMPLIED, WRITTEN OR ORAL, STATUTORY OR OTHERWISE, RELATED TO        **
// **   THE INFORMATION PROVIDED TO YOU, INCLUDING BUT NOT LIMITED TO ITS CONDITION, QUALITY,           **
// **   PERFORMANCE, MERCHANTABILITY, NON-INFRINGEMENT, OR FITNESS FOR PURPOSE.                         **
// **   MICROCHIP IS NOT LIABLE, UNDER ANY CIRCUMSTANCES, FOR SPECIAL, INCIDENTAL OR CONSEQUENTIAL      **
// **   DAMAGES, FOR ANY REASON WHATSOEVER.                                                             **
// **                                                                                                   **
// **   It is your responsibility to ensure that your application meets with your specifications.       **
// **                                                                                                   **
// *******************************************************************************************************
// **                                                                                                   **
// **   Revision       : 1.0                                                                            **
// **   Modified Date  : 02/05/2009                                                                     **
// **   Revision History:                                                                               **
// **                                                                                                   **
// **   02/05/2009:  Initial design                                                                     **
// **                                                                                                   **
// *******************************************************************************************************
// **                                       TABLE OF CONTENTS                                           **
// *******************************************************************************************************
// **---------------------------------------------------------------------------------------------------**
// **   DECLARATIONS                                                                                    **
// **---------------------------------------------------------------------------------------------------**
// **---------------------------------------------------------------------------------------------------**
// **   INITIALIZATION                                                                                  **
// **---------------------------------------------------------------------------------------------------**
// **---------------------------------------------------------------------------------------------------**
// **   CORE LOGIC                                                                                      **
// **---------------------------------------------------------------------------------------------------**
// **   1.01:  Internal Reset Logic                                                                     **
// **   1.02:  Input Data Shifter                                                                       **
// **   1.03:  Bit Clock Counter                                                                        **
// **   1.04:  Instruction Register                                                                     **
// **   1.05:  Address Register                                                                         **
// **   1.06:  Block Protect Bits                                                                       **
// **   1.07:  Write Protect Enable                                                                     **
// **   1.08:  Write Data Buffer                                                                        **
// **   1.09:  Write Enable Bit                                                                         **
// **   1.10:  Write Cycle Processor                                                                    **
// **   1.11:  Erase Cycle Processor                                                                    **
// **   1.12:  Deep Power-Down Logic                                                                    **
// **   1.13:  Output Data Shifter                                                                      **
// **   1.14:  Output Data Buffer                                                                       **
// **                                                                                                   **
// **---------------------------------------------------------------------------------------------------**
// **   DEBUG LOGIC                                                                                     **
// **---------------------------------------------------------------------------------------------------**
// **   2.01:  Memory Data Bytes                                                                        **
// **   2.02:  Page Buffer Bytes                                                                        **
// **                                                                                                   **
// **---------------------------------------------------------------------------------------------------**
// **   TIMING CHECKS                                                                                   **
// **---------------------------------------------------------------------------------------------------**
// **                                                                                                   **
// *******************************************************************************************************


`timescale 1ns/10ps

module M25AA512 (SI, SO, SCK, CS_N, WP_N, HOLD_N, RESET);

   input                SI;                             // serial data input
   input                SCK;                            // serial data clock

   input                CS_N;                           // chip select - active low
   input                WP_N;                           // write protect pin - active low

   input                HOLD_N;                         // interface suspend - active low

   input                RESET;                          // model reset/power-on reset

   output               SO;                             // serial data output


// *******************************************************************************************************
// **   DECLARATIONS                                                                                    **
// *******************************************************************************************************

   reg  [15:00]         DataShifterI;                   // serial input data shifter
   reg  [07:00]         DataShifterO;                   // serial output data shifter
   reg  [31:00]         BitCounter;                     // serial input bit counter
   reg  [07:00]         InstRegister;                   // instruction register
   reg  [15:00]         AddrRegister;                   // address register

   wire                 InstructionREAD;                // decoded instruction byte
   wire                 InstructionRDSR;                // decoded instruction byte
   wire                 InstructionWRSR;                // decoded instruction byte
   wire                 InstructionWRDI;                // decoded instruction byte
   wire                 InstructionWREN;                // decoded instruction byte
   wire                 InstructionWRITE;               // decoded instruction byte
   wire                 InstructionPE;                  // decoded instruction byte
   wire                 InstructionSE;                  // decoded instruction byte
   wire                 InstructionCE;                  // decoded instruction byte
   wire                 InstructionDPD;                 // decoded instruction byte
   wire                 InstructionRDID;                // decoded instruction byte

   reg  [07:00]         WriteBuffer [0:127];            // 128-byte page write buffer
   reg  [06:00]         WritePointer;                   // page buffer pointer
   reg  [07:00]         WriteCounter;                   // byte write counter

   reg                  WriteEnable;                    // memory write enable bit
   wire                 RstWriteEnable;                 // asynchronous reset
   wire                 SetWriteEnable;                 // register set
   wire                 ClrWriteEnable;                 // register clear

   reg                  WriteActive;                    // write operation in progress

   reg                  BlockProtect0;                  // memory block write protect
   reg                  BlockProtect1;                  // memory block write protect
   reg                  BlockProtect0_New;              // memory block write protect to be written
   reg                  BlockProtect1_New;              // memory block write protect to be written
   reg                  BlockProtect0_Old;              // old memory block write protect
   reg                  BlockProtect1_Old;              // old memory block write protect

   reg                  WP_Enable;                      // write protect pin enable
   reg                  WP_Enable_New;                  // write protect pin enable to be written
   reg                  WP_Enable_Old;                  // old write protect pin enable
   wire                 StatusWriteProtected;           // status register write protected

   reg  [06:00]         PageAddress;                    // page buffer address
   reg  [15:00]         BaseAddress;                    // memory write base address
   reg  [15:00]         MemWrAddress;                   // memory write address
   reg  [15:00]         MemRdAddress;                   // memory read address

   reg  [07:00]         MemoryBlock [0:65536];          // EEPROM data memory array (65536x8)

   reg                  DeepPowerDown;                  // deep power-down mode

   reg                  SO_DO;                          // serial output data - data
   wire                 SO_OE;                          // serial output data - output enable

   reg                  SO_Enable;                      // serial data output enable

   wire                 OutputEnable1;                  // timing accurate output enable
   wire                 OutputEnable2;                  // timing accurate output enable
   wire                 OutputEnable3;                  // timing accurate output enable

   integer              LoopIndex;                      // iterative loop index

   integer              tWC;                            // timing parameter
   integer              tV;                             // timing parameter
   integer              tHZ;                            // timing parameter
   integer              tHV;                            // timing parameter
   integer              tDIS;                           // timing parameter
   integer              tPD;                            // timing parameter
   integer              tREL;                           // timing parameter
   integer              tCE;                            // timing parameter
   integer              tSE;                            // timing parameter

`define PAGE_SIZE 128                                   // 256-byte page size
`define SECTOR_SIZE 16384                               // 16-Kbyte sector size
`define ARRAY_SIZE 65536                                // 512-Kbit array size
`define MFG_ID 8'h29                                    // Manufacturer's ID
`define WREN  8'b0000_0110                              // Write Enable instruction
`define READ  8'b0000_0011                              // Read instruction
`define WRDI  8'b0000_0100                              // Write Disable instruction
`define WRSR  8'b0000_0001                              // Write Status Register instruction
`define WRITE 8'b0000_0010                              // Write instruction
`define RDSR  8'b0000_0101                              // Read Status Register instruction
`define PE    8'b0100_0010                              // Page Erase instruction
`define SE    8'b1101_1000                              // Sector Erase instruction
`define CE    8'b1100_0111                              // Chip Erase instruction
`define DPD   8'b1011_1001                              // Deep Power-Down instruction
`define RDID  8'b1010_1011                              // Read ID and Release from Deep Power-Down instruction

// *******************************************************************************************************
// **   INITIALIZATION                                                                                  **
// *******************************************************************************************************

   initial begin
      `ifdef VCC_1_8V_TO_2_5V
         tWC  = 5000000;                                // memory write cycle time
         tV   = 250;                                    // output valid from SCK low
         tHZ  = 150;                                    // HOLD_N low to output high-Z
         tHV  = 150;                                    // HOLD_N high to output valid
         tDIS = 250;                                    // CS_N high to output disable
         tPD  = 100000;                                 // CS_N high to deep power-down
         tREL = 100000;                                 // CS_N high to standby mode
      `else
      `ifdef VCC_2_5V_TO_4_5V
         tWC  = 5000000;                                // memory write cycle time
         tV   = 50;                                     // output valid from SCK low
         tHZ  = 30;                                     // HOLD_N low to output high-Z
         tHV  = 30;                                     // HOLD_N high to output valid
         tDIS = 50;                                     // CS_N high to output disable
         tPD  = 100000;                                 // CS_N high to deep power-down
         tREL = 100000;                                 // CS_N high to standby mode
      `else
      `ifdef VCC_4_5V_TO_5_5V
         tWC  = 5000000;                                // memory write cycle time
         tV   = 25;                                     // output valid from SCK low
         tHZ  = 15;                                     // HOLD_N low to output high-Z
         tHV  = 15;                                     // HOLD_N high to output valid
         tDIS = 25;                                     // CS_N high to output disable
         tPD  = 100000;                                 // CS_N high to deep power-down
         tREL = 100000;                                 // CS_N high to standby mode
      `else
         tWC  = 5000000;                                // memory write cycle time
         tV   = 25;                                     // output valid from SCK low
         tHZ  = 15;                                     // HOLD_N low to output high-Z
         tHV  = 15;                                     // HOLD_N high to output valid
         tDIS = 25;                                     // CS_N high to output disable
         tPD  = 100000;                                 // CS_N high to deep power-down
         tREL = 100000;                                 // CS_N high to standby mode
      `endif
      `endif
      `endif
   end

   initial begin
      tCE = 10000000;                                   // chip erase cycle time
      tSE = 10000000;                                   // sector erase cycle time
   end

   initial begin
      BlockProtect0 = 0;
      BlockProtect1 = 0;

      WP_Enable = 0;

      WriteActive = 0;
      WriteEnable = 0;

      DeepPowerDown = 0;
   end


// *******************************************************************************************************
// **   CORE LOGIC                                                                                      **
// *******************************************************************************************************
// -------------------------------------------------------------------------------------------------------
//      1.01:  Internal Reset Logic
// -------------------------------------------------------------------------------------------------------

   always @(negedge CS_N) BitCounter   <= 0;
   always @(negedge CS_N) SO_Enable    <= 0;
   always @(negedge CS_N) if (!WriteActive) WritePointer <= 0;
   always @(negedge CS_N) if (!WriteActive) WriteCounter <= 0;

// -------------------------------------------------------------------------------------------------------
//      1.02:  Input Data Shifter
// -------------------------------------------------------------------------------------------------------

   always @(posedge SCK) begin
      if (HOLD_N == 1) begin
         if (CS_N == 0)         DataShifterI <= {DataShifterI[14:00],SI};
      end
   end

// -------------------------------------------------------------------------------------------------------
//      1.03:  Bit Clock Counter
// -------------------------------------------------------------------------------------------------------

   always @(posedge SCK) begin
      if (HOLD_N == 1) begin
         if (CS_N == 0)         BitCounter <= BitCounter + 1;
      end
   end

// -------------------------------------------------------------------------------------------------------
//      1.04:  Instruction Register
// -------------------------------------------------------------------------------------------------------

   always @(posedge SCK) begin
      if (HOLD_N == 1) begin
         if (BitCounter == 7)   InstRegister <= {DataShifterI[06:00],SI};
      end
   end

   assign InstructionREAD  = (InstRegister[7:0] == `READ);
   assign InstructionRDSR  = (InstRegister[7:0] == `RDSR);
   assign InstructionWRSR  = (InstRegister[7:0] == `WRSR);
   assign InstructionWRDI  = (InstRegister[7:0] == `WRDI);
   assign InstructionWREN  = (InstRegister[7:0] == `WREN);
   assign InstructionWRITE = (InstRegister[7:0] == `WRITE);
   assign InstructionPE    = (InstRegister[7:0] == `PE);
   assign InstructionSE    = (InstRegister[7:0] == `SE);
   assign InstructionCE    = (InstRegister[7:0] == `CE);
   assign InstructionDPD   = (InstRegister[7:0] == `DPD);
   assign InstructionRDID  = (InstRegister[7:0] == `RDID);

// -------------------------------------------------------------------------------------------------------
//      1.05:  Address Register
// -------------------------------------------------------------------------------------------------------

   always @(posedge SCK) begin
      if (HOLD_N == 1) begin
         if ((BitCounter == 23) & !WriteActive) AddrRegister <= {DataShifterI[14:00],SI};
      end
   end

// -------------------------------------------------------------------------------------------------------
//      1.06:  Block Protect Bits
// -------------------------------------------------------------------------------------------------------

   always @(posedge SCK) begin
      if (HOLD_N == 1) begin
         if (DeepPowerDown == 0) begin
            if ((BitCounter == 15) & InstructionWRSR & WriteEnable & !WriteActive & !StatusWriteProtected) begin
                BlockProtect1_New <= DataShifterI[02];
                BlockProtect0_New <= DataShifterI[01];
            end
         end
      end
   end
   
   always @(negedge CS_N) begin
      if (!WriteActive) begin
         BlockProtect0_Old <= BlockProtect0;
         BlockProtect1_Old <= BlockProtect1;
         WP_Enable_Old <= WP_Enable;
      end
   end

// -------------------------------------------------------------------------------------------------------
//      1.07:  Write Protect Enable
// -------------------------------------------------------------------------------------------------------

   always @(posedge SCK) begin
      if (HOLD_N == 1) begin
         if (DeepPowerDown == 0) begin
            if ((BitCounter == 15) & InstructionWRSR & WriteEnable & !WriteActive & !StatusWriteProtected) begin
               WP_Enable_New <= DataShifterI[06];
            end
         end
      end
   end

   assign StatusWriteProtected = WP_Enable & (WP_N == 0);

// -------------------------------------------------------------------------------------------------------
//      1.08:  Write Data Buffer
// -------------------------------------------------------------------------------------------------------

   always @(posedge SCK) begin
      if (HOLD_N == 1) begin
         if (DeepPowerDown == 0) begin
            if ((BitCounter >= 31) & (BitCounter[2:0] == 7) & InstructionWRITE & WriteEnable & !WriteActive) begin
               WriteBuffer[WritePointer] <= {DataShifterI[06:00],SI};

               WritePointer <= WritePointer + 1;
               if (WriteCounter < `PAGE_SIZE) WriteCounter <= WriteCounter + 1;
            end
         end
      end
   end

// -------------------------------------------------------------------------------------------------------
//      1.09:  Write Enable Bit
// -------------------------------------------------------------------------------------------------------

   always @(posedge CS_N or posedge RstWriteEnable) begin
      if (RstWriteEnable)       WriteEnable <= 0;
      else if (DeepPowerDown == 0) begin
         if (SetWriteEnable)    WriteEnable <= 1;
         if (ClrWriteEnable)    WriteEnable <= 0;
      end
   end

   assign RstWriteEnable = RESET;

   assign SetWriteEnable = (BitCounter == 8) & InstructionWREN & !WriteActive;
   assign ClrWriteEnable = (BitCounter == 8) & InstructionWRDI & !WriteActive;

// -------------------------------------------------------------------------------------------------------
//      1.10:  Write Cycle Processor
// -------------------------------------------------------------------------------------------------------

   always @(posedge CS_N) begin
      if (DeepPowerDown == 0) begin
         if ((BitCounter == 16) & (BitCounter[2:0] == 0) & InstructionWRSR  & WriteEnable & !WriteActive) begin
            if (!StatusWriteProtected) begin
                WriteActive = 1;
                #(tWC);

                BlockProtect1 = BlockProtect1_New;
                BlockProtect0 = BlockProtect0_New;
                WP_Enable = WP_Enable_New;
            end

            WriteActive = 0;
            WriteEnable = 0;
         end
         if ((BitCounter >= 32) & (BitCounter[2:0] == 0) & InstructionWRITE & WriteEnable & !WriteActive) begin
            for (LoopIndex = 0; LoopIndex < WriteCounter; LoopIndex = LoopIndex + 1) begin
               BaseAddress = {AddrRegister[15:07],7'h00};
               PageAddress = (AddrRegister[06:00] + LoopIndex);

               MemWrAddress = {BaseAddress[15:07],PageAddress[06:00]};

               if ({BlockProtect1,BlockProtect0} == 2'b00) begin
                  WriteActive = 1;
               end
               if ({BlockProtect1,BlockProtect0} == 2'b01) begin
                  if ((MemWrAddress >= 16'hC000) && (MemWrAddress <= 16'hFFFF)) begin
                     // write protected region
                  end
                  else begin
                     WriteActive = 1;
                  end
               end
               if ({BlockProtect1,BlockProtect0} == 2'b10) begin
                  if ((MemWrAddress >= 16'h8000) && (MemWrAddress <= 16'hFFFF)) begin
                     // write protected region
                  end
                  else begin
                     WriteActive = 1;
                  end
               end
               if ({BlockProtect1,BlockProtect0} == 2'b11) begin
                  // write protected region
               end
            end

            if (WriteActive) begin
                #(tWC);
                
                for (LoopIndex = 0; LoopIndex < WriteCounter; LoopIndex = LoopIndex + 1) begin
                   BaseAddress = {AddrRegister[15:07],7'h00};
                   PageAddress = (AddrRegister[06:00] + LoopIndex);
    
                   MemWrAddress = {BaseAddress[15:07],PageAddress[06:00]};
    
                   if ({BlockProtect1,BlockProtect0} == 2'b00) begin
                      MemoryBlock[MemWrAddress] = WriteBuffer[LoopIndex];
                   end
                   if ({BlockProtect1,BlockProtect0} == 2'b01) begin
                      if ((MemWrAddress >= 16'hC000) && (MemWrAddress <= 16'hFFFF)) begin
                         // write protected region
                      end
                      else begin
                         MemoryBlock[MemWrAddress] = WriteBuffer[LoopIndex];
                      end
                   end
                   if ({BlockProtect1,BlockProtect0} == 2'b10) begin
                      if ((MemWrAddress >= 16'h8000) && (MemWrAddress <= 16'hFFFF)) begin
                         // write protected region
                      end
                      else begin
                         MemoryBlock[MemWrAddress] = WriteBuffer[LoopIndex];
                      end
                   end
                   if ({BlockProtect1,BlockProtect0} == 2'b11) begin
                      // write protected region
                   end
                end
            end

            WriteActive = 0;
            WriteEnable = 0;
         end
      end
   end

// -------------------------------------------------------------------------------------------------------
//      1.11:  Erase Cycle Processor
// -------------------------------------------------------------------------------------------------------

   always @(posedge CS_N) begin
      if (DeepPowerDown == 0) begin
         if ((BitCounter == 24) & InstructionPE & WriteEnable & !WriteActive) begin
            for (LoopIndex = 0; LoopIndex < `PAGE_SIZE; LoopIndex = LoopIndex + 1) begin
               BaseAddress = {AddrRegister[15:07],7'h00};
               MemWrAddress = BaseAddress + LoopIndex;

               if ({BlockProtect1,BlockProtect0} == 2'b00) begin
                  WriteActive = 1;
               end
               if ({BlockProtect1,BlockProtect0} == 2'b01) begin
                  if ((BaseAddress >= 16'hC000) && (BaseAddress <= 16'hFFFF)) begin
                     // write protected region
                  end
                  else begin
                     WriteActive = 1;
                  end
               end
               if ({BlockProtect1,BlockProtect0} == 2'b10) begin
                  if ((BaseAddress >= 16'h8000) && (BaseAddress <= 16'hFFFF)) begin
                     // write protected region
                  end
                  else begin
                     WriteActive = 1;
                  end
               end
               if ({BlockProtect1,BlockProtect0} == 2'b11) begin
                  // write protected region
               end
            end

            if (WriteActive) begin
                #(tWC);
                
                for (LoopIndex = 0; LoopIndex < `PAGE_SIZE; LoopIndex = LoopIndex + 1) begin
                   BaseAddress = {AddrRegister[15:07],7'h00};
                   MemWrAddress = BaseAddress + LoopIndex;
    
                   if ({BlockProtect1,BlockProtect0} == 2'b00) begin
                      MemoryBlock[MemWrAddress] = 8'hFF;
                   end
                   if ({BlockProtect1,BlockProtect0} == 2'b01) begin
                      if ((BaseAddress >= 16'hC000) && (BaseAddress <= 16'hFFFF)) begin
                         // write protected region
                      end
                      else begin
                         MemoryBlock[MemWrAddress] = 8'hFF;
                      end
                   end
                   if ({BlockProtect1,BlockProtect0} == 2'b10) begin
                      if ((BaseAddress >= 16'h8000) && (BaseAddress <= 16'hFFFF)) begin
                         // write protected region
                      end
                      else begin
                         MemoryBlock[MemWrAddress] = 8'hFF;
                      end
                   end
                   if ({BlockProtect1,BlockProtect0} == 2'b11) begin
                      // write protected region
                   end
                end                
            end

            WriteActive = 0;
            WriteEnable = 0;
         end
         if ((BitCounter == 24) & InstructionSE & WriteEnable & !WriteActive) begin
            for (LoopIndex = 0; LoopIndex < `SECTOR_SIZE; LoopIndex = LoopIndex + 1) begin
               BaseAddress = {AddrRegister[15:14],14'h0000};
               MemWrAddress = BaseAddress + LoopIndex;

               if ({BlockProtect1,BlockProtect0} == 2'b00) begin
                  WriteActive = 1;
               end
               if ({BlockProtect1,BlockProtect0} == 2'b01) begin
                  if ((BaseAddress >= 16'hC000) && (BaseAddress <= 16'hFFFF)) begin
                     // write protected region
                  end
                  else begin
                     WriteActive = 1;
                  end
               end
               if ({BlockProtect1,BlockProtect0} == 2'b10) begin
                  if ((BaseAddress >= 16'h8000) && (BaseAddress <= 16'hFFFF)) begin
                     // write protected region
                  end
                  else begin
                     WriteActive = 1;
                  end
               end
               if ({BlockProtect1,BlockProtect0} == 2'b11) begin
                  // write protected region
               end
            end

            if (WriteActive) begin
                #(tSE);

                for (LoopIndex = 0; LoopIndex < `SECTOR_SIZE; LoopIndex = LoopIndex + 1) begin
                   BaseAddress = {AddrRegister[15:14],14'h0000};
                   MemWrAddress = BaseAddress + LoopIndex;
    
                   if ({BlockProtect1,BlockProtect0} == 2'b00) begin
                      MemoryBlock[MemWrAddress] = 8'hFF;
                   end
                   if ({BlockProtect1,BlockProtect0} == 2'b01) begin
                      if ((BaseAddress >= 16'hC000) && (BaseAddress <= 16'hFFFF)) begin
                         // write protected region
                      end
                      else begin
                         MemoryBlock[MemWrAddress] = 8'hFF;
                      end
                   end
                   if ({BlockProtect1,BlockProtect0} == 2'b10) begin
                      if ((BaseAddress >= 16'h8000) && (BaseAddress <= 16'hFFFF)) begin
                         // write protected region
                      end
                      else begin
                         MemoryBlock[MemWrAddress] = 8'hFF;
                      end
                   end
                   if ({BlockProtect1,BlockProtect0} == 2'b11) begin
                      // write protected region
                   end
                end
            end

            WriteActive = 0;
            WriteEnable = 0;
         end
         if ((BitCounter == 8) & InstructionCE & WriteEnable & !WriteActive) begin
            for (LoopIndex = 0; LoopIndex < `ARRAY_SIZE; LoopIndex = LoopIndex + 1) begin
               MemWrAddress = LoopIndex;

               if ({BlockProtect1,BlockProtect0} == 2'b00) begin
                  WriteActive = 1;
               end
            end

            if (WriteActive) begin
                #(tCE);

                for (LoopIndex = 0; LoopIndex < `ARRAY_SIZE; LoopIndex = LoopIndex + 1) begin
                   MemWrAddress = LoopIndex;
    
                   if ({BlockProtect1,BlockProtect0} == 2'b00) begin
                      MemoryBlock[MemWrAddress] = 8'hFF;
                   end
                end
            end

            WriteActive = 0;
            WriteEnable = 0;
         end
      end
   end

// -------------------------------------------------------------------------------------------------------
//      1.12:  Deep Power-Down Logic
// -------------------------------------------------------------------------------------------------------

   always @(posedge CS_N or posedge RESET) begin
      if (RESET)        DeepPowerDown = 0;
      else if ((BitCounter == 8) & InstructionDPD  & !WriteActive) begin
         #(tPD);
         DeepPowerDown = 1;
      end
      else if ((BitCounter > 7) & InstructionRDID & !WriteActive) begin
         #(tREL);
         DeepPowerDown = 0;
      end
   end

// -------------------------------------------------------------------------------------------------------
//      1.13:  Output Data Shifter
// -------------------------------------------------------------------------------------------------------

   always @(negedge SCK) begin
      if (HOLD_N == 1) begin
         if ((BitCounter >= 24) & (BitCounter[2:0] == 0) & InstructionREAD & !DeepPowerDown) begin
            if (BitCounter == 24) begin
               DataShifterO <= MemoryBlock[AddrRegister[15:00]];
               MemRdAddress <= AddrRegister + 1;
               SO_Enable    <= 1;
            end
            else begin
               DataShifterO <= MemoryBlock[MemRdAddress[15:00]];
               MemRdAddress <= MemRdAddress + 1;
            end
         end
         else if ((BitCounter > 7) & (BitCounter[2:0] == 3'b000) & InstructionRDSR & !DeepPowerDown) begin
            DataShifterO <= {WP_Enable_Old,3'b000,BlockProtect1_Old,BlockProtect0_Old,WriteEnable,WriteActive};
            SO_Enable    <= 1;
         end
         else if ((BitCounter >= 24) & (BitCounter[2:0] == 0) & InstructionRDID) begin
            if (BitCounter == 24) begin
               DataShifterO <= `MFG_ID;
               SO_Enable    <= 1;

            end
            else begin
               DataShifterO <= `MFG_ID;
            end
         end
         else begin
            DataShifterO <= DataShifterO << 1;
         end
      end
   end

// -------------------------------------------------------------------------------------------------------
//      1.14:  Output Data Buffer
// -------------------------------------------------------------------------------------------------------

   bufif1 (SO, SO_DO, SO_OE);

   always @(DataShifterO) SO_DO <= #(tV) DataShifterO[07];

   bufif1 #(tV,0)    (OutputEnable1, SO_Enable, 1);
   notif1 #(tDIS)    (OutputEnable2, CS_N,   1);
   bufif1 #(tHV,tHZ) (OutputEnable3, HOLD_N, 1);

   assign SO_OE = OutputEnable1 & OutputEnable2 & OutputEnable3;


// *******************************************************************************************************
// **   DEBUG LOGIC                                                                                     **
// *******************************************************************************************************
// -------------------------------------------------------------------------------------------------------
//      2.01:  Memory Data Bytes
// -------------------------------------------------------------------------------------------------------

   wire [07:00] MemoryByte0000 = MemoryBlock[00000];
   wire [07:00] MemoryByte0001 = MemoryBlock[00001];
   wire [07:00] MemoryByte0002 = MemoryBlock[00002];
   wire [07:00] MemoryByte0003 = MemoryBlock[00003];
   wire [07:00] MemoryByte0004 = MemoryBlock[00004];
   wire [07:00] MemoryByte0005 = MemoryBlock[00005];
   wire [07:00] MemoryByte0006 = MemoryBlock[00006];
   wire [07:00] MemoryByte0007 = MemoryBlock[00007];
   wire [07:00] MemoryByte0008 = MemoryBlock[00008];
   wire [07:00] MemoryByte0009 = MemoryBlock[00009];
   wire [07:00] MemoryByte000A = MemoryBlock[00010];
   wire [07:00] MemoryByte000B = MemoryBlock[00011];
   wire [07:00] MemoryByte000C = MemoryBlock[00012];
   wire [07:00] MemoryByte000D = MemoryBlock[00013];
   wire [07:00] MemoryByte000E = MemoryBlock[00014];
   wire [07:00] MemoryByte000F = MemoryBlock[00015];

   wire [07:00] MemoryByteFFF0 = MemoryBlock[65520];
   wire [07:00] MemoryByteFFF1 = MemoryBlock[65521];
   wire [07:00] MemoryByteFFF2 = MemoryBlock[65522];
   wire [07:00] MemoryByteFFF3 = MemoryBlock[65523];
   wire [07:00] MemoryByteFFF4 = MemoryBlock[65524];
   wire [07:00] MemoryByteFFF5 = MemoryBlock[65525];
   wire [07:00] MemoryByteFFF6 = MemoryBlock[65526];
   wire [07:00] MemoryByteFFF7 = MemoryBlock[65527];
   wire [07:00] MemoryByteFFF8 = MemoryBlock[65528];
   wire [07:00] MemoryByteFFF9 = MemoryBlock[65529];
   wire [07:00] MemoryByteFFFA = MemoryBlock[65530];
   wire [07:00] MemoryByteFFFB = MemoryBlock[65531];
   wire [07:00] MemoryByteFFFC = MemoryBlock[65532];
   wire [07:00] MemoryByteFFFD = MemoryBlock[65533];
   wire [07:00] MemoryByteFFFE = MemoryBlock[65534];
   wire [07:00] MemoryByteFFFF = MemoryBlock[65535];

// -------------------------------------------------------------------------------------------------------
//      2.02:  Page Buffer Bytes
// -------------------------------------------------------------------------------------------------------

   wire [07:00] PageBuffer00 = WriteBuffer[000];
   wire [07:00] PageBuffer01 = WriteBuffer[001];
   wire [07:00] PageBuffer02 = WriteBuffer[002];
   wire [07:00] PageBuffer03 = WriteBuffer[003];
   wire [07:00] PageBuffer04 = WriteBuffer[004];
   wire [07:00] PageBuffer05 = WriteBuffer[005];
   wire [07:00] PageBuffer06 = WriteBuffer[006];
   wire [07:00] PageBuffer07 = WriteBuffer[007];
   wire [07:00] PageBuffer08 = WriteBuffer[008];
   wire [07:00] PageBuffer09 = WriteBuffer[009];
   wire [07:00] PageBuffer0A = WriteBuffer[010];
   wire [07:00] PageBuffer0B = WriteBuffer[011];
   wire [07:00] PageBuffer0C = WriteBuffer[012];
   wire [07:00] PageBuffer0D = WriteBuffer[013];
   wire [07:00] PageBuffer0E = WriteBuffer[014];
   wire [07:00] PageBuffer0F = WriteBuffer[015];

   wire [07:00] PageBuffer10 = WriteBuffer[016];
   wire [07:00] PageBuffer11 = WriteBuffer[017];
   wire [07:00] PageBuffer12 = WriteBuffer[018];
   wire [07:00] PageBuffer13 = WriteBuffer[019];
   wire [07:00] PageBuffer14 = WriteBuffer[020];
   wire [07:00] PageBuffer15 = WriteBuffer[021];
   wire [07:00] PageBuffer16 = WriteBuffer[022];
   wire [07:00] PageBuffer17 = WriteBuffer[023];
   wire [07:00] PageBuffer18 = WriteBuffer[024];
   wire [07:00] PageBuffer19 = WriteBuffer[025];
   wire [07:00] PageBuffer1A = WriteBuffer[026];
   wire [07:00] PageBuffer1B = WriteBuffer[027];
   wire [07:00] PageBuffer1C = WriteBuffer[028];
   wire [07:00] PageBuffer1D = WriteBuffer[029];
   wire [07:00] PageBuffer1E = WriteBuffer[030];
   wire [07:00] PageBuffer1F = WriteBuffer[031];

   wire [07:00] PageBuffer60 = WriteBuffer[096];
   wire [07:00] PageBuffer61 = WriteBuffer[097];
   wire [07:00] PageBuffer62 = WriteBuffer[098];
   wire [07:00] PageBuffer63 = WriteBuffer[099];
   wire [07:00] PageBuffer64 = WriteBuffer[100];
   wire [07:00] PageBuffer65 = WriteBuffer[101];
   wire [07:00] PageBuffer66 = WriteBuffer[102];
   wire [07:00] PageBuffer67 = WriteBuffer[103];
   wire [07:00] PageBuffer68 = WriteBuffer[104];
   wire [07:00] PageBuffer69 = WriteBuffer[105];
   wire [07:00] PageBuffer6A = WriteBuffer[106];
   wire [07:00] PageBuffer6B = WriteBuffer[107];
   wire [07:00] PageBuffer6C = WriteBuffer[108];
   wire [07:00] PageBuffer6D = WriteBuffer[109];
   wire [07:00] PageBuffer6E = WriteBuffer[110];
   wire [07:00] PageBuffer6F = WriteBuffer[111];

   wire [07:00] PageBuffer70 = WriteBuffer[112];
   wire [07:00] PageBuffer71 = WriteBuffer[113];
   wire [07:00] PageBuffer72 = WriteBuffer[114];
   wire [07:00] PageBuffer73 = WriteBuffer[115];
   wire [07:00] PageBuffer74 = WriteBuffer[116];
   wire [07:00] PageBuffer75 = WriteBuffer[117];
   wire [07:00] PageBuffer76 = WriteBuffer[118];
   wire [07:00] PageBuffer77 = WriteBuffer[119];
   wire [07:00] PageBuffer78 = WriteBuffer[120];
   wire [07:00] PageBuffer79 = WriteBuffer[121];
   wire [07:00] PageBuffer7A = WriteBuffer[122];
   wire [07:00] PageBuffer7B = WriteBuffer[123];
   wire [07:00] PageBuffer7C = WriteBuffer[124];
   wire [07:00] PageBuffer7D = WriteBuffer[125];
   wire [07:00] PageBuffer7E = WriteBuffer[126];
   wire [07:00] PageBuffer7F = WriteBuffer[127];


// *******************************************************************************************************
// **   TIMING CHECKS                                                                                   **
// *******************************************************************************************************

   wire TimingCheckEnable = (RESET == 0) & (CS_N == 0);

   specify
      `ifdef VCC_1_8V_TO_2_5V
         specparam
            tHI  =  250,                                // SCK pulse width - high
            tLO  =  250,                                // SCK pulse width - low
            tSU  =  50,                                 // SI to SCK setup time
            tHD  =  100,                                // SI to SCK hold  time
            tHS  =  100,                                // HOLD_N to SCK setup time
            tHH  =  100,                                // HOLD_N to SCK hold  time
            tCSD =  50,                                 // CS_N disable time
            tCSS =  250,                                // CS_N to SCK setup time
            tCSH = 500,                                 // CS_N to SCK hold  time
            tCLD = 50,                                  // Clock delay time
            tCLE = 50;                                  // Clock enable time
      `else
      `ifdef VCC_2_5V_TO_4_5V
         specparam
            tHI  =  50,                                 // SCK pulse width - high
            tLO  =  50,                                 // SCK pulse width - low
            tSU  =  10,                                 // SI to SCK setup time
            tHD  =  20,                                 // SI to SCK hold  time
            tHS  =  20,                                 // HOLD_N to SCK setup time
            tHH  =  20,                                 // HOLD_N to SCK hold  time
            tCSD =  50,                                 // CS_N disable time
            tCSS =  50,                                 // CS_N to SCK setup time
            tCSH = 100,                                 // CS_N to SCK hold  time
            tCLD = 50,                                  // Clock delay time
            tCLE = 50;                                  // Clock enable time
      `else
      `ifdef VCC_4_5V_TO_5_5V
         specparam
            tHI  =  25,                                 // SCK pulse width - high
            tLO  =  25,                                 // SCK pulse width - low
            tSU  =   5,                                 // SI to SCK setup time
            tHD  =  10,                                 // SI to SCK hold  time
            tHS  =  10,                                 // HOLD_N to SCK setup time
            tHH  =  10,                                 // HOLD_N to SCK hold  time
            tCSD =  50,                                 // CS_N disable time
            tCSS =  25,                                 // CS_N to SCK setup time
            tCSH =  50,                                 // CS_N to SCK hold  time
            tCLD = 50,                                  // Clock delay time
            tCLE = 50;                                  // Clock enable time
      `else
         specparam
            tHI  =  25,                                 // SCK pulse width - high
            tLO  =  25,                                 // SCK pulse width - low
            tSU  =   5,                                 // SI to SCK setup time
            tHD  =  10,                                 // SI to SCK hold  time
            tHS  =  10,                                 // HOLD_N to SCK setup time
            tHH  =  10,                                 // HOLD_N to SCK hold  time
            tCSD =  50,                                 // CS_N disable time
            tCSS =  25,                                 // CS_N to SCK setup time
            tCSH =  50,                                 // CS_N to SCK hold  time
            tCLD = 50,                                  // Clock delay time
            tCLE = 50;                                  // Clock enable time
      `endif
      `endif
      `endif

      $width (posedge SCK,  tHI);
      $width (negedge SCK,  tLO);
      $width (posedge CS_N, tCSD);

      $setup (SI, posedge SCK &&& TimingCheckEnable, tSU);
      $setup (negedge CS_N, posedge SCK &&& TimingCheckEnable, tCSS);
      $setup (negedge SCK, negedge HOLD_N &&& TimingCheckEnable, tHS);
      $setup (posedge CS_N, posedge SCK &&& TimingCheckEnable, tCLD);

      $hold  (posedge SCK    &&& TimingCheckEnable, SI,   tHD);
      $hold  (posedge SCK    &&& TimingCheckEnable, posedge CS_N, tCSH);
      $hold  (posedge HOLD_N &&& TimingCheckEnable, posedge SCK,  tHH);
      $hold  (posedge SCK    &&& TimingCheckEnable, negedge CS_N, tCLE);
  endspecify

endmodule
