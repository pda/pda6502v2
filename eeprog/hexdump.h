#include <stdio.h>
#include <stdlib.h>

struct hexdump_context {
  FILE *output;
  uint32_t addr;
  uint8_t line_buf[2][16];
  uint8_t *curr, *prev;
  uint8_t i; // offset within 16-byte line
  uint8_t dedup, deduping; // bool
};

void hexdump_reset(struct hexdump_context *ctx);

void hexdump_byte(struct hexdump_context *, uint8_t);

void hexdump_finish(struct hexdump_context *);
