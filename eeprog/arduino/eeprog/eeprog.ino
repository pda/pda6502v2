#include <SPI.h>

#define SECTOR_SIZE 4096
#define PAGE_SIZE 256

// CAT25M01 instruction set
#define EEPROM_WRSR  0x01 // Write Status Register
#define EEPROM_WRITE 0x02 // Write Data to Memory
#define EEPROM_READ  0x03 // Read Data from Memory
#define EEPROM_WRDI  0x04 // Disable Write Operations
#define EEPROM_RDSR  0x05 // Read Status Register
#define EEPROM_WREN  0x06 // Enable Write Operations
// Winbond W25Q80 instructions
#define EEPROM_CHIP_ERASE  0xC7 // Chip Erase sets all bits to 1
#define EEPROM_SECTOR_ERASE 0x20 // Sector Erase (4KB)
#define EEPROM_POWER_UP 0xAB // Release Power Down / Device ID
#define EEPROM_POWER_DOWN 0xB9 // Power Down

const int pinCS = 10;
const int pinReset = 9;
SPISettings spi_settings(1000000, MSBFIRST, SPI_MODE0);

const char *prompt = "EEPROM> ";

// hacky hexdump
struct hexdump_context {
  uint32_t addr;
  uint8_t line_buf[2][16];
  uint8_t *curr, *prev;
  uint8_t i; // offset within 16-byte line
  uint8_t dedup, deduping; // bool
};

// global state
char cmd[128];
char *cmd_p = &cmd[0];
bool reset_hold = false;

void setup() {
  Serial.begin(115200);
  while (!Serial);
  Serial.println("\r\nSPI EEPROM programmer");
  Serial.print(prompt);
}

void loop() {
  while (Serial.available() > 0) {
    prompt_read();
  }
}

void prompt_read() {
  char data = Serial.read();
  switch (data) {
    case '\n':
    case '\r':
      Serial.println();
      *cmd_p = '\0'; // null-terminate command
      cmd_p = &cmd[0]; // reset cmd pointer
      if (strlen(cmd) > 0) cmd_exec();
      Serial.print(prompt);
      break;
    case 0x08: // backspace (bs)
    case 0x7F: // delete (del)
      if (cmd_p == cmd) break;
      Serial.print("\b \b");
      *(--cmd_p) = '\0';
      break;
    case 0x03: // End-of-text (etx); Ctrl-C
      Serial.println("^C");
      Serial.print(prompt);
      cmd_p = &cmd[0]; // reset cmd pointer
      break;
    default:
      if ((size_t)(cmd_p - cmd) >= (sizeof(cmd) - 1)) {
        break;
      }
      if (data < 0x20) {
        Serial.print("?\b");
      } else {
        Serial.write(data); // echo
        *cmd_p++ = data;
      }
      break;
  }
}

void cmd_exec() {
  char *argstr = &cmd[0];
  char *cmd0 = strsep(&argstr, " ");
  if (strcmp("help", cmd0) == 0) {
    Serial.println("Commands:");
    Serial.println("  help");
    Serial.println("  info");
    Serial.println("  read <addr> <length>");
    Serial.println("  hexdump <addr> <length>");
    Serial.println("  write <addr> <length>");
    Serial.println("  reset [hold|release]");
  } else if (strcmp("info", cmd0) == 0) {
    cmd_info();
  } else if (strcmp("read", cmd0) == 0) {
    cmd_read(argstr);
  } else if (strcmp("hexdump", cmd0) == 0 || strcmp("hd", cmd0) == 0) {
    cmd_hexdump(argstr);
  } else if (strcmp("write", cmd0) == 0) {
    cmd_write(argstr);
  } else if (strcmp("reset", cmd0) == 0) {
    cmd_reset(argstr);
  } else {
    Serial.print("command not found: ");
    Serial.println(cmd);
  }
}

void cmd_read(char *argstr) {
  long addr = (uint32_t)strtol(strsep(&argstr, " "), NULL, 0);
  long len = strtol(strsep(&argstr, " "), NULL, 0);
  Serial.print("Reading "); Serial.print(len); Serial.print(" bytes from ");
  printhex("0x", &addr, sizeof(addr)*8, ":\r\n");
  spi_begin();
  SPI.beginTransaction(spi_settings);
  digitalWrite(pinCS, LOW);
  SPI.transfer(EEPROM_READ);
  SPI.transfer(addr>>16&0xFF); // ADDR[23:16]
  SPI.transfer(addr>>8&0xFF); // ADDR[15:8]
  SPI.transfer(addr&0xFF); // ADDR[7:0]
  for (int i = 0; i < len; i++) {
    Serial.write(SPI.transfer(0x00));
  }
  digitalWrite(pinCS, HIGH);
  SPI.endTransaction();
  spi_end();
}

