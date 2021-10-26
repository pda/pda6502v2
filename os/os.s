.import BlinkenStart
.import BlinkenTick
.import UartMain

.import BLINKEN, BLINKSRC
.import VIA1, VIA2, VIA_IFR : zp

.segment "os"

.PROC Main
          JSR BlinkenStart
          JSR UartMain
          ;LDA #$04           ; 0:reg 1:data 2:addrl 3:addrh 4:IRQ
          ;STA BLINKSRC
halt:     JMP halt
.ENDPROC


HandleReset:
          SEI                 ; mask interrupts during start-up
          LDX #$FF            ;
          TXS                 ; set stack pointer to $ff ($01FF)
          CLI                 ; resume interrupts
          CLD                 ; don't be in crazy decimal mode.
          JMP Main

HandleInterrupt:
          BIT VIA1+VIA_IFR
          BPL after_via1      ; not VIA1 interrupt
          BVC after_via1_t1   ; not VIA1 T1 interrupt
          JSR BlinkenTick
after_via1_t1:
after_via1:
          RTI

HandleNonMaskableInterrupt:
          RTI

.segment "vectors"

.word HandleNonMaskableInterrupt ; $FFFA: NMIB
.word HandleReset                ; $FFFC: RESB
.word HandleInterrupt            ; $FFFE: BRK/IRQB
