.export SidInit
.export SidTick
.export SidTunes
.export SidPlay
.export SidPause

.importzp ZP_INTERRUPT
.importzp ZP_SID0, ZP_SID1, ZP_SID2, ZP_SID3
.import BLINKEN, BLINKSRC, ZP_BLINKENWAT : zp

.import VIA2
.importzp VIA_IER, VIA_ACR, VIA_T1CL, VIA_T1CH, VIA_IFR

.export SID  := $D400 ; (SID register names from to Mapping the Commodore 64, 1984)
.export FRELO1 = $00 ; Voice 1 Frequency Control (low byte)
.export FREHI1 = $01 ; Voice 1 Frequency Control (high byte)
.export PWLO1  = $02 ; Voice 1 Pulse Waveform Width (low byte)
.export PWHI1  = $03 ; Voice 1 Pulse Waveform Width (high nybble)
.export VCREG1 = $04 ; Voice 1 Control Register
.export ATDCY1 = $05 ; Voice 1 Attack/Decay Register
.export SUREL1 = $06 ; Voice 1 Sustain/Release Control Register
.export FRELO2 = $07 ; Voice 2 Frequency Control (low byte)
.export FREHI2 = $08 ; Voice 2 Frequency Control (high byte)
.export PWLO2  = $09 ; Voice 2 Pulse Waveform Width (low byte)
.export PWHI2  = $0A ; Voice 2 Pulse Waveform Width (high nybble)
.export VCREG2 = $0B ; Voice 2 Control Register
.export ATDCY2 = $0C ; Voice 2 Attack/Decay Register
.export SUREL2 = $0D ; Voice 2 Sustain/Release Control Register
.export FRELO3 = $0E ; Voice 3 Frequency Control (low byte)
.export FREHI3 = $0F ; Voice 3 Frequency Control (high byte)
.export PWLO3  = $10 ; Voice 3 Pulse Waveform Width (low byte)
.export PWHI3  = $11 ; Voice 3 Pulse Waveform Width (high nybble)
.export VCREG3 = $12 ; Voice 3 Control Register
.export ATDCY3 = $13 ; Voice 3 Attack/Decay Register
.export SUREL3 = $14 ; Voice 3 Sustain/Release Control Register
.export CUTLO  = $15 ; [2:0] Low portion of filter cutoff frequency
.export CUTHI  = $16 ; Filter Cutoff Frequency (high byte)
.export RESON  = $17 ; Filter Resonance Control Register
.export SIGVOL = $18 ; Volume and Filter Select Register
.export POTX   = $19 ; Read Game Paddle 1 (or 3) Position
.export POTY   = $20 ; Read Game Paddle 2 (or 4) Position
.export RANDOM = $1B ; Read Oscillator 3/Random Number Generator
.export ENV3   = $1C ; Envelope Generator 3 Output

C_0 = $00 ; 32.7 Hz
Cs0 = $01 ; 34.644342 Hz
D_0 = $02 ; 36.70429458 Hz
Ds0 = $03 ; 38.88673193 Hz
E_0 = $04 ; 41.19893701 Hz
F_0 = $05 ; 43.64862581 Hz
Fs0 = $06 ; 46.2439731 Hz
G_0 = $07 ; 48.99363974 Hz
Gs0 = $08 ; 51.90680156 Hz
A_0 = $09 ; 54.99317998 Hz
As0 = $0A ; 58.26307446 Hz
B_0 = $0B ; 61.72739686 Hz
C_1 = $0C ; 65.39770788 Hz
Cs1 = $0D ; 69.28625559 Hz
D_1 = $0E ; 73.40601635 Hz
Ds1 = $0F ; 77.77073808 Hz
E_1 = $10 ; 82.39498617 Hz
F_1 = $11 ; 87.29419205 Hz
Fs1 = $12 ; 92.48470471 Hz
G_1 = $13 ; 97.98384525 Hz
Gs1 = $14 ; 103.8099647 Hz
A_1 = $15 ; 109.9825052 Hz
As1 = $16 ; 116.5220649 Hz
B_1 = $17 ; 123.4504669 Hz
C_2 = $18 ; 130.7908317 Hz
Cs2 = $19 ; 138.5676545 Hz
D_2 = $1A ; 146.8068873 Hz
Ds2 = $1B ; 155.5360248 Hz
E_2 = $1C ; 164.7841968 Hz
F_2 = $1D ; 174.5822652 Hz
Fs2 = $1E ; 184.9629267 Hz
G_2 = $1F ; 195.9608223 Hz
Gs2 = $20 ; 207.6126528 Hz
A_2 = $21 ; 219.9573011 Hz
As2 = $22 ; 233.0359622 Hz
B_2 = $23 ; 246.8922805 Hz
C_3 = $24 ; 261.5724956 Hz
Cs3 = $25 ; 277.1255961 Hz
D_3 = $26 ; 293.6034841 Hz
Ds3 = $27 ; 311.0611472 Hz
E_3 = $28 ; 329.5568431 Hz
F_3 = $29 ; 349.1522929 Hz
Fs3 = $2A ; 369.9128883 Hz
G_3 = $2B ; 391.9079086 Hz
Gs3 = $2C ; 415.2107529 Hz
A_3 = $2D ; 439.8991842 Hz
As3 = $2E ; 466.0555897 Hz
B_3 = $2F ; 493.7672551 Hz
C_4 = $30 ; 523.1266561 Hz
Cs4 = $31 ; 554.2317671 Hz
D_4 = $32 ; 587.1863879 Hz
Ds4 = $33 ; 622.1004906 Hz
E_4 = $34 ; 659.0905857 Hz
F_4 = $35 ; 698.2801119 Hz
Fs4 = $36 ; 739.7998474 Hz
G_4 = $37 ; 783.7883463 Hz
Gs4 = $38 ; 830.3924014 Hz
A_4 = $39 ; 879.7675336 Hz
As4 = $3A ; 932.0785111 Hz
B_4 = $3B ; 987.4998994 Hz
C_5 = $3C ; 1046.216643 Hz
Cs5 = $3D ; 1108.424685 Hz
D_5 = $3E ; 1174.331617 Hz
Ds5 = $3F ; 1244.157375 Hz
E_5 = $40 ; 1318.134972 Hz
F_5 = $41 ; 1396.511278 Hz
Fs5 = $42 ; 1479.547838 Hz
G_5 = $43 ; 1567.521753 Hz
Gs5 = $44 ; 1660.726596 Hz
A_5 = $45 ; 1759.4734 Hz
As5 = $46 ; 1864.091688 Hz
B_5 = $47 ; 1974.93058 Hz
C_6 = $48 ; 2092.359952 Hz
Cs6 = $49 ; 2216.771675 Hz
D_6 = $4A ; 2348.580918 Hz
Ds6 = $4B ; 2488.22754 Hz
E_6 = $4C ; 2636.177549 Hz
F_6 = $4D ; 2792.924666 Hz
Fs6 = $4E ; 2958.991967 Hz
G_6 = $50 ; 3134.93363 Hz
Gs6 = $4F ; 3321.336783 Hz
A_6 = $51 ; 3518.823468 Hz
As6 = $52 ; 3728.052712 Hz
B_6 = $53 ; 3949.722726 Hz

