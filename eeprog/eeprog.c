#include <stdio.h>
#include <stdlib.h>

#include "spi.h"
#include "hexdump.h"

int test_things(struct spi_context *);

int main(int argc, char *argv[])
{
  struct spi_context spi;
  if (spi_init(&spi) != 0) return EXIT_FAILURE;

  if (test_things(&spi) != 0) {
    fprintf(stderr, "test_things failed, hopefully it produced some stderr\n");
    spi_deinit(&spi);
    return EXIT_FAILURE;
  }

  if (spi_deinit(&spi) != 0) return EXIT_FAILURE;

  return EXIT_SUCCESS;
}

int test_things(struct spi_context *spi) {
  uint32_t addr;

  printf("-- enable writes\n");
  CHECK(spi_select(spi));
  CHECK(spi_transfer(spi, EEPROM_WREN));
  CHECK(spi_deselect(spi));

  printf("-- read status register\n");
  CHECK(spi_select(spi));
  CHECK(spi_transfer(spi, EEPROM_RDSR));
  CHECK(spi_transfer(spi, 0));
  print_status(spi->data);
  CHECK(spi_deselect(spi));

  addr = 0x000380;
  printf("-- read data\n");
  CHECK(spi_select(spi));
  CHECK(spi_transfer(spi, EEPROM_READ));
  CHECK(spi_transfer(spi, addr>>16&0xFF)); // ADDR[23:16]
  CHECK(spi_transfer(spi, addr>>8&0xFF)); // ADDR[15:8]
  CHECK(spi_transfer(spi, addr&0xFF)); // ADDR[7:0]
  struct hexdump_context hd = {.output = stderr, .addr = addr};
  for (int i = 0; i < 256; i++) {
    CHECK(spi_transfer(spi, 0));
    hexdump_byte(&hd, spi->data);
  }
  hexdump_finish(&hd);
  CHECK(spi_deselect(spi));

  return 0;
}
