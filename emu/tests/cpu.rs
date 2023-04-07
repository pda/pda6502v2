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
fn test_bit() {
    let mut cpu = Cpu::new(Bus::default());
    let mut asm = Assembler::new();
    cpu.bus.load(
        cpu.pc,
        asm.org(cpu.pc)
            .label("data")
            .data(vec![0xFF, 0x00])
            .label("prog")
            .bit(Operand::Z(cpu.pc.try_into().unwrap()))
            .bit(Operand::Abs(val(cpu.pc)))
            .print_listing()
            .assemble()
            .unwrap(),
    );
    cpu.pc += 2; // skip the data

    cpu.a = 0xFF;
    cpu.step(); // BIT $00 (#$FF)
    println!("{:?}", cpu);
    assert_eq!(cpu.pc, 0x0004, "{:#04X} != {:#04X}", cpu.pc, 0x0004);
    assert_eq!(stat(&cpu.sr), "NV-bdizc"); // 0b11111111 AND 0b11111111 = 0b11111111 = z

    cpu.a = 0x00;
    cpu.step(); // BIT $0001 (#$00)
    println!("{:?}", cpu);
    assert_eq!(cpu.pc, 0x0007, "{:#04X} != {:#04X}", cpu.pc, 0x0007);
    assert_eq!(stat(&cpu.sr), "nv-bdiZc"); // 0b00000000 AND 0b00000000 = 0b00000000 = Z
}

#[test]
fn test_bmi() {
    let mut cpu = Cpu::new(Bus::default());
    let mut asm = Assembler::new();
    cpu.bus.load(
        cpu.pc,
        asm.ldx(Operand::Imm(0xFF)) // SR N=1
            .label("a")
            .bmi(Operand::Rel(BranchTarget::Label("b".to_string())))
            .nop()
            .label("b")
            .ldx(Operand::Imm(0x10)) // SR N=0
            .bmi(Operand::Rel(BranchTarget::Label("a".to_string())))
            .nop()
            .print_listing()
            .assemble()
            .unwrap(),
    );

    cpu.step(); // LDX #$FF
    println!("{cpu:?}");
    cpu.step(); // BMI b
    println!("{cpu:?}");
    assert_eq!(cpu.pc, 0x0005, "{:#04X} != {:#04X}", cpu.pc, 0x0005);
    cpu.step(); // LDX #$10
    cpu.step(); // BMI a
    println!("{cpu:?}");
    assert_eq!(cpu.pc, 0x0009, "{:#04X} != {:#04X}", cpu.pc, 0x0009);
}

#[test]
fn test_bne() {
    let mut cpu = Cpu::new(Bus::default());
    let mut asm = Assembler::new();
    cpu.bus.load(
        cpu.pc,
        asm.label("a")
            .ldx(Operand::Imm(0x01))
            .bne(Operand::Rel(BranchTarget::Label("b".to_string())))
            .nop()
            .label("b")
            .ldx(Operand::Imm(0x00))
            .bne(Operand::Rel(BranchTarget::Label("a".to_string())))
            .nop()
            .print_listing()
            .assemble()
            .unwrap(),
    );

    cpu.step(); // LDX #$01
    cpu.step(); // BNE b
    println!("{cpu:?}");
    assert_eq!(cpu.pc, 0x0005, "{:#04X} != {:#04X}", cpu.pc, 0x0005);

    cpu.step(); // LDX #$00
    cpu.step(); // BNE a
    println!("{cpu:?}");
    assert_eq!(cpu.pc, 0x0009, "{:#04X} != {:#04X}", cpu.pc, 0x0009);
}

#[test]
fn test_bpl() {
    let mut cpu = Cpu::new(Bus::default());
    let mut asm = Assembler::new();
    cpu.bus.load(
        cpu.pc,
        asm.label("a")
            .ldx(Operand::Imm(0x10)) // SR N=0
            .bpl(Operand::Rel(BranchTarget::Label("b".to_string())))
            .nop()
            .label("b")
            .ldx(Operand::Imm(0xF0)) // SR N=1
            .bpl(Operand::Rel(BranchTarget::Label("a".to_string())))
            .nop()
            .print_listing()
            .assemble()
            .unwrap(),
    );

    cpu.step(); // LDX #$10
    cpu.step(); // BPL b
    println!("{cpu:?}");
    assert_eq!(cpu.pc, 0x0005, "{:#04X} != {:#04X}", cpu.pc, 0x0005);
    cpu.step(); // LDX #$F0
    cpu.step(); // BPL a
    println!("{cpu:?}");
    assert_eq!(cpu.pc, 0x0009, "{:#04X} != {:#04X}", cpu.pc, 0x0009);
}

