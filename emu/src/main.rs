mod asm;
mod bus;
mod cpu;
mod isa;

fn main() {
    // syntax brevity for Assembler args
    use asm::{label, val, Operand::*};

    // prepare an address bus
    let mut bus = bus::Bus::default();

    // assemble a trivial demo program
    let mut asm = asm::Assembler::new();
    asm.org(0x1234)
        .nop()
        .ldx(Imm(0x10))
        .label("loop")
        .inx()
        .nop()
        .jmp(Abs(label("loop")))
        .jmp(Abs(val(0)));

    // print assembly listing
    println!("\n{}", asm);

    // preload program to RAM
    for (i, byte) in asm.assemble().unwrap().iter().enumerate() {
        bus.write(asm.org + (i as u16), *byte);
    }

    // set reset vector to program address
    bus.write(0xFFFC, asm.org as u8);
    bus.write(0xFFFD, (asm.org >> 8) as u8);

    let mut cpu = cpu::Cpu::new(bus);
    cpu.reset();

    // run some instructions
    for _ in 0..10 {
        println!("{:?}", cpu);
        cpu.step();
    }
}
