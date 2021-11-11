.export LifeMain

.import UartNewline, UartTxStr, UartTxBufWriteBlocking
.import VIA1, VIA_IRA : zp, VIA_T1CL : zp

.segment "bss"

.segment "os"

GRIDADDR  = $0200 ; a page of memory for Game of Life main grid
GRIDADDRB = $0300 ; a page of memory for Game of Life tmp/next grid

message:        .byte "A STRANGE GAME.", $0D, $0A
                .byte "THE ONLY WINNING MOVE IS", $0D, $0A
                .byte "NOT TO PLAY.", $0D, $0A
                .byte $00

.proc LifeMain
                LDX #<message           ; A fitting welcome.
                LDY #>message
                JSR UartTxStr
                JSR LifeInit
forever:        JSR LifeRender
                JSR LifeTick
                JSR LifeCursorUp
                LDX #0
                LDY #0
delay:          INX
                BNE delay
                INY
                BNE delay
                JMP forever
                RTS
.endproc


.proc LifeInit
                LDX #0
eachcell:       TXA
                ; seed a glider
                CMP #1
                BEQ makealive
                CMP #18
                BEQ makealive
                CMP #32
                BEQ makealive
                CMP #33
                BEQ makealive
                CMP #34
                BEQ makealive
                JMP makedead
makealive:      LDA #$FF
                JMP store
makedead:       LDA #$00
store:          STA GRIDADDR,X
                INX
                BNE eachcell
                RTS
.endproc

; render a 16x16 game of life grid as ASCII over UART
.proc LifeRender
                LDX #0
eachcell:       BIT GRIDADDR,X
                BPL alive
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

; For each cell, count the living neighbors (N); byte offsets:
; -17 -16 -15
;  -1  X   +1
; +15 +16 +17
;
; if X is alive:
;   if N <= 1, X dies
;   else if N <= 3, X lives
;   else X dies
; if X is dead:
;   if N == 3, X lives
;   else X dies
;
; X register is cell index
; Y for alive-neighbor count
; A is used for rando stuff
; bit 7 of each byte indicates liveness
.proc LifeTick
                LDX #0
eachcell:       LDY #0                  ; neighbor count
                ; TODO: obvs not this shit to get to offset -17
                DEX ; - 1
                DEX ; - 2
                DEX ; - 3
                DEX ; - 4
                DEX ; - 5
                DEX ; - 6
                DEX ; - 7
                DEX ; - 8
                DEX ; - 9
                DEX ; -10
                DEX ; -11
                DEX ; -12
                DEX ; -13
                DEX ; -14
                DEX ; -15
                DEX ; -16
                DEX ; -17
                BIT GRIDADDR,X          ; NW alive?
                BPL nopeNW
                INY                     ; increment neighbor count
nopeNW:         INX ; -16
                BIT GRIDADDR,X          ; N alive?
                BPL nopeN
                INY                     ; increment neighbor count
nopeN:          INX ; -15
                BIT GRIDADDR,X          ; NE alive?
                BPL nopeNE
                INY                     ; increment neighbor count
nopeNE:         ; TODO: not this shit
                INX ; -14
                INX ; -13
                INX ; -12
                INX ; -11
                INX ; -10
                INX ; - 9
                INX ; - 8
                INX ; - 7
                INX ; - 6
                INX ; - 5
                INX ; - 4
                INX ; - 3
                INX ; - 2
                INX ; - 1
                BIT GRIDADDR,X          ; W alive?
                BPL nopeW
                INY                     ; increment neighbor count
nopeW:          INX ;   0
                INX ; + 1
                BIT GRIDADDR,X          ; E alive?
                BPL nopeE
                INY                     ; increment neighbor count
nopeE:          INX ; + 2
                INX ; + 3
                INX ; + 4
                INX ; + 5
                INX ; + 6
                INX ; + 7
                INX ; + 8
                INX ; + 9
                INX ; +10
                INX ; +11
                INX ; +12
                INX ; +13
                INX ; +14
                INX ; +15
                BIT GRIDADDR,X          ; SW alive?
                BPL nopeSW
                INY                     ; increment neighbor count
nopeSW:         INX ; +16
                BIT GRIDADDR,X          ; S alive?
                BPL nopeS
                INY                     ; increment neighbor count
nopeS:          INX ; +17
                BIT GRIDADDR,X          ; SE alive?
                BPL nopeSE
                INY                     ; increment neighbor count
nopeSE:         ; TODO: not _any_ of this shit (get back to original cell)
                DEX ; -16
                DEX ; -15
                DEX ; -14
                DEX ; -13
                DEX ; -12
                DEX ; -11
                DEX ; -10
                DEX ; - 9
                DEX ; - 8
                DEX ; - 7
                DEX ; - 6
                DEX ; - 5
                DEX ; - 4
                DEX ; - 3
                DEX ; - 2
                DEX ; - 1
                DEX ;   0
                ; after that horror-show, Y should be the alive-neighbor count
                TYA
                BIT GRIDADDR,X          ; cell alive?
                BPL wasdead
wasalive:       CMP #2                  ; if this is negative, N <= 1
                BMI makedead
                CMP #4                  ; if this is negative, N <= 3 (and N <= 1)
                BMI makealive
                JMP makedead            ; otherwise, N > 3
wasdead:        CMP #3
                BEQ makealive
                JMP makedead
makealive:      LDA #$FF
                JMP writecell
makedead:       LDA #$00
writecell:      STA GRIDADDRB,X         ; write the new state into the shadow grid
donecell:       INX
                BEQ doneallcells        ; branch else JMP because target is too far away ;(
                JMP eachcell
doneallcells:
copyeachcell:   LDA GRIDADDRB,X
                STA GRIDADDR,X
                INX
                BNE copyeachcell
                RTS
.endproc

.proc LifeCursorUp
                LDX #0
eachchar:       LDA vt100up16,X
                BEQ return
                JSR UartTxBufWriteBlocking
                INX
                JMP eachchar
return:         RTS
vt100up16:      .byte $1B, "[16A", $00
.endproc
