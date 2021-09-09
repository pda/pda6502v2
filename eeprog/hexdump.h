#include <stdio.h>
#include <stdlib.h>

struct hexdump_context {
  FILE *output;
  uint32_t addr;
  char asciibuf[16];
  int i;
};

void hexdump_byte(struct hexdump_context *, uint8_t);

void hexdump_finish(struct hexdump_context *);
