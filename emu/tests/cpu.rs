use pda6502v2emu::asm::BranchTarget;
use pda6502v2emu::asm::{label, val};
use pda6502v2emu::asm::{Assembler, Operand};
use pda6502v2emu::bus::Bus;
use pda6502v2emu::cpu::stat;
use pda6502v2emu::cpu::Cpu;

#[test]
fn test_adc() {
    let mut cpu = Cpu::new(Bus::default());
    cpu.a = 0x10; // starting value
    cpu.x = 0x21; // for testing X-indexed address modes
    cpu.y = 0x46; // for testing Y-indexed address modes
    cpu.bus.write(0x00F0, 0xEF); // for testing zero-page address mode
    cpu.bus.write(0x00F1, 0x70); // for testing zp,X address mode
    cpu.bus.write(0x00F2, 0x37); // for testing X,ind address mode (ptr LO)
    cpu.bus.write(0x00F3, 0x12); // for testing X,ind address mode (ptr HI)
    cpu.bus.write(0x00F4, 0xF2); // for testing ind,Y address mode (ptr LO)
    cpu.bus.write(0x00F5, 0x11); // for testing ind,Y address mode (ptr HI)
    cpu.bus.write(0x1234, 0x84); // for testing absolute address mode
    cpu.bus.write(0x1235, 0xFA); // for testing abs,X address mode
    cpu.bus.write(0x1236, 0x00); // for testing abs,X address mode
    cpu.bus.write(0x1237, 0x42); // for testing X,ind address mode (val)
    cpu.bus.write(0x1238, 0xBC); // for testing ind,Y address mode (val)

    use Operand::{Abs, AbsX, AbsY, Imm, IndY, XInd, Z, ZX};
    let mut asm = Assembler::new();
    cpu.bus.load(
        cpu.pc,
        asm.adc(Imm(0x11)) //          C:0+A:$10+#$11                            =$21+C:0
            .adc(Z(0xF0)) //           C:0+A:$21+[$F0→#$EF]                      =$10+C:1
            .adc(ZX(0xD0)) //          C:1+A:$10+[$D0+X:$21→$F1→#$70]            =$81+C:0(V)
            .adc(Abs(val(0x1234))) //  C:0+A:$81+[$1234→#$84]                    =$05+C:1(V)
            .adc(AbsX(val(0x1214))) // C:1+A:$05+[$1214+X:$21→$1235→#$FA]        =$00+C:1
            .adc(AbsY(val(0x12F0))) // C:1+A:$00+[$12F0+Y:$46→$1236→#$00]        =$01+C:0
            .adc(XInd(0xD1)) //        C:0+A:$01+[($D1+X:$21)→($F2)→$1237→#$42]  =$43+C:0
            .adc(IndY(0xF4)) //        C:0+A:$43+[($F4)+Y→$11F2+Y:$46→$1238→#$BC]=$FF+C:0
            .print_listing()
            .assemble()
            .unwrap(),
    );

    cpu.step(); // ADC #$11
    println!("{:?}", cpu);
    assert_eq!(cpu.a, 0x21, "{:#04X} != {:#04X}", cpu.a, 0x21);
    assert_eq!(stat(&cpu.sr), "nv-bdizc");

    cpu.step(); // ADC $F0
    println!("{:?}", cpu);
    assert_eq!(cpu.a, 0x10, "{:#04X} != {:#04X}", cpu.a, 0x10);
    assert_eq!(stat(&cpu.sr), "nv-bdizC");

    cpu.step(); // ADC $D0,X
    println!("{:?}", cpu);
    assert_eq!(cpu.a, 0x81, "{:#04X} != {:#04X}", cpu.a, 0x81);
    assert_eq!(stat(&cpu.sr), "NV-bdizc");

    cpu.step(); // ADC $1234
    println!("{:?}", cpu);
    assert_eq!(cpu.a, 0x05, "{:#04X} != {:#04X}", cpu.a, 0x05);
    assert_eq!(stat(&cpu.sr), "nV-bdizC");

    cpu.step(); // ADC $1214,X
    println!("{:?}", cpu);
    assert_eq!(cpu.a, 0x00, "{:#04X} != {:#04X}", cpu.a, 0x00);
    assert_eq!(stat(&cpu.sr), "nv-bdiZC");

    cpu.step(); // ADC $12F0,Y
    println!("{:?}", cpu);
    assert_eq!(cpu.a, 0x01, "{:#04X} != {:#04X}", cpu.a, 0x01);
    assert_eq!(stat(&cpu.sr), "nv-bdizc");

    cpu.step(); // ADC ($D1,X)
    println!("{:?}", cpu);
    assert_eq!(cpu.a, 0x43, "{:#04X} != {:#04X}", cpu.a, 0x43);
    assert_eq!(stat(&cpu.sr), "nv-bdizc");

    cpu.step(); // ADC ($F4),Y
    println!("{:?}", cpu);
    assert_eq!(cpu.a, 0xFF, "{:#04X} != {:#04X}", cpu.a, 0xFF);
    assert_eq!(stat(&cpu.sr), "Nv-bdizc");
}

