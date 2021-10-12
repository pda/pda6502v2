; asmsyntax=asmM6502 (http://cc65.github.io/cc65/)

BLINKEN = $DE00               ; BIFRÖST BLINKEN register
ZP_BLINKEN = $42              ; zero-page BLINKEN storage

.segment "os"

.PROC Main
          ; slightly convuluted to verify RAM write/read works
          LDA #$AA            ; load initial 10101010 LED pattern into A
          STA ZP_BLINKEN      ; store LED pattern at ZP_BLINKEN for later changes
          LDX ZP_BLINKEN      ; load X ← ZP_BLINKEN in zero-page (value should be $AA)
          STX BLINKEN         ; write X → BLINKEN (LEDs should be 10101010)

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
          RTI

HandleNonMaskableInterrupt:
          RTI

.segment "vectors"

.word HandleNonMaskableInterrupt ; $FFFA: NMIB
.word HandleReset                ; $FFFC: RESB
.word HandleInterrupt            ; $FFFE: BRK/IRQB
