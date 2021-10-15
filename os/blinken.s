.export BlinkenStart
.export BlinkenTick

.export BLINKEN   = $DE00             ; BIFRÖST BLINKEN register

.import VIA2_IFR, VIA2_T1CH, VIA2_T1CL, VIA2_ACR, VIA2_IER

; TODO: these need to go somewhere generally useful

ZP_BLINKEN = $42              ; BIFRÖST BLINKEN value (TODO: make the register r/w)
ZP_BLNKAUX = $43              ; BIFRÖST BLINKEN auxiliary data

.segment "os"

.PROC BlinkenStart
          PHA

          LDA #%10000000      ; bit 7:enable, 6:dir(0:L,1:R)
          STA ZP_BLNKAUX
          LDA #1
          STA ZP_BLINKEN
          STA BLINKEN

          ; enable T1 interrupt
          LDA #%11000000      ; “If bit 7 is a "1", then each Logic 1 in bits 0-6 enables the corresponding interrupt”
          STA VIA2_IER

          ; set T1 to continuous interrupts ACR[7:6] = %01
          LDA VIA2_ACR
          AND #%01111111      ; ACR[7] = 0
          ORA #%01000000      ; ACR[6] = 1
          STA VIA2_ACR

          ; trigger T1 counter from $FFFF
          LDA #$FF
          STA VIA2_T1CL
          STA VIA2_T1CH

          PLA
          RTS
.ENDPROC

; Called by interrupt handler to update BLINKEN
.PROC BlinkenTick
          BIT ZP_BLNKAUX
          BMI enabled
          RTS
enabled:
          PHA
          PHX

          ; “individual flag bits may be cleared by writing
          ; a Logic 1 into the appropriate bit of the IFR”
          LDA #%01000000      ; clear VIA2 T1 interrupt
          STA VIA2_IFR

          LDA ZP_BLINKEN
          TAX                 ; ZP_BLINKEN value A -> X
testl:    CMP #%10000000      ; BLINKEN reached far-left
          BNE testr
          LDA #%01000000
          TSB ZP_BLNKAUX      ; set ZP_BLNKAUX[6] = 1 (dir=R)
          JMP right
testr:    CMP #%00000001      ; BLINKEN reached far-right
          BNE testdir
          LDA #%01000000
          TRB ZP_BLNKAUX      ; set ZP_BLNKAUX[6] = 0 (dir=L)
          JMP left
testdir:  BIT ZP_BLNKAUX      ; S[overflow] set to ZP_BLNKAUX[6]
          BVS right           ; branch if ZP_BLNKAUX[6] is 1 (dir=R)
left:     TXA                 ; ZP_BLINKEN value A <- X
          CLC
          ROL A
          JMP store
right:    TXA                 ; ZP_BLINKEN value A <- X
          CLC
          ROR A
store:    STA ZP_BLINKEN
          STA BLINKEN

          PLX
          PLA
          RTS
.ENDPROC
