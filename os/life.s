.export LifeMain

.importzp R1
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
                LDY #0
delay:          INX
                BNE delay
                INY
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
; A is used for rando stuff
; bit 7 of each byte indicates liveness
.proc LifeTick
                LDX #0                  ; cell index
eachcell:       LDA #0
                STA R1                  ; neighbor count
                TXA                     ; transfer cell index to A for subtraction
                SEC                     ; prepare carry bit for subtraction
                SBC #17                 ; jump back a full row (16) plus one cell (origin->NW neighbor)
                TAX                     ; transfer result back into X for indexed addressing
                BIT gridcurr,X          ; NW alive?
                BPL deadNW
                INC R1                  ; add NW neighbor to tally
deadNW:         INX                     ; move from NW to N neighbor (origin-16)
                BIT gridcurr,X          ; N alive?
                BPL deadN
                INC R1                  ; add N neighbor to tally
deadN:          INX                     ; move from N->NE neighbor (origin-15)
                BIT gridcurr,X          ; NE alive?
                BPL deadNE
                INC R1                  ; add N neighbor to tally
deadNE:         TXA                     ; transfer cell index to A for addition
                CLC                     ; prepare carry bit for addition
                ADC #14                 ; jump one row (16) minus two cells (NE->W) (origin-1)
                TAX                     ; transfer result back into X for indexed addressing
                BIT gridcurr,X          ; W alive?
                BPL deadW
                INC R1                  ; add W neighbor to tally
deadW:          INX                     ; move W->origin, skip this one
                INX                     ; move origin->E (origin+1)
                BIT gridcurr,X          ; E alive?
                BPL deadE
                INC R1                  ; add E neighbor to tally
deadE:          TXA                     ; transfer cell index to A for addition
                CLC                     ; prepare carry bit for addition
                ADC #14                 ; jump one row (16) minus two cells (E->SW) (origin+15)
                TAX                     ; transfer result back into X for indexed addressing
                BIT gridcurr,X          ; SW alive?
                BPL deadSW
                INC R1                  ; add SW neighbor to tally
deadSW:         INX                     ; move SW->S (origin+16)
                BIT gridcurr,X          ; S alive?
                BPL deadS
                INC R1                  ; add S neighbor to tally
deadS:          INX                     ; move S->SE (origin+17)
                BIT gridcurr,X          ; SE alive?
                BPL deadSE
                INC R1                  ; add SE neighbor to tally
deadSE:         TXA                     ; transfer cell index to A for subtraction
                SEC                     ; prepare carry bit for subtraction
                SBC #17                 ; jump one row (16) plus one cell (SE->origin)
                TAX                     ; transfer result back into X for indexed addressing
                LDA R1                  ; transfer alive-neighbor tally to A
                BIT gridcurr,X          ; origin cell alive?
                BPL origindead
originalive:    CMP #2                  ; if neighbors<=1 (i.e. neighbors-2 is negative)
                BMI makedead            ; our living cell dies from underpopulation
                CMP #4                  ; if neighbors<=3 (i.e. neighbors-4 is is negative)
                BMI makealive           ; our living cell lives on, 2..3 neighbors
                JMP makedead            ; else neighbors>3, our living cell dies from overpopulation
origindead:     CMP #3                  ; if neighbors==3
                BEQ makealive           ; our empty cell spawns life
                JMP makedead            ; else it stays dead
makealive:      LDA #$FF                ; prepare to set our cell's bit 7 (and other bits for good measure)
                JMP writecell
makedead:       LDA #$00                ; prepare to clear our cell's bit 7 (and other bits, for good measure)
writecell:      STA gridnext,X          ; write the new state into the "next generation" grid.
donecell:       INX                     ; Move to the next cell.
                BNE eachcell            ; process it unless we've wrapped around to zero.
copyeachcell:   LDA gridnext,X          ; All cells evaluated; copy each cell from the next generation grid...
                STA gridcurr,X          ; ... into the primary grid.
                INX
                BNE copyeachcell        ; Until we wrap around to zero.
                RTS
.endproc
