.segment "os"

.export UartMain
.export UartRxInterrupt

.export UART := $DC20

.export UART_MRA      = $0 ; read + write
.export UART_SRA      = $1 ; read
.export UART_CSRA     = $1 ; write
.export UART_CRA      = $2 ; write
.export UART_RXFIFOA  = $3 ; read
.export UART_TXFIFOA  = $3 ; write
.export UART_IPCR     = $4 ; read
.export UART_ACR      = $4 ; write
.export UART_ISR      = $5 ; read
.export UART_IMR      = $5 ; write
.export UART_CTU      = $6 ; read
.export UART_CTPU     = $6 ; write
.export UART_CTL      = $7 ; read
.export UART_CTPL     = $7 ; write
.export UART_MRB      = $8 ; read + write
.export UART_SRB      = $9 ; read
.export UART_CSRB     = $9 ; write
.export UART_CRB      = $A ; write
.export UART_RXFIFOB  = $B ; read
.export UART_TXFIFOB  = $B ; write
.export UART_MISC     = $C ; read + write
.export UART_IPR      = $D ; read
.export UART_OPCR     = $D ; write
.export UART_SOPR     = $E ; write
.export UART_ROPR     = $F ; write

.proc UartMain
                JSR UartConfigure
                JSR UartRxBufInit
                JSR UartHello
                JSR UartEcho
                RTS
.endproc

.proc UartConfigure
                LDA #%10110000          ; Select MR0A
                STA UART+UART_CRA

                LDA #%10001000          ; Mode Register 0 channel A (MR0A)
                    ; |||||||+---------->   0: baud rate extended I
                    ; ||||||+----------->   1: TEST2
                    ; |||||+------------>   2: baud rate extended II
                    ; ||||+------------->   3: FIFO size (8, 16)
                    ; ||++--------------> 5:4: TX Interrupt fill level
                    ; |+---------------->   6: RxINT[2] fill level
                    ; +----------------->   7: RxWATCHDOG
                STA UART+UART_MRA

                LDA #%00010000          ; Reset to MR1A
                STA UART+UART_CRA
                LDA #%11010011          ; Mode Register 1 channel A (MR1A)
                    ; ||||||++----------> 1:0: bits per char (5,6,7,*8*)
                    ; |||||+------------>   2: parity type (even, odd)
                    ; |||++-------------> 4:3: parity mode (with, force, *no*, multi)
                    ; ||+--------------->   5: error mode (char, block)
                    ; |+---------------->   6: RxINT[1]
                    ; +----------------->   7: RxRTS (also needs OPR[0]=1)
                STA UART+UART_MRA            ; auto advances to MR2A after this

                LDA #%00110111          ; Mode Register 2 channel A (MR2A)
                    ; ||||++++----------> 3:0 stop bit length (111=0x7: 1.000)
                    ; |||+-------------->   4 CTSN Enable Tx (1: CTS on)
                    ; ||+--------------->   5 RTSN Control Tx (1: RTS on)
                    ; ++----------------> 6:7 Channel mode (normal, echo, lloop, rloop)
                STA UART+UART_MRA            ; Mode Register 2 channel A (MR2A)

                LDA #%10111011          ; Clock Select Register
                    ; ||||++++----------> 3:0: TX baud (1011 w/ MR0[0]=0: 9,600)
                    ; ++++--------------> 7:4: RX baud
                STA UART+UART_CSRA

                LDA #%01100000          ; Auxiliary Control Register
                    ; ||||++++----------> 3:0: IP3:0 change interrupt enable
                    ; |+++--------------> 6:4: counter/timer mode/clock source (110=X1/CLK)
                    ; +----------------->   7: baud rate generator select
                STA UART+UART_ACR

                LDA #%00000001          ; Set Output Port bits Register (SOPR)
                    ;        +---------->   0: needed for MR1A[7] RxRTS=1
                STA UART+UART_SOPR

                LDA #%00000010          ; Interrupt mask register (IMR)
                    ; |||||||+---------->   0: TxRDYA
                    ; ||||||+----------->   1: RxRDYA / FFULLA
                    ; |||||+------------>   2: channel A change in break
                    ; ||||+------------->   3: counter ready
                    ; |||+-------------->   4: TxRDYB
                    ; ||+--------------->   5: RxRDYB
                    ; |+---------------->   6: channel B change in break
                    ; +----------------->   7: IP0[3:0] change (subject to ACR[3:0])
                STA UART+UART_IMR

                LDA #%00000101          ; Command Register A (CRA)
                    ;      | +---------->   0: enable RX
                    ;      +------------>   2: enable TX
                STA UART+UART_CRA

                RTS