void cmd_hexdump(char *argstr) {
  long addr = strtol(strsep(&argstr, " "), NULL, 0);
  long len = strtol(strsep(&argstr, " "), NULL, 0);
  spi_begin();
  SPI.beginTransaction(spi_settings);
  digitalWrite(pinCS, LOW);
  SPI.transfer(EEPROM_READ);
  SPI.transfer(addr>>16&0xFF); // ADDR[23:16]
  SPI.transfer(addr>>8&0xFF); // ADDR[15:8]
  SPI.transfer(addr&0xFF); // ADDR[7:0]
  struct hexdump_context hd;
  hexdump_reset(&hd);
  hd.addr = addr;
  for (int i = 0; i < len; i++) {
    hexdump_byte(&hd, SPI.transfer(0x00));
  }
  hexdump_finish(&hd);
  digitalWrite(pinCS, HIGH);
  SPI.endTransaction();
  spi_end();
}

void cmd_write(char *argstr) {
  spi_begin();
  long addr = strtol(strsep(&argstr, " "), NULL, 0);
  long len = strtol(strsep(&argstr, " "), NULL, 0);

  uint32_t addr_end = addr + len - 1;

  uint32_t first_sector = addr/SECTOR_SIZE * SECTOR_SIZE;
  uint32_t last_sector = addr_end/SECTOR_SIZE * SECTOR_SIZE;
  uint32_t first_page = addr/PAGE_SIZE * PAGE_SIZE;
  uint32_t last_page = addr_end/PAGE_SIZE * PAGE_SIZE;

  uint32_t bytes_written = 0;
  uint32_t addr_in_sector = first_page % SECTOR_SIZE;
  uint32_t addr_in_page = addr % PAGE_SIZE;

  uint8_t page_buffer[PAGE_SIZE];

  for (uint32_t s = first_sector; s <= last_sector; s += SECTOR_SIZE) {
    printhex("erasing sector 0x", &s, 32, "\r\n");
    eeprom_sector_erase(s);
  }
  delay(100);

  Serial.println("ready for data"); // indicate ready for data
  Serial.setTimeout(1000); // milliseconds
  // for each sector `s` touched by this write
  for (uint32_t s = first_sector; s <= last_sector; s += SECTOR_SIZE) {
    // for each page `p` (in this sector) touched by this write
    for (uint32_t p = s + addr_in_sector; p < s + SECTOR_SIZE && p <= last_page; p += PAGE_SIZE) {

      // read data for page from serial into buffer
      int bytes_in_page = (p == last_page) ?
        (addr_end % PAGE_SIZE + 1) - addr_in_page :
        PAGE_SIZE - addr_in_page;
      int bytes_read = 0;
      while (bytes_read < bytes_in_page) {
        while (Serial.available() == 0);
        int r = Serial.read();
        if (r >= 0) {
          page_buffer[addr_in_page + bytes_read] = (uint8_t)r;
          bytes_read++;
        } else {
          Serial.write('?');
        }
        if (bytes_read % 64 == 0) Serial.write('.'); // acknowledge each chunk
      }

      // write data from buffer to EEPROM via SPI
      uint32_t write_addr = p + addr_in_page;
      eeprom_write_enable();
      SPI.beginTransaction(spi_settings);
      digitalWrite(pinCS, LOW);
      SPI.transfer(EEPROM_WRITE);
      SPI.transfer(write_addr>>16&0xFF); // ADDR[23:16]
      SPI.transfer(write_addr>>8&0xFF); // ADDR[15:8]
      SPI.transfer(write_addr&0xFF); // ADDR[7:0]
      // for each byte `b` (in this page) touched by this write
      for (uint32_t b = p + addr_in_page; b < p + PAGE_SIZE && b <= addr_end; b++) {
        SPI.transfer(page_buffer[b % PAGE_SIZE]); // each byte in page
        bytes_written++;
      }
      digitalWrite(pinCS, HIGH);
      SPI.endTransaction();
      wait_for_ready();

      addr_in_page = 0; // only the first page written might start at non-zero.
    }
    Serial.println();
    addr_in_sector = 0; // only the first sector written might start at non-zero.
  }
  Serial.print(bytes_written);
  Serial.println(" bytes written!");
  spi_end();
}

