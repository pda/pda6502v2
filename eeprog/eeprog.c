#include <stdio.h>
#include <stdlib.h>

#include <ftdi.h>

#include "hexdump.h"

#define FTDI_VENDOR_ID  0x0403 // FTDI
#define FTDI_PRODUCT_ID 0x6015 // http://jim.sh/ftx/

// FT230X pin bitbang bit offsets
#define BIT_TX  0
#define BIT_RX  1
#define BIT_RTS 2
#define BIT_CTS 3

// SPI EEPROM ICSP connections
#define BIT_MO BIT_RX
#define BIT_MI BIT_CTS
#define BIT_CS BIT_TX
#define BIT_SCK BIT_RTS

// CAT25M01 instruction set
#define EEPROM_WRSR  0x01 // Write Status Register
#define EEPROM_WRITE 0x02 // Write Data to Memory
#define EEPROM_READ  0x03 // Read Data from Memory
#define EEPROM_WRDI  0x04 // Disable Write Operations
#define EEPROM_RDSR  0x05 // Read Status Register
#define EEPROM_WREN  0x06 // Enable Write Operations

#define SPI_SELECT 0
#define SPI_DESELECT 1

#define CHECK(x) if (x != 0) return -1

struct spi_context {
  struct ftdi_context *ftdi;
  uint8_t pins;
  uint8_t data;
};

int spi_select(struct spi_context *spi);
int spi_deselect(struct spi_context *spi);
int spi_transfer(struct spi_context *, uint8_t mosi);
int spi_read_pins(struct spi_context *);
int spi_write_pins(struct spi_context *, uint8_t pins);
void print_binary(uint8_t x);
void print_pins(uint8_t p, char *msg);
void print_status(uint8_t status);
int test_things(struct spi_context *);

int debug = 0;

