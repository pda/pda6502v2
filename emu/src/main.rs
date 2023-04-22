mod asm;
mod bus;
mod cpu;
mod isa;

fn main() {
    // syntax brevity for Assembler args
    use asm::{branch, label, val, Operand::*};

    // prepare an address bus
    let mut bus = bus::Bus::default();

    // assemble a nonsense demo program using diverse instructions
    let org: u16 = 0x1234;
    let mut asm = asm::Assembler::new();
    asm.org(org)
        .ldx(Imm(0xFF))
        //.txs()
        .lda(Imm(0xAA))
        .ldx(Imm(0x10))
        .ldy(Imm(0xAA))
        .lsr(A)
        .ora(Imm(0x00))
        .pha()
        .lda(Imm(0xFF))
        .pla()
        .php()
        .sec()
        .plp()
        .rol(A)
        .ror(A)
        .label("loop")
        .inx()
        .iny()
        .adc(Abs(val(org + 2))) // LDX #$10 operand
        .asl(A)
        .and(ZX(0x00))
        .bit(Z(0x00))
        .bcc(Rel(branch("branch_to")))
        .bcs(Rel(branch("branch_to")))
        .beq(Rel(branch("branch_to")))
        .bmi(Rel(branch("branch_to")))
        .bne(Rel(branch("branch_to")))
        .bpl(Rel(branch("branch_to")))
        .bvc(Rel(branch("branch_to")))
        .bvs(Rel(branch("branch_to")))
        .nop()
        .label("branch_to")
        .sec()
        .sed()
        .sei()
        .clc()
        .cld()
        .cli()
        .clv()
        .cmp(AbsX(label("message")))
        .cpx(Imm(0x12))
        .cpy(Imm(0x34))
        .dec(Z(0x00))
        .dex()
        .dey()
        .eor(AbsY(val(0x8000)))
        .inc(ZX(0x80))
        .jsr(Abs(label("subroutine")))
        .jmp(Abs(label("loop")))
        .jmp(Abs(val(0)))
        .brk()
        .label("subroutine")
        .rts()
        .label("message")
        .data("Hello world!\nHow are you?\n".into());

    asm.print_listing();

    // preload program to RAM
    bus.load(asm.org, asm.assemble().unwrap());

    // set reset vector to program address
    bus.write(0xFFFC, asm.org as u8);
    bus.write(0xFFFD, (asm.org >> 8) as u8);

    let mut cpu = cpu::Cpu::new(bus);
    cpu.reset();
    cpu.sp = 0xFF; // TODO: use LDX, TXS in ASM

    // run some instructions
    for _ in 0..20 {
        println!("{:?}", cpu);
        cpu.step();
    }
}
