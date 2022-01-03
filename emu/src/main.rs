use core::fmt::Debug;
use core::fmt::Formatter;

const RAM_SIZE: usize = 512 * 1024;
const OP_NOP: u8 = 0xEA;

struct Bus {
    ram: [u8; RAM_SIZE],
}

impl Bus {
    fn read(&self, addr: u16) -> u8 {
        return self.ram[addr as usize];
    }

    fn write(&mut self, addr: u16, data: u8) {
        self.ram[addr as usize] = data;
    }
}

impl Default for Bus {
    fn default() -> Self {
        Bus {
            ram: [OP_NOP; RAM_SIZE],
        }
    }
}

#[derive(Default)]
struct Cpu {
    bus: Bus,
    pc: u16,
    sp: u8,
    a: u8,
    x: u8,
    y: u8,
    sr: u8,
}

impl Cpu {
    fn reset(&mut self) {
        self.pc = 0xFFFC; // TODO: load vector
        self.sp = 0x00;
        self.a = 0x00;
        self.x = 0x00;
        self.y = 0x00;
        self.sr = 0x00;
    }

    fn step(&mut self) {
        let op = self.bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        if op == OP_NOP {
            println!("NOP") // TODO
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

fn main() {
    let mut cpu = Cpu::default();
    cpu.bus.write(0xFFFC, 0x34);
    cpu.bus.write(0xFFFD, 0x12);
    cpu.reset();

    println!("{:?}", cpu);
    cpu.step();
    println!("{:?}", cpu);
    cpu.step();
    println!("{:?}", cpu);
    cpu.step();
    println!("{:?}", cpu);
    cpu.step();
    println!("{:?}", cpu);
    cpu.step();
    println!("{:?}", cpu);
}
