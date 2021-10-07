.PHONY: test
test: blinken_tb boot_tb
	./blinken_tb
	./boot_tb

%_tb: %_tb.v %.v
	iverilog -Wall -o "$@" "$<"

%.json: %.v
	yosys -q -p "synth_ice40 -top $* -json $@" "$^"

%.asc: %.json %.pcf
	nextpnr-ice40 -q --hx4k --package tq144 --json $(filter %.json, $^) --pcf $(filter %.pcf, $^) --asc "$@"

%.bin: %.asc
	icepack -v $< $@

.PHONY: prog
prog: blinken.bin
	iceprog $<

.PHONY: synth
synth: blinken.json

.PHONY: pnr
pnr: blinken.asc

.PHONY: clean
clean:
	rm -f -- *_tb *.json *.asc *.bin

.PHONY: wave
wave:
	/Applications/gtkwave.app/Contents/Resources/bin/gtkwave blinken_tb.vcd &