#[test]
fn test_brk() {
    let mut cpu = Cpu::new(Bus::default());
    cpu.bus.write(0xFFFE, 0x68); // IRQ vector (lo)
    cpu.bus.write(0xFFFF, 0x24); // IRQ vector (hi)
    cpu.pc = 0x0400;
    cpu.sp = 0xFF;

    let mut asm = Assembler::new();
    cpu.bus
        .load(cpu.pc, asm.brk().print_listing().assemble().unwrap());

    assert_eq!(stat(&cpu.sr), "nv-bdizc");
    cpu.step(); // BRK
    println!("{cpu:?}");
    assert_eq!(stat(&cpu.sr), "nv-bdizc");
    assert_eq!(cpu.pc, 0x2468, "PC {:#04X} != {:#04X}", cpu.pc, 0x2468);
    assert_eq!(cpu.sp, 0xFC, "SP {:#02X} != {:#02X}", cpu.sp, 0xFC);
    assert_eq!(cpu.bus.read(0x01FF), 0x04);
    assert_eq!(cpu.bus.read(0x01FE), 0x02);
    assert_eq!(stat(&cpu.bus.read(0x01FD)), "nv-Bdizc");
}

#[test]
fn test_bvc() {
    let mut cpu = Cpu::new(Bus::default());

    let mut asm = Assembler::new();
    cpu.bus.load(
        cpu.pc,
        asm.bvc(Operand::Rel(BranchTarget::Offset(0x10)))
            .bvc(Operand::Rel(BranchTarget::Offset(0x20)))
            .print_listing()
            .assemble()
            .unwrap(),
    );

    use pda6502v2emu::cpu::StatusMask;

    cpu.set_sr_bit(StatusMask::Overflow, true);
    cpu.step(); // BVC 0x10 (don't branch)
    println!("{:?}", cpu);
    assert_eq!(cpu.pc, 0x0002, "{:#04X} != {:#04X}", cpu.pc, 0x0002);
    assert_eq!(stat(&cpu.sr), "nV-bdizc");

    cpu.set_sr_bit(StatusMask::Overflow, false);
    cpu.step(); // BVC 0x20 (do branch)
    println!("{:?}", cpu);
    assert_eq!(cpu.pc, 0x0024, "{:#04X} != {:#04X}", cpu.pc, 0x0024);
    assert_eq!(stat(&cpu.sr), "nv-bdizc");
}

#[test]
fn test_bvs() {
    let mut cpu = Cpu::new(Bus::default());

    let mut asm = Assembler::new();
    cpu.bus.load(
        cpu.pc,
        asm.bvs(Operand::Rel(BranchTarget::Offset(0x10)))
            .bvs(Operand::Rel(BranchTarget::Offset(0x20)))
            .print_listing()
            .assemble()
            .unwrap(),
    );

    use pda6502v2emu::cpu::StatusMask;

    cpu.set_sr_bit(StatusMask::Overflow, false);
    cpu.step(); // BVS 0x10 (don't branch)
    println!("{:?}", cpu);
    assert_eq!(cpu.pc, 0x0002, "{:#04X} != {:#04X}", cpu.pc, 0x0002);
    assert_eq!(stat(&cpu.sr), "nv-bdizc");

    cpu.set_sr_bit(StatusMask::Overflow, true);
    cpu.step(); // BVS 0x20 (do branch)
    println!("{:?}", cpu);
    assert_eq!(cpu.pc, 0x0024, "{:#04X} != {:#04X}", cpu.pc, 0x0024);
    assert_eq!(stat(&cpu.sr), "nV-bdizc");
}

