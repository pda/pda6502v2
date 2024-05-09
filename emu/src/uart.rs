use std::ops::RangeInclusive;

pub const BASE: u16 = 0xDC20;
pub const SIZE: usize = 16;
pub const RANGE: RangeInclusive<u16> = BASE..=(BASE + (SIZE as u16) - 1);

const REG_READ: [&'static str; SIZE] = [
    "MRA", "SRA", "", "RXFIFOA", "IPCR", "ISR", "CTU", "CTL", "MRB", "SRB", "", "RXFIFOB", "MISC",
    "IPR", "", "",
];

const REG_WRITE: [&'static str; SIZE] = [
    "MRA", "CSRA", "CRA", "TXFIFOA", "ACR", "IMR", "CTPU", "CTPL", "MRB", "CSRB", "CRB", "TXFIFOB",
    "MISC", "OPCR", "SOPR", "ROPR",
];

pub struct Uart {
    registers: [u8; SIZE],
}

impl Uart {
    pub fn new() -> Self {
        Self {
            registers: [0x00; SIZE],
        }
    }

    pub fn read(&self, reg: u16) -> u8 {
        self.registers[reg as usize]
    }

    pub fn write(&mut self, reg: u16, data: u8) {
        self.registers[reg as usize] = data;
    }

    pub fn name_for_read(&self, reg: u16) -> String {
        format!("UART:{} #${:02X}", REG_READ[reg as usize], self.read(reg))
    }

    pub fn name_for_write(&self, reg: u16) -> String {
        format!("UART:{}", REG_WRITE[reg as usize])
    }
}
