.import BlinkenStart, BlinkenTick
.import UartRxInterrupt, UartTxInterrupt
.import ShellMain
.import SidInit

.import BLINKEN, BLINKSRC
.import VIA1, VIA2, VIA_IFR : zp
.import UART, UART_ISR : zp, UART_MISC : zp
.importzp ZP_INTERRUPT
.import SidTick

.segment "os"

.proc Main
                LDA #$00
                STA ZP_INTERRUPT        ; Clear the user-mode interrupt flag(s)
                LDA #$04
                STA BLINKSRC            ; 0:reg 1:data 2:addrl 3:addrh 4:IRQ
                JSR SidInit
                JSR ShellMain
halt:           JMP halt
.endproc


.proc HandleReset
                SEI                     ; mask interrupts during start-up
                LDX #$FF                ;
                TXS                     ; set stack pointer to $ff ($01FF)
                CLI                     ; resume interrupts
                CLD                     ; don't be in crazy decimal mode.
                JMP Main
.endproc

.proc HandleInterrupt
                PHA
                BIT VIA1+VIA_IFR
                BVS v1t1                ; VIA1 T1 if V=1 from BIT:6
                BIT VIA2+VIA_IFR
                BVS v2t1                ; VIA2 T1 if V=1 from BIT:6
                LDA #1<<0               ; TxRDYA
                AND UART+UART_MISC      ; readable copy of UART Interrupt Mask Register
                AND UART+UART_ISR       ; UART Interrupt Status Register
                BNE uarttx
                LDA #1<<1               ; RxRDYA
                AND UART+UART_MISC      ; readable copy of UART Interrupt Mask Register
                AND UART+UART_ISR       ; UART Interrupt Status Register
                BNE uartrx
                JMP done

v1t1:           JSR BlinkenTick
                JMP done
v2t1:           JSR SidTick
                JMP done
uartrx:         JSR UartRxInterrupt
                JMP done
uarttx:         JSR UartTxInterrupt
                JMP done
done:           PLA
                RTI
.endproc

.proc HandleNonMaskableInterrupt
                RTI
.endproc

.segment "vectors"

.word HandleNonMaskableInterrupt        ; $FFFA: NMIB
.word HandleReset                       ; $FFFC: RESB
.word HandleInterrupt                   ; $FFFE: BRK/IRQB
