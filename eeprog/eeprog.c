#include <stdio.h>
#include <stdlib.h>

#include <ftdi.h>

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

int spi_transfer(struct ftdi_context *ftdi, uint8_t * data);
void print_binary(uint8_t x);
void print_pins(uint8_t p, char *msg);

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
  print_pins(mask, "IO mask; 0:input, 1:output");
  if (ftdi_set_bitmode(&ftdi, mask, BITMODE_BITBANG) < 0) {
    fprintf(stderr, "can't enable bitbang mode\n");
    exit(EXIT_FAILURE);
  }
  //printf("bitbang_enabled: 0x%02X, bitbang_mode: 0x%02X\n", ftdi.bitbang_enabled, ftdi.bitbang_mode);

  uint8_t pins = 1<<BIT_CS | 0<<BIT_SCK | 0<<BIT_MO | 0<<BIT_MI;
  print_pins(pins, "initial pin setup");
  if ((ret = ftdi_write_data(&ftdi, &pins, 1)) < 0) {
    fprintf(stderr, "ftdi_write_data failed: %d\n", ret);
    return -1;
  }

  uint8_t data = EEPROM_RDSR;
  pins &= ~(1<<BIT_CS);
  if ((ret = ftdi_write_data(&ftdi, &pins, 1)) < 0) {
    fprintf(stderr, "ftdi_write_data failed: %d\n", ret);
    return -1;
  }
  spi_transfer(&ftdi, &data); // write command
  spi_transfer(&ftdi, &data); // read status register
  pins |= 1<<BIT_CS;
  if ((ret = ftdi_write_data(&ftdi, &pins, 1)) < 0) {
    fprintf(stderr, "ftdi_write_data failed: %d\n", ret);
    return -1;
  }
  printf("status: 0x%02X\n", data);

  if ((ret = ftdi_usb_close(&ftdi)) < 0) {
    fprintf(stderr, "unable to close ftdi device: %d (%s)\n", ret, ftdi_get_error_string(&ftdi));
    ftdi_deinit(&ftdi);
    return EXIT_FAILURE;
  }

  ftdi_deinit(&ftdi);

  return EXIT_SUCCESS;
}

int spi_transfer(struct ftdi_context *ftdi, uint8_t * data) {
  uint8_t pins;
  int ret;
  // get current state of pins
  if (ftdi_read_pins(ftdi, &pins) < 0) {
    fprintf(stderr, "ftdi_read_pins failed\n");
    return -1;
  }
  print_pins(pins, "spi_transfer start");
  if ((pins & 1<<BIT_SCK) != 0) {
    fprintf(stderr, "spi_transfer expects low clock\n");
    return -1;
  }
  for (int i = 0; i <= 7; i++) {
    printf("MOSI: %d\n", *data>>7);
    // during low clock, set MSB from data buffer into MOSI bit of pins buffer
    pins = (pins & ~(1<<BIT_MO)) | ((*data>>7) << BIT_MO);
    print_pins(pins, "set MOSI");
    if ((ret = ftdi_write_data(ftdi, &pins, 1)) < 0) {
      fprintf(stderr, "ftdi_write_data failed: %d\n", ret);
      return -1;
    }
    // set SPI clock high; input data is latched on the rising edge
    pins |= 1<<BIT_SCK;
    print_pins(pins, "rising clock");
    if ((ret = ftdi_write_data(ftdi, &pins, 1)) < 0) {
      fprintf(stderr, "ftdi_write_data failed: %d\n", ret);
      return -1;
    }
    // set SPI clock low; data is shifted out on the falling edge
    pins &= ~(1<<BIT_SCK);
    print_pins(pins, "falling clock");
    if ((ret = ftdi_write_data(ftdi, &pins, 1)) < 0) {
      fprintf(stderr, "ftdi_write_data failed: %d\n", ret);
      return -1;
    }
    // read MISO bit from pins buffer into LSB of data buffer
    if (ftdi_read_pins(ftdi, &pins) < 0) {
      fprintf(stderr, "ftdi_read_pins failed\n");
      return -1;
    }
    print_pins(pins, "read MISO");
    // shift new MISO bit in from the left, discarding written MOSI bit off the left.
    *data = (*data << 1) | (pins>>BIT_MI & 1);
    //print_binary(*data);
  }
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
  printf("CS:%d SCK:%d MOSI:%d MISO:%d (%s)\n", p>>BIT_CS&1, p>>BIT_SCK&1, p>>BIT_MO&1, p>>BIT_MI&1, msg);
}
