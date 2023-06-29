use std::fmt;

const RAM_SIZE: usize = 512 * 1024;

// Bus maps memory read/write to different devices based on the address.
pub struct Bus {
    ram: [u8; RAM_SIZE],
    // TODO: more than just RAM
}

impl Bus {
    pub fn read(&self, addr: u16) -> u8 {
        let val = self.ram[addr as usize];
        //println!("Bus read(${:04X}) → ${:02X}", addr, val);
        val
    }

    /// Read a u16 in little-endian order from the bus, crossing page boundaries.
    pub fn read_u16(&self, addr: u16) -> u16 {
        let lo = self.read(addr) as u16;
        let hi = self.read(addr.wrapping_add(1)) as u16;
        hi << 8 | lo
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        self.ram[addr as usize] = data;
        //println!("Bus write(${:04X}) ← ${:02X}", addr, data);
    }

    // load is a convenience method to bulk-write data to RAM
    pub fn load(&mut self, addr: u16, data: Vec<u8>) {
        for (i, byte) in data.iter().enumerate() {
            //println!("load: {:#06X} ← {:#04X}", addr + (i as u16), *byte);
            self.write(addr + (i as u16), *byte);
        }
    }
}

impl Default for Bus {
    fn default() -> Self {
        Bus {
            ram: [0x00; RAM_SIZE],
        }
    }
}

impl fmt::Debug for Bus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Bus {{ RAM: {} KiB }}", RAM_SIZE / 1024))
    }
}
