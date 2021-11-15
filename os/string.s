.export StrEq

.importzp R0, R2

.segment "os"

; StrEq compares two null-terminated strings.
; The results is returned by the Z bit of the status register.
; Args are ZP register pointers R0,R1 and R2,R3
; Maximum string length is 255 plus null-terminator.
.proc StrEq
                LDY #0
loop:           LDA (R0),Y              ; load byte from string A
                CMP (R2),Y              ; load byte from string B
                BNE return              ; if this byte differs, strings aren't equal (Z=1 ready for return)
                CMP #0                  ; the bytes are equal; was it null?
                BEQ return              ; in which case the strings match (Z=0 ready for return)
                INY                     ; increment Y index to the next byte
                BEQ err                 ; if it's zero, it's overflowed, return with Z=1
                JMP loop
err:            LDA #1
return:         RTS
.endproc
