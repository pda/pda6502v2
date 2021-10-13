; asmsyntax=asmM6502 (http://cc65.github.io/cc65/)


; zero-page map
ZP_BLINKEN = $42              ; BIFRÖST BLINKEN value (TODO: make the register r/w)
ZP_BLNKAUX = $43              ; BIFRÖST BLINKEN auxiliary data

BLINKEN   = $DE00             ; BIFRÖST BLINKEN register

VIA2_ORB  = $DC10
VIA2_IRB  = $DC10
VIA2_ORA  = $DC11
VIA2_IRA  = $DC11
VIA2_DDRB = $DC12
VIA2_DDRA = $DC13
VIA2_T1CL = $DC14
VIA2_T1CH = $DC15
VIA2_T1LL = $DC16
VIA2_T1LH = $DC17
VIA2_T2CL = $DC18
VIA2_T2CH = $DC19
VIA2_SR   = $DC1A
VIA2_ACR  = $DC1B
VIA2_PCR  = $DC1C
VIA2_IFR  = $DC1D
VIA2_IER  = $DC1E
VIA2_ORAH = $DC1F
VIA2_IRBH = $DC1F

.segment "os"

.PROC Main
          LDA #%00000000      ; bit 7:dir(0:L,1:R)
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

halt:     JMP halt
.ENDPROC

HandleReset:
          SEI                 ; mask interrupts during start-up
          LDX #$FF            ;
          TXS                 ; set stack pointer to $ff ($01FF)
          CLI                 ; resume interrupts
          CLD                 ; don't be in crazy decimal mode.
          JMP Main

HandleInterrupt:
          PHA
          PHX

          ; clear VIA2 T1 interrupt
          LDA #%01000000      ; “individual flag bits may be cleared by writing a Logic 1 into the appropriate bit of the IFR”
          STA VIA2_IFR

          LDA ZP_BLINKEN
          TAX                 ; ZP_BLINKEN value A -> X
testl:    CMP #%10000000      ; BLINKEN reached far-left
          BNE testr
          LDA #%10000000
          TSB ZP_BLNKAUX      ; set ZP_BLNKAUX[7] = 1 (dir=R)
          JMP right
testr:    CMP #%00000001      ; BLINKEN reached far-right
          BNE testdir
          LDA #%10000000
          TRB ZP_BLNKAUX      ; set ZP_BLNKAUX[7] = 0 (dir=L)
          JMP left
testdir:  BIT ZP_BLNKAUX      ; S[neg] set to ZP_BLNKAUX[7]
          BMI right           ; branch if ZP_BLNKAUX[7] is 1 (dir=R)
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
          RTI

HandleNonMaskableInterrupt:
          RTI

.segment "vectors"

.word HandleNonMaskableInterrupt ; $FFFA: NMIB
.word HandleReset                ; $FFFC: RESB
.word HandleInterrupt            ; $FFFE: BRK/IRQB