#[test]
fn test_and() {
    let mut cpu = Cpu::new(Bus::default());
    cpu.a = 0b10011001; // starting value

    use Operand::Imm;
    let mut asm = Assembler::new();
    cpu.bus.load(
        cpu.pc,
        asm.and(Imm(0b11110000)).print_listing().assemble().unwrap(),
    );

    cpu.step(); // ADC #$A0
    println!("{:?}", cpu);
    assert_eq!(cpu.a, 0b10010000, "{:#04X} != {:#04X}", cpu.a, 0b10010000);
    assert_eq!(stat(&cpu.sr), "Nv-bdizc");
}

#[test]
fn test_asl() {
    let mut cpu = Cpu::new(Bus::default());
    cpu.a = 0b01000000; // for testing Accumulator address mode
    cpu.x = 0x01; // for testing zp,X address mode
    cpu.bus.write(0xF1, 0b11011011); // for testing zp,X address mode

    let mut asm = Assembler::new();
    cpu.bus.load(
        cpu.pc,
        asm.asl(Operand::A)
            .asl(Operand::A)
            .asl(Operand::ZX(0xF0))
            .print_listing()
            .assemble()
            .unwrap(),
    );

    use pda6502v2emu::cpu::StatusMask;

    cpu.set_sr_bit(StatusMask::Carry, true);
    cpu.step(); // ASL A
    println!("{:?}", cpu);
    assert_eq!(cpu.a, 0b10000000, "{:#04X} != {:#04X}", cpu.a, 0b10000000);
    assert_eq!(stat(&cpu.sr), "Nv-bdizc");

    cpu.set_sr_bit(StatusMask::Carry, true);
    cpu.step(); // ASL A
    println!("{:?}", cpu);
    assert_eq!(cpu.a, 0b00000000, "{:#04X} != {:#04X}", cpu.a, 0b00000000);
    assert_eq!(stat(&cpu.sr), "nv-bdiZC");

    cpu.step(); // ASL $F0,X
    println!("{:?}", cpu);
    let val = cpu.bus.read(0xF1);
    assert_eq!(val, 0b10110110, "{:#04X} != {:#04X}", val, 0b10110110);
    assert_eq!(stat(&cpu.sr), "Nv-bdizC");
}