.endproc

; UartRxBufInit initializes the in-memory buffer for UART receive data.  This
; is filled from the UART RX FIFO by an interrupt.
.proc UartRxBufInit
                LDA rxbuf_r             ; doesn't matter where rxbuf_r points...
                STA rxbuf_w             ; ... as long as rxbuf_w is the same.
                RTS
.endproc

; UartRxBufWrite queues the byte in A register that was previoulsy received by
; UART RX.
.proc UartRxBufWrite
                LDX rxbuf_w             ; load write pointer (next addr to write)
                STA rxbuf,X             ; store the RX byte in A into the buffer
                INC rxbuf_w             ; increment write pointer, with wrap-around
                RTS
.endproc

; UartRxBufRead pulls a byte from the in-memory RX buffer.
; If carry bit is set, blocks polling for available data first.
; If carry bit is clear, it is assumed the caller knows there is data available
; in the buffer, in which case result will be invalid if the buffer is empty.
.proc UartRxBufRead
                BCC no_poll
poll:           JSR UartRxBufLen
                BEQ poll
no_poll:        LDX rxbuf_r             ; load read pointer (first unread byte)
                LDA rxbuf,X             ; load RX byte from buffer into A
                INC rxbuf_r             ; increment read pointer, with wrap-around
                RTS                     ; return A: RX byte
.endproc

; UartRxBufLen calculates the length of data that has been pulled from UART RX
; FIFO to in-memory RX buffer but not yet read.
.proc UartRxBufLen
                LDA rxbuf_w             ; load write pointer
                SEC                     ; prepare carry bit for subtraction
                SBC rxbuf_r             ; subtract read pointer from write pointer
                RTS                     ; return A: length (and associated status flags)
.endproc

; UartRxInterrupt is triggered when UART RX FIFO has data (fill level reached,
; or watchdog timer elapsed).  All data in UART RX FIFO is pulled into the
; larger in-memory buffer, ready for UartRxBufRead. This keeps the UART FIFO
; more empty more often, increasing throughput.
.proc UartRxInterrupt
                PHA
                PHX
again:          LDA #1<<0               ; RxRDY: char is waiting in UART RX FIFO
                BIT UART+UART_SRA
                BEQ done                ; if RxRDY is 0, UART RX FIFO is empty
                JSR UartRxBufLen        ; A <- length of data in buffer
                CMP #220                ; in-memory buffer nearly full?
                BCS done                ; ... then don't pull a byte off the UART FIFO.
                ; TODO: ideally, de-assert RTS to tell the sender to stop
                ; transmitting, continue to pull UART FIFO into buffer, then
                ; re-assert RTS after the in-memory buffer is empty enough.
                LDA UART+UART_RXFIFOA   ; A <- FIFO
                JSR UartRxBufWrite      ; buf <- A
                JMP again
done:           PLX
                PLA
                RTS
.endproc

.proc UartHello
                LDX #0
msgloop:        LDA UART+UART_SRA
                AND #1<<2               ; TxRDYA
                BEQ msgloop             ; keep waiting

                LDA welcome,X
                BEQ msgdone
                STA UART+UART_TXFIFOA

                INX
                JMP msgloop
msgdone:        RTS
.endproc

.proc UartEcho
loop:           SEC                     ; UartRxBufRead blocking mode
                JSR UartRxBufRead       ; A <- byte
                TAX                     ; X <- A
                JSR UartPutc            ; UART TX <- X
                TXA
                CMP #$0D                ; if RX was CR
                BNE notcr
                LDX #$0A                ; then also send LF
                JSR UartPutc
notcr:          TXA
                CMP #$08                ; if RX was backspace, also send:
                BNE notbksp
                LDX #$20                ; space
                JSR UartPutc
                LDX #$08                ; backspace
                JSR UartPutc
notbksp:        JMP loop
                RTS
.endproc

; X: char to put on UART FIFO
.proc UartPutc
                LDA #1<<2               ; TxRDY: TX FIFO is not full
waitloop:       BIT UART+UART_SRA
                BEQ waitloop
                STX UART+UART_TXFIFOA
                RTS
.endproc

welcome:        .byte $0D, $0A, "Welcome to pda6502v2", $0D, $0A, "> ", $00

.segment "bss"

rxbuf:          .res 256                ; UART receive buffer; filled from UART RX FIFO by ISR
rxbuf_r:        .res 1                  ; read pointer (first addr not yet read)
rxbuf_w:        .res 1                  ; write pointer (next addr to be written)