#[test]
fn test_cmp() {
    let mut cpu = Cpu::new(Bus::default());
    cpu.pc = 0x0200;
    cpu.a = 0xC0;
    cpu.x = 0x03;
    let mut asm = Assembler::new();
    cpu.bus.load(
        cpu.pc,
        asm.org(cpu.pc)
            .label("prog")
            .cmp(Operand::Abs(label("data")))
            .cmp(Operand::AbsX(label("data")))
            .label("data")
            .data(vec![0xAA, 0xBB, 0xCC, 0xDD])
            .print_listing()
            .assemble()
            .unwrap(),
    );

    cpu.step(); // CMP data
    println!("{cpu:?}");
    assert_eq!(cpu.pc, 0x0203, "{:#04X} != {:#04X}", cpu.pc, 0x0203);
    assert_eq!(stat(&cpu.sr), "nv-bdizc");
    cpu.step(); // CMP data,X
    println!("{cpu:?}");
    assert_eq!(cpu.pc, 0x0206, "{:#04X} != {:#04X}", cpu.pc, 0x0206);
    assert_eq!(stat(&cpu.sr), "Nv-bdizC");
}

#[test]
fn test_cpx_and_cpy() {
    let mut cpu = Cpu::new(Bus::default());
    cpu.pc = 0x0200;
    cpu.x = 0x04;
    cpu.y = 0x04;
    let mut asm = Assembler::new();
    cpu.bus.load(
        cpu.pc,
        asm.org(cpu.pc)
            .label("prog")
            .cpx(Operand::Imm(0x04))
            .cpy(Operand::Imm(0x08))
            .print_listing()
            .assemble()
            .unwrap(),
    );

    cpu.step(); // CPX #$04
    println!("{cpu:?}");
    assert_eq!(cpu.pc, 0x0202, "{:#04X} != {:#04X}", cpu.pc, 0x0202);
    assert_eq!(stat(&cpu.sr), "nv-bdiZc");
    cpu.step(); // CPY #$08
    println!("{cpu:?}");
    assert_eq!(cpu.pc, 0x0204, "{:#04X} != {:#04X}", cpu.pc, 0x0204);
    assert_eq!(stat(&cpu.sr), "Nv-bdizC");
}

#[test]
fn test_dec() {
    let mut cpu = Cpu::new(Bus::default());
    cpu.pc = 0x1000;
    cpu.x = 0x10;
    cpu.bus.write(0x0010, 100);
    cpu.bus.write(0x0020, 200);
    cpu.bus.write(0x2000, 1);
    cpu.bus.write(0x2010, 200);
    let mut asm = Assembler::new();
    cpu.bus.load(
        cpu.pc,
        asm.org(cpu.pc)
            .dec(Operand::Z(0x10))
            .dec(Operand::ZX(0x10))
            .dec(Operand::Abs(val(0x2000)))
            .dec(Operand::AbsX(val(0x2000)))
            .print_listing()
            .assemble()
            .unwrap(),
    );

    cpu.step(); // DEC 0x10
    println!("{cpu:?}");
    assert_eq!(cpu.pc, 0x1002, "{:#04X} != {:#04X}", cpu.pc, 0x1002);
    assert_eq!(stat(&cpu.sr), "nv-bdizc");
    assert_eq!(cpu.bus.read(0x0010), 99);

    cpu.step(); // DEC 0x10,X where X=10
    println!("{cpu:?}");
    assert_eq!(cpu.pc, 0x1004, "{:#04X} != {:#04X}", cpu.pc, 0x1004);
    assert_eq!(stat(&cpu.sr), "Nv-bdizc");
    assert_eq!(cpu.bus.read(0x0020), 199);

    cpu.step(); // DEC 0x2000
    println!("{cpu:?}");
    assert_eq!(cpu.pc, 0x1007, "{:#04X} != {:#04X}", cpu.pc, 0x1007);
    assert_eq!(stat(&cpu.sr), "nv-bdiZc");
    assert_eq!(cpu.bus.read(0x2000), 0);

    cpu.step(); // DEC 0x2000,X where X=10
    println!("{cpu:?}");
    assert_eq!(cpu.pc, 0x100A, "{:#04X} != {:#04X}", cpu.pc, 0x100A);
    assert_eq!(stat(&cpu.sr), "Nv-bdizc");
    assert_eq!(cpu.bus.read(0x2010), 199);
}

