#include <termios.h>
#include <stdio.h>
#include <unistd.h>
#include <fcntl.h>
#include <strings.h>
#include <stdlib.h>
#include <errno.h>
#include <sys/stat.h>

#define PAGE_SIZE 256

void usage(char *argv[]) {
  printf("Read & write SPI EEPROM (W25Q80BV or similar) via zeeprog on Arduino\n");
  printf("Usage:\n");
  //printf("  %s erase <port>\n", argv[0]);
  printf("  %s upload <filename> <port> <addr>\n", argv[0]);
  printf("  %s download <filename> <port> <addr> <size>\n", argv[0]);
  printf("  %s verify <filename> <port> <addr>\n", argv[0]);
  //printf("  %s info <port>\n", argv[0]);
  printf("\n");
}

// quick rush copy/paste & fixup from pda/Arduino-ARMSID-configurator
int open_serial(char *name) {
  struct termios term;
  int file = open(name, O_RDWR | O_NOCTTY);
  if (file == -1) {
    fprintf(stderr, "Error\n");
    return -1;
  }
  if (tcgetattr(file, &term) != 0) {
    fprintf(stderr, "Error getting device state\n");
    close(file);
    return -1;
  }
  cfmakeraw(&term);
  cfsetspeed(&term, B115200);
  term.c_cc[VMIN] = 1; // read doesn't return until at least N bytes
  term.c_cc[VTIME] = 0; // read timeout between bytes (0 = no timeout)
  if (tcsetattr(file, TCSANOW, &term) != 0) {
    fprintf(stderr, "Error setting device parameters\n");
    close(file);
    return -1;
  }
  return file;
}

int expect(int serial, char *str) {
  char *match_ptr = str;
  do {
    char byte_in;
    int r = read(serial, &byte_in, 1);
    if (r == -1) {
      perror("expect read");
      return -1;
    } else if (r == 0) {
      continue;
    }
    printf("%c", byte_in);
    if (byte_in == *match_ptr) {
      match_ptr++; // byte matches, prepare to test the next one
    } else  {
      match_ptr = str;
    }
  } while (*match_ptr != 0);
  return 0;
}

int cmd_upload(char *srcfile, char *port, char *addr_str) {
  FILE *file = fopen(srcfile, "r");
  if (file < 0) {
    fprintf(stderr, "error opening file %s\n", srcfile);
    return(-1);
  }
  struct stat stat;
  if (fstat(fileno(file), &stat) == -1) {
    perror("fstat");
    return(-1);
  }
  uint32_t size = stat.st_size;

  int serial = open_serial(port);
  if (serial < 0) {
    fprintf(stderr, "error opening serial port %s\n", port);
    return(-1);
  }

  uint32_t addr_from = strtol(addr_str, NULL, 0);
  uint32_t addr_to = addr_from + size;

  printf("Write %s → %s 0x%06X–0x%06X (%d bytes)\n", srcfile, port, addr_from, addr_to, size);

  char *data = "\x03"; // End-of-text (etx); Ctrl-C
  if (write(serial, data, 1) == -1) {
    perror("writing ^C to serial");
    return(-1);
  }
  tcdrain(serial);
  expect(serial, "\r\nEEPROM> ");
  char * reset_disable = "reset disable\n";
  write(serial, reset_disable, strlen(reset_disable));
  expect(serial, "RESET is disabled; Hi-Z");
  expect(serial, "\r\nEEPROM> ");

  char cmd[32];
  snprintf(cmd, sizeof(cmd), "write 0x%06X %d\n", addr_from, size);
  int cmdlen = strlen(cmd);
  //printf("%s", cmd);
  int bytes_written = write(serial, cmd, cmdlen);
  if (bytes_written != cmdlen) {
    fprintf(stderr, "short write %d != %d\n", bytes_written, cmdlen);
  }
  expect(serial, "\r\nready for data\r\n");

  const int chunk_size = 64;
  uint8_t buf[chunk_size];
  while (!feof(file)) {
    int bytes_read = fread(&buf[0], 1, chunk_size, file);
    if (bytes_read != chunk_size && !feof(file)) {
      fprintf(stderr, "short read %d != %d\n", bytes_read, chunk_size);
    }
    if (ferror(file) != 0) {
      printf("file error: %d\n", ferror(file));
    }
    int bytes_written = write(serial, buf, bytes_read);
    tcdrain(serial);
    if (bytes_written != bytes_read) {
      fprintf(stderr, "short write %d != %d\n", bytes_written, bytes_read);
    }
    if (!feof(file)) expect(serial, ".");
  }
  expect(serial, "bytes written!");

  char * reset_now = "reset\n";
  write(serial, reset_now, strlen(reset_now));
  expect(serial, "\r\nEEPROM> ");

  printf("\n");
  return 0;
}