int main(int argc, char *argv[])
{
  struct ftdi_context ftdi;

  //struct ftdi_version_info version = ftdi_get_library_version();
  //printf("libftdi %s (major: %d, minor: %d, micro: %d, snapshot ver: %s)\n",
  //    version.version_str, version.major, version.minor, version.micro, version.snapshot_str);

  if (ftdi_init(&ftdi) < 0) {
    fprintf(stderr, "ftdi_new failed\n");
    return EXIT_FAILURE;
  }

  int ret;
  if ((ret = ftdi_usb_open(&ftdi, FTDI_VENDOR_ID, FTDI_PRODUCT_ID)) < 0) {
    fprintf(stderr, "unable to open ftdi device: %d (%s)\n", ret, ftdi_get_error_string(&ftdi));
    ftdi_deinit(&ftdi);
    return EXIT_FAILURE;
  }

  printf("FTDI chip type: ");
  switch (ftdi.type) {
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

  /* Enable bitbang */
  uint8_t mask = 1<<BIT_CS | 1<<BIT_SCK | 1<<BIT_MO | 0<<BIT_MI;
  if (ftdi_set_bitmode(&ftdi, mask, BITMODE_BITBANG) < 0) {
    fprintf(stderr, "can't enable bitbang mode\n");
    exit(EXIT_FAILURE);
  }
  //printf("bitbang_enabled: 0x%02X, bitbang_mode: 0x%02X\n", ftdi.bitbang_enabled, ftdi.bitbang_mode);

  struct spi_context spi = {
    .ftdi = &ftdi,
    .pins = 1<<BIT_CS | 0<<BIT_SCK | 0<<BIT_MO | 0<<BIT_MI,
  };
  spi_write_pins(&spi, spi.pins);

  if (test_things(&spi) != 0) {
    fprintf(stderr, "test_things failed, hopefully it produced some stderr\n");
  }

  if ((ret = ftdi_usb_close(&ftdi)) < 0) {
    fprintf(stderr, "unable to close ftdi device: %d (%s)\n", ret, ftdi_get_error_string(&ftdi));
    ftdi_deinit(&ftdi);
    return EXIT_FAILURE;
  }

  ftdi_deinit(&ftdi);

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

  //addr = 1034;
  //printf("-- write data\n");
  //CHECK(spi_select(spi));
  //CHECK(spi_transfer(spi, EEPROM_WRITE));
  //CHECK(spi_transfer(spi, addr>>16&0xFF)); // ADDR[23:16]
  //CHECK(spi_transfer(spi, addr>>8&0xFF)); // ADDR[15:8]
  //CHECK(spi_transfer(spi, addr&0xFF)); // ADDR[7:0]
  //CHECK(spi_transfer(spi, 'h'));
  //CHECK(spi_transfer(spi, 'e'));
  //CHECK(spi_transfer(spi, 'l'));
  //CHECK(spi_transfer(spi, 'l'));
  //CHECK(spi_transfer(spi, 'o'));
  //CHECK(spi_deselect(spi));

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

// int spi_init(struct spi_context *spi, ...) {
//   ...
//   // get current state of pins
//   if (ftdi_read_pins(ftdi, &pins) < 0) {
//     fprintf(stderr, "ftdi_read_pins failed\n");
//     return -1;
//   }
//   ...
// }

int spi_select(struct spi_context *spi) {
  return spi_write_pins(spi, spi->pins & ~(1<<BIT_CS));
}

int spi_deselect(struct spi_context *spi) {
  return spi_write_pins(spi, spi->pins | 1<<BIT_CS);
}

// note: on error, spi->pins may be left out of sync with reality.
int spi_transfer(struct spi_context *spi, uint8_t data) {
  if (debug) {
    printf("MOSI → 0x%02X ", data);
    print_binary(data);
    printf("\n");
  }
  for (int i = 0; i <= 7; i++) {
    // read into pins buffer to get MISO bit from previous falling clock
    CHECK(spi_read_pins(spi));

    // ensure clock is low
    if ((spi->pins & 1<<BIT_SCK) != 0) {
      fprintf(stderr, "spi_transfer expects low clock\n");
      return -1;
    }

    // set MSB from data buffer into MOSI bit of pins buffer
    spi->pins = (spi->pins & ~(1<<BIT_MO)) | ((data>>7) << BIT_MO);

    // shift written MOSI bit off the right, set new MISO bit into the left.
    data = (data << 1) | (spi->pins>>BIT_MI & 1);

    // write MOSI bit before rising clock
    CHECK(spi_write_pins(spi, spi->pins));

    // set SPI clock high; input data is latched on the rising edge
    CHECK(spi_write_pins(spi, spi->pins | 1<<BIT_SCK));

    // set SPI clock low; data shifted out on falling edge will be read on next iteration
    CHECK(spi_write_pins(spi, spi->pins & ~(1<<BIT_SCK)));
  }
  if (debug) {
    printf("MISO ← 0x%02X ", data);
    print_binary(data);
    printf("\n");
  }
  spi->data = data;
  return 0;
}

int spi_read_pins(struct spi_context *spi) {
  int ret;
  if ((ret = ftdi_read_pins(spi->ftdi, &spi->pins)) < 0) {
    fprintf(stderr, "spi_read_pins ftdi_read_pins failed: %d\n", ret);
    return -1;
  }
  return 0;
}

int spi_write_pins(struct spi_context *spi, uint8_t pins) {
  int ret;
  if ((ret = ftdi_write_data(spi->ftdi, &pins, 1)) < 0) {
    fprintf(stderr, "spi_write_pins ftdi_write_data failed: %d\n", ret);
    return -1;
  }
  spi->pins = pins;
  return 0;
}

void print_binary(uint8_t x) {
  char bits[9];
  bits[8] = 0;
  for (int i = 0; i <= 7; i++) {
    bits[7-i] = ((x & 1<<i) == 0) ? '0' : '1';
  }
  printf("0b%s", bits);
}

void print_pins(uint8_t p, char *msg) {
  printf("  CS:%d SCK:%d MOSI:%d MISO:%d (%s)\n",
      p>>BIT_CS&1, p>>BIT_SCK&1, p>>BIT_MO&1, p>>BIT_MI&1, msg);
}

void print_status(uint8_t s) {
  printf("status 0x%02X: WPEN:%d IPL:%d 0:%d LIP:%d BP:%d%d WEL:%d ~RDY:%d\n",
      s, s>>7&1, s>>6&1, s>>5&1, s>>4&1, s>>3&1, s>>2&1, s>>1&1, s&1);
}
