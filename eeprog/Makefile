CC=clang
CFLAGS=-Wall -g
LDFLAGS=-lftdi1

all: eeprog

eeprog: eeprog.o spi.o hexdump.o

clean:
	rm -f -- eeprog *.o

.PHONY: all clean
