.segment "os"

; delay100ms busy-loops for 100ms (at 1 MHz)
; 5XY+7Y+32 cycles; X=131,Y=151 -> 99994; 99.994 ms @ 1MHz
.proc delay100ms                        ; 6 cycles (JSR)
                PHX                     ; + 3
                PHY                     ; + 3
                LDX #131                ; + 2
                LDY #151                ; + 2
loop:           DEX                     ; + 2*X*Y
                BNE loop                ; + 3*X*Y + 2*Y
                DEY                     ; + 2*Y
                BNE loop                ; + 3*Y + 2
                PLY                     ; + 4
                PLX                     ; + 4
                RTS                     ; + 6
.endproc
