SID := $D400

.export SidBleep

.segment "os"

; Thanks https://www.youtube.com/watch?v=kxc46GNVDIk
; Octave #3:    C    C#   D    D#   E    F    F#   G    G#   A    A#   B
scale_lo: .byte $C3, $C2, $D0, $EF, $1E, $60, $B4, $1D, $9B, $30, $DD, $A4
scale_hi: .byte $10, $11, $12, $13, $15, $16, $17, $19, $1A, $1C, $1D, $1F

tune: .byte $00, $01, $02, $03, $04, $05, $06, $07, $08, $09, $0A, $0B
      .byte $FF ; terminator

.proc SidBleep
again:
                LDA #%00001111
                    ; |+----------------> main volume 0..15
                    ; +-----------------> mute voice 3, high pass, band pass, lowpass
                STA SID+$18

                ; ADSR (Attack, Decay, Sustain, Release)
                LDA #$00
                    ; |+----------------> decay duration (voice 1)
                    ; +-----------------> attack duration
                STA SID+$05
                LDA #$91
                    ; |+----------------> release duration
                    ; +-----------------> sustain level
                STA SID+$06

                LDX #0
eachnote:
                LDY tune,X              ; Y <- index into scale array
                BMI return
                LDA scale_lo,Y          ; A <- low byte of frequency for this note
                STA SID+$00             ; frequency voice 1 low byte
                LDA scale_hi,Y          ; A <- high byte of frequency for this note
                STA SID+$01             ; frequency voice 1 high byte

                LDA #%00010001
                    ; NPST
                    ; |||||||+----------> gate
                    ; ||||||+-----------> sync with voice 3
                    ; |||||+------------> ring modulation with voice 3
                    ; ||||+-------------> "test"
                    ; |||+--------------> triangle
                    ; ||+---------------> sawtooth
                    ; |+----------------> pulse
                    ; +-----------------> noise
                STA SID+$04             ; control register voice 1

                PHX
                LDX #$FF
                LDY #$20
delay:          DEX
                BNE delay
                DEY
                BNE delay
                PLX

                LDA #1<<0
                TRB SID+$04             ; close voice 1 gate

                INX
                JMP eachnote

return:         RTS
.endproc
