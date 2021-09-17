#include <stdio.h>
#include <stdlib.h>
#include <strings.h>

#include "spi.h"
#include "hexdump.h"

int cmd_status(struct spi_context *);
int cmd_upload(struct spi_context *, char *srcfile);
int cmd_download(struct spi_context *, char *dstfile);
int cmd_verify(struct spi_context *, char *srcfile);
void usage(char *argv[]);

/**
 * Usage:
 *     eeprog upload <filename>
 *     eeprog test
 */
int main(int argc, char *argv[]) {
  struct spi_context spi;

  if (argc == 3 && strcmp(argv[1], "upload") == 0) {
    if (spi_init(&spi) != 0) return EXIT_FAILURE;
    if (cmd_upload(&spi, argv[2]) != 0) {
      fprintf(stderr, "cmd_upload failed\n");
      spi_deinit(&spi);
      return EXIT_FAILURE;
    }
  } else if (argc == 3 && strcmp(argv[1], "download") == 0) {
    if (spi_init(&spi) != 0) return EXIT_FAILURE;
    if (cmd_download(&spi, argv[2]) != 0) {
      fprintf(stderr, "cmd_download failed\n");
      spi_deinit(&spi);
      return EXIT_FAILURE;
    }
  } else if (argc == 3 && strcmp(argv[1], "verify") == 0) {
    int ret;
    if (spi_init(&spi) != 0) return EXIT_FAILURE;
    if ((ret = cmd_verify(&spi, argv[2])) != 0) {
      // ret > 0 indicates a logical failure (i.e. data differed), which should
      // result in EXIT_FAILURE but no further failure message.
      if (ret < 0) {
        fprintf(stderr, "cmd_verify failed\n");
      }
      spi_deinit(&spi);
      return EXIT_FAILURE;
    }
  } else if (argc == 2 && strcmp("status", argv[1]) == 0) {
    if (spi_init(&spi) != 0) return EXIT_FAILURE;
    if (cmd_status(&spi) != 0) {
      fprintf(stderr, "cmd_status failed\n");
      spi_deinit(&spi);
      return EXIT_FAILURE;
    }
  } else {
    usage(argv);
    return (argc >= 2 && strcmp("help", argv[1]) == 0) ? EXIT_SUCCESS : EXIT_FAILURE;
  }

  return EXIT_SUCCESS;
}

void usage(char *argv[]) {
  printf("\nRead & write SPI EEPROM (AT25M01 or similar) via FTDI (FT230X or similar)\n\n");
  printf("Usage:\n\n");
  printf("  %s upload   <filename> -- write file to EEPROM\n", argv[0]);
  printf("  %s download <filename> -- dump EEPROM contents to file\n", argv[0]);
  printf("  %s verify   <filename> -- verify EEPROM contents matches file\n", argv[0]);
  printf("  %s status              -- print EEPROM status register\n", argv[0]);
  printf("\n");
  printf("Wiring:\n\n");
  printf("  RX  → MISO\n");
  printf("  TX  → MOSI\n");
  printf("  CTS → CS\n");
  printf("  RTS → SCK\n");
}

int cmd_upload(struct spi_context *spi, char *srcfile) {
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

int cmd_download(struct spi_context *spi, char *dstfile) {
  FILE * f;
  if ((f = fopen(dstfile, "w")) == NULL) {
    perror("fopen");
    return -1;
  }

  uint32_t addr;
  addr = 0x000000;
  CHECK(spi_select(spi));
  CHECK(spi_transfer(spi, EEPROM_READ));
  CHECK(spi_transfer(spi, addr>>16&0xFF)); // ADDR[23:16]
  CHECK(spi_transfer(spi, addr>>8&0xFF)); // ADDR[15:8]
  CHECK(spi_transfer(spi, addr&0xFF)); // ADDR[7:0]
  struct hexdump_context hd = {.output = stdout, .addr = addr};
  for (int i = 0; i < 1024; i++) {
    CHECK(spi_transfer(spi, 0x00));
    // TODO: fwrite() per byte is probably shit, but probably not the bottleneck.
    if (fwrite(&spi->data, 1, 1, f) < 1) {
      fprintf(stderr, "fwrite failed\n");
    };
    hexdump_byte(&hd, spi->data);
  }
  hexdump_finish(&hd);
  CHECK(spi_deselect(spi));

  if (fclose(f) != 0) {
    perror("fclose");
    return -1;
  }
  return 0;
}

int cmd_verify(struct spi_context *spi, char *srcfile) {
  uint32_t fail_count = 0;
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
  uint32_t addr;
  addr = 0x000000;
  CHECK(spi_select(spi));
  CHECK(spi_transfer(spi, EEPROM_READ));
  CHECK(spi_transfer(spi, addr>>16&0xFF)); // ADDR[23:16]
  CHECK(spi_transfer(spi, addr>>8&0xFF)); // ADDR[15:8]
  CHECK(spi_transfer(spi, addr&0xFF)); // ADDR[7:0]
  for (int i = 0; i < size; i++) {
    uint8_t data;
    // TODO: fread() per byte is probably shit, but probably not the bottleneck.
    if (fread(&data, 1, 1, f) < 1) {
      fprintf(stderr, "fread failed\n");
    };
    CHECK(spi_transfer(spi, 0x00));
    if (data != spi->data) {
      fail_count++;
    }
  }
  CHECK(spi_deselect(spi));
  if (fclose(f) != 0) {
    perror("fclose");
    return -1;
  }
  if (fail_count == 0) {
    printf("✔ verify successful, first %ld bytes of EEPROM match %s\n", size, srcfile);
  } else {
    fprintf(stderr, "⨯ verify failed; %d of first %ld bytes of EEPROM differ\n", fail_count, size);
    return 1;
  }
  return 0;
}

int cmd_status(struct spi_context *spi) {
  printf("Connected to FTDI device: ");
  switch (spi->ftdi->type) {
    case TYPE_2232C: printf("2232C\n"); break;
    case TYPE_2232H: printf("2232H\n"); break;
    case TYPE_230X:  printf("230X\n");  break;
    case TYPE_232H:  printf("232H\n");  break;
    case TYPE_4232H: printf("4232H\n"); break;
    case TYPE_AM:    printf("AM\n");    break;
    case TYPE_BM:    printf("BM\n");    break;
    case TYPE_R:     printf("R\n");     break;
    default: printf("(unknown)\n");     break;
  }

  CHECK(spi_select(spi));
  CHECK(spi_transfer(spi, EEPROM_RDSR));
  CHECK(spi_transfer(spi, 0));
  print_status(spi->data);
  CHECK(spi_deselect(spi));
  return 0;
}
