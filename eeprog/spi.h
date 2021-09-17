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
#define BIT_MO  BIT_TX
#define BIT_MI  BIT_RX
#define BIT_SCK BIT_RTS
#define BIT_CS  BIT_CTS

// CAT25M01 instruction set
#define EEPROM_WRSR  0x01 // Write Status Register
#define EEPROM_WRITE 0x02 // Write Data to Memory
#define EEPROM_READ  0x03 // Read Data from Memory
#define EEPROM_WRDI  0x04 // Disable Write Operations
#define EEPROM_RDSR  0x05 // Read Status Register
#define EEPROM_WREN  0x06 // Enable Write Operations
// Winbond W25Q80 instructions
#define EEPROM_CHIP_ERASE  0xC7 // Chip Erase sets all bits to 1

#define SPI_SELECT 0
#define SPI_DESELECT 1

#define CHECK(x) if (x != 0) return -1

struct spi_context {
  struct ftdi_context *ftdi;
  uint8_t pins;
  uint8_t data;
  int debug;
};

int spi_init(struct spi_context *);

int spi_deinit(struct spi_context *);

int spi_select(struct spi_context *);

int spi_deselect(struct spi_context *);

int spi_transfer(struct spi_context *, uint8_t mosi);

int spi_read_pins(struct spi_context *);

int spi_write_pins(struct spi_context *, uint8_t pins);

int wait_for_ready(struct spi_context *spi);

void print_binary(uint8_t x);

void print_pins(uint8_t p, char *msg);

void print_status(uint8_t status);
