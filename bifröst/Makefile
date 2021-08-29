.PHONY: test
test: blinken_tb boot_tb
	./blinken_tb
	./boot_tb

%_tb: %_tb.v
	iverilog -Wall -o "$@" "$^"

%.json: %.v
	yosys -p "synth_ice40 -top $* -json $@" "$^"

%.asc: %.json bifröst.pcf
	nextpnr-ice40 --hx4k --package tq144 --json "$<" --pcf bifröst.pcf --asc "$@"

.PHONY: synth
synth: blinken.json

.PHONY: pnr
pnr: blinken.asc

.PHONY: clean
clean:
	rm -f -- *_tb *.json *.asc

.PHONY: wave
wave:
	/Applications/gtkwave.app/Contents/Resources/bin/gtkwave blinken_tb.vcd &
