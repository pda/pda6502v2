.export LifeMain

.importzp R0, R1
.import UartTxStr, UartTxBufWriteBlocking
.import TermNewline, TermCursorUp16, TermCursorHide, TermCursorShow
.import VIA1
.importzp VIA_IRA, VIA_T1CL
.importzp ZP_INTERRUPT

.segment "bss"

gridcurr:       .res 256                ; 16x16 Game of Life current generation grid
gridnext:       .res 256                ; 16x16 Game of Life next-generation grid

.segment "os"

message:        .byte "A STRANGE GAME.", $0D, $0A
                .byte "THE ONLY WINNING MOVE IS", $0D, $0A
                .byte "NOT TO PLAY.", $0D, $0A
                .byte $00

.proc LifeMain
                LDX #<message           ; A fitting welcome.
                LDY #>message
                JSR UartTxStr
                JSR LifeInit
forever:        JSR TermCursorHide
                JSR LifeRender
                JSR LifeTick
                JSR TermCursorUp16
                LDX #0
delay:          INX
                BNE delay
                BIT ZP_INTERRUPT
                BMI interrupted
                JMP forever
interrupted:    LDA #1<<7
                TRB ZP_INTERRUPT
                JSR TermCursorShow
                JSR TermNewline
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
store:          STA gridcurr,X
                INX
                BNE eachcell
                RTS
.endproc

; render a 16x16 game of life grid as ASCII over UART
.proc LifeRender
                LDX #0
eachcell:       BIT gridcurr,X
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
newline:        JSR TermNewline
donecell:       JMP eachcell
donegrid:       JSR TermNewline
return:         RTS
.endproc

; Core logic for Game of Life
; for each cell C with neighbour count N:
;   if C is alive:
;     if N <= 1, C dies
;     else if N <= 3, C lives
;     else C dies
;   if C is dead:
;     if N == 3, C lives
;     else C dies
.proc LifeTick
                LDA #0
                STA R0                  ; first cell index
eachcell:       LDA #0
                STA R1                  ; reset neighbour tally
                LDY #0                  ; index into list of neighbour byte offsets
eachneighbour:  LDA neighbours,Y        ; load the byte offset of each neighbour
                BEQ doneneighbours      ; (terminated by zero byte)
                CLC                     ; clear carry bit for addition
                ADC R0                  ; A (neighbour relative offset) += origin cell offset
                TAX                     ; X <- A to indexed-address into grid for neighbour
                BIT gridcurr,X          ; is this neighbour alive?
                BPL deadneighbour       ; if bit 7 was 0, then nope
                INC R1                  ; add this alive neighbour to tally
deadneighbour:  INY                     ; move to the next offset in neighbours list
                JMP eachneighbour
doneneighbours: LDX R0                  ; transfer origin cell offset back into X for indexed addressing
                LDA R1                  ; transfer alive-neighbour tally to A
                BIT gridcurr,X          ; origin cell alive?
                BPL origindead
originalive:    CMP #2                  ; if N<=1 (i.e. N-2 is negative)
                BMI makedead            ; our living cell dies from underpopulation
                CMP #4                  ; if N<=3 (i.e. N-4 is is negative)
                BMI makealive           ; our living cell lives on, 2..3 N
                JMP makedead            ; else N>3, our living cell dies from overpopulation
origindead:     CMP #3                  ; if N==3
                BEQ makealive           ; our empty cell spawns life
                JMP makedead            ; else it stays dead
makealive:      LDA #$FF                ; prepare to set our cell's bit 7 (and other bits for good measure)
                JMP writecell
makedead:       LDA #$00                ; prepare to clear our cell's bit 7 (and other bits, for good measure)
writecell:      STA gridnext,X          ; write the new state into the "next generation" grid.
donecell:       INC R0                  ; Move to the next cell.
                BNE eachcell            ; process it unless we've wrapped around to zero.
                LDX #0
copyeachcell:   LDA gridnext,X          ; All cells evaluated; copy each cell from the next generation grid...
                STA gridcurr,X          ; ... into the primary grid.
                INX
                BNE copyeachcell        ; Until we wrap around to zero.
                RTS
neighbours:     .byte 256-17, 256-16, 256-15, 256-1, 1, 15, 16, 17, $00
                ;         NW       N      NE      W  E  SW   S  SE
.endproc
