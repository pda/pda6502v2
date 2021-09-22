#include <stdio.h>
#include <strings.h>

#include "hexdump.h"

void hexdump_reset(struct hexdump_context *ctx) {
  ctx->curr = ctx->line_buf[0];
  ctx->prev = ctx->line_buf[1];
}

void hexdump_byte(struct hexdump_context *ctx, uint8_t byte) {
  FILE *out = ctx->output;
  ctx->curr[ctx->i] = byte;
  ctx->i++;
  if (ctx->i < 16) return;

  // output at end of line
  if (ctx->dedup && bcmp(ctx->curr, ctx->prev, 16) == 0) {
    if (!ctx->deduping) fprintf(out, "*\n");
    ctx->deduping = 1;
  } else {
    fprintf(out, "%08x ", ctx->addr); // base address of output line
    for (int i = 0; i <= 15; i++) {
      fprintf(out, " %02x", ctx->curr[i]); // hex byte
      if (i == 7) fprintf(out, " "); // 8-byte divider
    }
    fprintf(out, " |"); // begin ASCII dump
    for (int i = 0; i <= 15; i++) {
      char byte = ctx->curr[i];
      fputc((byte >= 32 && byte <= 126) ? byte : '.', out); // ASCII
    }
    fprintf(out, "|\n"); // end ASCII dump
    ctx->dedup = 1;
    ctx->deduping = 0;
  }

  // prepare for next line
  ctx->addr += 16;
  ctx->i = 0;
  uint8_t *prev = ctx->prev;
  ctx->prev = ctx->curr;
  ctx->curr = prev;
}

void hexdump_finish(struct hexdump_context *ctx) {
  FILE *out = ctx->output;
  if (ctx->i == 0) {
    if (ctx->deduping) fprintf(out, "%08x\n", ctx->addr);
    return;
  }
  fprintf(out, "%08x ", ctx->addr); // base address of output line
  for (int i = 0; i <= 15; i++) {
    if (i < ctx->i) {
      fprintf(out, " %02x", ctx->curr[i]); // hex byte
    } else {
      fprintf(out, "   ");
    }
    if (i == 7) fprintf(out, " ");
  }
  fprintf(out, " |");
  for (int i = 0; i <= ctx->i; i++) {
    char byte = ctx->curr[i];
    fputc((byte >= 32 && byte <= 126) ? byte : '.', out); // ASCII
  }
  fprintf(out, "|\n");

  bzero((void *)ctx, sizeof(ctx));
}
