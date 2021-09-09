#include <stdio.h>
#include <strings.h>

#include "hexdump.h"

void hexdump_byte(struct hexdump_context *ctx, uint8_t byte) {
  ctx->asciibuf[ctx->i%16] = (byte >= 32 && byte <= 126) ? byte : '.';
  if (ctx->i % 16 == 0) fprintf(ctx->output, "%08x  ", ctx->addr+ctx->i);
  fprintf(ctx->output, "%02x", byte);
  if (ctx->i % 16 == 7) fprintf(ctx->output, " ");
  if (ctx->i % 16 == 15) {
    fprintf(ctx->output, "  |");
    for (int j = 0; j < sizeof(ctx->asciibuf); j++) {
      fputc(ctx->asciibuf[j], ctx->output);
    }
    fprintf(ctx->output, "|\n");
  } else {
    fprintf(ctx->output, " ");
  }
  ctx->i++;
}

void hexdump_finish(struct hexdump_context *ctx) {
  int left = ctx->i % 16;
  int right = 16 - left;
  if (left > 0) {
    for (int i = 0; i < right; i++) {
      fprintf(ctx->output, "   ");
    }
    fprintf(ctx->output, " |");
    for (int j = 0; j < left; j++) {
      fputc(ctx->asciibuf[j], ctx->output);
    }
    fprintf(ctx->output, "|\n");
  }
  bzero((void *)ctx, sizeof(ctx));
}
