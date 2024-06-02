use std::fmt;
use std::ops::RangeInclusive;

use crate::uart;
use crate::uart::Uart;

const RAM_SIZE: usize = 512 * 1024;

const UART_BASE: u16 = 0xDC20;
const UART_RANGE: RangeInclusive<u16> = UART_BASE..=(UART_BASE + (uart::SIZE as u16) - 1);

// Bus maps memory read/write to different devices based on the address.
pub struct Bus {
    ram: [u8; RAM_SIZE],
    uart: Uart,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            ram: [0x00; RAM_SIZE],
            uart: Uart::new(),
        }
    }

    pub fn reset(&mut self) {
        self.uart.reset();
    }

    pub fn step(&mut self) {
        self.uart.step();
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        match addr {
            0xD41B => fastrand::u8(0..255),
            addr if UART_RANGE.contains(&addr) => self.uart.read((addr - UART_BASE) as u8),
            _ => self.ram[addr as usize],
        }
    }

    /// Read a u16 in little-endian order from the bus, crossing page boundaries.
    pub fn read_u16(&mut self, addr: u16) -> u16 {
        let lo = self.read(addr) as u16;
        let hi = self.read(addr.wrapping_add(1)) as u16;
        hi << 8 | lo
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        match addr {
            addr if UART_RANGE.contains(&addr) => self.uart.write((addr - UART_BASE) as u8, data),
            _ => self.ram[addr as usize] = data,
        };
    }

    pub fn is_interrupt(&self) -> bool {
        self.uart.is_interrupt()
    }

    // load is a convenience method to bulk-write data to RAM
    pub fn load(&mut self, addr: u16, data: Vec<u8>) {
        for (i, byte) in data.iter().enumerate() {
            self.write(addr + (i as u16), *byte);
        }
    }

    pub fn name_for_read(&mut self, addr: u16) -> String {
        match addr {
            addr if UART_RANGE.contains(&addr) => self.uart.name_for_read((addr - UART_BASE) as u8),
            _ => format!("#${:02X}", self.read(addr)),
        }
    }

    #[allow(unused)]
    pub fn name_for_write(&self, addr: u16) -> String {
        match addr {
            addr if UART_RANGE.contains(&addr) => {
                self.uart.name_for_write((addr - UART_BASE) as u8)
            }
            _ => "".to_string(),
        }
    }
}

impl fmt::Debug for Bus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Bus {{ RAM: {} KiB }}", RAM_SIZE / 1024))
    }
}
