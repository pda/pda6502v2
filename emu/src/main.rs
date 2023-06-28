mod asm;
mod bus;
mod cpu;
mod isa;
mod sys;

fn main() {
    // syntax brevity for Assembler args
    use asm::{branch, label, val, Operand::*};

    // assemble a nonsense demo program using diverse instructions
    let org: u16 = 0x1234;
    let mut asm = asm::Assembler::new();
    asm.org(org)
        .ldx(Imm(0xFF))
        .txs()
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
        .adc(Abs(val(org + 2)))
        .sbc(Imm(0x44))
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
        .sta(AbsX(val(0x0000)))
        .stx(ZY(0x10))
        .sty(ZX(0x20))
        .tax()
        .tay()
        .tsx()
        .txa()
        .txs()
        .tya()
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
        .label("message")
        .data("Hello world!\nHow are you?\n".into())
        .label("subroutine")
        .rts()
        .label("interrupt")
        .rti()
        .print_listing();

    let mut sys = sys::Sys::new();

    // preload program to RAM
    sys.bus.load(asm.org, asm.assemble().unwrap());

    // set reset vector to program address
    sys.bus.write(0xFFFC, asm.org as u8);
    sys.bus.write(0xFFFD, (asm.org >> 8) as u8);

    sys.reset();

    // run some instructions
    for _ in 0..80 {
        sys.step();
    }
}