#[test]
fn test_bcc() {
    let mut cpu = Cpu::new(Bus::default());

    let mut asm = Assembler::new();
    cpu.bus.load(
        cpu.pc,
        asm.bcc(Operand::Rel(BranchTarget::Offset(0x10)))
            .bcc(Operand::Rel(BranchTarget::Offset(0x20)))
            .print_listing()
            .assemble()
            .unwrap(),
    );

    use pda6502v2emu::cpu::StatusMask;

    cpu.set_sr_bit(StatusMask::Carry, true);
    cpu.step(); // BCC 0x10 (don't branch)
    println!("{:?}", cpu);
    assert_eq!(cpu.pc, 0x0002, "{:#04X} != {:#04X}", cpu.pc, 0x0002);
    assert_eq!(stat(&cpu.sr), "nv-bdizC");

    cpu.set_sr_bit(StatusMask::Carry, false);
    cpu.step(); // BCC 0x20 (do branch)
    println!("{:?}", cpu);
    assert_eq!(cpu.pc, 0x0024, "{:#04X} != {:#04X}", cpu.pc, 0x0024);
    assert_eq!(stat(&cpu.sr), "nv-bdizc");
}

#[test]
fn test_bcs() {
    let mut cpu = Cpu::new(Bus::default());
    let mut asm = Assembler::new();
    cpu.bus.load(
        cpu.pc,
        asm.bcs(Operand::Rel(BranchTarget::Offset(0x10)))
            .bcs(Operand::Rel(BranchTarget::Offset(0x20)))
            .print_listing()
            .assemble()
            .unwrap(),
    );

    use pda6502v2emu::cpu::StatusMask;

    cpu.set_sr_bit(StatusMask::Carry, false);
    cpu.step(); // BCS 0x10 (don't branch)
    println!("{:?}", cpu);
    assert_eq!(cpu.pc, 0x0002, "{:#04X} != {:#04X}", cpu.pc, 0x0002);
    assert_eq!(stat(&cpu.sr), "nv-bdizc");

    cpu.set_sr_bit(StatusMask::Carry, true);
    cpu.step(); // BCS 0x20 (do branch)
    println!("{:?}", cpu);
    assert_eq!(cpu.pc, 0x0024, "{:#04X} != {:#04X}", cpu.pc, 0x0024);
    assert_eq!(stat(&cpu.sr), "nv-bdizC");
}

#[test]
fn test_beq() {
    let mut cpu = Cpu::new(Bus::default());
    let mut asm = Assembler::new();
    cpu.bus.load(
        cpu.pc,
        asm.beq(Operand::Rel(BranchTarget::Offset(0x10)))
            .beq(Operand::Rel(BranchTarget::Offset(0x20)))
            .print_listing()
            .assemble()
            .unwrap(),
    );

    use pda6502v2emu::cpu::StatusMask;

    cpu.set_sr_bit(StatusMask::Zero, false);
    cpu.step(); // BEQ 0x10 (don't branch)
    println!("{:?}", cpu);
    assert_eq!(cpu.pc, 0x0002, "{:#04X} != {:#04X}", cpu.pc, 0x0002);
    assert_eq!(stat(&cpu.sr), "nv-bdizc");

    cpu.set_sr_bit(StatusMask::Zero, true);
    cpu.step(); // BEQ 0x20 (do branch)
    println!("{:?}", cpu);
    assert_eq!(cpu.pc, 0x0024, "{:#04X} != {:#04X}", cpu.pc, 0x0024);
    assert_eq!(stat(&cpu.sr), "nv-bdiZc");
}

#[test]
fn test_inx() {
    let mut cpu = Cpu::new(Bus::default());
    cpu.x = 0xFE;
    let mut asm = Assembler::new();
    cpu.bus.load(cpu.pc, asm.inx().inx().assemble().unwrap());
    cpu.step();
    assert_eq!(cpu.x, 0xFF, "{:#04X} != {:#04X}", cpu.x, 0xFF);
    assert_eq!(stat(&cpu.sr), "Nv-bdizc");
    cpu.step();
    assert_eq!(cpu.x, 0x00, "{:#04X} != {:#04X}", cpu.x, 0x00);
    assert_eq!(stat(&cpu.sr), "nv-bdiZc");
}