#[test]
fn test_dex_and_dey() {
    let mut cpu = Cpu::new(Bus::default());
    let mut asm = Assembler::new();
    cpu.pc = 0x2000;
    cpu.x = 1;
    cpu.y = 1;
    cpu.bus.load(
        cpu.pc,
        asm.dex()
            .dex()
            .dey()
            .dey()
            .print_listing()
            .assemble()
            .unwrap(),
    );

    cpu.step(); // DEX
    println!("{cpu:?}");
    assert_eq!(cpu.pc, 0x2001, "{:#04X} != {:#04X}", cpu.pc, 0x2001);
    assert_eq!(cpu.x, 0x00);
    assert_eq!(stat(&cpu.sr), "nv-bdiZc");

    cpu.step(); // DEX
    println!("{cpu:?}");
    assert_eq!(cpu.pc, 0x2002, "{:#04X} != {:#04X}", cpu.pc, 0x2002);
    assert_eq!(cpu.x, 0xFF);
    assert_eq!(stat(&cpu.sr), "Nv-bdizc");

    cpu.step(); // DEY
    println!("{cpu:?}");
    assert_eq!(cpu.pc, 0x2003, "{:#04X} != {:#04X}", cpu.pc, 0x2003);
    assert_eq!(cpu.y, 0x00);
    assert_eq!(stat(&cpu.sr), "nv-bdiZc");

    cpu.step(); // DEY
    println!("{cpu:?}");
    assert_eq!(cpu.pc, 0x2004, "{:#04X} != {:#04X}", cpu.pc, 0x2004);
    assert_eq!(cpu.y, 0xFF);
    assert_eq!(stat(&cpu.sr), "Nv-bdizc");
}

#[test]
fn test_inx_and_iny() {
    let mut cpu = Cpu::new(Bus::default());
    let mut asm = Assembler::new();
    cpu.x = 0xFE;
    cpu.y = 0xFE;
    cpu.bus.load(
        cpu.pc,
        asm.inx()
            .inx()
            .iny()
            .iny()
            .print_listing()
            .assemble()
            .unwrap(),
    );

    cpu.step(); // INX
    assert_eq!(cpu.x, 0xFF);
    assert_eq!(stat(&cpu.sr), "Nv-bdizc");

    cpu.step(); // INX
    assert_eq!(cpu.x, 0x00);
    assert_eq!(stat(&cpu.sr), "nv-bdiZc");

    cpu.step(); // INY
    assert_eq!(cpu.y, 0xFF);
    assert_eq!(stat(&cpu.sr), "Nv-bdizc");

    cpu.step(); // INY
    assert_eq!(cpu.y, 0x00);
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

#[test]
fn test_set_and_clear_flags() {
    let mut cpu = Cpu::new(Bus::default());
    let mut asm = Assembler::new();
    cpu.bus.load(
        cpu.pc,
        asm.sec()
            .sed()
            .sei()
            .clc()
            .cld()
            .cli()
            .clv()
            .print_listing()
            .assemble()
            .unwrap(),
    );

    use pda6502v2emu::cpu::StatusMask;
    cpu.set_sr_bit(StatusMask::Overflow, true);

    assert_eq!(stat(&cpu.sr), "nV-bdizc");

    cpu.step(); // SEC
    println!("{cpu:?}");
    assert_eq!(stat(&cpu.sr), "nV-bdizC");
    cpu.step(); // SED
    println!("{cpu:?}");
    assert_eq!(stat(&cpu.sr), "nV-bDizC");
    cpu.step(); // SEI
    println!("{cpu:?}");
    assert_eq!(stat(&cpu.sr), "nV-bDIzC");
    cpu.step(); // CLC
    println!("{cpu:?}");
    assert_eq!(stat(&cpu.sr), "nV-bDIzc");
    cpu.step(); // CLD
    println!("{cpu:?}");
    assert_eq!(stat(&cpu.sr), "nV-bdIzc");
    cpu.step(); // CLI
    println!("{cpu:?}");
    assert_eq!(stat(&cpu.sr), "nV-bdizc");
    cpu.step(); // CLV
    println!("{cpu:?}");
    assert_eq!(stat(&cpu.sr), "nv-bdizc");
}
