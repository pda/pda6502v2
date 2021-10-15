.import BlinkenStart
.import BlinkenTick
.import UartMain

.import BLINKEN

.segment "os"

.PROC Main
          ;JSR BlinkenStart
          JSR UartMain
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
          LDA #$AA
          STA BLINKEN
halt:     JMP halt
          RTI

HandleNonMaskableInterrupt:
          RTI

.segment "vectors"

.word HandleNonMaskableInterrupt ; $FFFA: NMIB
.word HandleReset                ; $FFFC: RESB
.word HandleInterrupt            ; $FFFE: BRK/IRQB