void cmd_info() {
  spi_begin();
  SPI.beginTransaction(spi_settings);
  digitalWrite(pinCS, LOW);
  SPI.transfer(0x90); // read manufacturer / device ID
  SPI.transfer(0x00);
  SPI.transfer(0x00);
  SPI.transfer(0x00);
  uint8_t manufacturer_id = SPI.transfer(0x00);
  uint8_t device_id = SPI.transfer(0x00);
  digitalWrite(pinCS, HIGH);
  SPI.endTransaction();

  SPI.beginTransaction(spi_settings);
  digitalWrite(pinCS, LOW);
  SPI.transfer(0x9F); // JEDEC
  uint8_t jedec_manufacturer_id = SPI.transfer(0x00);
  uint8_t jedec_type = SPI.transfer(0x00);
  uint8_t jedec_capacity = SPI.transfer(0x00);
  digitalWrite(pinCS, HIGH);
  SPI.endTransaction();

  SPI.beginTransaction(spi_settings);
  digitalWrite(pinCS, LOW);
  SPI.transfer(0x4B); // unique ID
  SPI.transfer(0x00);
  SPI.transfer(0x00);
  SPI.transfer(0x00);
  SPI.transfer(0x00);
  uint64_t unique_id = 0x00000000;
  for (int i = 0; i <= 64; i += 8) {
    unique_id <<= 8;
    unique_id |= SPI.transfer(0x00);
  }
  digitalWrite(pinCS, HIGH);
  SPI.endTransaction();

  Serial.print("manufacturer_id:0x");
  Serial.print(manufacturer_id, HEX);
  Serial.print(" device_id:0x");
  Serial.print(device_id, HEX);
  Serial.println();

  Serial.print("jedec manufacturer_id:0x");
  Serial.print(jedec_manufacturer_id, HEX);
  Serial.print(" type:0x");
  Serial.print(jedec_type, HEX);
  Serial.print(" capacity:0x");
  Serial.print(jedec_capacity, HEX);
  Serial.println();

  Serial.print("unique_id:0x");
  Serial.print(unique_id, HEX);
  Serial.println();
  spi_end();
}

void cmd_reset(char *argstr) {
  char *mode = strsep(&argstr, " ");
  if (strcmp(mode, "hold") == 0) {
    reset_hold = true;
    pinMode(pinReset, OUTPUT);
    digitalWrite(pinReset, LOW);
    Serial.println("RESET is held LOW");
  } else if (strcmp(mode, "release") == 0) {
    reset_hold = false;
    digitalWrite(pinReset, HIGH);
    pinMode(pinReset, INPUT);
    Serial.println("RESET is released to Hi-Z");
  } else {
    reset_hold = false;
    pinMode(pinReset, OUTPUT);
    digitalWrite(pinReset, LOW);
    delay(1);
    digitalWrite(pinReset, HIGH);
    pinMode(pinReset, INPUT);
  }
}

void power_up() {
  SPI.beginTransaction(spi_settings);
  digitalWrite(pinCS, LOW);
  SPI.transfer(EEPROM_POWER_UP);
  digitalWrite(pinCS, HIGH);
  SPI.endTransaction();
}

void power_down() {
  SPI.beginTransaction(spi_settings);
  digitalWrite(pinCS, LOW);
  SPI.transfer(EEPROM_POWER_DOWN);
  digitalWrite(pinCS, HIGH);
  SPI.endTransaction();
}

void eeprom_write_enable() {
  SPI.beginTransaction(spi_settings);
  digitalWrite(pinCS, LOW);
  SPI.transfer(EEPROM_WREN);
  digitalWrite(pinCS, HIGH);
  SPI.endTransaction();
}

void eeprom_sector_erase(uint32_t addr) {
  eeprom_write_enable();

  SPI.beginTransaction(spi_settings);
  digitalWrite(pinCS, LOW);
  SPI.transfer(EEPROM_SECTOR_ERASE);
  SPI.transfer(addr>>16&0xFF); // ADDR[23:16]
  SPI.transfer(addr>>8&0xFF); // ADDR[15:8]
  SPI.transfer(addr&0xFF); // ADDR[7:0]
  digitalWrite(pinCS, HIGH);
  SPI.endTransaction();

  wait_for_ready();
}

void wait_for_ready() {
  uint8_t reg;
  do {
    SPI.beginTransaction(spi_settings);
    digitalWrite(pinCS, LOW);
    SPI.transfer(EEPROM_RDSR);
    reg = SPI.transfer(0x00);
    digitalWrite(pinCS, HIGH);
    SPI.endTransaction();
  } while ((reg&0b00000001) == 1);
}


