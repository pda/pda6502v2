SOURCES=$(wildcard *.s)
OBJECTS=$(SOURCES:.s=.o)

ASM      = ca65
ASMFLAGS = --cpu 65C02 --debug-info
LD      = ld65
LDFLAGS = --mapfile map.out --config memory.conf --dbgfile debug.out

.DUMMY: all

all: $(OBJECTS)
	$(LD) $(LDFLAGS) $^

%.o: %.s
	$(ASM) $(ASMFLAGS) -o $@ --listing $(@:.o=.list) $(@:.o=.s)

clean:
	$(RM) -- $(OBJECTS) map.out debug.out
