const RAM_SIZE: usize = 512 * 1024;

pub struct Bus {
    ram: [u8; RAM_SIZE],
}

impl Bus {
    pub fn read(&self, addr: u16) -> u8 {
        let val = self.ram[addr as usize];
        println!("read(${:04X}) → ${:02X}", addr, val);
        val
    }

    pub fn read16(&self, addr: u16) -> u16 {
        let lo = self.ram[addr as usize] as u16;
        let hi = self.ram[addr.wrapping_add(1) as usize] as u16;
        let val = (hi << 8) | lo;
        println!("read16(${:04X}) → ${:04X}", addr, val);
        val
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
