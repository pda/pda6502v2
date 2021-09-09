#include <stdio.h>

#include "spi.h"

int spi_init(struct spi_context *spi) {
  int ret;

  if ((spi->ftdi = (struct ftdi_context *)malloc(sizeof(struct ftdi_context))) == NULL) {
    fprintf(stderr, "spi_init malloc failed");
    return -1;
  }
  if ((ret = ftdi_init(spi->ftdi)) < 0) {
    fprintf(stderr, "ftdi_init failed: %d (%s)\n",
        ret, ftdi_get_error_string(spi->ftdi));
    return -1;
  }

  if ((ret = ftdi_usb_open(spi->ftdi, FTDI_VENDOR_ID, FTDI_PRODUCT_ID)) < 0) {
    fprintf(stderr, "unable to open ftdi device: %d (%s)\n",
        ret, ftdi_get_error_string(spi->ftdi));
    ftdi_deinit(spi->ftdi);
    return -1;
  }

  fprintf(stderr, "Connected to FTDI device: ");
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
  fflush(stderr);

  uint8_t outputPins = 1<<BIT_CS | 1<<BIT_SCK | 1<<BIT_MO | 0<<BIT_MI;
  if (ftdi_set_bitmode(spi->ftdi, outputPins, BITMODE_BITBANG) < 0) {
    fprintf(stderr, "ftdi_set_bitmode to BITMODE_BITBANG failed: %d (%s)\n",
        ret, ftdi_get_error_string(spi->ftdi));
    ftdi_deinit(spi->ftdi);
    return -1;
  }

  if (spi_write_pins(spi, 1<<BIT_CS | 0<<BIT_SCK | 0<<BIT_MO | 0<<BIT_MI) != 0) {
    fprintf(stderr, "spi_write_pins for initial state failed: %d (%s)\n",
        ret, ftdi_get_error_string(spi->ftdi));
    ftdi_deinit(spi->ftdi);
    return -1;
  }

  return 0;
}

int spi_deinit(struct spi_context *spi) {
  int ret;
  if ((ret = ftdi_usb_close(spi->ftdi)) < 0) {
    fprintf(stderr, "unable to close ftdi device: %d (%s)\n",
        ret, ftdi_get_error_string(spi->ftdi));
    ftdi_deinit(spi->ftdi);
    return EXIT_FAILURE;
  }

  ftdi_deinit(spi->ftdi);
  return 0;
}

int spi_select(struct spi_context *spi) {
  return spi_write_pins(spi, spi->pins & ~(1<<BIT_CS));
}

int spi_deselect(struct spi_context *spi) {
  return spi_write_pins(spi, spi->pins | 1<<BIT_CS);
}

// spi_transfer sends the `data` byte over SPI MOSI, and stores the MISO byte
// in spi->data. Asserting CS for the SPI device is left to the caller.
int spi_transfer(struct spi_context *spi, uint8_t data) {
  if (spi->debug) {
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
  if (spi->debug) {
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
