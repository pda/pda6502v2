.import BlinkenStart, BlinkenTick
.import UartRxInterrupt, UartTxInterrupt
.import ShellMain

.import BLINKEN, BLINKSRC
.import VIA1, VIA2, VIA_IFR : zp
.import UART, UART_ISR : zp, UART_MISC : zp;

.segment "os"

.proc Main
                LDA #$04                ; 0:reg 1:data 2:addrl 3:addrh 4:IRQ
                STA BLINKSRC
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
