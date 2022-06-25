mod asm;
mod bus;
mod cpu;
mod isa;

use crate::asm::Assembler;
use crate::asm::Op;
use crate::bus::Bus;
use crate::cpu::Cpu;

fn main() {
    let mut bus = Bus::default();

    let base: u16 = 0x1234;
    let prog = Assembler::new()
        .nop() // 0
        .nop() // 1
        .nop() // 2 <-----------------.
        .inx() // 3                   |
        .jmp(Op::Abs(base + 2)) // ---
        .assemble();

    for (i, byte) in prog.iter().enumerate() {
        bus.write(base + (i as u16), *byte);
    }

    // set reset vector to base address where program is installed
    bus.write(0xFFFC, base as u8);
    bus.write(0xFFFD, (base >> 8) as u8);

    let mut cpu = Cpu::new(bus);
    cpu.reset();

    for _ in 0..10 {
        println!("{:?}", cpu);
        cpu.step();
    }
}
