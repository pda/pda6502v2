#include <stdio.h>

const int SECTOR_SIZE = 4 * 1024;
const int PAGE_SIZE = 256;

int main() {
  int addr = 0xABCD;
  int len = 180;

  int end = addr + len - 1;

  int first_sector = addr/SECTOR_SIZE * SECTOR_SIZE;
  int last_sector = end/SECTOR_SIZE * SECTOR_SIZE;
  int first_page = addr/PAGE_SIZE * PAGE_SIZE;
  int last_page = end/PAGE_SIZE * PAGE_SIZE;

  int bytes_written = 0;
  int addr_in_sector = first_page % SECTOR_SIZE;
  int addr_in_page = addr % PAGE_SIZE;

  for (int s = first_sector; s <= last_sector; s += SECTOR_SIZE) {
    printf("sector: 0x%04X\n", s);
    for (int p = s + addr_in_sector; p < s + SECTOR_SIZE && p <= last_page; p += PAGE_SIZE) {
      printf("    page: 0x%04X\n", p);
      for (int b = p + addr_in_page; b < p + PAGE_SIZE && b <= end; b++) {
        printf("      byte: 0x%04X\n", b);
        bytes_written++;
      }
      addr_in_page = 0;
    }
    addr_in_sector = 0;
  }

  printf("bytes_written: %d\n", bytes_written);
}