uint32_t parseint(char *str) {
  return strtol(str, NULL, 0);
}

// hacky hexdump

void hexdump_reset(struct hexdump_context *ctx) {
  ctx->addr = 0;
  ctx->curr = ctx->line_buf[0];
  ctx->prev = ctx->line_buf[1];
  ctx->i = 0;
  ctx->dedup = ctx->deduping = 0;
}

void hexdump_byte(struct hexdump_context *ctx, uint8_t byte) {
  ctx->curr[ctx->i] = byte;
  ctx->i++;
  if (ctx->i < 16) return;

  // output at end of line
  if (ctx->dedup && bcmp(ctx->curr, ctx->prev, 16) == 0) {
    if (!ctx->deduping) Serial.println("*");
    ctx->deduping = 1;
  } else {
    printhex("", &ctx->addr, 32, " "); // base address of output line
    for (int i = 0; i <= 15; i++) {
      printhex(" ", &ctx->curr[i], 8, "");
      if (i == 7) Serial.print(" "); // 8-byte divider
    }
    Serial.print(" |"); // begin ASCII dump
    for (int i = 0; i <= 15; i++) {
      char byte = ctx->curr[i];
      Serial.write((byte >= 32 && byte <= 126) ? byte : '.'); // ASCII
    }
    Serial.println("|"); // end ASCII dump
    ctx->dedup = 1;
    ctx->deduping = 0;
  }

  // prepare for next line
  ctx->addr += 16;
  ctx->i = 0;
  uint8_t *prev = ctx->prev;
  ctx->prev = ctx->curr;
  ctx->curr = prev;
}

void hexdump_finish(struct hexdump_context *ctx) {
  if (ctx->i > 0) {
    printhex("", &ctx->addr, 32, " "); // base address of output line
    for (int i = 0; i <= 15; i++) {
      if (i < ctx->i) {
        printhex(" ", &ctx->curr[i], 8, "");
      } else {
        Serial.print("   ");
      }
      if (i == 7) Serial.print(" "); // 8-byte divider
    }
    Serial.print(" |");
    for (int i = 0; i < ctx->i; i++) {
      char byte = ctx->curr[i];
      Serial.write((byte >= 32 && byte <= 126) ? byte : '.'); // ASCII
    }
    Serial.println("|");
  }
  ctx->addr += 16;
  printhex("", &ctx->addr, 32, "\r\n"); // base address of would-be-next line

  bzero((void *)ctx, sizeof(ctx));
}

// sketchy variable-size hex printer.
// prints one nibble at a time, in order to get leading zeroes etc.
// there's probably a much simpler way (other than printf).
void printhex(const char *prefix, void *data, int bits, const char *suffix) {
  if (prefix != NULL) Serial.print(prefix);
  uint8_t *ptr = (uint8_t *)data + bits/8; // little-endian MSB
  for (int i = 0; i < (bits/4); i++) {
    int shift = 0;
    if (i % 2 == 0) {
      shift = 4;
      ptr--; // next-most significant byte in little-endian
    }
    Serial.print(*ptr>>shift&0x0F, HEX);
  }
  if (suffix != NULL) Serial.print(suffix);
}


// spi_begin prepares the SPI bus, enabling & initializing outputs
void spi_begin() {
  // Hold pda6502v2 in reset state during SPI, to keep FPGA off SPI bus.
  // Eventually this will be conditional based on a --reset CLI flag.
  if (!reset_hold) {
    pinMode(pinReset, OUTPUT);
    digitalWrite(pinReset, LOW);
  }

  pinMode(pinCS, OUTPUT);
  digitalWrite(pinCS, HIGH);
  // SPI.begin Initializes the SPI bus by setting SCK, MOSI, and SS to outputs,
  // pulling SCK and MOSI low, and SS high.
  // (but there is no "native" SS on Arduino Zero)
  SPI.begin();
  delay(10);
  power_up();
}

// spi_end releases the SPI bus back into Hi-Z state
void spi_end() {
  power_down();
  SPI.end();
  // return SPI outputs to Hi-Z
  pinMode(pinCS, INPUT);
  pinMode(SCK, INPUT);
  pinMode(MOSI, INPUT);

  if (!reset_hold) {
    digitalWrite(pinReset, HIGH);
    pinMode(pinReset, INPUT);
  }
}