int cmd_download(char *dstfile, char * port, char * addr_str, char * size_str) {
  FILE *file = fopen(dstfile, "w");
  if (file < 0) {
    fprintf(stderr, "error opening file %s\n", dstfile);
    return(-1);
  }
  int serial = open_serial(port);
  if (serial < 0) {
    fprintf(stderr, "error opening serial port %s\n", port);
    return(-1);
  }

  uint32_t addr_from = strtol(addr_str, NULL, 0);
  uint32_t size = strtol(size_str, NULL, 0);
  uint32_t addr_to = addr_from + size;

  printf("Download %s ← %s 0x%06X–0x%06X (%d bytes)\n", dstfile, port, addr_from, addr_to, size);

  char *data = "\x03"; // End-of-text (etx); Ctrl-C
  if (write(serial, data, 1) == -1) {
    perror("writing ^C to serial");
    return(-1);
  }
  tcdrain(serial);
  expect(serial, "\r\nEEPROM> ");
  //char * reset_hold = "reset hold\n";
  //write(serial, reset_hold, strlen(reset_hold));
  //expect(serial, "RESET is held LOW");
  //expect(serial, "\r\nEEPROM> ");

  char cmd[32];
  snprintf(cmd, sizeof(cmd), "read 0x%06X %d\n", addr_from, size);
  int cmdlen = strlen(cmd);
  int bytes_written = write(serial, cmd, cmdlen);
  if (bytes_written != cmdlen) {
    fprintf(stderr, "short write %d != %d\n", bytes_written, cmdlen);
  }
  expect(serial, ":\r\n"); // "Reading X bytes from 0xXXXXXXXX:\r\n"

  for (int i = 0; i < size; i++) {
    uint8_t byte_from_serial;
    // TODO: fread() per byte is probably shit, but probably not the bottleneck.
    if (read(serial, &byte_from_serial, 1) != 1) {
      fprintf(stderr, "read from serial failed\n");
    }
    fwrite(&byte_from_serial, 1, 1, file);
  }
  if (fclose(file) != 0) {
    perror("fclose");
    return -1;
  }
  expect(serial, "EEPROM> ");
  //char * reset_release = "reset release\n";
  //write(serial, reset_release, strlen(reset_release));
  //expect(serial, "RESET is released to Hi-Z");

  printf("\n");
  return 0;
}

int cmd_verify(char *srcfile, char *port, char *addr_str) {
  FILE *file = fopen(srcfile, "r");
  if (file < 0) {
    fprintf(stderr, "error opening file %s\n", srcfile);
    return(-1);
  }
  struct stat stat;
  if (fstat(fileno(file), &stat) == -1) {
    perror("fstat");
    return(-1);
  }
  uint32_t size = stat.st_size;

  int serial = open_serial(port);
  if (serial < 0) {
    fprintf(stderr, "error opening serial port %s\n", port);
    return(-1);
  }

  uint32_t addr_from = strtol(addr_str, NULL, 0);
  uint32_t addr_to = addr_from + size;

  printf("Verify %s ← %s 0x%06X–0x%06X (%d bytes)\n", srcfile, port, addr_from, addr_to, size);

  char *data = "\x03"; // End-of-text (etx); Ctrl-C
  if (write(serial, data, 1) == -1) {
    perror("writing ^C to serial");
    return(-1);
  }
  tcdrain(serial);
  expect(serial, "\r\nEEPROM> ");
  char * reset_hold = "reset hold\n";
  write(serial, reset_hold, strlen(reset_hold));
  expect(serial, "RESET is held LOW");
  expect(serial, "\r\nEEPROM> ");

  char cmd[32];
  snprintf(cmd, sizeof(cmd), "read 0x%06X %d\n", addr_from, size);
  int cmdlen = strlen(cmd);
  int bytes_written = write(serial, cmd, cmdlen);
  if (bytes_written != cmdlen) {
    fprintf(stderr, "short write %d != %d\n", bytes_written, cmdlen);
  }
  expect(serial, ":\r\n"); // "Reading X bytes from 0xXXXXXXXX:\r\n"

  uint32_t fail_count = 0;
  for (int i = 0; i < size; i++) {
    uint8_t byte_from_file, byte_from_serial;
    // TODO: fread() per byte is probably shit, but probably not the bottleneck.
    if (fread(&byte_from_file, 1, 1, file) < 1) {
      fprintf(stderr, "fread from file failed\n");
    };
    if (read(serial, &byte_from_serial, 1) != 1) {
      fprintf(stderr, "read from serial failed\n");
    }
    if (byte_from_serial != byte_from_file) {
      fail_count++;
      //printf("%06X:%02X!=%02X ", addr_from+i, byte_from_serial, byte_from_file);
      //if (fail_count % 8 == 0) printf("\n");
    }
  }
  //if (fail_count > 0 && fail_count % 8 != 0) printf("\n");
  if (fclose(file) != 0) {
    perror("fclose");
    return -1;
  }
  expect(serial, "EEPROM> ");
  char * reset_release = "reset release\n";
  write(serial, reset_release, strlen(reset_release));
  expect(serial, "RESET is released to Hi-Z");

  printf("\n");
  if (fail_count == 0) {
    printf("✔ verify successful: %s\n", srcfile);
  } else {
    fprintf(stderr, "⨯ verify failed; %d of %d bytes differ\n", fail_count, size);
    return 1;
  }
  return 0;
}

int cmd_info() {
  fprintf(stderr, "TODO\n");
  return -1;
}

/**
 * Usage:
 *     eeprog upload <filename>
 *     eeprog test
 */
int main(int argc, char *argv[]) {
  if (argc == 5 && strcmp(argv[1], "upload") == 0) {
    if (cmd_upload(argv[2], argv[3], argv[4]) != 0) {
      return EXIT_FAILURE;
    }
  } else if (argc == 6 && strcmp(argv[1], "download") == 0) {
    if (cmd_download(argv[2], argv[3], argv[4], argv[5]) != 0) {
      return EXIT_FAILURE;
    }
  } else if (argc == 5 && strcmp(argv[1], "verify") == 0) {
    int ret;
    if ((ret = cmd_verify(argv[2], argv[3], argv[4])) != 0) {
      return EXIT_FAILURE;
    }
  } else if (argc == 2 && strcmp("info", argv[1]) == 0) {
    if (cmd_info() != 0) {
      return EXIT_FAILURE;
    }
  } else if (argc >= 2 && strcmp("help", argv[1]) == 0) {
    usage(argv);
  } else {
    usage(argv);
    return EXIT_FAILURE;
  }

  return EXIT_SUCCESS;
}
