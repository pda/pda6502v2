CC=clang
CFLAGS=-Wall
LDFLAGS=-lftdi1

all: eeprog

eeprog: eeprog.o hexdump.o

clean:
	rm -f -- eeprog *.o

.PHONY: all clean
