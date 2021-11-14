.export TermNewline
.export TermCursorUp16
.export TermCursorHide
.export TermCursorShow

.import UartTxBufWriteBlocking

.segment "os"

; TermNewline sends CR/LF after waiting for space on txbuf.
.proc TermNewline
                PHA
                LDA #$0D
                JSR UartTxBufWriteBlocking
                LDA #$0A
                JSR UartTxBufWriteBlocking
                PLA
                RTS
.endproc

.proc TermCursorUp16
                LDX #0
eachchar:       LDA vt100up16,X
                BEQ return
                JSR UartTxBufWriteBlocking
                INX
                JMP eachchar
return:         RTS
vt100up16:      .byte $1B, "[16A", $00
.endproc

.proc TermCursorHide
                LDX #0
eachchar:       LDA DECTCEM_L,X
                BEQ return
                JSR UartTxBufWriteBlocking
                INX
                JMP eachchar
return:         RTS
DECTCEM_L:      .byte $1B, "[?25l", $00
.endproc

.proc TermCursorShow
                LDX #0
eachchar:       LDA DECTCEM_H,X
                BEQ return
                JSR UartTxBufWriteBlocking
                INX
                JMP eachchar
return:         RTS
DECTCEM_H:      .byte $1B, "[?25h", $00
.endproc
