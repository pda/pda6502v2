.import BlinkenStart
.import BlinkenTick
.import UartMain

.import BLINKEN
.import BLINKSRC

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
          JSR BlinkenTick
          RTI

HandleNonMaskableInterrupt:
          RTI

.segment "vectors"

.word HandleNonMaskableInterrupt ; $FFFA: NMIB
.word HandleReset                ; $FFFC: RESB
.word HandleInterrupt            ; $FFFE: BRK/IRQB
