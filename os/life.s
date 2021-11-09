.export LifeMain

.import BLINKEN, BLINKSRC
.import UartNewline, UartTxStr, UartTxBufWriteBlocking

.segment "bss"

.segment "os"

GRIDADDR = $0600 ; any old place in RAM, we're just reading it for now

message:        .byte "A STRANGE GAME.", $0D, $0A
                .byte "THE ONLY WINNING MOVE IS", $0D, $0A
                .byte "NOT TO PLAY.", $0D, $0A
                .byte $00

.proc LifeMain
                LDX #<message           ; A fitting welcome.
                LDY #>message
                JSR UartTxStr
                JSR LifeRender
                LDA #$00
                STA BLINKSRC
                LDA #$AA
                STA BLINKEN
                RTS
.endproc


; render a 16x16 game of life grid as ASCII over UART
.proc LifeRender
                LDX #0
                STX BLINKSRC
                STX BLINKEN
eachcell:       BIT GRIDADDR,X
                BMI alive
dead:           LDA #'*'
                JMP deadoralive
alive:          LDA #' '
deadoralive:    JSR UartTxBufWriteBlocking
                LDA #' '
                JSR UartTxBufWriteBlocking
                INX
                BEQ donegrid           ; wrapped around; done
                TXA
                SEC
modulus:        SBC #16                ; number of columns
                BEQ newline            ; end of column
                BCS modulus
                JMP donecell
newline:        JSR UartNewline
donecell:       JMP eachcell
donegrid:       JSR UartNewline
return:         RTS
.endproc
