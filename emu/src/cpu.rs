use crate::bus::Bus;
use core::fmt::Debug;
use core::fmt::Formatter;

pub const OP_NOP: u8 = 0xEA;

#[derive(Default)]
pub struct Cpu {
    bus: Bus,
    pc: u16,
    sp: u8,
    a: u8,
    x: u8,
    y: u8,
    sr: u8,
}

impl Cpu {
    pub fn new(bus: Bus) -> Cpu {
        Cpu {
            bus,
            ..Cpu::default()
        }
    }

    pub fn reset(&mut self) {
        self.pc = self.bus.read16(0xFFFC); // TODO: load vector
        self.sp = 0x00;
        self.a = 0x00;
        self.x = 0x00;
        self.y = 0x00;
        self.sr = 0x00;
    }

    pub fn step(&mut self) {
        let op = self.bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        if op == OP_NOP {
            // TODO
        } else {
            todo!("anything but NOP")
        }
    }
}

impl Debug for Cpu {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let stat: String = "nv-bdizc"
            .chars()
            .enumerate()
            .map(|(i, x)| {
                if self.sr >> (7 - i) & 1 == 1 {
                    x.to_ascii_uppercase()
                } else {
                    x
                }
            })
            .collect();
        f.write_fmt(format_args!(
            "<6502 {} PC:${:04X} SP:${:02X} A:${:02X} X:${:02X} Y:${:02X}>",
            stat, self.pc, self.sp, self.a, self.x, self.y,
        ))
    }
}
