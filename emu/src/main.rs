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
    let org: u16 = 0x1234;
    let mut asm = asm::Assembler::new();
    asm.org(org)
        .nop()
        .ldx(Imm(0x10))
        .label("loop")
        .inx()
        .adc(Abs(val(org + 2))) // LDX #$10 operand
        .nop()
        .jmp(Abs(label("loop")))
        .jmp(Abs(val(0)));

    asm.print_listing();

    // preload program to RAM
    bus.load(asm.org, asm.assemble().unwrap());

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
