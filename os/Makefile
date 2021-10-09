SOURCES=$(wildcard *.s)
OBJECTS=$(SOURCES:.s=.o)

ASM      = ca65
ASMFLAGS = --debug-info
LD      = ld65
LDFLAGS = --mapfile map.out --config memory.conf --dbgfile debug.out

.DUMMY: all

all: $(OBJECTS)
	$(LD) $(LDFLAGS) $^

%.o: %.s
	$(ASM) $(ASMFLAGS) -o $@ $(@:.o=.s)

clean:
	$(RM) -- $(OBJECTS) map.out debug.out
