#include <stdio.h>
#include <stdlib.h>
#include <strings.h>

#include "spi.h"
#include "hexdump.h"

int test_things(struct spi_context *);
int write(struct spi_context *, char *srcfile);

/**
 * Usage:
 *     eeprog write <filename>
 *     eeprog test
 */
int main(int argc, char *argv[]) {
  struct spi_context spi;
  if (spi_init(&spi) != 0) return EXIT_FAILURE;

  if (argc == 3 && strcmp(argv[1], "write") == 0) {
    if (write(&spi, argv[2]) != 0) {
      fprintf(stderr, "test_things failed, hopefully it produced some stderr\n");
      spi_deinit(&spi);
      return EXIT_FAILURE;
    }
  } else if (argc == 2 && strcmp("test", argv[1]) == 0) {
    if (test_things(&spi) != 0) {
      fprintf(stderr, "test_things failed, hopefully it produced some stderr\n");
      spi_deinit(&spi);
      return EXIT_FAILURE;
    }
  }

  if (spi_deinit(&spi) != 0) return EXIT_FAILURE;

  return EXIT_SUCCESS;
}

int write(struct spi_context *spi, char *srcfile) {
  printf("opening %s\n", srcfile);
  FILE * f;
  if ((f = fopen(srcfile, "r")) == NULL) {
    perror("fopen");
    return -1;
  }
  if (fseek(f, 0, SEEK_END) != 0) {
    perror("fseek");
    fclose(f);
    return -1;
  }
  long size;
  if ((size = ftell(f)) < 0) {
    perror("ftell SEEK_END");
    fclose(f);
    return -1;
  }
  if (fseek(f, 0, SEEK_SET) != 0) {
    perror("fseek SEEK_SET");
    fclose(f);
    return -1;
  }

  CHECK(spi_select(spi));
  CHECK(spi_transfer(spi, EEPROM_WREN));
  CHECK(spi_deselect(spi));

  uint32_t addr;
  addr = 0x000000;
  CHECK(spi_select(spi));
  CHECK(spi_transfer(spi, EEPROM_WRITE));
  CHECK(spi_transfer(spi, addr>>16&0xFF)); // ADDR[23:16]
  CHECK(spi_transfer(spi, addr>>8&0xFF)); // ADDR[15:8]
  CHECK(spi_transfer(spi, addr&0xFF)); // ADDR[7:0]
  struct hexdump_context hd = {.output = stdout, .addr = addr};
  uint8_t data;
  for (int i = 0; i < size; i++) {
    // TODO: fread() per byte is probably shit, but probably not the bottleneck.
    if (fread(&data, 1, 1, f) < 1) {
      fprintf(stderr, "fread failed\n");
    };
    CHECK(spi_transfer(spi, data));
    hexdump_byte(&hd, data);
    if (i % 256 == 255) {
      fprintf(hd.output, "\n"); // page boundary
      addr += 256;
      CHECK(spi_deselect(spi));

      CHECK(spi_select(spi));
      CHECK(spi_transfer(spi, EEPROM_WREN));
      CHECK(spi_deselect(spi));

      CHECK(spi_select(spi));
      CHECK(spi_transfer(spi, EEPROM_WRITE));
      CHECK(spi_transfer(spi, addr>>16&0xFF)); // ADDR[23:16]
      CHECK(spi_transfer(spi, addr>>8&0xFF)); // ADDR[15:8]
      CHECK(spi_transfer(spi, addr&0xFF)); // ADDR[7:0]
    }
  }
  hexdump_finish(&hd);
  CHECK(spi_deselect(spi));

  if (fclose(f) != 0) {
    perror("fclose");
    return -1;
  }
  return 0;
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

  addr = 0x000000;
  printf("-- read data\n");
  CHECK(spi_select(spi));
  CHECK(spi_transfer(spi, EEPROM_READ));
  CHECK(spi_transfer(spi, addr>>16&0xFF)); // ADDR[23:16]
  CHECK(spi_transfer(spi, addr>>8&0xFF)); // ADDR[15:8]
  CHECK(spi_transfer(spi, addr&0xFF)); // ADDR[7:0]
  struct hexdump_context hd = {.output = stderr, .addr = addr};
  for (int i = 0; i < 486; i++) {
    CHECK(spi_transfer(spi, 0));
    hexdump_byte(&hd, spi->data);
  }
  hexdump_finish(&hd);
  CHECK(spi_deselect(spi));

  return 0;
}
