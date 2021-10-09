; asmsyntax=asmM6502 (http://cc65.github.io/cc65/)

.segment "os"

.PROC Main
LDA #$AA     ; arbitrary sentinel
STA $DE00    ; BIFRÖST BLINKEN register
halt: JMP halt
.ENDPROC

HandleReset:
SEI          ; mask interrupts during start-up
LDX #$FF     ;
TXS          ; set stack pointer to $ff ($01FF)
CLI          ; resume interrupts
CLD          ; don't be in crazy decimal mode.
JMP Main

HandleInterrupt:
RTI

HandleNonMaskableInterrupt:
RTI

.segment "vectors"

.word HandleNonMaskableInterrupt ; $FFFA: NMIB
.word HandleReset                ; $FFFC: RESB
.word HandleInterrupt            ; $FFFE: BRK/IRQB
