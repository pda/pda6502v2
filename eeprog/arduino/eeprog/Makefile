FQBN=arduino:samd:arduino_zero_edbg
PORT=/dev/cu.usbmodem144102

compile:
	arduino-cli compile --fqbn ${FQBN} --warnings all

upload:
	arduino-cli upload --fqbn ${FQBN} --port ${PORT}
