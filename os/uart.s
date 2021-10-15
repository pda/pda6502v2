.segment "os"

.export UartMain

.global BLINKEN

UART_MRA      = $DC20 ; read + write
UART_SRA      = $DC21 ; read
UART_CSRA     = $DC21 ; write
UART_CRA      = $DC22 ; write
UART_RXFIFOA  = $DC23 ; read
UART_TXFIFOA  = $DC23 ; write
UART_IPCR     = $DC24 ; read
UART_ACR      = $DC24 ; write
UART_ISR      = $DC25 ; read
UART_IMR      = $DC25 ; write
UART_CTU      = $DC26 ; read
UART_CTPU     = $DC26 ; write
UART_CTL      = $DC27 ; read
UART_CTPL     = $DC27 ; write
UART_MRB      = $DC28 ; read + write
UART_SRB      = $DC29 ; read
UART_CSRB     = $DC29 ; write
UART_CRB      = $DC2A ; write
UART_RXFIFOB  = $DC2B ; read
UART_TXFIFOB  = $DC2B ; write
UART_MISC     = $DC2C ; read + write
UART_IPR      = $DC2D ; read
UART_OPCR     = $DC2D ; write
UART_SOPR     = $DC2E ; write
UART_ROPR     = $DC2F ; write

.PROC UartMain
          JSR UartConfigure
          JSR UartHello
forever:  JSR UartEcho
          LDA UART_SRA
          STA BLINKEN
          JMP forever
.ENDPROC

.PROC UartConfigure
          ; Mode Select Registers
          LDA #%10110000      ; Select MR0A
          STA UART_CRA
          LDA #%00001000      ; Mode Register 0 channel A (MR0A)
          ;     |||||||+-------->   0: baud rate extended I
          ;     ||||||+--------->   1: TEST2
          ;     |||||+---------->   2: baud rate extended II
          ;     ||||+----------->   3: FIFO size (8, 16)
          ;     ||++------------> 5:4: TX Interrupt fill level
          ;     |+-------------->   6: RxINT[2] fill level
          ;     +--------------->   7: RxWATCHDOG
          STA UART_MRA
          LDA #%00010000      ; Reset to MR1A
          STA UART_CRA
          LDA #%10010011      ; Mode Register 1 channel A (MR1A)
          ;     ||||||++--------> 1:0: bits per char (5,6,7,*8*)
          ;     |||||+---------->   2: parity type (even, odd)
          ;     |||++-----------> 4:3: parity mode (with, force, *no*, multi)
          ;     ||+------------->   5: error mode (char, block)
          ;     |+-------------->   6: RxINT[1]
          ;     +--------------->   7: RxRTS (also needs OPR[0]=1)
          STA UART_MRA        ; auto advances to MR2A after this
          LDA #%00110111      ; MR2A
          ;     ||||++++--------> 3:0 stop bit length (111=0x7: 1.000)
          ;     |||+------------>   4 CTSN Enable Tx (1: CTS on)
          ;     ||+------------->   5 RTSN Control Tx (1: RTS on)
          ;     ++--------------> 6:7 Channel mode (normal, echo, lloop, rloop)
          STA UART_MRA        ; Mode Register 2 channel A (MR2A)
          LDA #%10111011      ; Clock Select Register
          ;     ||||++++--------> 3:0: TX baud (1011 w/ MR0[0]=0: 9,600)
          ;     ++++------------> 7:4: RX baud
          STA UART_CSRA
          LDA #%01100000      ; Auxiliary Control Register
          ;     ||||++++--------> 3:0: IP3:0 change interrupt enable
          ;     |+++------------> 6:4: counter/timer mode/clock source (110=X1/CLK)
          ;     +--------------->   7: baud rate generator select
          STA UART_ACR
          LDA #%00000001      ; Set Output Port bits Register (SOPR)
          ;            +-------->   0: needed for MR1A[7] RxRTS=1
          STA UART_SOPR
          LDA #%00000101      ; Command Register A (CRA)
          ;          | +-------->   0: enable RX
          ;          +---------->   2: enable TX
          STA UART_CRA
          RTS
.ENDPROC

.PROC UartHello
          LDX #0
msgloop:  LDA UART_SRA
          AND #1<<2           ; TxRDYA
          BEQ msgloop         ; keep waiting

          LDA message,X
          BEQ msgdone
          STA UART_TXFIFOA

          LDA UART_SRA
          STA BLINKEN

          INX
          JMP msgloop
msgdone:  RTS
.ENDPROC

.PROC UartEcho
          LDA #1<<0           ; RxRDY: char is waiting in RX FIFO
          BIT UART_SRA
          BEQ empty
          LDX UART_RXFIFOA
          JSR UartPutc        ; TX the RX char
          TXA
          CMP #$0D            ; if RX was CR
          BNE notcr
          LDX #$0A            ; then also send LF
          JSR UartPutc
notcr:    TXA
          CMP #$08            ; if RX was backspace, also send:
          BNE notbs
          LDX #$20            ; space
          JSR UartPutc
          LDX #$08            ; backspace
          JSR UartPutc
notbs:    JMP UartEcho        ; check for more
empty:    RTS
.ENDPROC

; X: char to put on UART FIFO
.PROC UartPutc
          LDA #1<<2           ; TxRDY: TX FIFO is not full
waitloop: BIT UART_SRA
          BEQ waitloop
          STX UART_TXFIFOA
          RTS
.ENDPROC

message: .BYTE $0D, $0A, "Welcome to pda6502v2", $0D, $0A, "> ", $00

; delay for approx 5*X*Y cycles (max 326ms @ 1MHz)
.PROC delayXY                 ; cycles:
          DEX                 ; 2*X*Y
          BNE delayXY         ; 3*X*Y + 2*Y
          DEY                 ; 2*Y
          BNE delayXY         ; 3*Y + 2
          RTS                 ; max: 2*255*255 + 3*255*255 + 2*255 + 2*255 + 3*255 + 2
                              ;      = 326,912 cycles (326 ms @ 1MHz)
.ENDPROC

; delay for approx 5*Y cycles
.PROC delayY                 ; cycles:
          DEY
          BNE delayY
          RTS
.ENDPROC


yesplz:   PHX
          LDX $E012           ; weekend counter
          INX                 ; moar weekend!
          STX $E012
          PLX
          RTS
