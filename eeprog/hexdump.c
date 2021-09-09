#include <stdio.h>
#include <strings.h>

#include "hexdump.h"

void hexdump_byte(struct hexdump_context *ctx, uint8_t byte) {
  FILE * out = ctx->output;
  ctx->asciibuf[ctx->i%16] = (byte >= 32 && byte <= 126) ? byte : '.';
  if (ctx->i % 16 == 0) fprintf(out, "%08x  ", ctx->addr+ctx->i);
  fprintf(out, "%02x", byte);
  if (ctx->i % 16 == 7) fprintf(out, " ");
  if (ctx->i % 16 == 15) {
    fprintf(out, "  |");
    for (int j = 0; j < sizeof(ctx->asciibuf); j++) {
      fputc(ctx->asciibuf[j], out);
    }
    fprintf(out, "|\n");
  } else {
    fprintf(out, " ");
  }
  ctx->i++;
}

void hexdump_finish(struct hexdump_context *ctx) {
  FILE * out = ctx->output;
  int left = ctx->i % 16;
  int right = 16 - left;
  if (left > 0) {
    for (int i = 0; i < right; i++) {
      fprintf(out, "   ");
    }
    if (left < 8) fputc(' ', out);
    fprintf(out, " |");
    for (int j = 0; j < left; j++) {
      fputc(ctx->asciibuf[j], out);
    }
    fprintf(out, "|\n");
  }
  bzero((void *)ctx, sizeof(ctx));
}
