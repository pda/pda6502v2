const RAM_SIZE: usize = 512 * 1024;

pub struct Bus {
    ram: [u8; RAM_SIZE],
}

impl Bus {
    pub fn read(&self, addr: u16) -> u8 {
        self.ram[addr as usize]
    }

    pub fn read16(&self, addr: u16) -> u16 {
        let lo = self.ram[addr as usize] as u16;
        let hi = self.ram[addr.wrapping_add(1) as usize] as u16;
        (hi << 8) | lo
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        self.ram[addr as usize] = data;
    }
}

impl Default for Bus {
    fn default() -> Self {
        Bus {
            ram: [0x00; RAM_SIZE],
        }
    }
}
