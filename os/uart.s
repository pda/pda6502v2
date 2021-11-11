.segment "os"

.export UartInit
.export UartRxBufRead
.export UartTxBufWrite
.export UartTxBufWriteBlocking
.export UartNewline
.export UartTxStr
.export UartRxInterrupt
.export UartTxInterrupt

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

.proc UartInit
                JSR UartRxBufInit
                JSR UartTxBufInit
                JSR UartConfigure
                RTS
.endproc

.proc UartConfigure
                LDA #%10110000          ; Select MR0A
                STA UART+UART_CRA

                LDA #%10001100          ; Mode Register 0 channel A (MR0A)
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

                LDA #%01100110          ; Clock Select Register
                    ; ||||++++----------> 3:0: TX baud (0110 w/ MR0[2]=1: 115,200)
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
                STA UART+UART_MISC      ; Maintain readable copy of IMR in MISC register
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

; UartTxBufInit initialised the in-memory buffer for UART transmit data.  This
; is flushed to the UART TX FIFO by an interrupt.
.proc UartTxBufInit
                LDA txbuf_r             ; doesn't matter where txbuf_r points...
                STA txbuf_w             ; ... as long as txbuf_w is the same.
                RTS
.endproc

; UartTxBufWrite queues the byte in A register to be written to UART TX FIFO.
; UART interrupts for TxRDY are enabled.
; If carry bit is set, blocks polling for space on txbuf first.
; If carry bit is clear, it is assumed the caller knows there is space available
; in the buffer, in which case the buffer may be corrupted.
.proc UartTxBufWrite
                PHA
                PHX
                LDX txbuf_w             ; load write pointer (next addr to write)
                STA txbuf,X             ; store the TX byte in A into the buffer
                INC txbuf_w             ; increment write pointer, with wrap-around
                LDA UART+UART_MISC      ; readable copy of IMR
                ORA #1<<0               ; UART_IMR TxRDYA bit
                STA UART+UART_MISC      ; maintain readable copy of IMR
                STA UART+UART_IMR       ; Interrupt when UART TX FIFO is below fill level
                PLX
                PLA
                RTS
.endproc

; UartTxBufWriteBlocking blocks until there is space on txbuf,
; and then calls UartTxBufWrite
.proc UartTxBufWriteBlocking
                PHA
waittxbuf:      JSR UartTxBufLen
                CMP #$FF
                BEQ waittxbuf
                PLA
                JMP UartTxBufWrite
.endproc

; UartTxBufRead pulls a byte from txbuf into A.
; Generally called by UartTxInterrupt during TxRDY interrupt.
; X register is not preserved.
.proc UartTxBufRead
                LDX txbuf_r             ; load read pointer (first unread byte)
                LDA txbuf,X             ; load TX byte from buffer into A
                INC txbuf_r             ; increment read pointer, with wrap-around
                RTS                     ; return A: TX byte
.endproc

; UartRxBufLen calculates the length of data that has been buffered to the
; in-memory TX buffer but not yet flushed to UART TX FIFO.
.proc UartTxBufLen
                LDA txbuf_w             ; load write pointer
                SEC                     ; prepare carry bit for subtraction
                SBC txbuf_r             ; subtract read pointer from write pointer
                RTS                     ; resulting length returned in A register.
.endproc

; UartNewline sends CR/LF after waiting for space on txbuf.
.proc UartNewline
                PHA
waittxbuf:      JSR UartTxBufLen        ; A <- len
                SEC
                SBC #2                  ; at least 2 chars free
                BCS waittxbuf
                LDA #$0D
                JSR UartTxBufWrite
                LDA #$0A
                JSR UartTxBufWrite
                PLA
                RTS
.endproc

; UartTxStr copies null-terminated string to txbuf.
; X,Y: string pointer
.proc UartTxStr
                PHA
                PHX
                PHY
                LDA $00                 ; preserve $00 zero-page word...
                PHA                     ; (this is probably a terrible calling convention,
                LDA $01                 ; and I should just reserve a ZP word for this)
                PHA
                STX $00                 ; X: *string low byte
                STY $01                 ; Y: *string high byte
waittxbuf:      JSR UartTxBufLen        ; Wait for space on txbuf
                BNE waittxbuf
                SEI                     ; mask IRQ so UartTxInterrupt only fires once at the end
                LDY #0
msgloop:        LDA ($00),Y             ; string[Y]
                BEQ msgdone             ; terminate on null byte
                JSR UartTxBufWrite
                INY
                JMP msgloop
msgdone:        CLI                     ; unmask interrupts
                PLA                     ; restore $00 zero-page word...
                STA $01
                PLA
                STA $00
                PLY
                PLX
                PLA
                RTS
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
                JSR UartRxBufWrite      ; rxbuf <- A
                JMP again
done:           PLX
                PLA
                RTS
.endproc

.proc UartTxInterrupt
                PHA
                PHX
again:          JSR UartTxBufLen        ; A <- txbuf length
                BEQ empty
waittxready:    LDA UART+UART_SRA       ; Is UART ready? Load UART status register...
                AND #1<<2               ; SRA TxRDY: check TX FIFO is not full
                BEQ waittxready         ; zero means it's not ready (full), wait.
                JSR UartTxBufRead       ; A <- txbuf (kills X)
                STA UART+UART_TXFIFOA   ; UART FIFO <- A
                LDA UART+UART_SRA       ; Flush another byte? Load UART status register...
                AND #1<<2               ; SRA TxRDY: check TX FIFO is not full
                BNE again               ; If TxRDY, check for another byte to flush to FIFO
                JMP done                ; else stop here, but leave the TxRDY interrupt enabled.
empty:          LDA UART+UART_MISC      ; readable copy of Interrupt Mask Register
                AND #<~1<<0             ; clear UART_IMR TxRDYA bit
                STA UART+UART_MISC      ; Maintain readable copy of IMR in MISC register
                STA UART+UART_IMR       ; disable source of this interrupt, no data to TX
done:           PLX
                PLA
                RTS
.endproc

; UartEcho loops forever echo UART RX bytes back to UART TX.
; Some extra character handling is done for newline, backspace etc.
.proc UartEcho
loop:           SEC                     ; UartRxBufRead blocking mode
                JSR UartRxBufRead       ; A <- rxbuf
                TAY                     ; Y <- A (spare copy because A is destroyed by UartTxBufWrite)
                JSR UartTxBufWrite      ; txbuf <- A
                TYA
                CMP #$0D                ; if RX was CR
                BNE notcr
                LDA #$0A                ; then also send LF
                JSR UartTxBufWrite
notcr:          TYA
                CMP #$08                ; if RX was backspace, also send:
                BNE notbksp
                LDA #$20                ; space
                JSR UartTxBufWrite
                LDA #$08                ; backspace
                JSR UartTxBufWrite
notbksp:        JMP loop
                RTS
.endproc

.segment "bss"

rxbuf:          .res 256                ; UART receive buffer; filled from UART RX FIFO by ISR
rxbuf_r:        .res 1                  ; read pointer (first addr not yet read)
rxbuf_w:        .res 1                  ; write pointer (next addr to be written)

txbuf:          .res 256                ; UART transmit buffer; drained to UART TX FIFO by ISR
txbuf_r:        .res 1                  ; read pointer (first addr not yet sent to TX FIFO)
txbuf_w:        .res 1                  ; write pointer (next addr to be filled from TX FIFO)
