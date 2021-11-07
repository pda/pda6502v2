.export ShellMain

.import UartInit, UartRxBufRead, UartTxBufWrite, UartTxStr

.segment "bss"

cmdbuf:         .res 256
cmdbuf_pos:     .res 1

.segment "os"

welcome:        .byte $0D, $0A, "Welcome to pda6502v2", $0D, $0A, $00
prompt:         .byte "> ", $00
cmdplaceholder: .byte "command not found", $0D, $0A, $00

.proc ShellMain
                JSR UartInit
                JSR ShellHello
                JSR ShellPrompt
halt:           JMP halt
                RTS
.endproc

.proc ShellHello
                LDX #<welcome
                LDY #>welcome
                JSR UartTxStr
                RTS
.endproc

.proc ShellPrompt
                LDA #0
                STA cmdbuf_pos          ; init cmdbuf position
showprompt:     LDX #<prompt
                LDY #>prompt
                JSR UartTxStr
eachchar:       SEC                     ; UartRxBufRead blocking mode
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
                JSR ShellCmd            ;   else evaluate command
                JMP showprompt          ;     and then jump back to show a fresh prompt.
chardone:       JMP eachchar            ; again, forever
return:         RTS                     ; this never happens
.endproc

.proc ShellCmd
                LDX #<cmdplaceholder
                LDY #>cmdplaceholder    ; watch out; spare copy of UartRxBufRead is gone
                JSR UartTxStr           ; print a fresh prompt
                LDX #0
                STX cmdbuf_pos          ; reset position in cmdbuf to empty
                RTS
.endproc
