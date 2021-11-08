.export ShellMain

.import UartInit, UartRxBufRead, UartTxBufWrite, UartTxStr

.segment "bss"

cmdbuf:         .res 256
cmdbuf_pos:     .res 1

.segment "os"

welcome:        .byte "Welcome to pda6502v2", $0D, $0A, $00
prompt:         .byte "> ", $00
e_notfound:     .byte "command not found", $0D, $0A, $00
cmdhello:       .byte "hello", $00

.proc ShellMain
                JSR UartInit
                JSR ShellHello
                JSR ShellPrompt
halt:           JMP halt
                RTS
.endproc

.proc ShellHello
                JSR ShellNewline
                LDX #<welcome
                LDY #>welcome
                JSR UartTxStr
                RTS
.endproc

.proc ShellNewline
                LDA #$0D
                JSR UartTxBufWrite
                LDA #$0A
                JSR UartTxBufWrite
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
                LDX #<cmdbuf            ; $00 pointer to cmdbuf...
                STX $00
                LDX #>cmdbuf
                STX $01
                LDX #<cmdhello          ; $02 pointer to cmdhello...
                STX $02
                LDX #>cmdhello
                STX $03
                JSR StrEq               ; compare strings
                BEQ hello
                JMP default
hello:          LDX #<welcome
                LDY #>welcome
                JSR UartTxStr
                JMP return
default:        LDX #<e_notfound
                LDY #>e_notfound
                JSR UartTxStr
return:         RTS
.endproc

; StrEq compares two null-terminated strings.
; The results is returned by the Z bit of the status register.
; Args are ZP pointers $00 and $02
; Maximum string length is 255 plus null-terminator.
.proc StrEq
                LDY #0
loop:           LDA ($00),Y             ; load byte from string A
                CMP ($02),Y             ; load byte from string B
                BNE return              ; if this byte differs, strings aren't equal (Z=1 ready for return)
                CMP #0                  ; the bytes are equal; was it null?
                BEQ return              ; in which case the strings match (Z=0 ready for return)
                INY                     ; increment Y index to the next byte
                BEQ err                 ; if it's zero, it's overflowed, return with Z=1
                JMP loop
err:            LDA #1
return:         RTS
.endproc
