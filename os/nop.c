#include <stdlib.h>
#include <string.h>
#include <stdio.h>

const unsigned int length = 8*1024;
const unsigned int offset = 0xE000;
const char nop = 0xEA;
const char * default_path = "nop.bin";

int main(int argc, char *argv[]) {
  const char * path;
  if (argc == 2) {
    path = argv[1];
  } else {
    path = default_path;
  }
  fprintf(stderr, "writing to %s\n", path);

  char buf[length];
  memset(buf, nop, length);
  buf[0xFFF7 - offset] = 0x4C; buf[0xFFF8 - offset] = offset&0xFF; buf[0xFFF9 - offset] = (offset>>8)&0xFF; // JMP to offset
  buf[0xFFFA - offset] = 0;           buf[0xFFFB - offset] = 0;                // BRK/IRQB
  buf[0xFFFC - offset] = offset&0xFF; buf[0xFFFD - offset] = (offset>>8)&0xFF; // RESB
  buf[0xFFFE - offset] = 0;           buf[0xFFFF - offset] = 0;                // NMIB

  FILE *f;
  if ((f = fopen(path, "w")) == NULL) {
    perror("fopen");
    exit(1);
  }
  if (fwrite(buf, length, 1, f) != 1) {
    fprintf(stderr, "fwrite failed");
    exit(1);
  };
  return 0;
}
