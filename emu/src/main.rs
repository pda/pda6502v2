mod asm;
mod bus;
mod cpu;
mod isa;

use crate::asm::Assembler;
use crate::bus::Bus;
use crate::cpu::Cpu;

fn main() {
    let asm = asm();
    println!("\n{}", asm);

    let mut bus = Bus::default();
    for (i, byte) in asm.assemble().unwrap().iter().enumerate() {
        bus.write(asm.org + (i as u16), *byte);
    }

    // set reset vector to base address where program is installed
    bus.write(0xFFFC, asm.org as u8);
    bus.write(0xFFFD, (asm.org >> 8) as u8);

    let mut cpu = Cpu::new(bus);
    cpu.reset();

    for _ in 0..10 {
        println!("{:?}", cpu);
        cpu.step();
    }
}

fn asm() -> Assembler {
    use asm::{label, val, Operand::*};
    let mut asm = Assembler::new();
    asm.org(0x1234)
        .nop()
        .ldx(Imm(0x10))
        .label("loop")
        .inx()
        .nop()
        .jmp(Abs(label("loop")))
        .jmp(Abs(val(0)));
    asm
}
