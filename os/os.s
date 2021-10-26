.import BlinkenStart, BlinkenTick
.import UartMain, UartRxInterrupt

.import BLINKEN, BLINKSRC
.import VIA1, VIA2, VIA_IFR : zp
.import UART, UART_ISR : zp;

.segment "os"

.PROC Main
          ;JSR BlinkenStart
          LDA #$04           ; 0:reg 1:data 2:addrl 3:addrh 4:IRQ
          STA BLINKSRC
          JSR UartMain
halt:     JMP halt
.ENDPROC


.PROC HandleReset
          SEI                 ; mask interrupts during start-up
          LDX #$FF            ;
          TXS                 ; set stack pointer to $ff ($01FF)
          CLI                 ; resume interrupts
          CLD                 ; don't be in crazy decimal mode.
          JMP Main
.ENDPROC

.PROC HandleInterrupt
          PHA
          BIT VIA1+VIA_IFR
          BPL after_via1      ; not VIA1 interrupt
          BVC after_via1_t1   ; not VIA1 T1 interrupt
          JSR BlinkenTick     ; VIA1 T1 animates BLINKEN
after_via1_t1:
after_via1:
          BIT VIA2+VIA_IFR
          BPL after_via2      ; not VIA2 interrupt
after_via2:
          LDA #%00000010
          AND UART+UART_ISR
          BEQ after_uart      ; not UART RX interrupt
          JSR UartRxInterrupt
after_uart:
          PLA
          RTI
.ENDPROC

.PROC HandleNonMaskableInterrupt
          RTI
.ENDPROC

.segment "vectors"

.word HandleNonMaskableInterrupt ; $FFFA: NMIB
.word HandleReset                ; $FFFC: RESB
.word HandleInterrupt            ; $FFFE: BRK/IRQB
