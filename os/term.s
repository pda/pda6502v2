.export TermNewline
.export TermCursorUp16

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