.segment "os"

; Thanks https://www.youtube.com/watch?v=kxc46GNVDIk
scale_lo:       ;     C    C#   D    D#   E    F    F#   G    G#   A    A#   B
oct0_lo:        .byte $18, $38, $5A, $7E, $A4, $CC, $F7, $24, $53, $86, $BC, $F5
oct1_lo:        .byte $31, $71, $B4, $FC, $48, $98, $ED, $47, $A7, $0C, $77, $E9
oct2_lo:        .byte $62, $E1, $68, $F7, $8F, $30, $DA, $8F, $4E, $18, $EF, $D2
oct3_lo:        .byte $C3, $C2, $D0, $EF, $1E, $60, $B4, $1D, $9B, $30, $DD, $A4
oct4_lo:        .byte $85, $84, $A0, $DD, $3C, $BF, $68, $39, $36, $60, $BA, $47
oct5_lo:        .byte $0A, $07, $40, $B9, $77, $7D, $CF, $72, $6B, $BF, $73, $8D
oct6_lo:        .byte $13, $0C, $7F, $71, $EC, $F8, $9C, $E2, $D4, $7B, $E4, $18
scale_hi:       ;     C    C#   D    D#   E    F    F#   G    G#   A    A#   B
oct0_hi:        .byte $02, $02, $02, $02, $02, $02, $02, $03, $03, $03, $03, $03
oct1_hi:        .byte $04, $04, $04, $04, $05, $05, $05, $06, $06, $07, $07, $07
oct2_hi:        .byte $08, $08, $09, $09, $0A, $0B, $0B, $0C, $0D, $0E, $0E, $0F
oct3_hi:        .byte $10, $11, $12, $13, $15, $16, $17, $19, $1A, $1C, $1D, $1F
oct4_hi:        .byte $21, $23, $25, $27, $2A, $2C, $2F, $32, $35, $38, $3B, $3F
oct5_hi:        .byte $43, $47, $4B, $4F, $54, $59, $5E, $64, $6A, $70, $77, $7E
oct6_hi:        .byte $86, $8E, $96, $9F, $A8, $B2, $BD, $C8, $D4, $E1, $EE, $FD

tune:
                ;.byte C_0, D_0, E_0, F_0, G_0, A_0, B_0, C_1
                .byte C_1, D_1, E_1, F_1, G_1, A_1, B_1, C_2
                .byte C_2, B_1, A_1, G_1, F_1, E_1, D_1, C_1
                ;.byte C_1, B_0, A_0, G_0, F_0, E_0, D_0, C_0
                .byte $FF ; terminator

.proc SidInit
                LDA #0
                STA ZP_BLINKENWAT
                STA BLINKSRC
                STA BLINKEN

                LDA #$00
                STA SID+SIGVOL          ; turn off SID main volume (and filters)

                LDA #$00
                STA ZP_SID0
                STA ZP_SID1
                STA ZP_SID2
                STA ZP_SID3

                RTS
.endproc

