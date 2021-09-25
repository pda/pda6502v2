#include "hexdump.h"

int main() {
  struct hexdump_context hd = {.output = stdout};
  hexdump_reset(&hd);
  for (int i = 0; i <= 255; i += 1) {
    hexdump_byte(&hd, i);
  }
  for (int i = 0; i <= 100; i++) {
    hexdump_byte(&hd, 0);
  }
  for (int i = 0; i <= 1000; i++) {
    hexdump_byte(&hd, 0xAA);
  }
  hexdump_byte(&hd, 0xAA);
  hexdump_byte(&hd, 0xAA);
  //hexdump_byte(&hd, 0xCC);
  //hexdump_byte(&hd, 0xDD);
  //hexdump_byte(&hd, 0xCA);
  //hexdump_byte(&hd, 0xFE);
  hexdump_finish(&hd);
}
