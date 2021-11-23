.export ShellMain

.importzp R0, R1, R2, R3
.import UartInit, UartRxBufRead, UartTxBufWrite, UartTxStr
.import TermNewline
.import LifeMain
.import BLINKEN, BLINKSRC
.import StrEq
.import SidBleep

.segment "bss"

cmdbuf:         .res 256
cmdbuf_pos:     .res 1

.segment "os"

welcome:        .byte "Welcome to pda6502v2", $0D, $0A, $00
prompt:         .byte "> ", $00
helpmsg:        .byte "Available commands:", $0D, $0A
                .byte "  help: this message", $0D, $0A
                .byte "  hello: just being friendly", $0D, $0A
                .byte "  life: conway's game of life (ctrl-c to interrupt)", $0D, $0A
                .byte "  spi: send a greeting over SPI", $0D, $0A
                .byte "  bleep: make some noise on the SID", $0D, $0A
                .byte $00
e_notfound:     .byte "command not found", $0D, $0A, $00
cmdhelp:        .byte "help", $00
cmdhello:       .byte "hello", $00
cmdlife:        .byte "life", $00
cmdspi:         .byte "spi", $00
cmdbleep:       .byte "bleep", $00

.proc ShellMain
                JSR UartInit
                JSR ShellHello
                JSR ShellPrompt
                RTS
.endproc

.proc ShellHello
                JSR TermNewline
                LDX #<welcome
                LDY #>welcome
                JSR UartTxStr
                RTS
.endproc

.proc ShellPrompt
                LDA #0
                STA BLINKSRC
                LDA #0
                STA cmdbuf_pos          ; init cmdbuf position
showprompt:     LDX #<prompt
                LDY #>prompt
                JSR UartTxStr
eachchar:       LDA cmdbuf_pos
                STA BLINKEN
                SEC                     ; UartRxBufRead blocking mode
                JSR UartRxBufRead       ; A <- rxbuf after waiting
                TAY                     ; Y <- A (spare copy)
                CMP #$08                ; ASCII backspace
                BEQ backspace
                CMP #$0D                ; ASCII carriage return (CR, \r)
                BEQ newline
                CMP #$0A                ; ASCII new line (LF, \n)
                BEQ newline
default:        LDX cmdbuf_pos
                STA cmdbuf,X            ; append received byte onto cmdbuf
                INC cmdbuf_pos          ; TODO: check overflow
                JSR UartTxBufWrite
                JMP chardone
backspace:      LDX cmdbuf_pos          ; check position in cmdbuf...
                BEQ chardone            ; if it's zero, don't backspace.
                DEC cmdbuf_pos          ; shorten cmdbuf by one
                JSR UartTxBufWrite      ; print the backspace to move cursor back
                LDA #' '
                JSR UartTxBufWrite      ; then a space to overwrite the char being backspaced
                TYA
                JSR UartTxBufWrite      ; then another backspace
                JMP chardone
newline:        LDA #$0D                ; CR
                JSR UartTxBufWrite
                LDA #$0A                ; LF
                JSR UartTxBufWrite
                LDX cmdbuf_pos          ; check if cmdbuf is empty..
                BEQ showprompt          ;   then jump back to show a fresh prompt.
                LDA #0                  ;   else null-terminate cmdbuf..
                LDX cmdbuf_pos
                STA cmdbuf,X
                JSR ShellCmd            ; .. and evaluate command
                LDX #0                  ;
                STX cmdbuf_pos          ; reset position in cmdbuf to empty
                JMP showprompt          ;     and then jump back to show a fresh prompt.
chardone:       JMP eachchar            ; again, forever
return:         RTS                     ; this never happens
.endproc

.proc ShellCmd
                LDX #<cmdbuf            ; R0,R1 pointer to cmdbuf...
                STX R0
                LDX #>cmdbuf
                STX R1
                LDX #<cmdhelp           ; R2,R3 pointer to cmdhelp...
                STX R2
                LDX #>cmdhelp
                STX R3
                JSR StrEq               ; compare (R0) and (R2)
                BEQ help
                LDX #<cmdhello          ; R2,R3 pointer to cmdhello...
                STX R2
                LDX #>cmdhello
                STX R3
                JSR StrEq               ; compare (R0) and (R2)
                BEQ hello
                LDX #<cmdlife           ; R2,R3 pointer to cmdlife...
                STX R2
                LDX #>cmdlife
                STX R3
                JSR StrEq               ; compare (R0) and (R2)
                BEQ life
                LDX #<cmdspi            ; R2,R3 pointer to cmdspi...
                STX R2
                LDX #>cmdspi
                STX R3
                JSR StrEq               ; compare (R0) and (R2)
                BEQ spi
                LDX #<cmdbleep          ; R2,R3 pointer to cmdbleep...
                STX R2
                LDX #>cmdbleep
                STX R3
                JSR StrEq               ; compare (R0) and (R2)
                BEQ bleep
                JMP default             ; cmdbuf didn't match any commands
help:           LDX #<helpmsg
                LDY #>helpmsg
                JSR UartTxStr
                JMP return
hello:          LDX #<welcome
                LDY #>welcome
                JSR UartTxStr
                JMP return
life:           JSR LifeMain
                JMP return
spi:            JSR ShellSPI
                JMP return
bleep:          JSR SidBleep
                JMP return
default:        LDX #<e_notfound
                LDY #>e_notfound
                JSR UartTxStr
return:         RTS
.endproc

.proc ShellSPI
                LDA #1<<0
                TRB $DE10               ; SPI CS[0] active low
                LDX #0
eachchar:       LDA welcome,X           ;                 4 cycles ] Total:
                BEQ done                ;                 2 cycles ]   15 cycles
                STA $DE11               ; SPI data        4 cycles ]    3.75 µs per byte @ 4 MHz
                INX                     ;                 2 cycles ]  260 KiB/sec @ 4 MHz
                JMP eachchar            ;                 3 cycles ]  (bottleneck: 6502, not SPI clock)
done:           LDA #1<<0
                TSB $DE10               ; release SPI CS[0]
                RTS
.endproc