.proc SidPlay
                ; experimenting.. counter
                LDA #10
                STA ZP_SID0

                LDA #%00011111
                    ; ||||++++----------> main volume 0..15
                    ; |||+--------------> 4: lowpass
                    ; ||+---------------> 5: band pass
                    ; |+----------------> 6: high pass
                    ; +-----------------> 7: mute voice 3
                STA SID+SIGVOL

                LDA #%00000000
                    ;      +++----------> filter cutoff frequency low bits 2:0
                STA SID+CUTLO
                LDA #$08
                STA SID+CUTHI           ; filter cutoff frequency high bits 10:3

                LDA #%00000001
                    ; |||||||+----------> filter voice 1?
                    ; ||||||+-----------> filter voice 2?
                    ; |||||+------------> filter voice 3?
                    ; ||||+-------------> filter external?
                    ; ++++--------------> filter resonance
                STA SID+RESON

                ; ADSR (Attack, Decay, Sustain, Release)
                ; Delay map from SID register nybbles to milliseconds (or seconds).
                ; Values are from Mapping the Commodore 64, 1984.
                ;          0  1  2  3   4   5   6   7   8   9    A    B  C  D   E   F
                ;  attack: 2  8 16 24  38  56  68  80 100 250  500  800 1s 3s  5s  8s
                ;   decay: 6 24 48 72 114 168 204 240 300 750 1500 2400 3s 9s 15s 24s
                ; release: 6 24 48 72 114 168 204 240 300 750 1500 2400 3s 9s 15s 24s
                LDA #$22                ; attack, decay
                STA SID+ATDCY1
                LDA #$A8                ; sustain, release
                STA SID+SUREL1


                LDA #%11000000
                    ; |+----------------> 6: Timer 1 (T1)
                    ; +-----------------> 7: enable/disable selected interrupts
                STA VIA2+VIA_IER
                LDA VIA2+VIA_ACR        ; 7:6: T1 timer control
                AND #%01111111          ; ACR[7] = 0 (PB6 disabled)
                ORA #%01000000          ; ACR[6] = 1 (Continuous interrupts)
                STA VIA2+VIA_ACR
                irqfreq = 16666         ; 1 MHz / 16666 = 60 times per second
                LDA #<irqfreq
                STA VIA2+VIA_T1CL
                LDA #>irqfreq
                STA VIA2+VIA_T1CH       ; trigger T1 counter
                RTS
.endproc

.proc SidPause
                LDA #%00000000
                    ; ||||++++----------> main volume 0..15
                    ; |||+--------------> 4: lowpass
                    ; ||+---------------> 5: band pass
                    ; |+----------------> 6: high pass
                    ; +-----------------> 7: mute voice 3
                STA SID+SIGVOL

                LDA #1<<6               ; T1 continuous interrupt
                TRB VIA2+VIA_ACR        ; reset (clear) bit to disable
                RTS
.endproc

; Called frequently by interrupt handler
.proc SidTick
                SEI
                LDA #%01000000
                    ;  +----------------> 6: clear Timer 1 (T1) interrupt
                STA VIA2+VIA_IFR

                DEC ZP_SID0
                BNE return
                LDA #10
                STA ZP_SID0

                ;INC BLINKEN seems to be reading 0, always setting 1
                LDX ZP_BLINKENWAT
                INX
                STX ZP_BLINKENWAT
                STX BLINKEN

                INC ZP_SID2
                LDA ZP_SID2
                AND #%00000001
                BEQ attack
                JMP release
attack:         JSR SidAttack
                JMP played
release:        JSR SidRelease
played:
return:
                CLI
                RTS
.endproc

.proc SidAttack
                LDX ZP_SID1             ; X will be index into tune data
                LDA tune,X              ; A <- index into scale array
                TAY                     ; Y <- A to use as index into scale data
                LDA scale_lo,Y          ; A <- low byte of frequency for this note
                STA SID+FRELO1
                LDA scale_hi,Y          ; A <- high byte of frequency for this note
                STA SID+FREHI1
                LDA #%00100001
                    ; NPST
                    ; |||||||+----------> gate
                    ; ||||||+-----------> sync with voice 3
                    ; |||||+------------> ring modulation with voice 3
                    ; ||||+-------------> "test"
                    ; |||+--------------> triangle
                    ; ||+---------------> sawtooth
                    ; |+----------------> pulse
                    ; +-----------------> noise
                STA SID+VCREG1
                RTS
.endproc

.proc SidRelease
                LDA #%00100000
                    ; NPST
                    ; |||||||+----------> gate
                    ; ||||||+-----------> sync with voice 3
                    ; |||||+------------> ring modulation with voice 3
                    ; ||||+-------------> "test"
                    ; |||+--------------> triangle
                    ; ||+---------------> sawtooth
                    ; |+----------------> pulse
                    ; +-----------------> noise
                STA SID+VCREG1          ; close voice 1 gate

                LDX ZP_SID1
                INX                     ; next note in tune
                LDA tune,X
                CMP #$FF
                BNE storesid1
                LDX #0
storesid1:      STX ZP_SID1
                RTS
.endproc

.proc SidTunes
                RTS
.endproc