#[test]
fn test_jmp() {
    let mut cpu = Cpu::new(Bus::default());
    cpu.bus.write(0xFFFC, 0x00);
    cpu.bus.write(0xFFFD, 0x80);
    cpu.reset(); // reset vector 0x8000

    let mut asm = Assembler::new();
    cpu.bus.load(
        cpu.pc,
        asm.org(cpu.pc)
            .jmp(Operand::Abs(label("testlabel")))
            .nop()
            .label("testlabel")
            .jmp(Operand::Ind(val(0xFFFC))) // back to start via reset vector
            .print_listing()
            .assemble()
            .unwrap(),
    );

    cpu.step(); // JMP testlabel
    println!("{:?}", cpu);
    assert_eq!(cpu.pc, 0x8004, "{:#06X} != {:#06X}", cpu.pc, 0x8004);
    assert_eq!(stat(&cpu.sr), "nv-BdIzc"); // unchanged

    cpu.sr = !cpu.sr;
    cpu.step(); // JMP ($FFFC)
    println!("{:?}", cpu);
    assert_eq!(cpu.pc, 0x8000, "{:#06X} != {:#06X}", cpu.pc, 0x8000);
    assert_eq!(stat(&cpu.sr), "NV-bDiZC"); // unchanged
}

#[test]
fn test_ldx() {
    let mut cpu = Cpu::new(Bus::default());
    cpu.y = 0x02; // for testing AddressMode::ZeropageY & AddressMode::AbsoluteY
    cpu.bus.write(0x01FF, 0x11); // for testing AddressMode::Absolute
    cpu.bus.write(0x0201, 0x22); // for testing AddressMode::AbsoluteY

    let mut asm = Assembler::new();
    cpu.bus.load(
        cpu.pc,
        asm.ldx(Operand::Imm(0xAA))
            .ldx(Operand::Imm(0x00))
            .ldx(Operand::Z(0x04))
            .ldx(Operand::ZY(0x04))
            .ldx(Operand::Abs(val(0x01FF)))
            .ldx(Operand::AbsY(val(0x01FF)))
            .print_listing()
            .assemble()
            .unwrap(),
    );

    cpu.step(); // LDX #$AA
    println!("{:?}", cpu);
    assert_eq!(cpu.x, 0xAA);
    assert_eq!(stat(&cpu.sr), "Nv-bdizc");

    cpu.step(); // LDX #$00
    println!("{:?}", cpu);
    assert_eq!(cpu.x, 0x00);
    assert_eq!(stat(&cpu.sr), "nv-bdiZc");

    cpu.step(); // LDX $04
    println!("{:?}", cpu);
    assert_eq!(cpu.x, 0xA6);
    assert_eq!(stat(&cpu.sr), "Nv-bdizc");

    cpu.step(); // LDX $04,Y
    println!("{:?}", cpu);
    assert_eq!(cpu.x, 0xB6, "{:#04X} != {:#04X}", cpu.x, 0xB6);
    assert_eq!(stat(&cpu.sr), "Nv-bdizc");

    cpu.step(); // LDX $01FF ; Y=2
    println!("{:?}", cpu);
    assert_eq!(cpu.x, 0x11, "{:#04X} != {:#04X}", cpu.x, 0x11);
    assert_eq!(stat(&cpu.sr), "nv-bdizc");

    cpu.step(); // LDX $01FF,Y ; Y=2
    println!("{:?}", cpu);
    assert_eq!(cpu.x, 0x22, "{:#04X} != {:#04X}", cpu.x, 0x22);
    assert_eq!(stat(&cpu.sr), "nv-bdizc");
}

#[test]
fn test_nop() {
    let mut cpu = Cpu::new(Bus::default());
    let mut asm = Assembler::new();
    cpu.bus.load(cpu.pc, asm.nop().assemble().unwrap());
    cpu.step();
    assert_eq!(cpu.pc, 0x0001, "{:#06X} != {:#06X}", cpu.x, 0x0001);
}
