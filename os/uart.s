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
          ; Mode Select Registers
          LDA #%10110000      ; Select MR0A
          STA UART+UART_CRA
          LDA #%10001000      ; Mode Register 0 channel A (MR0A)
          ;     |||||||+-------->   0: baud rate extended I
          ;     ||||||+--------->   1: TEST2
          ;     |||||+---------->   2: baud rate extended II
          ;     ||||+----------->   3: FIFO size (8, 16)
          ;     ||++------------> 5:4: TX Interrupt fill level
          ;     |+-------------->   6: RxINT[2] fill level
          ;     +--------------->   7: RxWATCHDOG
          STA UART+UART_MRA
          LDA #%00010000      ; Reset to MR1A
          STA UART+UART_CRA
          LDA #%11010011      ; Mode Register 1 channel A (MR1A)
          ;     ||||||++--------> 1:0: bits per char (5,6,7,*8*)
          ;     |||||+---------->   2: parity type (even, odd)
          ;     |||++-----------> 4:3: parity mode (with, force, *no*, multi)
          ;     ||+------------->   5: error mode (char, block)
          ;     |+-------------->   6: RxINT[1]
          ;     +--------------->   7: RxRTS (also needs OPR[0]=1)
          STA UART+UART_MRA        ; auto advances to MR2A after this
          LDA #%00110111      ; MR2A
          ;     ||||++++--------> 3:0 stop bit length (111=0x7: 1.000)
          ;     |||+------------>   4 CTSN Enable Tx (1: CTS on)
          ;     ||+------------->   5 RTSN Control Tx (1: RTS on)
          ;     ++--------------> 6:7 Channel mode (normal, echo, lloop, rloop)
          STA UART+UART_MRA        ; Mode Register 2 channel A (MR2A)
          LDA #%10111011      ; Clock Select Register
          ;     ||||++++--------> 3:0: TX baud (1011 w/ MR0[0]=0: 9,600)
          ;     ++++------------> 7:4: RX baud
          STA UART+UART_CSRA
          LDA #%01100000      ; Auxiliary Control Register
          ;     ||||++++--------> 3:0: IP3:0 change interrupt enable
          ;     |+++------------> 6:4: counter/timer mode/clock source (110=X1/CLK)
          ;     +--------------->   7: baud rate generator select
          STA UART+UART_ACR
          LDA #%00000001      ; Set Output Port bits Register (SOPR)
          ;            +-------->   0: needed for MR1A[7] RxRTS=1
          STA UART+UART_SOPR
          LDA #%00000010      ; Interrupt mask register (IMR)
          ;     |||||||+-------->   0: TxRDYA
          ;     ||||||+--------->   1: RxRDYA / FFULLA
          ;     |||||+---------->   2: channel A change in break
          ;     ||||+----------->   3: counter ready
          ;     |||+------------>   4: TxRDYB
          ;     ||+------------->   5: RxRDYB
          ;     |+-------------->   6: channel B change in break
          ;     +--------------->   7: IP0[3:0] change (subject to ACR[3:0])
          STA UART+UART_IMR
          LDA #%00000101      ; Command Register A (CRA)
          ;          | +-------->   0: enable RX
          ;          +---------->   2: enable TX
          STA UART+UART_CRA
          RTS
.endproc

.proc UartRxBufInit
          LDA rxbuf_r         ; doesn't matter where rxbuf_r points...
          STA rxbuf_w         ; ... as long as rxbuf_w is the same.
          RTS
.endproc

; input A: byte to write
.proc UartRxBufWrite
          LDX rxbuf_w
          STA rxbuf,X
          INC rxbuf_w
          RTS
.endproc

; output A: byte read
.proc UartRxBufRead
          LDX rxbuf_r
          LDA rxbuf,X
          INC rxbuf_r
          RTS
.endproc

; output A: length of data in buffer
.proc UartRxBufLen
          LDA rxbuf_w
          SEC
          SBC rxbuf_r
          RTS
.endproc

.proc UartRxInterrupt
          PHA
          PHX
again:    LDA #1<<0           ; RxRDY: char is waiting in RX FIFO
          BIT UART+UART_SRA
          BEQ done
          JSR UartRxBufLen    ; A <- len
          CMP #250            ; quite full?
          BCS done            ; then don't pull a byte off the UART FIFO
          LDA UART+UART_RXFIFOA ; A <- FIFO
          JSR UartRxBufWrite  ; buf <- A
          JMP again
done:     PLX
          PLA
          RTS
.endproc

; output A: byte from buffer
.proc UartBlockingReadFromBuffer
poll:     JSR UartRxBufLen
          CMP #0
          BEQ poll
          JSR UartRxBufRead   ; A <- buf
          RTS
.endproc

.proc UartHello
          LDX #0
msgloop:  LDA UART+UART_SRA
          AND #1<<2           ; TxRDYA
          BEQ msgloop         ; keep waiting

          LDA welcome,X
          BEQ msgdone
          STA UART+UART_TXFIFOA

          INX
          JMP msgloop
msgdone:  RTS
.endproc

.proc UartEcho
loop:     JSR UartBlockingReadFromBuffer          ; A <- byte
          TAX                                     ; X <- A
          JSR UartPutc                            ; UART TX <- X
          TXA
          CMP #$0D            ; if RX was CR
          BNE notcr
          LDX #$0A            ; then also send LF
          JSR UartPutc
notcr:    TXA
          CMP #$08            ; if RX was backspace, also send:
          BNE notbksp
          LDX #$20            ; space
          JSR UartPutc
          LDX #$08            ; backspace
          JSR UartPutc
notbksp:  JMP loop
          RTS
.endproc

; output X: char received
.proc UartGetcBlocking
          PHA
          LDA #1<<0           ; RxRDY: char is waiting in RX FIFO
poll:     BIT UART+UART_SRA
          BNE poll
          LDX UART+UART_RXFIFOA
          PLA
          RTS
.endproc

; X: char to put on UART FIFO
.proc UartPutc
          LDA #1<<2           ; TxRDY: TX FIFO is not full
waitloop: BIT UART+UART_SRA
          BEQ waitloop
          STX UART+UART_TXFIFOA
          RTS
.endproc

; delay for approx 5*X*Y cycles (max 326ms @ 1MHz)
.proc delayXY                 ; cycles:
          DEX                 ; 2*X*Y
          BNE delayXY         ; 3*X*Y + 2*Y
          DEY                 ; 2*Y
          BNE delayXY         ; 3*Y + 2
          RTS                 ; max: 2*255*255 + 3*255*255 + 2*255 + 2*255 + 3*255 + 2
                              ;      = 326,912 cycles (326 ms @ 1MHz)
.endproc

welcome:  .byte $0D, $0A, "Welcome to pda6502v2", $0D, $0A, "> ", $00

.segment "bss"

rxbuf:    .res 256            ; UART RX buffer (probably needs to be page-aligned?)
rxbuf_r:  .res 1              ; read pointer (first addr not yet read)
rxbuf_w:  .res 1              ; write pointer (next addr to be written)
