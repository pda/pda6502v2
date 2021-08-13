.PHONY: test
test: blinken_tb
	./blinken_tb

blinken_tb: blinken_tb.v
	iverilog -Wall -o $@ blinken_tb.v

blinken.json: blinken.v
	yosys -p "synth_ice40 -top blinken -json blinken.json" blinken.v

blinken.asc: blinken.json bifröst.pcf
	nextpnr-ice40 --hx4k --package tq144 --json blinken.json --pcf bifröst.pcf --asc blinken.asc

.PHONY: synth
synth: blinken.json

.PHONY: pnr
pnr: blinken.asc

.PHONY: clean
clean:
	rm -f -- blinken_tb blinken.json blinken.asc

.PHONY: wave
wave:
	/Applications/gtkwave.app/Contents/Resources/bin/gtkwave blinken_tb.vcd &
